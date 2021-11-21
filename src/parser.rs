// TODO: Implement parser.

use anyhow::bail;

use crate::errors::Err;
use crate::nodes::Node;
use crate::tokens::Token;

macro_rules! unwrap_or_eof {
	($to_match: expr) => {
		match $to_match {
			Some(thing) => thing,
			None => bail!(Err::UnexpectedEOF)
		}
	}
}

pub fn parse(tokens: Vec<Token>) -> anyhow::Result<Vec<Node>> {
	// Convert tokens -> terms.
	let mut tokens =
		tokens
			.into_iter()
			.map(|x| Node::Term(x))
			.collect::<Vec<Node>>();
	
	let mut final_out = vec![];
	
	// Parse arrays first and foremost,
	// then groups,
	// then terms, then function expressions.
	parse_array(&mut tokens)?;
	
	Ok(final_out)
}

#[derive(Debug)]
pub enum State {
	Searching,
	Growing(usize),
}

/// Parses arrays.
pub fn parse_array(tokens: &mut Vec<Node>) -> anyhow::Result<()> {
	let mut token_list = tokens.clone();
	let mut token_list = token_list.iter_mut().enumerate();
	
	// State machine.
	let mut state = State::Searching;
	let mut passes: usize = 0;
	
	for (idx, token) in token_list {
		match state {
			State::Searching => match token {
				Node::Term(Token::OpenSquare) => {
					*tokens.get_mut(idx).unwrap() = Node::Array(vec![]);
					state = State::Growing(idx);
				}
				Node::Term(Token::CloseSquare) => bail!(
					Err::UnexpectedToken(
						Some(Token::CloseSquare)
					)
				),
				_ => {}
			},
			State::Growing(arr_idx) => {
				match tokens.get_mut(arr_idx) {
					Some(Node::Term(Token::CloseSquare)) => state = State::Searching,
					Some(Node::Array(content)) => {}
					any => {
						panic!("Expected an Array node, found a {:?}", any);
					}
				}
			}
		}
	}
	
	todo!()
}