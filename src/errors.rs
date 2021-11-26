use thiserror::Error;

use crate::nodes::Node;
use crate::tokens::Token;

/// Stores all user-made error types
#[derive(Error, Debug)]
pub enum Err {
    #[error("Unexpected EOF in line {0} at char {1}")]
    UnexpectedEOF(usize, usize),
    /// The parser has no character-specific debug info,
    /// so it can't say at exactly which char/line.
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
    #[error("Mismatched types. Expected {0}, found {} instead.", match .1 {
            Some(node) => node.to_string(),
            None => "nothing".to_string()
    })]
    TypeMismatch(Node, Option<Node>),
}
