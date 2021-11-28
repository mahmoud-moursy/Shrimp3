use crate::errors::Err;
use crate::nodes::Node;

use std::collections::HashMap;

#[derive(Clone)]
pub enum Variable {
    Ident(String),
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
                Ident(string) => panic!("{}", Err::UndefinedVar(self.clone())),
                Str(string) => string.to_string(),
                Num(num) => num.to_string(),
                Bool(var) => var.to_string(),
                Array(arr) => {
                    let mut out = String::from("[");
                    for i in arr {
                        // Kinda jank.
                        // Pushes the std::fmt::Display impl
                        // of the inner variable enum to the
                        // string
                        out.extend(i.to_string().chars())
                    }
                    out.push(']');
                    out
                }
                Function(func) => match func {
                    Node::FunctionDecl { name, args, nodes } => {
                        format!("@{} [{:?}] -> {{ {:?} }}", name, args, nodes)
                    }
                    any => panic!("SPE: FunctionDecl was in fact {}", any.as_words()),
                },
                NativeFunction(_) => {
                    String::from("<[Native function representing not supported.]>")
                }
                Void => String::from("()"),
            }
        )
    }
}

impl std::fmt::Debug for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> anyhow::Result<(), std::fmt::Error> {
        use Variable::*;
        write!(
            f,
            "{}",
            match self {
                NativeFunction(func) =>
                    String::from("<[Native function representing not supported.]>"),
                any => any.to_string(),
            }
        )
    }
}
