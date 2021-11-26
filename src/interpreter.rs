use crate::data_types::*;
use crate::errors::Err;
use crate::nodes::Node;

use crate::std_lib::construct_lib;
use crate::tokens::Token;

use std::collections::HashMap;

use anyhow::bail;

pub fn interpret(nodes: Vec<Node>) -> anyhow::Result<()> {
    let mut scope = Scope::new();

    scope.insert("!global".to_string(), construct_lib());

    let mut nodes = nodes.into_iter();

    while let Some(val) = nodes.next() {
        match val {
            // Only function decls should be at the top level
            Node::FunctionDecl { name, args, nodes } => {
                scope.get_mut("!global").unwrap().insert(
                    name.clone(),
                    Variable::Function(Node::FunctionDecl { name, args, nodes }),
                );
            }
            any => {
                bail!(Err::UnexpectedNode(Some(any)))
            }
        }
    }

    run_main(scope)
}

struct FuncDecl {
    name: String,
    args: Vec<Node>,
    nodes: Vec<Node>,
}

pub fn run_main(mut scopes: Scope) -> anyhow::Result<()> {
    let main = match scopes.get("!global").unwrap().get("main") {
        Some(data) => match data {
            Variable::Function(func) => match func.clone() {
                Node::FunctionDecl { name, args, nodes } => FuncDecl { name, args, nodes },
                _ => todo!(),
            },
            _ => todo!(),
        },
        None => {
            todo!()
        }
    };

    for node in main.nodes {
        match node {
            Node::CallExpr {
                name,
                args,
                assign_to,
            } => {
                match scopes.get("!global").unwrap().get(&name) {
                    Some(Variable::NativeFunction(exec)) => {
                        let mut res = exec(
                            args.into_iter()
                                .map(|x| match x {
                                    Node::Term(Token::Ident(string)) => {
                                        scopes.get(&main.name).unwrap().get(&string).unwrap()
                                    }
                                    x => x.as_var(),
                                })
                                .collect(),
                            scopes.get_mut(&main.name).unwrap(),
                        );
                        if let Some(var) = assign_to {
                            scopes
                                .get_mut(&main.name)
                                .unwrap()
                                .get_mut(&var)
                                .insert(&mut res);
                        }
                    }
                    Some(any) => todo!(),
                    None => todo!(),
                };
            }
            any => bail!("{}", any),
        }
    }

    Ok(())
}
