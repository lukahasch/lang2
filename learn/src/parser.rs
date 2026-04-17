use std::{ops::Range, sync::Arc};

use derive_more::Display;
use logos::{Lexer, Logos, SpannedIter};
use skim::{
    Context, Delimiter, Green, ParseError, Parsed, Parser, Red, Spanned, close_delimiter, eof,
    just, open_delimiter, select, select_ctx,
};
use typy::Typy;

use crate::{Diag, Expected, Expr, Expression, Found, Type};

#[derive(Debug, Clone, PartialEq, Logos, Display)]
#[logos(skip("( |\n)"))]
pub enum Token {
    #[regex("[a-zA-Z]+", string_to_arc)]
    #[display("{_0}")]
    Identifier(Arc<str>),
    #[regex("[0-9]+", string_to_arc)]
    #[display("{_0}")]
    Number(Arc<str>),
    #[token(".")]
    #[display(".")]
    Dot,
    #[token("\\")]
    #[display("\\")]
    Backslash,
    #[token("+")]
    #[display("+")]
    Plus,
    #[token("(")]
    #[display("(")]
    OpenParenthesis,
    #[token(")")]
    #[display(")")]
    CloseParenthesis,
    #[token("[")]
    #[display("[")]
    OpenBracket,
    #[token("]")]
    #[display("[")]
    CloseBracket,
    #[token("let")]
    #[display("let")]
    Let,
    #[token("=")]
    #[display("=")]
    Equal,
    #[token("in")]
    #[display("in")]
    In,
}

fn string_to_arc(lex: &mut Lexer<Token>) -> Arc<str> {
    Arc::from(lex.slice())
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum Delim {
    #[display("opening parenthesis '('")]
    OpenParenthesis,
    #[display("closing parenthesis ')'")]
    CloseParenthesis,
    #[display("opening bracket '['")]
    OpenBracket,
    #[display("closing bracket ']'")]
    CloseBracket,
}

impl Delimiter for Delim {
    fn opposite(&self) -> Self {
        match self {
            Delim::CloseParenthesis => Delim::OpenParenthesis,
            Delim::OpenParenthesis => Delim::CloseParenthesis,
            Delim::CloseBracket => Delim::OpenBracket,
            Delim::OpenBracket => Delim::CloseBracket,
        }
    }
}

impl From<Delim> for Token {
    fn from(value: Delim) -> Self {
        match value {
            Delim::CloseParenthesis => Token::CloseParenthesis,
            Delim::OpenParenthesis => Token::OpenParenthesis,
            Delim::CloseBracket => Token::CloseBracket,
            Delim::OpenBracket => Token::OpenBracket,
        }
    }
}

impl TryFrom<Token> for Delim {
    type Error = ();

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        Ok(match value {
            Token::CloseBracket => Delim::CloseBracket,
            Token::CloseParenthesis => Delim::CloseParenthesis,
            Token::OpenBracket => Delim::OpenBracket,
            Token::OpenParenthesis => Delim::OpenParenthesis,
            _ => return Err(()),
        })
    }
}

impl ParseError<Token, (), Range<usize>, Delim, Typy<Type>> for Diag {
    type Expected = Expected;

    fn lexer_error(_: (), span: Range<usize>, _: &Typy<Type>) -> Self {
        Diag::LexerError { span }
    }
    fn expected_found(
        expected: Self::Expected,
        found: Token,
        span: Range<usize>,
        _: &Typy<Type>,
    ) -> Self {
        Diag::ExpectedFound {
            expected: Green(expected),
            found: Red(Found::Token(found)),
            span,
        }
    }
    fn expected_found_eof(expected: Self::Expected, span: Range<usize>, _: &Typy<Type>) -> Self {
        Diag::ExpectedFound {
            expected: Green(expected),
            found: Red(Found::Eof),
            span: span.start..span.start,
        }
    }
    fn expected_eof_found(found: Token, span: Range<usize>, _: &Typy<Type>) -> Self {
        Diag::ExpectedFound {
            expected: Green(Expected::Eof),
            found: Red(Found::Token(found)),
            span,
        }
    }
    fn tried_to_close_unopened_delimiter(
        closer: Delim,
        span: Range<usize>,
        _: &Typy<Type>,
    ) -> Self {
        Diag::TriedToCloseUnopenedDelimiter {
            span,
            delim: Red(closer),
        }
    }
    fn tried_to_close_mismatching_delimiter(
        matching_open: (Range<usize>, Delim),
        current_open: (Range<usize>, Delim),
        close: (Range<usize>, Delim),
        _: &Typy<Type>,
    ) -> Self {
        Diag::TriedToCloseMismatchingDelimiteer {
            matching_open_span: matching_open.0,
            matching_open: matching_open.1,
            current_open_span: current_open.0,
            current_open: current_open.1,
            close_span: close.0,
            close: Red(close.1),
        }
    }
    fn did_not_close_delimiter(
        opener: Delim,
        span: Range<usize>,
        _: Range<usize>,
        _: &Typy<Type>,
    ) -> Self {
        Diag::DidNotCloseDelimiter {
            opener: Red(opener),
            span,
        }
    }

    fn expected(self, expected: Self::Expected) -> Self {
        match self {
            Diag::ExpectedFound {
                expected: _,
                found,
                span,
            } => Diag::ExpectedFound {
                expected: Green(expected),
                found,
                span,
            },
            Diag::Merged { main, others } => Diag::Merged {
                main: Box::new(main.expected(expected)),
                others,
            },
            i => i,
        }
    }
    fn merge(a: Self, b: Self) -> Self {
        Self::Merged {
            main: Box::new(a),
            others: vec![b],
        }
    }
}

pub type PCtx<'a> =
    skim::Context<SpannedIter<'a, Token>, Token, (), Range<usize>, Delim, Typy<Type>>;

fn identifier(ctx: &mut PCtx<'_>) -> Parsed<Arc<str>, Diag> {
    select(Expected::Identifier, |token| match token {
        Token::Identifier(ident) => Some(ident),
        _ => None,
    })
    .parse(ctx)
}

fn number(ctx: &mut PCtx<'_>) -> Parsed<Expression, Diag> {
    select_ctx(Expected::Number, |token, ctx: &mut PCtx| match token {
        Token::Number(number) => Some(Expression {
            expr: Box::new(Expr::Number(number.parse().unwrap())),
            r#type: ctx.ctx.known(Type::Number),
            span: ctx.span(),
        }),
        _ => None,
    })
    .parse(ctx)
}

fn variable(ctx: &mut PCtx) -> Parsed<Expression, Diag> {
    identifier
        .spanned()
        .map_ctx(|Spanned(ident, span), ctx| Expression {
            expr: Box::new(Expr::Variable(ident)),
            r#type: ctx.ctx.unknown(),
            span,
        })
        .parse(ctx)
}

fn function(ctx: &mut PCtx) -> Parsed<Expression, Diag> {
    identifier
        .and_ignore(just(Token::Dot))
        .cut()
        .and(expr)
        .spanned()
        .map_ctx(|Spanned((ident, expr), span), ctx| Expression {
            expr: Box::new(Expr::Function(ident, expr)),
            r#type: ctx.ctx.unknown(),
            span,
        })
        .parse(ctx)
}

fn r#let(ctx: &mut PCtx) -> Parsed<Expression, Diag> {
    just(Token::Let)
        .cut()
        .ignore_and(identifier)
        .and_ignore(just(Token::Equal))
        .and(expr)
        .and_ignore(just(Token::In))
        .and(expr)
        .spanned()
        .map_ctx(|Spanned(((ident, val), expr), span), ctx| Expression {
            expr: Box::new(Expr::Application(
                Expression {
                    expr: Box::new(Expr::Function(ident, expr)),
                    r#type: ctx.ctx.unknown(),
                    span: val.span.clone(),
                },
                val,
            )),
            r#type: ctx.ctx.unknown(),
            span,
        })
        .parse(ctx)
}

fn atom(ctx: &mut PCtx) -> Parsed<Expression, Diag> {
    function
        .or(r#let)
        .or(variable)
        .or(number)
        .or(open_delimiter(Delim::OpenParenthesis)
            .cut()
            .ignore_and(expr)
            .and_ignore(close_delimiter(Delim::CloseParenthesis)))
        .parse(ctx)
}

fn application(ctx: &mut PCtx) -> Parsed<Expression, Diag> {
    atom.repeated()
        .map_ctx(|atoms, ctx| {
            atoms
                .into_iter()
                .reduce(|a, b| Expression {
                    span: a.span.start..b.span.end,
                    expr: Box::new(Expr::Application(a, b)),
                    r#type: ctx.ctx.unknown(),
                })
                .expect("atoms.len() > 0")
        })
        .parse(ctx)
}

fn expr(ctx: &mut PCtx) -> Parsed<Expression, Diag> {
    application
        .and_ignore(just(Token::Plus))
        .cut()
        .and(expr)
        .spanned()
        .map_ctx(|Spanned((a, b), span), ctx| Expression {
            expr: Box::new(Expr::Add(a, b)),
            r#type: ctx.ctx.unknown(),
            span,
        })
        .or(application)
        .expect(Expected::Expression)
        .parse(ctx)
}

pub fn parse(code: &str, typy: Typy<Type>) -> Parsed<(Expression, Typy<Type>), Diag> {
    let lexer = Token::lexer(code).spanned();
    let mut ctx = Context::new(lexer, 0..0, typy);
    expr.and_ignore(eof)
        .parse(&mut ctx)
        .map(|value| (value, ctx.take_ctx()))
}
