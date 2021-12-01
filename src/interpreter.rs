use crate::data_types::*;
use crate::errors::Err;
use crate::nodes::Node;

use crate::panic;

use crate::std_lib::{self, construct_lib};
use crate::tokens::Token;

use std::collections::HashMap;

use anyhow::bail;

pub fn interpret(nodes: Vec<Node>) -> anyhow::Result<()> {
    let mut variables = construct_lib();

    let mut nodes = nodes.into_iter();

    while let Some(node) = nodes.next() {
        // Insert all functions into the variables.
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
                _ => bail!(Err::UnexpectedNode(Some(Node::Term(Token::Ident(id))))),
            },
            any => panic!(Err::UnexpectedNode(Some(any))),
        }
    }

    let main = variables.remove("main");

    if main.is_none() {
        bail!(Err::NoMain)
    }

    run(
        main,
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
            any => panic!(Err::UnexpectedNode(Some(any))),
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
        Some(any) => panic!(Err::VarTypeMismatch(
            Variable::Function(Node::Array(vec![])),
            any
        )),
        None => panic!(Err::NonexistentVar(
            "Name unknown, originated in a function call".to_string()
        )),
    };

    if args.len() != func.1.len() {
        println!("(Error originated in call to {} function)", func.0);
        panic!(Err::IncorrectArgCount(args.len(), func.1.len()))
    }

    let mut temp_vars = vec![];

    for var in func.1.into_iter() {
        match var {
            Node::Term(Token::Ident(id)) => {
                temp_vars.push(id.clone());
                variables.insert(id, args.remove(0));
            }
            any => panic!(Err::UnexpectedNode(Some(any))),
        }
    }

    macro_rules! arr_into_var {
        ($args: expr) => {
            Variable::Array(into_var($args, variables))
        };
    }

    macro_rules! get_var {
        ($var_name: expr) => {
            match variables.get(&$var_name) {
                Some(var) => var.clone(),
                None => {
                    panic!(Err::NonexistentVar($var_name));
                }
            }
        };
        (g_mut => $var_name: expr) => {
            match variables.get_mut(&$var_name) {
                Some(var) => var,
                None => {
                    variables.insert($var_name.clone(), Variable::Void);
                    variables.get_mut(&$var_name).unwrap()
                }
            }
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
                                let res = into_var(vec![tok], variables).remove(0);
                                variables.insert(var, res);
                            }
                            any => bail!(Err::UnexpectedNode(any)),
                        },
                        any => bail!(Err::UnexpectedNode(any)),
                    },
                    "del" => match func.next() {
                        Some(Node::Term(Token::Ident(id))) => {
                            variables.remove(&id);
                        }
                        None => panic!(Err::EOF),
                        any => panic!(Err::UnexpectedNode(any)),
                    },
                    "return" => match func.next() {
                        Some(Node::Term(tok)) => match tok {
                            Token::Ident(id) => return Ok(get_var!(id)),
                            any => return Ok(any.as_var()),
                        },
                        Some(Node::Array(arr)) => return Ok(arr_into_var!(arr)),
                        any => bail!(Err::UnexpectedNode(any)),
                    },
                    "use" => match func.next() {
                        Some(node) => match node {
                            Node::Term(Token::Ident(lib)) => match lib.as_str() {
                                "fs" => std_lib::fs(variables),
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
                    "if" => match func.next() {
                        Some(thing) => match into_var(vec![thing], variables).remove(0) {
                            Variable::Bool(boolean) => {
                                if boolean {
                                    run(
                                        Some(Variable::Function(Node::FunctionDecl {
                                            name: "if statement".to_string(),
                                            args: vec![],
                                            nodes: match func.next() {
                                                Some(Node::Block(arr)) => arr,
                                                any => panic!(Err::UnexpectedNode(any)),
                                            },
                                        })),
                                        variables,
                                        args.clone(),
                                        assign_to.clone(),
                                    )?;
                                } else {
                                    // Ensure that syntax is proper even
                                    // if the statement was false.
                                    match func.next() {
                                        Some(Node::Block(_)) => {}
                                        any => panic!(Err::UnexpectedNode(any)),
                                    }
                                }
                            }
                            any => panic!(Err::VarTypeMismatch(Variable::Bool(true), any)),
                        },
                        None => panic!(Err::EOF),
                    },
                    "for" => match func.next() {
                        Some(node) => match into_var(vec![node], variables).remove(0) {
                            Variable::Array(arr) => match func.next() {
                                Some(Node::Term(Token::ForAssigner)) => match func.next() {
                                    Some(Node::Term(Token::Ident(id))) => {
                                        let block = match func.next() {
                                            Some(Node::Block(block)) => block,
                                            None => panic!(Err::EOF),
                                            any => panic!(Err::UnexpectedNode(any)),
                                        };

                                        for var in arr {
                                            variables.insert(id.clone(), var);
                                            run(
                                                Some(Variable::Function(Node::FunctionDecl {
                                                    name: "for loop".to_string(),
                                                    args: vec![],
                                                    nodes: block.clone(),
                                                })),
                                                variables,
                                                args.clone(),
                                                assign_to.clone(),
                                            )?;
                                        }
                                    }
                                    None => panic!(Err::EOF),
                                    any => panic!(Err::UnexpectedNode(any)),
                                },
                                None => panic!(Err::EOF),
                                any => panic!(Err::UnexpectedNode(any)),
                            },
                            any => panic!(Err::VarTypeMismatch(Variable::Array(vec![]), any)),
                        },
                        None => panic!(Err::EOF),
                    },
                    "while" => match func.next() {
                        Some(node) => {
                            let block = match func.next() {
                                Some(Node::Block(block)) => block,
                                None => panic!(Err::EOF),
                                any => panic!(Err::UnexpectedNode(any)),
                            };

                            while into_var(vec![node.clone()], variables).remove(0)
                                == Variable::Bool(true)
                            {
                                run(
                                    Some(Variable::Function(Node::FunctionDecl {
                                        name: "while loop".to_string(),
                                        args: vec![],
                                        nodes: block.clone(),
                                    })),
                                    variables,
                                    args.clone(),
                                    assign_to.clone(),
                                )?;
                            }
                        }
                        None => panic!(Err::EOF),
                    },
                    any => {
                        panic!(Err::UnknownKeyword(any.to_string()))
                    }
                };
            }
            Node::CallExpr {
                name,
                args,
                assign_to,
            } => {
                match variables.clone().get(&name) {
                    Some(Variable::Function(fn_decl)) => {
                        let res = into_var(args, variables);

                        run(
                            Some(Variable::Function(fn_decl.clone())),
                            variables,
                            res,
                            assign_to,
                        )?;
                    }
                    Some(Variable::NativeFunction(func)) => match assign_to {
                        Some(string) => {
                            let res = into_var(args, variables);

                            *(get_var!(g_mut => string)) = func(res, variables);
                        }
                        None => {
                            let res = into_var(args, variables);
                            func(res, variables);
                        }
                    },
                    None => {
                        if name == "if" {
                            println!("Don't put a group (any tokens enclosed in brackets) after an if statement!");
                        }
                        panic!(Err::NonexistentVar(name))
                    }
                    any => panic!(Err::VarTypeMismatch(
                        Variable::Function(Node::Array(vec![])),
                        any.unwrap().clone()
                    )),
                };
            }
            any => panic!(Err::UnexpectedNode(Some(any))),
        }
    }

    Ok(Variable::Void)
}

pub fn into_var(args: Vec<Node>, variables: &mut HashMap<String, Variable>) -> Vec<Variable> {
    macro_rules! get_var {
        ($var_name: expr) => {
            match variables.get(&$var_name) {
                Some(var) => var.clone(),
                None => {
                    panic!(Err::NonexistentVar($var_name));
                }
            }
        };
        (g_mut => $var_name: expr) => {
            match variables.get_mut(&$var_name) {
                Some(var) => var,
                None => {
                    panic!(Err::NonexistentVar($var_name));
                }
            }
        };
    }

    args.into_iter()
        .map(|x| match x {
            Node::Term(Token::Ident(var)) => get_var!(var),
            Node::CallExpr {
                name,
                args,
                assign_to,
            } => {
                let res = into_var(args, variables);
                match run(Some(get_var!(name)), variables, res, assign_to) {
                    Ok(res) => res,
                    Err(err) => panic!(err),
                }
            }
            any => any.as_var(),
        })
        .collect()
}
