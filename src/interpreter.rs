use crate::data_types::*;
use crate::errors::Err;
use crate::nodes::Node;

use crate::std_lib::{self, construct_lib};
use crate::tokens::Token;

use std::collections::HashMap;

use anyhow::bail;

pub fn interpret(nodes: Vec<Node>) -> anyhow::Result<()> {
    let mut variables = construct_lib();

    let mut nodes = nodes.into_iter();

    while let Some(node) = nodes.next() {
        match node {
            Node::FunctionDecl { name, args, nodes } => {
                variables.insert(
                    name.clone(),
                    Variable::Function(Node::FunctionDecl { name, args, nodes }),
                );
            }
            Node::Term(Token::Ident(id)) => match id.as_str() {
                "use" => match nodes.next() {
                    Some(node) => match node {
                        Node::Term(Token::Ident(lib)) => match lib.as_str() {
                            "internet" => std_lib::internet(&mut variables),
                            any => {
                                bail!(Err::UnknownLib(Node::Term(Token::Ident(any.to_string()))))
                            }
                        },
                        any => bail!(Err::UnknownLib(any)),
                    },
                    None => bail!(Err::EOF),
                },
                any => bail!(Err::UnexpectedNode(Some(Node::Term(Token::Ident(id))))),
            },
            any => todo!(),
        }
    }

    run(
        variables.remove("main"),
        &mut variables,
        vec![Variable::Array(
            std::env::args().map(|x| Variable::Str(x)).collect(),
        )],
        None,
    )?;

    Ok(())
}

pub fn run(
    func: Option<Variable>,
    variables: &mut HashMap<String, Variable>,
    mut args: Vec<Variable>,
    assign_to: Option<String>,
) -> anyhow::Result<()> {
    let mut func = match func {
        Some(Variable::Function(func)) => match func {
            Node::FunctionDecl { name, args, nodes } => (name, args, nodes),
            any => panic!("{}", Err::UnexpectedNode(Some(any))),
        },
        Some(Variable::NativeFunction(exec)) => {
            let res = exec(args, variables);
            match assign_to {
                Some(val) => {
                    variables.insert(val, res);
                }
                None => {}
            };
            return Ok(());
        }
        any => todo!(),
    };

    if args.len() != func.1.len() {
        // TODO: Impl errors.rs error
        todo!("Incorrect arg count")
    }

    let mut temp_vars = vec![];

    for var in func.1.into_iter() {
        match var {
            Node::Term(Token::Ident(id)) => {
                temp_vars.push(id.clone());
                variables.insert(id, args.remove(0));
            }
            any => panic!("{}", any),
        }
    }

    for node in func.2.into_iter() {
        match node {
            Node::CallExpr {
                name,
                args,
                assign_to,
            } => {}
            any => todo!(
                "SPE TODO ERR: incorrect type of expr found in node (non-call expr) {}",
                any
            ),
        }
    }

    Ok(())
}
