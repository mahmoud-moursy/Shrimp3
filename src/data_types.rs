use crate::errors::Err;
use crate::nodes::Node;
use crate::panic;

use std::collections::HashMap;

#[derive(Clone)]
pub enum Variable {
    Str(String),
    Num(f32),
    Bool(bool),
    Array(Vec<Variable>),
    Function(Node),
    NativeFunction(fn(Vec<Variable>, &mut HashMap<String, Variable>) -> Variable),
    Void,
}

impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> anyhow::Result<(), std::fmt::Error> {
        use Variable::*;
        write!(
            f,
            "{}",
            match self {
                Str(string) => string.to_string(),
                Num(num) => num.to_string(),
                Bool(var) => var.to_string(),
                Array(arr) => {
                    let mut out = String::from("[");
                    for i in arr {
                        out.push(' ');
                        // Kinda jank.
                        // Pushes the std::fmt::Display impl
                        // of the inner variable enum to the
                        // string
                        out.extend(i.to_string().chars());

                        out.push(' ');
                    }
                    out.push(']');
                    out
                }
                Function(func) => match func {
                    Node::FunctionDecl { name, args, nodes } => {
                        format!("@{} [{:?}] -> {{ {:?} }}", name, args, nodes)
                    }
                    any => panic!(Err::SPEUnexpectedNode(
                        Node::FunctionDecl {
                            name: "".to_string(),
                            args: vec![],
                            nodes: vec![]
                        },
                        any.clone()
                    )),
                },
                NativeFunction(_) => {
                    String::from("<[Native function representing not supported.]>")
                }
                Void => String::from("()"),
            }
        )
    }
}

impl std::cmp::PartialEq for Variable {
    fn eq(&self, rhs: &Variable) -> bool {
        return match self {
            Variable::Array(arr) => {
                arr == match rhs {
                    Variable::Array(arr) => arr,
                    _ => return false,
                }
            }
            Variable::Bool(b) => {
                b == match rhs {
                    Variable::Bool(b) => b,
                    _ => return false,
                }
            }
            Variable::Str(string) => {
                string
                    == match rhs {
                        Variable::Str(string) => string,
                        _ => return false,
                    }
            }
            Variable::Function(_) => panic!("Cannot compare functions"),
            Variable::NativeFunction(_) => panic!("Cannot compare functions"),
            Variable::Num(num) => {
                num == match rhs {
                    Variable::Num(num) => num,
                    _ => return false,
                }
            }
            Variable::Void => match rhs {
                Variable::Void => true,
                _ => false,
            },
        };
    }
}

impl std::fmt::Debug for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> anyhow::Result<(), std::fmt::Error> {
        use Variable::*;
        write!(
            f,
            "{}",
            match self {
                NativeFunction(_) =>
                    String::from("<[Native function representing not supported.]>"),
                any => any.to_string(),
            }
        )
    }
}

impl Variable {
    pub fn as_words(&self) -> String {
        match self {
            Variable::Num(_) => "a number",
            Variable::Str(_) => "a string",
            Variable::Array(_) => "an array",
            Variable::Bool(_) => "a boolean",
            Variable::Function(_) => "a function",
            Variable::NativeFunction(_) => "a function",
            Variable::Void => "nothing",
        }
        .to_string()
    }
}
