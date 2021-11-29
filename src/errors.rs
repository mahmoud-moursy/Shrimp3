use thiserror::Error;

use crate::nodes::Node;
use crate::tokens::Token;
use crate::data_types::Variable;

/// Stores all user-made error types
#[derive(Error, Debug)]
pub enum Err {
    // Should not be here, kept due to laziness.
    #[error("Unexpected EOF in line {0} at char {1}")]
    UnexpectedEOF(usize, usize),
    // TODO: Get rid of this and change the delete UnexpectedEOF
    #[error("Unexpected EOF")]
    EOF,
    #[error("Unexpected character `{2}` in line {0} at char {1}")]
    UnexpectedChar(usize, usize, char),
    #[error("Found an unexpected token ({})", match.0 {
            Some(token) => token.to_string(),
            None => "the end of file.".to_string()
        })]
    UnexpectedToken(Option<Token>),
    #[error("Found an unexpected token ({})", match.0 {
            Some(token) => token.to_string(),
            None => "the end of file.".to_string()
        })]
    UnexpectedNode(Option<Node>),
    #[error("Mismatched types. Expected {}, found {} instead.", .0.as_words(), match .1 {
            Some(node) => node.as_words(),
            None => "nothing".to_string()
    })]
    TypeMismatch(Node, Option<Node>),
    #[error("Missing an argument in function call `{0}`")]
    MissingArgs(String),
    #[error("Unknown library specified `{0:?}`")]
    UnknownLib(Node),
    #[error("Nonexistent variable called `{0}`")]
    NonexistentVar(String),
    #[error("Unknown keyword `{0}`")]
    UnknownKeyword(String),
    #[error("Variable type mismatch: Expected {}, found {}", .0.as_words(), .1.as_words())] 
    VarTypeMismatch(Variable, Variable),
    #[error("SPE: {0} was in fact a {1}")]
    SPEUnexpectedNode(Node, Node),
    #[error("Incorrect amount of arguments. Expected {0}, found {1}")]
    IncorrectArgCount(usize, usize),
    #[error("No defined main function!")]
    NoMain,
}

#[macro_export]
macro_rules! panic {
    ($panic_msg: expr) => {
        {
            println!("Error: {}", $panic_msg);
            std::process::exit(1)
        }
    };
}