use std::fmt::Formatter;

use Token::*;
use crate::data_types::Variable;

#[derive(Debug, PartialEq)]
/// The token enum -- holds all token variants.
/// - Used in generating the AST.
pub enum Token {
    /// Any string. 'Hello world!', "Hello world!"
    /// or					'\'Hello world!\''
    Str(String),
    /// Any int or float. 123, 1.23, 33.32
    Num(f32),
    /// Any letters not in a string.
    Ident(String),
    /// Function token `@`.
    /// A.K.A At Symbol
    FunctionDecl,
    /// Line ends `;`
    EndLine,
    /// Curly brace opening `{`
    OpenCurly,
    /// Curly brace closing `}`
    CloseCurly,
    /// Bracket opening `(`
    OpenBracket,
    /// Bracket closing `)`
    CloseBracket,
    /// Square bracket opening `[`
    OpenSquare,
    /// Square bracket closing `]`
    CloseSquare,
    /// An arrow assigner `->`
    /// e.g: add(3, 3) -> x
    ArrowAssigner,
    /// A for assigner `=>`.
    /// e.g, for range(0 100) => count {}
    ForAssigner
}

impl Clone for Token {
    fn clone(&self) -> Self {
        match self {
            Str(string) => Str(string.clone()),
            Ident(id) => Ident(id.clone()),
            Num(num) => Num(*num),
            FunctionDecl => FunctionDecl,
            EndLine => EndLine,
            OpenCurly => OpenCurly,
            CloseCurly => CloseCurly,
            OpenBracket => OpenBracket,
            CloseBracket => CloseBracket,
            OpenSquare => OpenSquare,
            CloseSquare => CloseSquare,
            ArrowAssigner => ArrowAssigner,
            ForAssigner => ForAssigner
        }
    }
}

impl Token {
    pub fn as_var(self) -> Variable {
        match self {
            Str(string) => Variable::Str(string),
            Num(num) => Variable::Num(num),
            any => panic!("Tried to convert incorrect data: {:?}", any)
        }
    }
    pub fn as_words(&self) -> String {
        match self {
            // Everything in Token is imported into the scope
            // so that Token::* is not necessary.
            Str(_) => "a string",
            Num(_) => "a number",
            Ident(_) => "an identifier",
            FunctionDecl => "a function declaration token (`@`)",
            EndLine => "a line ending token (`;`)",
            OpenCurly => "a curly brace (`{`)",
            CloseCurly => "a curly brace (`}`)",
            OpenBracket => "a bracket (`(`)",
            CloseBracket => "a bracket (`)`)",
            OpenSquare => "a square bracket (`[`)",
            CloseSquare => "a square bracket (`]`)",
            ArrowAssigner => "an arrow assigner (`->`)",
            ForAssigner => "a for assigner (`=>`)"
        }
        .to_string()
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "`{}`",
            match self {
                // Each token's representation.
                Str(string) => string.clone(),
                Num(num) => num.to_string(),
                Ident(ident) => ident.clone(),
                // P	A  I	N 		A	U		 C H O C O L A T
                any => match any {
                    FunctionDecl => "@",
                    EndLine => ";",
                    OpenCurly => "{",
                    CloseCurly => "}",
                    OpenBracket => "(",
                    CloseBracket => ")",
                    OpenSquare => "[",
                    CloseSquare => "]",
                    ForAssigner => "=>",
                    ArrowAssigner => "->",
                    _ => {
                        "Shrimp parser fmt::display has had an error. :("
                    }
                }
                .to_string(),
            }
        )
    }
}
