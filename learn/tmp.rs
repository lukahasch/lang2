#![feature(prelude_import)]
#![feature(impl_trait_in_bindings)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use std::{ops::Range, sync::Arc, time::SystemTime};
use derive_more::{Display, From};
use fern::log_file;
use miette::Diagnostic;
use skim::{Green, ParseError, Parsed, Red};
use thiserror::Error;
use typy::{TypeID, Typy, Visit};
use crate::parser::{Delim, Token};
pub mod parser {
    use std::{ops::Range, sync::Arc};
    use derive_more::Display;
    use logos::{Lexer, Logos, SpannedIter};
    use skim::{
        Context, Delimiter, Green, ParseError, Parsed, Parser, Red, Spanned,
        close_delimiter, eof, just, open_delimiter, select, select_ctx,
    };
    use typy::Typy;
    use crate::{Diag, Expected, Expr, Expression, Found, Type};
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
    #[automatically_derived]
    impl ::core::fmt::Debug for Token {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Token::Identifier(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Identifier",
                        &__self_0,
                    )
                }
                Token::Number(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Number",
                        &__self_0,
                    )
                }
                Token::Dot => ::core::fmt::Formatter::write_str(f, "Dot"),
                Token::Backslash => ::core::fmt::Formatter::write_str(f, "Backslash"),
                Token::Plus => ::core::fmt::Formatter::write_str(f, "Plus"),
                Token::OpenParenthesis => {
                    ::core::fmt::Formatter::write_str(f, "OpenParenthesis")
                }
                Token::CloseParenthesis => {
                    ::core::fmt::Formatter::write_str(f, "CloseParenthesis")
                }
                Token::OpenBracket => ::core::fmt::Formatter::write_str(f, "OpenBracket"),
                Token::CloseBracket => {
                    ::core::fmt::Formatter::write_str(f, "CloseBracket")
                }
                Token::Let => ::core::fmt::Formatter::write_str(f, "Let"),
                Token::Equal => ::core::fmt::Formatter::write_str(f, "Equal"),
                Token::In => ::core::fmt::Formatter::write_str(f, "In"),
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Token {
        #[inline]
        fn clone(&self) -> Token {
            match self {
                Token::Identifier(__self_0) => {
                    Token::Identifier(::core::clone::Clone::clone(__self_0))
                }
                Token::Number(__self_0) => {
                    Token::Number(::core::clone::Clone::clone(__self_0))
                }
                Token::Dot => Token::Dot,
                Token::Backslash => Token::Backslash,
                Token::Plus => Token::Plus,
                Token::OpenParenthesis => Token::OpenParenthesis,
                Token::CloseParenthesis => Token::CloseParenthesis,
                Token::OpenBracket => Token::OpenBracket,
                Token::CloseBracket => Token::CloseBracket,
                Token::Let => Token::Let,
                Token::Equal => Token::Equal,
                Token::In => Token::In,
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Token {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Token {
        #[inline]
        fn eq(&self, other: &Token) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
                && match (self, other) {
                    (Token::Identifier(__self_0), Token::Identifier(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    (Token::Number(__self_0), Token::Number(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    _ => true,
                }
        }
    }
    impl<'s> ::logos::Logos<'s> for Token {
        type Error = ();
        type Extras = ();
        type Source = str;
        fn lex(
            lex: &mut ::logos::Lexer<'s, Self>,
        ) -> core::option::Option<
            core::result::Result<Self, <Self as ::logos::Logos<'s>>::Error>,
        > {
            use ::logos::internal::{
                LexerInternal, CallbackRetVal, CallbackResult, SkipRetVal, SkipResult,
            };
            use core::result::Result as _Result;
            use core::option::Option as _Option;
            use ::logos::Logos;
            type _Lexer<'s> = ::logos::Lexer<'s, Token>;
            const _TABLE_0: [u8; 256] = [
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8,
                4u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 27u8, 27u8, 27u8, 27u8,
                27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8,
                27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 27u8, 27u8, 27u8, 27u8, 11u8, 27u8, 27u8, 27u8, 27u8,
                27u8, 27u8, 27u8, 27u8, 19u8, 27u8, 27u8, 27u8, 27u8, 27u8, 25u8, 27u8,
                27u8, 27u8, 27u8, 27u8, 27u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            ];
            #[inline]
            fn _make_error<'s>(lex: &mut _Lexer<'s>) -> <Token as Logos<'s>>::Error {
                <Token as Logos<'s>>::Error::default()
            }
            #[inline]
            fn _get_action<'s>(
                lex: &mut _Lexer<'s>,
                offset: usize,
                context: _Option<LogosLeaf>,
            ) -> CallbackResult<'s, Token> {
                match context {
                    None => {
                        lex.end_to_boundary(offset.max(lex.offset() + 1));
                        CallbackResult::Error(_make_error(lex))
                    }
                    Some(LogosLeaf::Leaf0) => CallbackResult::Skip,
                    Some(LogosLeaf::Leaf1) => {
                        let cb_result = string_to_arc(lex);
                        CallbackRetVal::<
                            's,
                            Arc<str>,
                            Token,
                        >::construct(cb_result, Token::Identifier)
                    }
                    Some(LogosLeaf::Leaf2) => {
                        let cb_result = string_to_arc(lex);
                        CallbackRetVal::<
                            's,
                            Arc<str>,
                            Token,
                        >::construct(cb_result, Token::Number)
                    }
                    Some(LogosLeaf::Leaf3) => CallbackResult::Emit(Token::Dot),
                    Some(LogosLeaf::Leaf4) => CallbackResult::Emit(Token::Backslash),
                    Some(LogosLeaf::Leaf5) => CallbackResult::Emit(Token::Plus),
                    Some(LogosLeaf::Leaf6) => {
                        CallbackResult::Emit(Token::OpenParenthesis)
                    }
                    Some(LogosLeaf::Leaf7) => {
                        CallbackResult::Emit(Token::CloseParenthesis)
                    }
                    Some(LogosLeaf::Leaf8) => CallbackResult::Emit(Token::OpenBracket),
                    Some(LogosLeaf::Leaf9) => CallbackResult::Emit(Token::CloseBracket),
                    Some(LogosLeaf::Leaf10) => CallbackResult::Emit(Token::Let),
                    Some(LogosLeaf::Leaf11) => CallbackResult::Emit(Token::Equal),
                    Some(LogosLeaf::Leaf12) => CallbackResult::Emit(Token::In),
                }
            }
            enum LogosLeaf {
                Leaf0 = 0isize,
                Leaf1 = 1isize,
                Leaf2 = 2isize,
                Leaf3 = 3isize,
                Leaf4 = 4isize,
                Leaf5 = 5isize,
                Leaf6 = 6isize,
                Leaf7 = 7isize,
                Leaf8 = 8isize,
                Leaf9 = 9isize,
                Leaf10 = 10isize,
                Leaf11 = 11isize,
                Leaf12 = 12isize,
            }
            #[automatically_derived]
            #[doc(hidden)]
            unsafe impl ::core::clone::TrivialClone for LogosLeaf {}
            #[automatically_derived]
            impl ::core::clone::Clone for LogosLeaf {
                #[inline]
                fn clone(&self) -> LogosLeaf {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for LogosLeaf {}
            enum LogosState {
                State0,
                State1,
                State10,
                State11,
                State12,
                State13,
                State14,
                State15,
                State16,
                State2,
                State3,
                State4,
                State5,
                State6,
                State7,
                State8,
                State9,
            }
            #[automatically_derived]
            #[doc(hidden)]
            unsafe impl ::core::clone::TrivialClone for LogosState {}
            #[automatically_derived]
            impl ::core::clone::Clone for LogosState {
                #[inline]
                fn clone(&self) -> LogosState {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for LogosState {}
            let mut state = LogosState::State5;
            let mut offset = lex.offset();
            let mut context: _Option<LogosLeaf> = None;
            loop {
                match state {
                    LogosState::State0 => {
                        #[inline]
                        fn loop_test(byte: u8) -> bool {
                            _TABLE_0[byte as usize] & 1u8 == 0
                        }
                        'fast_loop: {
                            while let Some(arr) = lex.read::<&[u8; 8usize]>(offset) {
                                if loop_test(arr[0usize]) {
                                    offset += 0usize;
                                    break 'fast_loop;
                                }
                                if loop_test(arr[1usize]) {
                                    offset += 1usize;
                                    break 'fast_loop;
                                }
                                if loop_test(arr[2usize]) {
                                    offset += 2usize;
                                    break 'fast_loop;
                                }
                                if loop_test(arr[3usize]) {
                                    offset += 3usize;
                                    break 'fast_loop;
                                }
                                if loop_test(arr[4usize]) {
                                    offset += 4usize;
                                    break 'fast_loop;
                                }
                                if loop_test(arr[5usize]) {
                                    offset += 5usize;
                                    break 'fast_loop;
                                }
                                if loop_test(arr[6usize]) {
                                    offset += 6usize;
                                    break 'fast_loop;
                                }
                                if loop_test(arr[7usize]) {
                                    offset += 7usize;
                                    break 'fast_loop;
                                }
                                offset += 8usize;
                            }
                            while let Some(byte) = lex.read::<u8>(offset) {
                                if loop_test(byte) {
                                    break 'fast_loop;
                                }
                                offset += 1;
                            }
                        };
                        lex.end(offset);
                        context = Some(LogosLeaf::Leaf1);
                        let other = lex.read::<u8>(offset);
                        if let Some(byte) = other {} else {}
                        {
                            let action = _get_action(lex, offset, context);
                            match action {
                                CallbackResult::Emit(tok) => {
                                    return Some(Ok(tok));
                                }
                                CallbackResult::Skip => {
                                    lex.trivia();
                                    offset = lex.offset();
                                    context = None;
                                    state = LogosState::State5;
                                    continue;
                                }
                                CallbackResult::Error(err) => {
                                    return Some(Err(err));
                                }
                                CallbackResult::DefaultError => {
                                    return Some(Err(_make_error(lex)));
                                }
                            }
                        }
                    }
                    LogosState::State1 => {
                        lex.end(offset);
                        context = Some(LogosLeaf::Leaf1);
                        let other = lex.read::<u8>(offset);
                        if let Some(byte) = other {
                            if _TABLE_0[byte as usize] & 2u8 != 0 {
                                offset += 1;
                                state = LogosState::State0;
                                continue;
                            }
                            if (byte == b't') {
                                offset += 1;
                                state = LogosState::State2;
                                continue;
                            }
                        } else {}
                        {
                            let action = _get_action(lex, offset, context);
                            match action {
                                CallbackResult::Emit(tok) => {
                                    return Some(Ok(tok));
                                }
                                CallbackResult::Skip => {
                                    lex.trivia();
                                    offset = lex.offset();
                                    context = None;
                                    state = LogosState::State5;
                                    continue;
                                }
                                CallbackResult::Error(err) => {
                                    return Some(Err(err));
                                }
                                CallbackResult::DefaultError => {
                                    return Some(Err(_make_error(lex)));
                                }
                            }
                        }
                    }
                    LogosState::State2 => {
                        lex.end(offset);
                        context = Some(LogosLeaf::Leaf10);
                        let other = lex.read::<u8>(offset);
                        if let Some(byte) = other {
                            if _TABLE_0[byte as usize] & 1u8 != 0 {
                                offset += 1;
                                state = LogosState::State0;
                                continue;
                            }
                        } else {}
                        {
                            let action = _get_action(lex, offset, context);
                            match action {
                                CallbackResult::Emit(tok) => {
                                    return Some(Ok(tok));
                                }
                                CallbackResult::Skip => {
                                    lex.trivia();
                                    offset = lex.offset();
                                    context = None;
                                    state = LogosState::State5;
                                    continue;
                                }
                                CallbackResult::Error(err) => {
                                    return Some(Err(err));
                                }
                                CallbackResult::DefaultError => {
                                    return Some(Err(_make_error(lex)));
                                }
                            }
                        }
                    }
                    LogosState::State3 => {
                        lex.end(offset);
                        context = Some(LogosLeaf::Leaf12);
                        let other = lex.read::<u8>(offset);
                        if let Some(byte) = other {
                            if _TABLE_0[byte as usize] & 1u8 != 0 {
                                offset += 1;
                                state = LogosState::State0;
                                continue;
                            }
                        } else {}
                        {
                            let action = _get_action(lex, offset, context);
                            match action {
                                CallbackResult::Emit(tok) => {
                                    return Some(Ok(tok));
                                }
                                CallbackResult::Skip => {
                                    lex.trivia();
                                    offset = lex.offset();
                                    context = None;
                                    state = LogosState::State5;
                                    continue;
                                }
                                CallbackResult::Error(err) => {
                                    return Some(Err(err));
                                }
                                CallbackResult::DefaultError => {
                                    return Some(Err(_make_error(lex)));
                                }
                            }
                        }
                    }
                    LogosState::State4 => {
                        #[inline]
                        fn loop_test(byte: u8) -> bool {
                            _TABLE_0[byte as usize] & 4u8 == 0
                        }
                        'fast_loop: {
                            while let Some(arr) = lex.read::<&[u8; 8usize]>(offset) {
                                if loop_test(arr[0usize]) {
                                    offset += 0usize;
                                    break 'fast_loop;
                                }
                                if loop_test(arr[1usize]) {
                                    offset += 1usize;
                                    break 'fast_loop;
                                }
                                if loop_test(arr[2usize]) {
                                    offset += 2usize;
                                    break 'fast_loop;
                                }
                                if loop_test(arr[3usize]) {
                                    offset += 3usize;
                                    break 'fast_loop;
                                }
                                if loop_test(arr[4usize]) {
                                    offset += 4usize;
                                    break 'fast_loop;
                                }
                                if loop_test(arr[5usize]) {
                                    offset += 5usize;
                                    break 'fast_loop;
                                }
                                if loop_test(arr[6usize]) {
                                    offset += 6usize;
                                    break 'fast_loop;
                                }
                                if loop_test(arr[7usize]) {
                                    offset += 7usize;
                                    break 'fast_loop;
                                }
                                offset += 8usize;
                            }
                            while let Some(byte) = lex.read::<u8>(offset) {
                                if loop_test(byte) {
                                    break 'fast_loop;
                                }
                                offset += 1;
                            }
                        };
                        lex.end(offset);
                        context = Some(LogosLeaf::Leaf2);
                        let other = lex.read::<u8>(offset);
                        if let Some(byte) = other {} else {}
                        {
                            let action = _get_action(lex, offset, context);
                            match action {
                                CallbackResult::Emit(tok) => {
                                    return Some(Ok(tok));
                                }
                                CallbackResult::Skip => {
                                    lex.trivia();
                                    offset = lex.offset();
                                    context = None;
                                    state = LogosState::State5;
                                    continue;
                                }
                                CallbackResult::Error(err) => {
                                    return Some(Err(err));
                                }
                                CallbackResult::DefaultError => {
                                    return Some(Err(_make_error(lex)));
                                }
                            }
                        }
                    }
                    LogosState::State5 => {
                        let other = lex.read::<u8>(offset);
                        if let Some(byte) = other {
                            const TABLE: [_Option<LogosState>; 256] = [
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                Some(LogosState::State12),
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                Some(LogosState::State12),
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                Some(LogosState::State13),
                                Some(LogosState::State14),
                                None,
                                Some(LogosState::State15),
                                None,
                                None,
                                Some(LogosState::State16),
                                None,
                                Some(LogosState::State4),
                                Some(LogosState::State4),
                                Some(LogosState::State4),
                                Some(LogosState::State4),
                                Some(LogosState::State4),
                                Some(LogosState::State4),
                                Some(LogosState::State4),
                                Some(LogosState::State4),
                                Some(LogosState::State4),
                                Some(LogosState::State4),
                                None,
                                None,
                                None,
                                Some(LogosState::State6),
                                None,
                                None,
                                None,
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State7),
                                Some(LogosState::State8),
                                Some(LogosState::State9),
                                None,
                                None,
                                None,
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State10),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State11),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                Some(LogosState::State0),
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                            ];
                            let next_state = TABLE[byte as usize];
                            if let Some(next_state) = next_state {
                                offset += 1;
                                state = next_state;
                                continue;
                            }
                        } else {
                            if lex.offset() == offset {
                                return None;
                            }
                        }
                        {
                            let action = _get_action(lex, offset, context);
                            match action {
                                CallbackResult::Emit(tok) => {
                                    return Some(Ok(tok));
                                }
                                CallbackResult::Skip => {
                                    lex.trivia();
                                    offset = lex.offset();
                                    context = None;
                                    state = LogosState::State5;
                                    continue;
                                }
                                CallbackResult::Error(err) => {
                                    return Some(Err(err));
                                }
                                CallbackResult::DefaultError => {
                                    return Some(Err(_make_error(lex)));
                                }
                            }
                        }
                    }
                    LogosState::State6 => {
                        lex.end(offset);
                        context = Some(LogosLeaf::Leaf11);
                        let other = lex.read::<u8>(offset);
                        if let Some(byte) = other {} else {}
                        {
                            let action = _get_action(lex, offset, context);
                            match action {
                                CallbackResult::Emit(tok) => {
                                    return Some(Ok(tok));
                                }
                                CallbackResult::Skip => {
                                    lex.trivia();
                                    offset = lex.offset();
                                    context = None;
                                    state = LogosState::State5;
                                    continue;
                                }
                                CallbackResult::Error(err) => {
                                    return Some(Err(err));
                                }
                                CallbackResult::DefaultError => {
                                    return Some(Err(_make_error(lex)));
                                }
                            }
                        }
                    }
                    LogosState::State7 => {
                        lex.end(offset);
                        context = Some(LogosLeaf::Leaf8);
                        let other = lex.read::<u8>(offset);
                        if let Some(byte) = other {} else {}
                        {
                            let action = _get_action(lex, offset, context);
                            match action {
                                CallbackResult::Emit(tok) => {
                                    return Some(Ok(tok));
                                }
                                CallbackResult::Skip => {
                                    lex.trivia();
                                    offset = lex.offset();
                                    context = None;
                                    state = LogosState::State5;
                                    continue;
                                }
                                CallbackResult::Error(err) => {
                                    return Some(Err(err));
                                }
                                CallbackResult::DefaultError => {
                                    return Some(Err(_make_error(lex)));
                                }
                            }
                        }
                    }
                    LogosState::State8 => {
                        lex.end(offset);
                        context = Some(LogosLeaf::Leaf4);
                        let other = lex.read::<u8>(offset);
                        if let Some(byte) = other {} else {}
                        {
                            let action = _get_action(lex, offset, context);
                            match action {
                                CallbackResult::Emit(tok) => {
                                    return Some(Ok(tok));
                                }
                                CallbackResult::Skip => {
                                    lex.trivia();
                                    offset = lex.offset();
                                    context = None;
                                    state = LogosState::State5;
                                    continue;
                                }
                                CallbackResult::Error(err) => {
                                    return Some(Err(err));
                                }
                                CallbackResult::DefaultError => {
                                    return Some(Err(_make_error(lex)));
                                }
                            }
                        }
                    }
                    LogosState::State9 => {
                        lex.end(offset);
                        context = Some(LogosLeaf::Leaf9);
                        let other = lex.read::<u8>(offset);
                        if let Some(byte) = other {} else {}
                        {
                            let action = _get_action(lex, offset, context);
                            match action {
                                CallbackResult::Emit(tok) => {
                                    return Some(Ok(tok));
                                }
                                CallbackResult::Skip => {
                                    lex.trivia();
                                    offset = lex.offset();
                                    context = None;
                                    state = LogosState::State5;
                                    continue;
                                }
                                CallbackResult::Error(err) => {
                                    return Some(Err(err));
                                }
                                CallbackResult::DefaultError => {
                                    return Some(Err(_make_error(lex)));
                                }
                            }
                        }
                    }
                    LogosState::State10 => {
                        lex.end(offset);
                        context = Some(LogosLeaf::Leaf1);
                        let other = lex.read::<u8>(offset);
                        if let Some(byte) = other {
                            if _TABLE_0[byte as usize] & 8u8 != 0 {
                                offset += 1;
                                state = LogosState::State0;
                                continue;
                            }
                            if (byte == b'n') {
                                offset += 1;
                                state = LogosState::State3;
                                continue;
                            }
                        } else {}
                        {
                            let action = _get_action(lex, offset, context);
                            match action {
                                CallbackResult::Emit(tok) => {
                                    return Some(Ok(tok));
                                }
                                CallbackResult::Skip => {
                                    lex.trivia();
                                    offset = lex.offset();
                                    context = None;
                                    state = LogosState::State5;
                                    continue;
                                }
                                CallbackResult::Error(err) => {
                                    return Some(Err(err));
                                }
                                CallbackResult::DefaultError => {
                                    return Some(Err(_make_error(lex)));
                                }
                            }
                        }
                    }
                    LogosState::State11 => {
                        lex.end(offset);
                        context = Some(LogosLeaf::Leaf1);
                        let other = lex.read::<u8>(offset);
                        if let Some(byte) = other {
                            if _TABLE_0[byte as usize] & 16u8 != 0 {
                                offset += 1;
                                state = LogosState::State0;
                                continue;
                            }
                            if (byte == b'e') {
                                offset += 1;
                                state = LogosState::State1;
                                continue;
                            }
                        } else {}
                        {
                            let action = _get_action(lex, offset, context);
                            match action {
                                CallbackResult::Emit(tok) => {
                                    return Some(Ok(tok));
                                }
                                CallbackResult::Skip => {
                                    lex.trivia();
                                    offset = lex.offset();
                                    context = None;
                                    state = LogosState::State5;
                                    continue;
                                }
                                CallbackResult::Error(err) => {
                                    return Some(Err(err));
                                }
                                CallbackResult::DefaultError => {
                                    return Some(Err(_make_error(lex)));
                                }
                            }
                        }
                    }
                    LogosState::State12 => {
                        lex.end(offset);
                        context = Some(LogosLeaf::Leaf0);
                        let other = lex.read::<u8>(offset);
                        if let Some(byte) = other {} else {}
                        {
                            let action = _get_action(lex, offset, context);
                            match action {
                                CallbackResult::Emit(tok) => {
                                    return Some(Ok(tok));
                                }
                                CallbackResult::Skip => {
                                    lex.trivia();
                                    offset = lex.offset();
                                    context = None;
                                    state = LogosState::State5;
                                    continue;
                                }
                                CallbackResult::Error(err) => {
                                    return Some(Err(err));
                                }
                                CallbackResult::DefaultError => {
                                    return Some(Err(_make_error(lex)));
                                }
                            }
                        }
                    }
                    LogosState::State13 => {
                        lex.end(offset);
                        context = Some(LogosLeaf::Leaf6);
                        let other = lex.read::<u8>(offset);
                        if let Some(byte) = other {} else {}
                        {
                            let action = _get_action(lex, offset, context);
                            match action {
                                CallbackResult::Emit(tok) => {
                                    return Some(Ok(tok));
                                }
                                CallbackResult::Skip => {
                                    lex.trivia();
                                    offset = lex.offset();
                                    context = None;
                                    state = LogosState::State5;
                                    continue;
                                }
                                CallbackResult::Error(err) => {
                                    return Some(Err(err));
                                }
                                CallbackResult::DefaultError => {
                                    return Some(Err(_make_error(lex)));
                                }
                            }
                        }
                    }
                    LogosState::State14 => {
                        lex.end(offset);
                        context = Some(LogosLeaf::Leaf7);
                        let other = lex.read::<u8>(offset);
                        if let Some(byte) = other {} else {}
                        {
                            let action = _get_action(lex, offset, context);
                            match action {
                                CallbackResult::Emit(tok) => {
                                    return Some(Ok(tok));
                                }
                                CallbackResult::Skip => {
                                    lex.trivia();
                                    offset = lex.offset();
                                    context = None;
                                    state = LogosState::State5;
                                    continue;
                                }
                                CallbackResult::Error(err) => {
                                    return Some(Err(err));
                                }
                                CallbackResult::DefaultError => {
                                    return Some(Err(_make_error(lex)));
                                }
                            }
                        }
                    }
                    LogosState::State15 => {
                        lex.end(offset);
                        context = Some(LogosLeaf::Leaf5);
                        let other = lex.read::<u8>(offset);
                        if let Some(byte) = other {} else {}
                        {
                            let action = _get_action(lex, offset, context);
                            match action {
                                CallbackResult::Emit(tok) => {
                                    return Some(Ok(tok));
                                }
                                CallbackResult::Skip => {
                                    lex.trivia();
                                    offset = lex.offset();
                                    context = None;
                                    state = LogosState::State5;
                                    continue;
                                }
                                CallbackResult::Error(err) => {
                                    return Some(Err(err));
                                }
                                CallbackResult::DefaultError => {
                                    return Some(Err(_make_error(lex)));
                                }
                            }
                        }
                    }
                    LogosState::State16 => {
                        lex.end(offset);
                        context = Some(LogosLeaf::Leaf3);
                        let other = lex.read::<u8>(offset);
                        if let Some(byte) = other {} else {}
                        {
                            let action = _get_action(lex, offset, context);
                            match action {
                                CallbackResult::Emit(tok) => {
                                    return Some(Ok(tok));
                                }
                                CallbackResult::Skip => {
                                    lex.trivia();
                                    offset = lex.offset();
                                    context = None;
                                    state = LogosState::State5;
                                    continue;
                                }
                                CallbackResult::Error(err) => {
                                    return Some(Err(err));
                                }
                                CallbackResult::DefaultError => {
                                    return Some(Err(_make_error(lex)));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    #[allow(deprecated)]
    #[allow(unreachable_code)]
    #[automatically_derived]
    impl derive_more::core::fmt::Display for Token {
        fn fmt(
            &self,
            __derive_more_f: &mut derive_more::core::fmt::Formatter<'_>,
        ) -> derive_more::core::fmt::Result {
            match self {
                Self::Identifier(_0) => {
                    derive_more::core::fmt::Display::fmt(_0, __derive_more_f)
                }
                Self::Number(_0) => {
                    derive_more::core::fmt::Display::fmt(_0, __derive_more_f)
                }
                Self::Dot => __derive_more_f.write_fmt(format_args!(".")),
                Self::Backslash => __derive_more_f.write_fmt(format_args!("\\")),
                Self::Plus => __derive_more_f.write_fmt(format_args!("+")),
                Self::OpenParenthesis => __derive_more_f.write_fmt(format_args!("(")),
                Self::CloseParenthesis => __derive_more_f.write_fmt(format_args!(")")),
                Self::OpenBracket => __derive_more_f.write_fmt(format_args!("[")),
                Self::CloseBracket => __derive_more_f.write_fmt(format_args!("[")),
                Self::Let => __derive_more_f.write_fmt(format_args!("let")),
                Self::Equal => __derive_more_f.write_fmt(format_args!("=")),
                Self::In => __derive_more_f.write_fmt(format_args!("in")),
            }
        }
    }
    fn string_to_arc(lex: &mut Lexer<Token>) -> Arc<str> {
        Arc::from(lex.slice())
    }
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
    #[automatically_derived]
    impl ::core::fmt::Debug for Delim {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    Delim::OpenParenthesis => "OpenParenthesis",
                    Delim::CloseParenthesis => "CloseParenthesis",
                    Delim::OpenBracket => "OpenBracket",
                    Delim::CloseBracket => "CloseBracket",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Delim {
        #[inline]
        fn clone(&self) -> Delim {
            match self {
                Delim::OpenParenthesis => Delim::OpenParenthesis,
                Delim::CloseParenthesis => Delim::CloseParenthesis,
                Delim::OpenBracket => Delim::OpenBracket,
                Delim::CloseBracket => Delim::CloseBracket,
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Delim {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Delim {
        #[inline]
        fn eq(&self, other: &Delim) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[allow(deprecated)]
    #[allow(unreachable_code)]
    #[automatically_derived]
    impl derive_more::core::fmt::Display for Delim {
        fn fmt(
            &self,
            __derive_more_f: &mut derive_more::core::fmt::Formatter<'_>,
        ) -> derive_more::core::fmt::Result {
            match self {
                Self::OpenParenthesis => {
                    __derive_more_f.write_fmt(format_args!("opening parenthesis \'(\'"))
                }
                Self::CloseParenthesis => {
                    __derive_more_f.write_fmt(format_args!("closing parenthesis \')\'"))
                }
                Self::OpenBracket => {
                    __derive_more_f.write_fmt(format_args!("opening bracket \'[\'"))
                }
                Self::CloseBracket => {
                    __derive_more_f.write_fmt(format_args!("closing bracket \']\'"))
                }
            }
        }
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
            Ok(
                match value {
                    Token::CloseBracket => Delim::CloseBracket,
                    Token::CloseParenthesis => Delim::CloseParenthesis,
                    Token::OpenBracket => Delim::OpenBracket,
                    Token::OpenParenthesis => Delim::OpenParenthesis,
                    _ => return Err(()),
                },
            )
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
        fn expected_found_eof(
            expected: Self::Expected,
            span: Range<usize>,
            _: &Typy<Type>,
        ) -> Self {
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
                Diag::ExpectedFound { expected: _, found, span } => {
                    Diag::ExpectedFound {
                        expected: Green(expected),
                        found,
                        span,
                    }
                }
                Diag::Merged { main, others } => {
                    Diag::Merged {
                        main: Box::new(main.expected(expected)),
                        others,
                    }
                }
                i => i,
            }
        }
        fn merge(a: Self, b: Self) -> Self {
            Self::Merged {
                main: Box::new(a),
                others: ::alloc::boxed::box_assume_init_into_vec_unsafe(
                    ::alloc::intrinsics::write_box_via_move(
                        ::alloc::boxed::Box::new_uninit(),
                        [b],
                    ),
                ),
            }
        }
    }
    pub type PCtx<'a> = skim::Context<
        SpannedIter<'a, Token>,
        Token,
        (),
        Range<usize>,
        Delim,
        Typy<Type>,
    >;
    fn identifier(ctx: &mut PCtx<'_>) -> Parsed<Arc<str>, Diag> {
        select(
                Expected::Identifier,
                |token| match token {
                    Token::Identifier(ident) => Some(ident),
                    _ => None,
                },
            )
            .parse(ctx)
    }
    fn number(ctx: &mut PCtx<'_>) -> Parsed<Expression, Diag> {
        select_ctx(
                Expected::Number,
                |token, ctx: &mut PCtx| match token {
                    Token::Number(number) => {
                        Some(Expression {
                            expr: Box::new(Expr::Number(number.parse().unwrap())),
                            r#type: ctx.ctx.known(Type::Number),
                            span: ctx.span(),
                        })
                    }
                    _ => None,
                },
            )
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
                expr: Box::new(
                    Expr::Application(
                        Expression {
                            expr: Box::new(Expr::Function(ident, expr)),
                            r#type: ctx.ctx.unknown(),
                            span: span.clone(),
                        },
                        val,
                    ),
                ),
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
            .or(
                open_delimiter(Delim::OpenParenthesis)
                    .cut()
                    .ignore_and(expr)
                    .and_ignore(close_delimiter(Delim::CloseParenthesis)),
            )
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
    pub fn parse(
        code: &str,
        typy: Typy<Type>,
    ) -> Parsed<(Expression, Typy<Type>), Diag> {
        let lexer = Token::lexer(code).spanned();
        let mut ctx = Context::new(lexer, 0..0, typy);
        expr.and_ignore(eof).parse(&mut ctx).map(|value| (value, ctx.take_ctx()))
    }
}
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
#[automatically_derived]
impl ::core::fmt::Debug for Expected {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            Expected::Token(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Token", &__self_0)
            }
            Expected::Delim(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Delim", &__self_0)
            }
            Expected::Eof => ::core::fmt::Formatter::write_str(f, "Eof"),
            Expected::Identifier => ::core::fmt::Formatter::write_str(f, "Identifier"),
            Expected::Number => ::core::fmt::Formatter::write_str(f, "Number"),
            Expected::Expression => ::core::fmt::Formatter::write_str(f, "Expression"),
            Expected::Type(__self_0, __self_1) => {
                ::core::fmt::Formatter::debug_tuple_field2_finish(
                    f,
                    "Type",
                    __self_0,
                    &__self_1,
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Expected {
    #[inline]
    fn clone(&self) -> Expected {
        match self {
            Expected::Token(__self_0) => {
                Expected::Token(::core::clone::Clone::clone(__self_0))
            }
            Expected::Delim(__self_0) => {
                Expected::Delim(::core::clone::Clone::clone(__self_0))
            }
            Expected::Eof => Expected::Eof,
            Expected::Identifier => Expected::Identifier,
            Expected::Number => Expected::Number,
            Expected::Expression => Expected::Expression,
            Expected::Type(__self_0, __self_1) => {
                Expected::Type(
                    ::core::clone::Clone::clone(__self_0),
                    ::core::clone::Clone::clone(__self_1),
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Expected {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Expected {
    #[inline]
    fn eq(&self, other: &Expected) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
            && match (self, other) {
                (Expected::Token(__self_0), Expected::Token(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
                (Expected::Delim(__self_0), Expected::Delim(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
                (
                    Expected::Type(__self_0, __self_1),
                    Expected::Type(__arg1_0, __arg1_1),
                ) => __self_0 == __arg1_0 && __self_1 == __arg1_1,
                _ => true,
            }
    }
}
#[allow(deprecated)]
#[allow(unreachable_code)]
#[automatically_derived]
impl derive_more::core::fmt::Display for Expected {
    fn fmt(
        &self,
        __derive_more_f: &mut derive_more::core::fmt::Formatter<'_>,
    ) -> derive_more::core::fmt::Result {
        match self {
            Self::Token(_0) => __derive_more_f.write_fmt(format_args!("\'{0}\'", _0)),
            Self::Delim(_0) => derive_more::core::fmt::Display::fmt(_0, __derive_more_f),
            Self::Eof => __derive_more_f.write_str("Eof"),
            Self::Identifier => __derive_more_f.write_fmt(format_args!("identifier")),
            Self::Number => __derive_more_f.write_fmt(format_args!("number")),
            Self::Expression => __derive_more_f.write_fmt(format_args!("expression")),
            Self::Type(_0, _1) => {
                derive_more::core::fmt::Display::fmt(_0, __derive_more_f)
            }
        }
    }
}
#[allow(deprecated)]
#[allow(unreachable_code)]
#[automatically_derived]
impl derive_more::core::convert::From<(Token)> for Expected {
    #[inline]
    fn from(value: (Token)) -> Self {
        Expected::Token(value)
    }
}
#[allow(deprecated)]
#[allow(unreachable_code)]
#[automatically_derived]
impl derive_more::core::convert::From<(Delim)> for Expected {
    #[inline]
    fn from(value: (Delim)) -> Self {
        Expected::Delim(value)
    }
}
#[allow(deprecated)]
#[allow(unreachable_code)]
#[automatically_derived]
impl derive_more::core::convert::From<(String, Type)> for Expected {
    #[inline]
    fn from(value: (String, Type)) -> Self {
        Expected::Type(value.0, value.1)
    }
}
pub enum Found {
    #[display("'{_0}'")]
    Token(Token),
    Eof,
    #[display("{_0}")]
    Type(String, Type),
}
#[automatically_derived]
impl ::core::fmt::Debug for Found {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            Found::Token(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Token", &__self_0)
            }
            Found::Eof => ::core::fmt::Formatter::write_str(f, "Eof"),
            Found::Type(__self_0, __self_1) => {
                ::core::fmt::Formatter::debug_tuple_field2_finish(
                    f,
                    "Type",
                    __self_0,
                    &__self_1,
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Found {
    #[inline]
    fn clone(&self) -> Found {
        match self {
            Found::Token(__self_0) => Found::Token(::core::clone::Clone::clone(__self_0)),
            Found::Eof => Found::Eof,
            Found::Type(__self_0, __self_1) => {
                Found::Type(
                    ::core::clone::Clone::clone(__self_0),
                    ::core::clone::Clone::clone(__self_1),
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Found {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Found {
    #[inline]
    fn eq(&self, other: &Found) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
            && match (self, other) {
                (Found::Token(__self_0), Found::Token(__arg1_0)) => __self_0 == __arg1_0,
                (Found::Type(__self_0, __self_1), Found::Type(__arg1_0, __arg1_1)) => {
                    __self_0 == __arg1_0 && __self_1 == __arg1_1
                }
                _ => true,
            }
    }
}
#[allow(deprecated)]
#[allow(unreachable_code)]
#[automatically_derived]
impl derive_more::core::fmt::Display for Found {
    fn fmt(
        &self,
        __derive_more_f: &mut derive_more::core::fmt::Formatter<'_>,
    ) -> derive_more::core::fmt::Result {
        match self {
            Self::Token(_0) => __derive_more_f.write_fmt(format_args!("\'{0}\'", _0)),
            Self::Eof => __derive_more_f.write_str("Eof"),
            Self::Type(_0, _1) => {
                derive_more::core::fmt::Display::fmt(_0, __derive_more_f)
            }
        }
    }
}
#[allow(deprecated)]
#[allow(unreachable_code)]
#[automatically_derived]
impl derive_more::core::convert::From<(Token)> for Found {
    #[inline]
    fn from(value: (Token)) -> Self {
        Found::Token(value)
    }
}
#[allow(deprecated)]
#[allow(unreachable_code)]
#[automatically_derived]
impl derive_more::core::convert::From<(String, Type)> for Found {
    #[inline]
    fn from(value: (String, Type)) -> Self {
        Found::Type(value.0, value.1)
    }
}
pub enum Diag {
    #[error("a lexer error occured here")]
    LexerError { #[label] span: Range<usize> },
    #[error("expected {expected} found {found}")]
    ExpectedFound {
        expected: Green<Expected>,
        found: Red<Found>,
        #[label]
        span: Range<usize>,
    },
    #[error("did not close delimiter {opener}")]
    DidNotCloseDelimiter { opener: Red<Delim>, #[label] span: Range<usize> },
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
    TriedToCloseUnopenedDelimiter { #[label] span: Range<usize>, delim: Red<Delim> },
    #[error("{main}")]
    Merged {
        #[source]
        #[diagnostic_source]
        main: Box<Diag>,
        #[related]
        others: Vec<Self>,
    },
}
#[automatically_derived]
impl ::core::fmt::Debug for Diag {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            Diag::LexerError { span: __self_0 } => {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "LexerError",
                    "span",
                    &__self_0,
                )
            }
            Diag::ExpectedFound {
                expected: __self_0,
                found: __self_1,
                span: __self_2,
            } => {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "ExpectedFound",
                    "expected",
                    __self_0,
                    "found",
                    __self_1,
                    "span",
                    &__self_2,
                )
            }
            Diag::DidNotCloseDelimiter { opener: __self_0, span: __self_1 } => {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "DidNotCloseDelimiter",
                    "opener",
                    __self_0,
                    "span",
                    &__self_1,
                )
            }
            Diag::TriedToCloseMismatchingDelimiteer {
                matching_open_span: __self_0,
                matching_open: __self_1,
                current_open_span: __self_2,
                current_open: __self_3,
                close_span: __self_4,
                close: __self_5,
            } => {
                let names: &'static _ = &[
                    "matching_open_span",
                    "matching_open",
                    "current_open_span",
                    "current_open",
                    "close_span",
                    "close",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    __self_0,
                    __self_1,
                    __self_2,
                    __self_3,
                    __self_4,
                    &__self_5,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "TriedToCloseMismatchingDelimiteer",
                    names,
                    values,
                )
            }
            Diag::TriedToCloseUnopenedDelimiter { span: __self_0, delim: __self_1 } => {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "TriedToCloseUnopenedDelimiter",
                    "span",
                    __self_0,
                    "delim",
                    &__self_1,
                )
            }
            Diag::Merged { main: __self_0, others: __self_1 } => {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Merged",
                    "main",
                    __self_0,
                    "others",
                    &__self_1,
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Diag {
    #[inline]
    fn clone(&self) -> Diag {
        match self {
            Diag::LexerError { span: __self_0 } => {
                Diag::LexerError {
                    span: ::core::clone::Clone::clone(__self_0),
                }
            }
            Diag::ExpectedFound {
                expected: __self_0,
                found: __self_1,
                span: __self_2,
            } => {
                Diag::ExpectedFound {
                    expected: ::core::clone::Clone::clone(__self_0),
                    found: ::core::clone::Clone::clone(__self_1),
                    span: ::core::clone::Clone::clone(__self_2),
                }
            }
            Diag::DidNotCloseDelimiter { opener: __self_0, span: __self_1 } => {
                Diag::DidNotCloseDelimiter {
                    opener: ::core::clone::Clone::clone(__self_0),
                    span: ::core::clone::Clone::clone(__self_1),
                }
            }
            Diag::TriedToCloseMismatchingDelimiteer {
                matching_open_span: __self_0,
                matching_open: __self_1,
                current_open_span: __self_2,
                current_open: __self_3,
                close_span: __self_4,
                close: __self_5,
            } => {
                Diag::TriedToCloseMismatchingDelimiteer {
                    matching_open_span: ::core::clone::Clone::clone(__self_0),
                    matching_open: ::core::clone::Clone::clone(__self_1),
                    current_open_span: ::core::clone::Clone::clone(__self_2),
                    current_open: ::core::clone::Clone::clone(__self_3),
                    close_span: ::core::clone::Clone::clone(__self_4),
                    close: ::core::clone::Clone::clone(__self_5),
                }
            }
            Diag::TriedToCloseUnopenedDelimiter { span: __self_0, delim: __self_1 } => {
                Diag::TriedToCloseUnopenedDelimiter {
                    span: ::core::clone::Clone::clone(__self_0),
                    delim: ::core::clone::Clone::clone(__self_1),
                }
            }
            Diag::Merged { main: __self_0, others: __self_1 } => {
                Diag::Merged {
                    main: ::core::clone::Clone::clone(__self_0),
                    others: ::core::clone::Clone::clone(__self_1),
                }
            }
        }
    }
}
impl miette::Diagnostic for Diag {
    fn help(&self) -> std::option::Option<std::boxed::Box<dyn std::fmt::Display + '_>> {
        #[allow(unused_variables, deprecated)]
        match self {
            Self::TriedToCloseUnopenedDelimiter { span, delim } => {
                std::option::Option::Some(
                    std::boxed::Box::new(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!(
                                    "consider simply removing the closing delimiter",
                                ),
                            )
                        }),
                    ),
                )
            }
            _ => std::option::Option::None,
        }
    }
    fn labels(
        &self,
    ) -> std::option::Option<
        std::boxed::Box<dyn std::iter::Iterator<Item = miette::LabeledSpan> + '_>,
    > {
        #[allow(unused_variables, deprecated)]
        match self {
            Self::LexerError { span } => {
                use miette::macro_helpers::ToOption;
                let labels_iter = ::alloc::boxed::box_assume_init_into_vec_unsafe(
                        ::alloc::intrinsics::write_box_via_move(
                            ::alloc::boxed::Box::new_uninit(),
                            [
                                miette::macro_helpers::OptionalWrapper::<
                                    Range<usize>,
                                >::new()
                                    .to_option(span)
                                    .map(|__miette_internal_var| miette::LabeledSpan::new_with_span(
                                        std::option::Option::None,
                                        __miette_internal_var.clone(),
                                    )),
                            ],
                        ),
                    )
                    .into_iter();
                std::option::Option::Some(
                    std::boxed::Box::new(
                        labels_iter.filter(Option::is_some).map(Option::unwrap),
                    ),
                )
            }
            Self::ExpectedFound { expected, found, span } => {
                use miette::macro_helpers::ToOption;
                let labels_iter = ::alloc::boxed::box_assume_init_into_vec_unsafe(
                        ::alloc::intrinsics::write_box_via_move(
                            ::alloc::boxed::Box::new_uninit(),
                            [
                                miette::macro_helpers::OptionalWrapper::<
                                    Range<usize>,
                                >::new()
                                    .to_option(span)
                                    .map(|__miette_internal_var| miette::LabeledSpan::new_with_span(
                                        std::option::Option::None,
                                        __miette_internal_var.clone(),
                                    )),
                            ],
                        ),
                    )
                    .into_iter();
                std::option::Option::Some(
                    std::boxed::Box::new(
                        labels_iter.filter(Option::is_some).map(Option::unwrap),
                    ),
                )
            }
            Self::DidNotCloseDelimiter { opener, span } => {
                use miette::macro_helpers::ToOption;
                let labels_iter = ::alloc::boxed::box_assume_init_into_vec_unsafe(
                        ::alloc::intrinsics::write_box_via_move(
                            ::alloc::boxed::Box::new_uninit(),
                            [
                                miette::macro_helpers::OptionalWrapper::<
                                    Range<usize>,
                                >::new()
                                    .to_option(span)
                                    .map(|__miette_internal_var| miette::LabeledSpan::new_with_span(
                                        std::option::Option::None,
                                        __miette_internal_var.clone(),
                                    )),
                            ],
                        ),
                    )
                    .into_iter();
                std::option::Option::Some(
                    std::boxed::Box::new(
                        labels_iter.filter(Option::is_some).map(Option::unwrap),
                    ),
                )
            }
            Self::TriedToCloseMismatchingDelimiteer {
                matching_open_span,
                matching_open,
                current_open_span,
                current_open,
                close_span,
                close,
            } => {
                use miette::macro_helpers::ToOption;
                let labels_iter = ::alloc::boxed::box_assume_init_into_vec_unsafe(
                        ::alloc::intrinsics::write_box_via_move(
                            ::alloc::boxed::Box::new_uninit(),
                            [
                                miette::macro_helpers::OptionalWrapper::<
                                    Range<usize>,
                                >::new()
                                    .to_option(matching_open_span)
                                    .map(|__miette_internal_var| miette::LabeledSpan::new_with_span(
                                        std::option::Option::Some(
                                            ::alloc::__export::must_use({
                                                ::alloc::fmt::format(
                                                    format_args!(
                                                        "and the matching delimiter {0} was opened here",
                                                        matching_open,
                                                    ),
                                                )
                                            }),
                                        ),
                                        __miette_internal_var.clone(),
                                    )),
                                miette::macro_helpers::OptionalWrapper::<
                                    Range<usize>,
                                >::new()
                                    .to_option(current_open_span)
                                    .map(|__miette_internal_var| miette::LabeledSpan::new_with_span(
                                        std::option::Option::Some(
                                            ::alloc::__export::must_use({
                                                ::alloc::fmt::format(
                                                    format_args!(
                                                        "but the current open delimiter is {0} here",
                                                        current_open,
                                                    ),
                                                )
                                            }),
                                        ),
                                        __miette_internal_var.clone(),
                                    )),
                                miette::macro_helpers::OptionalWrapper::<
                                    Range<usize>,
                                >::new()
                                    .to_option(close_span)
                                    .map(|__miette_internal_var| miette::LabeledSpan::new_with_span(
                                        std::option::Option::Some(
                                            ::alloc::__export::must_use({
                                                ::alloc::fmt::format(
                                                    format_args!("tried to close here with {0}", close),
                                                )
                                            }),
                                        ),
                                        __miette_internal_var.clone(),
                                    )),
                            ],
                        ),
                    )
                    .into_iter();
                std::option::Option::Some(
                    std::boxed::Box::new(
                        labels_iter.filter(Option::is_some).map(Option::unwrap),
                    ),
                )
            }
            Self::TriedToCloseUnopenedDelimiter { span, delim } => {
                use miette::macro_helpers::ToOption;
                let labels_iter = ::alloc::boxed::box_assume_init_into_vec_unsafe(
                        ::alloc::intrinsics::write_box_via_move(
                            ::alloc::boxed::Box::new_uninit(),
                            [
                                miette::macro_helpers::OptionalWrapper::<
                                    Range<usize>,
                                >::new()
                                    .to_option(span)
                                    .map(|__miette_internal_var| miette::LabeledSpan::new_with_span(
                                        std::option::Option::None,
                                        __miette_internal_var.clone(),
                                    )),
                            ],
                        ),
                    )
                    .into_iter();
                std::option::Option::Some(
                    std::boxed::Box::new(
                        labels_iter.filter(Option::is_some).map(Option::unwrap),
                    ),
                )
            }
            _ => std::option::Option::None,
        }
    }
    fn related(
        &self,
    ) -> std::option::Option<
        std::boxed::Box<dyn std::iter::Iterator<Item = &dyn miette::Diagnostic> + '_>,
    > {
        #[allow(unused_variables, deprecated)]
        match self {
            Self::Merged { main, others } => {
                std::option::Option::Some(
                    std::boxed::Box::new(
                        others.iter().map(|x| -> &(dyn miette::Diagnostic) { &*x }),
                    ),
                )
            }
            _ => std::option::Option::None,
        }
    }
    fn diagnostic_source(&self) -> std::option::Option<&dyn miette::Diagnostic> {
        #[allow(unused_variables, deprecated)]
        match self {
            Self::Merged { main, others } => {
                std::option::Option::Some(std::borrow::Borrow::borrow(main))
            }
            _ => std::option::Option::None,
        }
    }
}
#[allow(unused_qualifications)]
#[automatically_derived]
impl ::thiserror::__private18::Error for Diag {
    fn source(
        &self,
    ) -> ::core::option::Option<&(dyn ::thiserror::__private18::Error + 'static)> {
        use ::thiserror::__private18::AsDynError as _;
        #[allow(deprecated)]
        match self {
            Diag::LexerError { .. } => ::core::option::Option::None,
            Diag::ExpectedFound { .. } => ::core::option::Option::None,
            Diag::DidNotCloseDelimiter { .. } => ::core::option::Option::None,
            Diag::TriedToCloseMismatchingDelimiteer { .. } => {
                ::core::option::Option::None
            }
            Diag::TriedToCloseUnopenedDelimiter { .. } => ::core::option::Option::None,
            Diag::Merged { main: source, .. } => {
                ::core::option::Option::Some(source.as_dyn_error())
            }
        }
    }
}
#[allow(unused_qualifications)]
#[automatically_derived]
impl ::core::fmt::Display for Diag {
    fn fmt(&self, __formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use ::thiserror::__private18::AsDisplay as _;
        #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
        match self {
            Diag::LexerError { span } => {
                __formatter.write_str("a lexer error occured here")
            }
            Diag::ExpectedFound { expected, found, span } => {
                match (expected.as_display(), found.as_display()) {
                    (__display_expected, __display_found) => {
                        __formatter
                            .write_fmt(
                                format_args!(
                                    "expected {0} found {1}",
                                    __display_expected,
                                    __display_found,
                                ),
                            )
                    }
                }
            }
            Diag::DidNotCloseDelimiter { opener, span } => {
                match (opener.as_display(),) {
                    (__display_opener,) => {
                        __formatter
                            .write_fmt(
                                format_args!(
                                    "did not close delimiter {0}",
                                    __display_opener,
                                ),
                            )
                    }
                }
            }
            Diag::TriedToCloseMismatchingDelimiteer {
                matching_open_span,
                matching_open,
                current_open_span,
                current_open,
                close_span,
                close,
            } => __formatter.write_str("tried to close mismatching delimiter"),
            Diag::TriedToCloseUnopenedDelimiter { span, delim } => {
                match (delim.as_display(),) {
                    (__display_delim,) => {
                        __formatter
                            .write_fmt(
                                format_args!(
                                    "tried to close unopened delimiter with {0}",
                                    __display_delim,
                                ),
                            )
                    }
                }
            }
            Diag::Merged { main, others } => {
                match (main.as_display(),) {
                    (__display_main,) => {
                        __formatter.write_fmt(format_args!("{0}", __display_main))
                    }
                }
            }
        }
    }
}
impl std::borrow::Borrow<dyn miette::Diagnostic> for std::boxed::Box<Diag> {
    fn borrow(&self) -> &(dyn miette::Diagnostic + 'static) {
        &**self
    }
}
#[display("{expr}")]
pub struct Expression {
    expr: Box<Expr>,
    r#type: TypeID<Type>,
    span: Range<usize>,
}
#[automatically_derived]
impl ::core::fmt::Debug for Expression {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "Expression",
            "expr",
            &self.expr,
            "type",
            &self.r#type,
            "span",
            &&self.span,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Expression {
    #[inline]
    fn clone(&self) -> Expression {
        Expression {
            expr: ::core::clone::Clone::clone(&self.expr),
            r#type: ::core::clone::Clone::clone(&self.r#type),
            span: ::core::clone::Clone::clone(&self.span),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Expression {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Expression {
    #[inline]
    fn eq(&self, other: &Expression) -> bool {
        self.expr == other.expr && self.r#type == other.r#type && self.span == other.span
    }
}
#[allow(deprecated)]
#[allow(unreachable_code)]
#[automatically_derived]
impl derive_more::core::fmt::Display for Expression {
    fn fmt(
        &self,
        __derive_more_f: &mut derive_more::core::fmt::Formatter<'_>,
    ) -> derive_more::core::fmt::Result {
        let expr = &self.expr;
        let r#type = &self.r#type;
        let span = &self.span;
        derive_more::core::fmt::Display::fmt(expr, __derive_more_f)
    }
}
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
#[automatically_derived]
impl ::core::fmt::Debug for Expr {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            Expr::Number(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Number", &__self_0)
            }
            Expr::Variable(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "Variable",
                    &__self_0,
                )
            }
            Expr::Function(__self_0, __self_1) => {
                ::core::fmt::Formatter::debug_tuple_field2_finish(
                    f,
                    "Function",
                    __self_0,
                    &__self_1,
                )
            }
            Expr::Application(__self_0, __self_1) => {
                ::core::fmt::Formatter::debug_tuple_field2_finish(
                    f,
                    "Application",
                    __self_0,
                    &__self_1,
                )
            }
            Expr::Add(__self_0, __self_1) => {
                ::core::fmt::Formatter::debug_tuple_field2_finish(
                    f,
                    "Add",
                    __self_0,
                    &__self_1,
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Expr {
    #[inline]
    fn clone(&self) -> Expr {
        match self {
            Expr::Number(__self_0) => Expr::Number(::core::clone::Clone::clone(__self_0)),
            Expr::Variable(__self_0) => {
                Expr::Variable(::core::clone::Clone::clone(__self_0))
            }
            Expr::Function(__self_0, __self_1) => {
                Expr::Function(
                    ::core::clone::Clone::clone(__self_0),
                    ::core::clone::Clone::clone(__self_1),
                )
            }
            Expr::Application(__self_0, __self_1) => {
                Expr::Application(
                    ::core::clone::Clone::clone(__self_0),
                    ::core::clone::Clone::clone(__self_1),
                )
            }
            Expr::Add(__self_0, __self_1) => {
                Expr::Add(
                    ::core::clone::Clone::clone(__self_0),
                    ::core::clone::Clone::clone(__self_1),
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Expr {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Expr {
    #[inline]
    fn eq(&self, other: &Expr) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
            && match (self, other) {
                (Expr::Number(__self_0), Expr::Number(__arg1_0)) => __self_0 == __arg1_0,
                (Expr::Variable(__self_0), Expr::Variable(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
                (
                    Expr::Function(__self_0, __self_1),
                    Expr::Function(__arg1_0, __arg1_1),
                ) => __self_0 == __arg1_0 && __self_1 == __arg1_1,
                (
                    Expr::Application(__self_0, __self_1),
                    Expr::Application(__arg1_0, __arg1_1),
                ) => __self_0 == __arg1_0 && __self_1 == __arg1_1,
                (Expr::Add(__self_0, __self_1), Expr::Add(__arg1_0, __arg1_1)) => {
                    __self_0 == __arg1_0 && __self_1 == __arg1_1
                }
                _ => unsafe { ::core::intrinsics::unreachable() }
            }
    }
}
#[allow(deprecated)]
#[allow(unreachable_code)]
#[automatically_derived]
impl derive_more::core::fmt::Display for Expr {
    fn fmt(
        &self,
        __derive_more_f: &mut derive_more::core::fmt::Formatter<'_>,
    ) -> derive_more::core::fmt::Result {
        match self {
            Self::Number(_0) => derive_more::core::fmt::Display::fmt(_0, __derive_more_f),
            Self::Variable(_0) => {
                derive_more::core::fmt::Display::fmt(_0, __derive_more_f)
            }
            Self::Function(_0, _1) => {
                __derive_more_f.write_fmt(format_args!("({0}.{1})", _0, _1))
            }
            Self::Application(_0, _1) => {
                __derive_more_f.write_fmt(format_args!("({0} {1})", _0, _1))
            }
            Self::Add(_0, _1) => {
                __derive_more_f.write_fmt(format_args!("(+ {0} {1})", _0, _1))
            }
        }
    }
}
fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(
                format_args!(
                    "[{0} {1} {2}] {3}",
                    humantime::format_rfc3339_seconds(SystemTime::now()),
                    record.level(),
                    record.target(),
                    message,
                ),
            )
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(log_file("log.log")?)
        .apply()?;
    Ok(())
}
pub enum Type {
    Number,
    Function(TypeID<Type>, TypeID<Type>),
}
#[automatically_derived]
impl ::core::fmt::Debug for Type {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            Type::Number => ::core::fmt::Formatter::write_str(f, "Number"),
            Type::Function(__self_0, __self_1) => {
                ::core::fmt::Formatter::debug_tuple_field2_finish(
                    f,
                    "Function",
                    __self_0,
                    &__self_1,
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Type {
    #[inline]
    fn clone(&self) -> Type {
        match self {
            Type::Number => Type::Number,
            Type::Function(__self_0, __self_1) => {
                Type::Function(
                    ::core::clone::Clone::clone(__self_0),
                    ::core::clone::Clone::clone(__self_1),
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Type {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Type {
    #[inline]
    fn eq(&self, other: &Type) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
            && match (self, other) {
                (
                    Type::Function(__self_0, __self_1),
                    Type::Function(__arg1_0, __arg1_1),
                ) => __self_0 == __arg1_0 && __self_1 == __arg1_1,
                _ => true,
            }
    }
}
impl Type {
    pub fn to_string(&self, typy: &mut Typy<Self>) -> String {
        match self {
            Self::Number => String::from("number"),
            Self::Function(a, b) => {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "{0} -> {1}",
                            typy
                                .get(a)
                                .map(|t| t.to_string(typy))
                                .unwrap_or(String::from("{unknown}")),
                            typy
                                .get(b)
                                .map(|t| t.to_string(typy))
                                .unwrap_or(String::from("{unknown}")),
                        ),
                    )
                })
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
    fn visit<C, E>(
        &self,
        c: &mut C,
        f: impl Fn(&Self, &mut C) -> Result<(), E>,
    ) -> Result<(), E> {
        f(self, c)?;
        match self.expr.as_ref() {
            Expr::Add(a, b) => {
                f(a, c)?;
                f(b, c)?;
            }
            Expr::Function(_, a) => f(a, c)?,
            Expr::Application(a, b) => {
                f(a, c)?;
                f(b, c)?;
            }
            _ => {}
        }
        Ok(())
    }
}
impl typy::Type for Type {
    type Error = Diag;
    type Reason = (Range<usize>, Range<usize>);
    fn unify(
        &self,
        other: &Self,
        (span_a, span_b): &Self::Reason,
        typy: &mut Typy<Self>,
    ) -> Result<Self, Self::Error> {
        match (self, other) {
            (Type::Number, Type::Number) => Ok(Type::Number),
            (a @ Type::Function(_, _), b @ Type::Number) => {
                Err(Diag::ExpectedFound {
                    expected: Green(Expected::Type(a.to_string(typy), a.clone())),
                    found: Red(Found::Type(b.to_string(typy), b.clone())),
                    span: span_b.clone(),
                })
            }
            (b @ Type::Number, a @ Type::Function(_, _)) => {
                Err(Diag::ExpectedFound {
                    expected: Green(Expected::Type(a.to_string(typy), a.clone())),
                    found: Red(Found::Type(b.to_string(typy), b.clone())),
                    span: span_b.clone(),
                })
            }
            (Type::Function(a, b), Type::Function(c, d)) => {
                typy.eq((span_a.clone(), span_b.clone()), a, c)?;
                typy.eq((span_a.clone(), span_b.clone()), b, d)?;
                Ok(Type::Function(*a, *b))
            }
        }
    }
}
fn type_chk(expression: &Expression, typy: &mut Typy<Type>) -> Result<(), Diag> {
    let add = {
        fn cast_rule<V, T, E>(
            r: impl Rule<V, T, Error = E>,
        ) -> impl Rule<V, T, Error = E>
        where
            V: Typed<T>,
            T: Type,
        {
            r
        }
        cast_rule(|this: &Expression, ctx: &mut Typy<_>| match &this.get_match() {
            Expr::Add(a, b) => {
                ctx.eq((a.span.clone(), b.span.clone()), this, a)?
                    .eq((a.span.clone(), b.span.clone()), this, b)
                    .map(|_| ())
            }
            _ => Ok(()),
        })
    };
    let function = {
        fn cast_rule<V, T, E>(
            r: impl Rule<V, T, Error = E>,
        ) -> impl Rule<V, T, Error = E>
        where
            V: Typed<T>,
            T: Type,
        {
            r
        }
        cast_rule(|this: &Expression, ctx: &mut Typy<_>| match &this.get_match() {
            Expr::Function(_, body) => {
                {
                    let input = ctx.unknown();
                    let output = ctx.unknown();
                    ctx.set(
                            (this.span.clone(), this.span.clone()),
                            this,
                            Type::Function(input, output),
                        )?
                        .eq((this.span.clone(), body.span.clone()), &output, body)?
                }
                    .map(|_| ())
            }
            _ => Ok(()),
        })
    };
    typy.apply(add, expression)?;
    {
        ::std::io::_print(
            format_args!(
                "{1} : {0}\n",
                typy
                    .get(expression)
                    .map(|t| t.to_string(typy))
                    .unwrap_or(String::from("{unknown}")),
                expression,
            ),
        );
    };
    Ok(())
}
fn main() {
    _ = std::fs::remove_file("log.log");
    setup_logger().unwrap();
    let code = r#"2+x.x"#;
    let out: Parsed<_, Diag> = parser::parse(code, Typy::new());
    let result = match out {
        Parsed::Ok((value, mut typy)) => type_chk(&value, &mut typy),
        Parsed::Err(err) => Err(err),
        Parsed::Fatal(err) => Err(err),
        Parsed::Recover(err, (value, mut typy)) => {
            match type_chk(&value, &mut typy) {
                Ok(()) => Err(err),
                Err(err2) => Err(Diag::merge(err, err2)),
            }
        }
    };
    match result {
        Ok(()) => {}
        Err(e) => {
            ::std::io::_print(
                format_args!("{0:?}\n", miette::Report::from(e).with_source_code(code)),
            );
        }
    }
}
