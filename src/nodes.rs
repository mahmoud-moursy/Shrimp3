use crate::data_types::Variable;
use crate::tokens::Token;
use crate::errors::Err;
use crate::panic;

#[derive(Debug, PartialEq)]
/// This file is responsible for all nodes in the AST
pub enum Node {
    /// The most basic Node.
    /// Literally just a token.
    Term(Token),
    /// Any values enclosed in [].
    /// E.g: [32, 32, "Array!"]
    Array(Vec<Node>),
    /// Any tokens enclosed in ().
    /// E.g: fn(32, 32, "String!", [32, 32, "Array!"])
    /// or   (add(10, -3))
    Group(Vec<Node>),
    /// Any tokens enclosed in {}.
    /// E.g: { print("Hello world!"); }
    Block(Vec<Node>),
    /// Any call expression
    /// i.e: print("Hello world! 🦐")
    CallExpr {
        name: String,
        args: Vec<Node>,
        assign_to: Option<String>,
    },
    /// Any function declaration. The typically, last part to be parsed.
    /// ```
    /// @main() {
    /// 	print("Hello world!");
    /// }
    /// ```
    FunctionDecl {
        name: String,
        args: Vec<Node>,
        nodes: Vec<Node>,
    },
}

impl Node {
    pub fn as_words(&self) -> String {
        match self {
            Node::Term(tok) => format!("{}", tok.as_words()),
            Node::Array(_) => "an array".to_string(),
            Node::Group(_) => "a group".to_string(),
            // Don't know what i was thinking when i wrote this,
            // but i'm too lazy to change the rest into
            // "a codeblock".into_string()
            Node::Block(_) => "a codeblock".to_string(),
            Node::CallExpr { .. } => format!("{}", "a call expression"),
            Node::FunctionDecl { .. } => format!("{}", "a function declaration"),
        }
    }
    pub fn as_var(self) -> Variable {
        match self {
            Node::Array(arr) => Variable::Array(arr.into_iter().map(|x| x.as_var()).collect()),
            Node::Term(Token::Num(num)) => Variable::Num(num),
            Node::Term(Token::Str(string)) => Variable::Str(string),
            any => panic!(Err::UnexpectedNode(Some(any))),
        }
    }
}

impl Clone for Node {
    fn clone(&self) -> Self {
        match self {
            Node::Term(any) => Node::Term(any.clone()),
            Node::Array(arr) => Node::Array(arr.clone()),
            Node::Group(arr) => Node::Group(arr.clone()),
            Node::Block(arr) => Node::Group(arr.clone()),
            Node::CallExpr {
                name,
                args,
                assign_to
            } => 
            Node::CallExpr {
                name: name.clone(),
                args: args.clone(),
                assign_to: assign_to.clone()
            },
            Node::FunctionDecl {
                name,
                args,
                nodes
            } => Node::FunctionDecl {
                name: name.clone(),
                args: args.clone(),
                nodes: nodes.clone()
            },
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Node::Term(tok) => write!(f, "{}", tok.as_words()),
            Node::Array(_) => write!(f, "an array"),
            Node::Group(gr) => write!(f, "a group of {:?}", gr),
            Node::Block(_) => write!(f, "a codeblock"),
            Node::CallExpr { .. } => write!(f, "a call expression"),
            Node::FunctionDecl { .. } => write!(f, "a function declaration"),
        }
    }
}
