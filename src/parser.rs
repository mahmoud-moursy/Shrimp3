// TODO: Implement parser.

use anyhow::bail;

use crate::errors::Err;
use crate::nodes::Node;
use crate::tokens::Token;

pub fn parse(tokens: Vec<Token>) -> anyhow::Result<Vec<Node>> {
    // Convert tokens -> terms.
    let mut tokens = tokens.into_iter();

    let mut final_out = Vec::new();

    /// This macro is derived from the open curly parsing.
    /// As such, some variables names may be named
    /// accordingly, and some comments may match
    /// that too.
    macro_rules! recurse {
        ($open_expr: expr => $open: pat, $close: pat, $out: path) => {
            // Content of the curly! :)
            let mut curly_content = Vec::new();
            // If you are going >16k curly braces/arrays,
            // re-evaluate life choices.
            let mut curly_count: u16 = 1;
            while curly_count > 0 {
                if let Some(token) = tokens.next() {
                    match token {
                        $open => {
                            curly_count += 1;
                            curly_content.push(token);
                        }
                        $close => {
                            curly_count -= 1;
                            curly_content.push(token);
                        }
                        any => {
                            curly_content.push(any);
                        }
                    };
                    continue;
                }
            }
            let mut out = parse(curly_content)?;

            // FIXME: Hacky/buggy workaround for a bug.
            // Prune closing statements.
            while matches!(
                if out.len() > 0 {
                    out.get(out.len() - 1)
                } else {
                    None
                },
                Some(Node::Term(
                    Token::CloseCurly | Token::CloseBracket | Token::CloseSquare
                ))
            ) {
                out.pop();
            }

            final_out.push($out(make_fn_call(out)?));
        };
    }

    while let Some(token) = tokens.next() {
        match token {
            /* // Content of the curly! :)
                // let mut curly_content = Vec::new();
                // // If you are going >16k curly braces/arrays,
                // // re-evaluate life choices.
                // let mut curly_count: u16 = 1;
                // while curly_count > 0 {
                // 	if let Some(token) = tokens.next() {
                // 		match token {
                // 			Token::CloseCurly => curly_count -= 1,
                // 			any => curly_content.push(any)
                // 		};
                // 		continue
                // 	}
                // 	bail!(Err::UnclosedDelim(Token::OpenCurly))
                // }
                // final_out.push(Node::Block(parse(curly_content)?));
                */
            /*
            Token::OpenCurly => {
                // Content of the curly! :)
                let mut curly_content = Vec::new();
                // If you are going >16k curly braces/arrays,
                // re-evaluate life choices.
                let mut curly_count: u16 = 1;
                while curly_count > 0 {
                    if let Some(token) = tokens.next() {
                        match token {
                            Token::CloseCurly => curly_count -= 1,
                            any => curly_content.push(any)
                        };
                        continue
                    }
                    bail!(Err::UnclosedDelim(Token::OpenCurly))
                }
                final_out.extend(parse(curly_content)?.into_iter());
            }
            */
            Token::OpenCurly => {
                recurse!(Token::OpenCurly => Token::OpenCurly, Token::CloseCurly, Node::Block);
            }

            Token::OpenSquare => {
                recurse!(Token::OpenSquare => Token::OpenSquare, Token::CloseSquare, Node::Array);
            }

            Token::OpenBracket => {
                recurse!(Token::OpenBracket => Token::OpenBracket, Token::CloseBracket, Node::Group);
            }

            Token::Str(_) | Token::Num(_) | Token::Ident(_) => {
                final_out.push(Node::Term(token));
            }
            any => final_out.push(Node::Term(any)),
        }
    }

    Ok(final_out)
}

/// Compacts everything into a series of function
/// declarations, so that function call expressions
/// may later be parsed.
pub fn make_fn(nodes: Vec<Node>) -> anyhow::Result<Vec<Node>> {
    let mut final_out = Vec::new();

    let mut node_list = nodes.into_iter();

    while let Some(node) = node_list.next() {
        let mut name = None;
        let mut args = None;
        let mut nodes = None;
        if node != Node::Term(Token::FunctionDecl) {
            continue;
        }
        // Better way to do this?
        if let Some(Node::Term(Token::Ident(fn_name))) = node_list.next() {
            name = Some(fn_name)
        }
        if let Some(Node::Group(arr)) = node_list.next() {
            for i in &arr {
                if let Node::Term(Token::Ident(_)) = i {
                    continue;
                }
                bail!(Err::TypeMismatch(
                    Node::Term(Token::Ident("identifier".to_string())),
                    Some(i.clone())
                ))
            }
            args = Some(arr)
        }
        if let Some(Node::Block(arr)) = node_list.next() {
            nodes = Some(arr)
        }

        if name.is_none() || args.is_none() || nodes.is_none() {
            bail!(Err::UnexpectedToken(Some(Token::FunctionDecl)))
        }

        final_out.push(Node::FunctionDecl {
            name: name.unwrap(),
            args: args.unwrap(),
            nodes: make_fn_call(nodes.unwrap())?,
        })
    }

    Ok(final_out)
}

pub fn make_fn_call(nodes: Vec<Node>) -> anyhow::Result<Vec<Node>> {
    let mut final_out = Vec::new();

    let mut nodes = nodes.into_iter().peekable();

    while let Some(node) = nodes.next() {
        if matches!(&node, Node::Term(Token::Ident(_)))
            && matches!(nodes.peek(), Some(Node::Group(_)))
        {
            final_out.push(Node::CallExpr {
                name: match node {
                    Node::Term(Token::Ident(ident)) => ident,
                    any => panic!("SPE: expected ident, found {}", any),
                },
                args: match nodes.next().unwrap() {
                    Node::Group(arr) => make_fn_call(arr)?,
                    any => panic!("SPE: expected group, found {}", any),
                },
                assign_to: match nodes.peek() {
                    Some(Node::Term(Token::ArrowAssigner)) => {
                        nodes.next();
                        match nodes.next() {
                            Some(Node::Term(Token::Ident(var_name))) => Some(var_name),
                            any => bail!(Err::UnexpectedNode(any)),
                        }
                    }
                    _ => None,
                },
            });
            continue;
        } else if node == Node::Term(Token::EndLine) {
            continue;
        };
        final_out.push(node)
    }

    Ok(final_out)
}

/*
DEPRECATED: Now generated using macros!
/// Parses arrays.
pub unsafe fn parse_array(tokens: RefCell<&mut Vec<Node>>) -> anyhow::Result<()> {
    let mut token_list = (tokens).as_ptr();

    let mut token_iter = (*token_list).iter_mut().enumerate();

    // Tracks amount of brackets
    let mut brack_count: usize = 1;

    // State machine.
    let mut state = State::Searching;

    // If this is true, the array inside the Node::Array enum will be re-processed
    let mut reprocess = false;

    for (idx, token) in token_iter {
        match state {
            // Searching state.
            // If a token matches `[`, then it will
            // switch state and start adding Nodes to the array.
            State::Searching => match token {
                Node::Term(Token::OpenSquare) => {
                    // Sets current token to the array Node.
                    // This will be what is pushed to every single
                    // time a term is added.
                    *token = Node::Array(Vec::new());
                    state = State::Growing(idx)
                },
                // Ignore everything else
                _ => {}
            },
            State::Growing(arr_idx) => {
                match (*token_list).get_mut(arr_idx) {
                    // Expected case (an array node at the index)
                    Some(Node::Array(arr)) => {
                        arr.push(
                            match (*token_list).remove(arr_idx + 1) {
                                Node::Term(tok) => match tok {
                                    // Regular/expected syntax.
                                    // Just adds the tokens to the Vec
                                    Token::Str(_)
                                    | Token::Num(_)
                                    | Token::Ident(_)
                                    | Token::OpenBracket
                                    | Token::CloseBracket
                                    => {
                                        Node::Term(tok)
                                    }
                                    // Ensure that the array is not closed
                                    // early in cases like [3 3 [4 4 5]]
                                    // where the array would end at `5 -->] ]`
                                    Token::OpenSquare => {
                                        brack_count += 1;
                                        reprocess = true;
                                        Node::Term(tok)
                                    }

                                    Token::CloseSquare => {
                                        brack_count -= 1;
                                        if brack_count < 1 {
                                            if reprocess {
                                                parse_array(RefCell::new(arr))?;
                                            }

                                            break
                                        }
                                        Node::Term(tok)
                                    }
                                    any => bail!(Err::UnexpectedToken(Some(any)))
                                }
                                any => panic!("SPE: Expected node to be term, but was {:?}", any)
                            }
                        );
                    }
                    // Fail case.
                    any => {
                        panic!("The Shrimp parser had an internal error! (Expected node to be array, but was in fact a {:?})", any)
                    }
                }
            }
        };
    }

    Ok(())
}
 */
