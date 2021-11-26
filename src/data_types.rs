use crate::nodes::Node;
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use Variable::*;
        write!(
            f,
            "{}",
            match self {
                Str(string) => string.to_string(),
                Num(num) => num.to_string(),
                Bool(var) => var.to_string(),
                Array(arr) => {
                    String::new()
                }
                Function(func) => match func {
                    Node::FunctionDecl { name, args, nodes } => {
                        format!("@{} [{:?}] -> {{ {:?} }}", name, args, nodes.join("\n"))
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

pub type Scope = HashMap<String, HashMap<String, Variable>>;
