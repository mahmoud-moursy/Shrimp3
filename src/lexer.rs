use std::fs::File;
use std::io::Read;

use anyhow::bail;
use anyhow::Result;

use super::errors::Err;
use super::tokens::Token;

use crate::panic;

// Generate all tokens for the parser to make an AST
pub fn make_tokens(mut file: File) -> Result<Vec<Token>> {
    // Track line position for use in error messages
    let mut line_pos: usize = 0;
    let mut line_num: usize = 0;

    // Get file contents
    let mut chars = String::new();
    file.read_to_string(&mut chars)?;

    // Iterator to go through all chars.
    let mut char_list = chars.chars().into_iter().peekable();

    // Free memory.
    drop(file);

    let mut final_out = vec![];

    // Loop until the iterator is done.
    while let Some(code) = char_list.next() {
        match code {
            '#' => {
                while let Some(code) = char_list.peek() {
                    match code {
                        '#' => {
                            char_list.next();
                            break;
                        }
                        _ => {
                            // Move cursor to the next char
                            char_list.next();
                        }
                    }
                }
            }
            // Identifier tokens.
            ident if ident.is_alphabetic() => {
                // Final output
                let mut out = code.to_string();

                // Loop will continue pushing all chars to out until
                // a non ascii-alphanumeric character is found.
                while let Some(code) = char_list.peek() {
                    // Break loop if code is not alphanumeric
                    if !code.is_ascii_alphanumeric() && *code != '_' {
                        break;
                    }
                    out.push(char_list.next().unwrap());
                }
                final_out.push(Token::Ident(out));
            }
            // Numerical tokens
            num if num.is_numeric() => {
                let mut out = num.to_string();

                // Set to true if a `.` is detected.
                // This is so that numbers can only
                // have one `.` so that the .parse()
                // function cannot fail.
                let mut float = false;

                while let Some(code) = char_list.peek() {
                    if !code.is_numeric() {
                        // If number is a float,
                        // add '.' to be parsed.
                        if float == false && *code == '.' {
                            float = true;
                            out.push(char_list.next().unwrap());
                            continue;
                        }
                        break;
                    }

                    out.push(char_list.next().unwrap())
                }

                final_out.push(Token::Num(out.parse().unwrap()))
            }
            // List of singular tokens.
            // Would try to make this DRY, but
            // some branches return (), so it'd
            // be pretty annoying.
            // Never mind that, I could have used `continue`.
            // too lazy to change it now.
            '@' => final_out.push(Token::FunctionDecl),
            '{' => final_out.push(Token::OpenCurly),
            '}' => final_out.push(Token::CloseCurly),
            '(' => final_out.push(Token::OpenBracket),
            ')' => final_out.push(Token::CloseBracket),
            '[' => final_out.push(Token::OpenSquare),
            ']' => final_out.push(Token::CloseSquare),
            ';' => final_out.push(Token::EndLine),
            '=' => match char_list.next() {
                Some('>') => final_out.push(Token::ForAssigner),
                Some(any) => panic!(Err::UnexpectedChar(line_num, line_pos, any)),
                None => panic!(Err::EOF),
            },
            // Handles arrow assigners
            '-' => match char_list.next() {
                Some(thing) => match thing {
                    '>' => final_out.push(Token::ArrowAssigner),
                    num if num.is_numeric() => {
                        let mut out = '-'.to_string() + &num.to_string();

                        // Set to true if a `.` is detected.
                        // This is so that numbers can only
                        // have one `.` so that the .parse()
                        // function cannot fail.
                        let mut float = false;

                        while let Some(code) = char_list.peek() {
                            if !code.is_numeric() {
                                // If number is a float,
                                // add '.' to be parsed.
                                if float == false && *code == '.' {
                                    float = true;
                                    out.push(char_list.next().unwrap());
                                    continue;
                                }
                                break;
                            }

                            out.push(char_list.next().unwrap())
                        }

                        final_out.push(Token::Num(out.parse().unwrap()))
                    }
                    any => bail!(Err::UnexpectedChar(line_num, line_pos, any)),
                },
                None => bail!(Err::UnexpectedEOF(line_num, line_pos)),
            },
            // Handles strings
            '"' => {
                // Final output
                let mut out = String::new();

                // If the file isn't over, add current char
                while let Some(thing) = char_list.next() {
                    // Stop if the apostrophe opened with
                    if thing == '"' {
                        break;
                    }
                    out.push(
                        // Mostly checks for escape
                        // sequences + newlines.
                        match thing {
                            // new lines do not end strings.
                            // who decided that was a good idea?
                            // and more importantly, who decided that
                            // """multiline""" was a good idea????
                            '\n' => {
                                line_num += 1;
                                line_pos = 0;
                                '\n'
                            }
                            // Handle escape sequences
                            '\\' => match char_list.next() {
                                Some(thing) => match thing {
                                    '\n' => continue,
                                    'n' => '\n',
                                    'u' => '\x00',
                                    other => other,
                                },
                                None => bail!(Err::UnexpectedEOF(line_num, line_pos)),
                            },
                            any => any,
                        },
                    );
                    line_pos += 1;
                }

                final_out.push(Token::Str(out))
            }
            '\n' => {
                line_num += 1;
                line_pos = 0;
            }
            any if any.is_whitespace() => {
                line_pos += 1;
            }
            any => bail!(Err::UnexpectedChar(line_num, line_pos, any)),
        }
        line_pos += 1;
    }

    Ok(final_out)
}
