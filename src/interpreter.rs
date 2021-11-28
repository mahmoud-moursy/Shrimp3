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
) -> anyhow::Result<Variable> {
    let func = match func {
        Some(Variable::Function(func)) => match func {
            Node::FunctionDecl { name, args, nodes } => (name, args, nodes),
            any => panic!("{}", Err::UnexpectedNode(Some(any))),
        },
        Some(Variable::NativeFunction(exec)) => {
            let res = exec(args, variables);
            match assign_to {
                Some(val) => {
                    variables.insert(val, res.clone());
                }
                None => {}
            };
            return Ok(res);
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

    macro_rules! arr_into_var {
        ($args: expr) => {
            Variable::Array(
                $args
                    .into_iter()
                    .map(|x| match x {
                        Node::Term(Token::Ident(var)) => variables.get(&var).unwrap().clone(),
                        any => any.as_var(),
                    })
                    .collect(),
            )
        };
    }

    let mut func = func.2.into_iter();

    while let Some(node) = func.next() {
        match node {
            Node::Term(Token::Ident(id)) => {
                match id.as_str() {
                    "decl" => match func.next() {
                        Some(Node::Term(Token::Ident(var))) => match func.next() {
                            Some(tok) => {
                                variables.insert(var, tok.as_var());
                            }
                            any => bail!(Err::UnexpectedNode(any)),
                        },
                        any => bail!(Err::UnexpectedNode(any)),
                    },
                    "return" => match func.next() {
                        Some(Node::Term(tok)) => match tok {
                            Token::Ident(id) => return Ok(variables.get(&id).unwrap().clone()),
                            any => return Ok(any.as_var()),
                        },
                        Some(Node::Array(arr)) => return Ok(arr_into_var!(arr)),
                        any => bail!(Err::UnexpectedNode(any)),
                    },
                    "use" => match func.next() {
                        Some(node) => match node {
                            Node::Term(Token::Ident(lib)) => match lib.as_str() {
                                "internet" => std_lib::internet(variables),
                                any => {
                                    bail!(Err::UnknownLib(Node::Term(Token::Ident(
                                        any.to_string()
                                    ))))
                                }
                            },
                            any => bail!(Err::UnknownLib(any)),
                        },
                        None => bail!(Err::EOF),
                    },
                    any => {
                        panic!("Unknown keyword {}", any)
                    }
                };
            }
            Node::CallExpr {
                name,
                args,
                assign_to,
            } => {
                match variables.get(&name) {
                    Some(Variable::Function(fndecl)) => {
                        run(
                            Some(Variable::Function(fndecl.clone())),
                            variables,
                            args.into_iter()
                                .map(|x| match x {
                                    Node::Term(Token::Ident(var)) => {
                                        variables.get(&var).unwrap().clone()
                                    }
                                    any => any.as_var(),
                                })
                                .collect(),
                            assign_to,
                        )?;
                    }
                    Some(Variable::NativeFunction(func)) => match assign_to {
                        Some(string) => {
                            *variables.get_mut(&string).unwrap() = func(
                                args.into_iter()
                                    .map(|x| match x {
                                        Node::Term(Token::Ident(var)) => {
                                            variables.get(&var).unwrap().clone()
                                        }
                                        any => any.as_var(),
                                    })
                                    .collect(),
                                variables,
                            );
                        }
                        None => {
                            func(
                                args.into_iter()
                                    .map(|x| match x {
                                        Node::Term(Token::Ident(var)) => {
                                            variables.get(&var).unwrap().clone()
                                        }
                                        any => any.as_var(),
                                    })
                                    .collect(),
                                variables,
                            );
                        }
                    },
                    None => panic!("Nonexistent function called"),
                    any => panic!("Incorrect variable typed called as function."),
                };
            }
            /*  FIXME: This happens with valid code because of a parser
                      error.
            */
            any => println!("potential runtime err ignored for debugging purposes"),
        }
    }

    Ok(Variable::Void)
}
