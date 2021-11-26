use crate::data_types::Variable;
use crate::tokens::Token;

#[derive(Debug, Clone, PartialEq)]
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
    NativeFn(fn() -> Variable),
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
            Node::Array(_) => format!("{}", "an array"),
            Node::Group(_) => format!("{}", "a group"),
            Node::Block(_) => format!("{}", "a codeblock"),
            Node::CallExpr { .. } => format!("{}", "a call expression"),
            Node::FunctionDecl { .. } => format!("{}", "a function declaration"),
            Node::NativeFn(_) => format!(
                "{}",
                "a native function token (This was an internal parser/compiler error)"
            ),
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Node::Term(tok) => write!(f, "{}", tok.as_words()),
            Node::Array(_) => write!(f, "an array"),
            Node::Group(_) => write!(f, "a group"),
            Node::Block(_) => write!(f, "a codeblock"),
            Node::CallExpr { .. } => write!(f, "a call expression"),
            Node::FunctionDecl { .. } => write!(f, "a function declaration"),
            Node::NativeFn(_) => write!(
                f,
                "a native function token (This was an internal parser/compiler error)"
            ),
        }
    }
}
