// TODO: Implement parser.

use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

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

pub unsafe fn parse(tokens: Vec<Token>) -> anyhow::Result<Vec<Node>> {
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
	parse_array(Rc::new(RefCell::new(&mut tokens)))?;
	
	println!("{:?}", tokens);
	
	Ok(final_out)
}

#[derive(Debug)]
pub enum State {
	Searching,
	Growing(usize),
}

/// Parses arrays.
pub unsafe fn parse_array(tokens: Rc<RefCell<&mut Vec<Node>>>) -> anyhow::Result<()> {
	let mut token_list = (*tokens).as_ptr();
	
	let mut tokens = (*token_list).iter_mut().enumerate();
	
	// State machine.
	let mut state = State::Searching;
	let mut passes: usize = 0;
	let mut brack_count: usize = 1;
	
	let mut re_process = false;
	
	for (idx, token) in tokens {
		match state {
			State::Searching => match token {
				Node::Term(Token::OpenSquare) => {
					*token = Node::Array(vec![]);
					state = State::Growing(idx)
				},
				_ => {}
			},
			State::Growing(arr_idx) => {
				match (*token_list).get_mut(arr_idx) {
					Some(Node::Array(arr)) => {
						arr.push(
							match (*token_list).remove(idx - passes) {
								Node::Term(tok) => match tok {
									Token::Str(_)
									| Token::Num(_)
									| Token::Ident(_)
									| Token::OpenBracket
									| Token::CloseBracket
									=> {
										Node::Term(tok)
									}
									Token::OpenSquare => {
										brack_count += 1;
										re_process = true;
										Node::Term(tok)
									}
									Token::CloseSquare => {
										brack_count -= 1;
										if brack_count < 1 {
											break
										}
										Node::Term(tok)
									}
									any => bail!(Err::UnexpectedToken(Some(any)))
								}
								any => panic!("SPE: Expected node to be term, but was {:?}", any)
							}
						);
						println!("{:?}", arr);
						passes += 1;
					}
					any => {
						panic!("The Shrimp parser had an internal error! (Expected node to be array, but was in fact a {:?})", any)
					}
				}
			}
		};
	}
	
	Ok(())
}