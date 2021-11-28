use thiserror::Error;

use crate::data_types::Variable;
use crate::nodes::Node;
use crate::tokens::Token;

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
    #[error("Found an unexpected token near {}", match.0 {
            Some(token) => token.to_string(),
            None => "the end of file.".to_string()
        })]
    UnexpectedToken(Option<Token>),
    #[error("Found an unexpected token near {}", match.0 {
            Some(token) => token.to_string(),
            None => "the end of file.".to_string()
        })]
    UnexpectedNode(Option<Node>),
    #[error("Mismatched types. Expected {}, found {} instead.", .0.as_words(), match .1 {
            Some(node) => node.as_words(),
            None => "nothing".to_string()
    })]
    TypeMismatch(Node, Option<Node>),
    #[error("Undefined variable `{0}`")]
    UndefinedVar(Variable),
    #[error("Missing an argument in a function call `{0}`")]
    MissingArgs(String),
    #[error("Unknown library specified `{0:?}`")]
    UnknownLib(Node),
    #[error("No defined main function!")]
    NoMain,
}
