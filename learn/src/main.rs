#![feature(impl_trait_in_bindings)]

use std::{
    collections::{BTreeMap, HashMap},
    ops::Range,
    sync::Arc,
    time::SystemTime,
};

use derive_more::{Display, From};
use fern::log_file;
use miette::Diagnostic;
use skim::{Green, ParseError, Parsed, Red};
use thiserror::Error;
use typy::{Rule, TypeID, Typy, Visit};

use crate::parser::{Delim, Token};

pub mod parser;

#[derive(Debug, Clone, PartialEq, Display, From)]
pub enum Expected {
    #[display("'{_0}'")]
    Token(Token),
    #[display("{_0}")]
    Delim(Delim),
    Eof,
    #[display("identifier")]
    Identifier,
    #[display("number")]
    Number,
    #[display("expression")]
    Expression,
    #[display("{_0}")]
    Type(String, Type),
}

#[derive(Debug, Clone, PartialEq, Display, From)]
pub enum Found {
    #[display("'{_0}'")]
    Token(Token),
    Eof,
    #[display("{_0}")]
    Type(String, Type),
}

#[derive(Debug, Clone, Diagnostic, Error)]
pub enum Diag {
    #[error("a lexer error occured here")]
    LexerError {
        #[label]
        span: Range<usize>,
    },
    #[error("expected {expected} found {found}")]
    ExpectedFound {
        expected: Green<Expected>,
        found: Red<Found>,
        #[label]
        span: Range<usize>,
    },
    #[error("did not close delimiter {opener}")]
    DidNotCloseDelimiter {
        opener: Red<Delim>,
        #[label]
        span: Range<usize>,
    },
    #[error("tried to close mismatching delimiter")]
    TriedToCloseMismatchingDelimiteer {
        #[label = "and the matching delimiter {matching_open} was opened here"]
        matching_open_span: Range<usize>,
        matching_open: Delim,
        #[label = "but the current open delimiter is {current_open} here"]
        current_open_span: Range<usize>,
        current_open: Delim,
        #[label = "tried to close here with {close}"]
        close_span: Range<usize>,
        close: Red<Delim>,
    },
    #[error("tried to close unopened delimiter with {delim}")]
    #[diagnostic(help("consider simply removing the closing delimiter"))]
    TriedToCloseUnopenedDelimiter {
        #[label]
        span: Range<usize>,
        delim: Red<Delim>,
    },
    #[error("{main}")]
    Merged {
        #[source]
        #[diagnostic_source]
        main: Box<Diag>,
        #[related]
        others: Vec<Self>,
    },
    #[error("undefined variable {name}")]
    UndefinedVariable {
        name: Red<Arc<str>>,
        #[label = "undefined variable {name}"]
        span: Range<usize>,
    },
}

impl std::borrow::Borrow<dyn miette::Diagnostic> for std::boxed::Box<Diag> {
    fn borrow(&self) -> &(dyn miette::Diagnostic + 'static) {
        &**self
    }
}

#[derive(Debug, Clone, PartialEq, Display)]
#[display("{expr}")]
pub struct Expression {
    expr: Box<Expr>,
    r#type: TypeID<Type>,
    span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum Expr {
    #[display("{_0}")]
    Number(f64),
    #[display("{_0}")]
    Variable(Arc<str>),
    #[display("({_0}.{_1})")]
    Function(Arc<str>, Expression),
    #[display("({_0} {_1})")]
    Application(Expression, Expression),
    #[display("(+ {_0} {_1})")]
    Add(Expression, Expression),
}

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(log_file("log.log")?)
        .apply()?;
    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Number,
    Function(TypeID<Type>, TypeID<Type>),
}

pub struct TypFormat {
    pub id: usize,
    pub names: BTreeMap<TypeID<Type>, usize>,
}

impl TypFormat {
    pub fn format(id: TypeID<Type>, typy: &mut Typy<Type>) -> String {
        TypFormat {
            id: 0,
            names: BTreeMap::new(),
        }
        .fmt(id, false, typy)
    }

    fn fmt(&mut self, id: TypeID<Type>, arg: bool, typy: &mut Typy<Type>) -> String {
        match typy.get(&id) {
            Some(Type::Number) => String::from("number"),
            Some(Type::Function(a, b)) => {
                if arg {
                    format!(
                        "({} -> {})",
                        self.fmt(a, true, typy),
                        self.fmt(b, false, typy)
                    )
                } else {
                    format!(
                        "{} -> {}",
                        self.fmt(a, true, typy),
                        self.fmt(b, false, typy)
                    )
                }
            }
            None => {
                let root = typy.root(&id);
                if let Some(id) = self.names.get(&root) {
                    format!("T{id}")
                } else {
                    self.names.insert(root, self.id);
                    self.id += 1;
                    format!("T{}", self.id - 1)
                }
            }
        }
    }
}

impl Type {
    pub fn to_string(&self, typy: &mut Typy<Self>) -> String {
        let mut typ_fmt = TypFormat {
            id: 0,
            names: BTreeMap::new(),
        };
        match self {
            Type::Number => String::from("number"),
            Type::Function(a, b) => {
                format!(
                    "{} -> {}",
                    typ_fmt.fmt(*a, true, typy),
                    typ_fmt.fmt(*b, false, typy)
                )
            }
        }
    }
}

impl typy::Typed<Type> for Expression {
    type Match = Expr;
    fn get_match(&self) -> &Self::Match {
        &self.expr
    }
    fn r#type(&self) -> TypeID<Type> {
        self.r#type
    }
}

impl Visit for Expression {
    fn visit<C, E>(&self, c: &mut C, f: &impl Fn(&Self, &mut C) -> Result<(), E>) -> Result<(), E> {
        f(self, c)?;
        match self.expr.as_ref() {
            Expr::Add(a, b) => {
                a.visit(c, f)?;
                b.visit(c, f)?;
            }
            Expr::Function(_, a) => a.visit(c, f)?,
            Expr::Application(a, b) => {
                a.visit(c, f)?;
                b.visit(c, f)?;
            }
            _ => {}
        }
        Ok(())
    }
}

impl typy::Type for Type {
    type Error = Diag;
    type Reason = Range<usize>;

    fn unify(
        &self,
        other: &Self,
        span: &Self::Reason,
        typy: &mut Typy<Self>,
    ) -> Result<Self, Self::Error> {
        match (self, other) {
            (Type::Number, Type::Number) => Ok(Type::Number),
            (a @ Type::Function(_, _), b @ Type::Number) => Err(Diag::ExpectedFound {
                expected: Green(Expected::Type(a.to_string(typy), a.clone())),
                found: Red(Found::Type(b.to_string(typy), b.clone())),
                span: span.clone(),
            }),
            (a @ Type::Number, b @ Type::Function(_, _)) => Err(Diag::ExpectedFound {
                expected: Green(Expected::Type(a.to_string(typy), a.clone())),
                found: Red(Found::Type(b.to_string(typy), b.clone())),
                span: span.clone(),
            }),
            (Type::Function(a, b), Type::Function(c, d)) => {
                typy.eq(span.clone(), a, c)?;
                typy.eq(span.clone(), b, d)?;
                Ok(Type::Function(*a, *b))
            }
        }
    }
}

pub struct Namespace<'a> {
    pub parent: Option<&'a Namespace<'a>>,
    pub variables: BTreeMap<Arc<str>, TypeID<Type>>,
}

impl<'a> Namespace<'a> {
    pub fn new(parent: Option<&'a Namespace<'a>>) -> Self {
        Self {
            parent,
            variables: BTreeMap::new(),
        }
    }

    pub fn get(&self, s: &Arc<str>) -> Option<TypeID<Type>> {
        self.variables
            .get(s)
            .cloned()
            .or_else(|| self.parent.and_then(|p| p.get(s)))
    }
}

fn variables(expression: &Expression, typy: &mut Typy<Type>) -> Result<(), Diag> {
    let parent = Namespace {
        parent: None,
        variables: BTreeMap::new(),
    };
    fn visit(
        expression: &Expression,
        typy: &mut Typy<Type>,
        mut namespace: Namespace<'_>,
    ) -> Result<(), Diag> {
        match expression.expr.as_ref() {
            Expr::Number(_) => Ok(()),
            Expr::Add(a, b) => {
                visit(a, typy, Namespace::new(Some(&namespace)))?;
                visit(b, typy, Namespace::new(Some(&namespace)))?;
                Ok(())
            }
            Expr::Application(a, b) => {
                visit(a, typy, Namespace::new(Some(&namespace)))?;
                visit(b, typy, Namespace::new(Some(&namespace)))?;
                Ok(())
            }
            Expr::Function(ident, body) => {
                let var = typy.unknown();
                namespace.variables.insert(Arc::clone(ident), var);
                typy.set(
                    expression.span.clone(),
                    expression,
                    Type::Function(var, body.r#type),
                )?;
                visit(body, typy, namespace)
            }
            Expr::Variable(x) => {
                if let Some(type_id) = namespace.get(x) {
                    typy.eq(expression.span.clone(), &type_id, expression)
                        .map(|_| ())
                } else {
                    Err(Diag::UndefinedVariable {
                        name: Red(Arc::clone(x)),
                        span: expression.span.clone(),
                    })
                }
            }
        }
    }
    visit(expression, typy, parent)
}

fn type_chk(expression: &Expression, typy: &mut Typy<Type>) -> Result<(), Diag> {
    variables(expression, typy)?;
    let add = typy::rule!(|ctx| Expr::Add(a, b) => |this: &Expression|
        ctx.eq(b.span.clone(), a, b)?.eq(a.span.clone(), a, this)
    );
    let function = typy::rule!(|ctx| Expr::Function(_, body) => |this: &Expression| {
        if matches!(ctx.get(this), Some(Type::Function(_, _))) {
            return Ok(());
        }
        let input = ctx.unknown();
        ctx.set(this.span.clone(), this, Type::Function(input, body.r#type))
    });
    let application = typy::rule!(|ctx| Expr::Application(a, b) => |this: &Expression| {
        ctx.set(this.span.clone(), a, Type::Function(b.r#type, this.r#type))
    });
    typy.apply(function.combine(add).combine(application), expression)?;
    println!(
        "{expression} : {}",
        TypFormat::format(expression.r#type, typy)
    );
    Ok(())
}

fn main() {
    _ = std::fs::remove_file("log.log");
    setup_logger().unwrap();
    let code = r#"(x.x+(x.x)) 2"#;
    let out: Parsed<_, Diag> = parser::parse(code, Typy::new());

    let result = match out {
        Parsed::Ok((value, mut typy)) => type_chk(&value, &mut typy),
        Parsed::Err(err) => Err(err),
        Parsed::Fatal(err) => Err(err),
        Parsed::Recover(err, (value, mut typy)) => match type_chk(&value, &mut typy) {
            Ok(()) => Err(err),
            Err(err2) => Err(Diag::merge(err, err2)),
        },
    };
    match result {
        Ok(()) => {}
        Err(e) => println!("{:?}", miette::Report::from(e).with_source_code(code)),
    }
}
