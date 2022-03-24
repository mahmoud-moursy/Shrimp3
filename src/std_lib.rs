use std::collections::HashMap;
use std::io::Write;

use std::convert::TryInto;

use std::io::Read;

use std::fs::File;

use crate::data_types::*;

use crate::panic;

use crate::errors::Err;

use rayon::prelude::*;

pub fn construct_lib() -> HashMap<String, Variable> {
    let mut map = HashMap::new();

    map.insert("void".to_string(), Variable::Void);

    map.insert("true".to_string(), Variable::Bool(true));
    map.insert("false".to_string(), Variable::Bool(false));

    map.insert("num_max".to_string(), Variable::Num(f32::MAX));
    map.insert("num_min".to_string(), Variable::Num(f32::MIN));

    map.insert("num_pi".to_string(), Variable::Num(std::f32::consts::PI));

    macro_rules! insert_fn {
        (
				$(
					$name: expr => $val: expr
				)*
			) => {
            $( map.insert($name.to_string(), Variable::NativeFunction($val)); )*
        };
    }
    insert_fn!(
        "hello_world" => |_, _| {
            println!("Hello world!");
            Variable::Void
        }
        "len" => |mut args, _| {
            if args.len() != 1 {
                panic!(Err::IncorrectArgCount(1, args.len()))
            }
            Variable::Num(match args.remove(0) {
                Variable::Array(arr) => arr.len(),
                Variable::Str(string) => string.len(),
                any => panic!(Err::VarTypeMismatch(
                    Variable::Array(vec![]),
                    any
                ))
            } as f32)
        }
        // bnd == bounds.
        // This function returns the length-1, AKA the maximum
        // bound for indexing.
        "bnd" => |mut args, _| {
            if args.len() != 1 {
                panic!(Err::IncorrectArgCount(1, args.len()))
            }
            Variable::Num(match args.remove(0) {
                Variable::Array(arr) => arr.len() - 1,
                Variable::Str(string) => string.len() - 1,
                any => panic!(Err::VarTypeMismatch(
                    Variable::Array(vec![]),
                    any
                ))
            } as f32)
        }
        "pop" => |mut args, _| {
            if args.len() != 1 {
                panic!(Err::IncorrectArgCount(1, args.len()));
            }

            return match args.remove(0) {
                Variable::Array(mut arr) => { arr.pop(); Variable::Array(arr) }
                any => panic!(Err::VarTypeMismatch(
                    Variable::Array(vec![]),
                    any
                ))
            }
        }
        "index" => |mut args, _| {
            if args.len() != 2 {
                panic!(Err::IncorrectArgCount(2, args.len()));
            }

            match args.remove(0) {
                Variable::Array(mut arr) => {
                    let idx = match args.remove(0) {
                        Variable::Num(num) => num as usize,
                        any => panic!(Err::VarTypeMismatch(
                            Variable::Num(0.0),
                            any
                        ))
                    };

                    if idx >= arr.len() {
                        panic!(Err::OutOfBoundsIndex(arr.len(), idx))
                    }

                    arr.remove(
                        idx
                    )
                },
                Variable::Str(mut arr) => {
                    let idx = match args.remove(0) {
                        Variable::Num(num) => num as usize,
                        any => panic!(Err::VarTypeMismatch(
                            Variable::Num(0.0),
                            any
                        ))
                    };

                    if idx >= arr.len() {
                        panic!(Err::OutOfBoundsIndex(arr.len(), idx))
                    }

                    Variable::Str(arr.remove(
                        idx
                    ).into())
                },
                any => panic!(Err::VarTypeMismatch(
                            Variable::Array(vec![]),
                            any
                ))
            }
        }
        "index_v" => |mut args, _| {
            if args.len() != 2 {
                panic!(Err::IncorrectArgCount(2, args.len()));
            }

            match args.remove(0) {
                Variable::Array(mut arr) => {
                    let idx = match args.remove(0) {
                        Variable::Num(num) => num as usize,
                        any => panic!(Err::VarTypeMismatch(
                            Variable::Num(0.0),
                            any
                        ))
                    };

                    if args.len() >= args.len() {
                        return Variable::Void
                    }

                    arr.remove(
                        idx
                    )
                },
                any => panic!(Err::VarTypeMismatch(
                            Variable::Array(vec![]),
                            any
                ))
            }
        }
        "replace" => |mut args, _| {
            if args.len() != 3 {
                panic!(
                    Err::IncorrectArgCount(
                        3,
                        args.len()
                    )
                )
            }

            let mut array = match args.remove(0) {
                Variable::Array(arr) => arr,
                any => panic!(
                    Err::VarTypeMismatch(
                        Variable::Array(vec![]),
                        any
                    )
                )
            };

            let index = match args.remove(0) {
                Variable::Num(num) => num as usize,
                any => panic!(
                    Err::VarTypeMismatch(
                        Variable::Num(0.0),
                        any
                    )
                )
            };

            let elem = args.remove(0);

            *match array.get_mut(index) {
                Some(arr) => arr,
                None => panic!(Err::OutOfBoundsIndex(array.len(), index))
            } = elem;

            Variable::Array(array)
        }
        "enumerate" => |mut args, _| {
            if args.len() != 1 {
                panic!(Err::IncorrectArgCount(
                    1,
                    args.len()
                ))
            }

            let array = match args.remove(0) {
                Variable::Array(array) => array,
                any => panic!(Err::VarTypeMismatch(
                    Variable::Array(vec![]),
                    any
                ))
            };

            let array = array.into_par_iter().enumerate().map(
                |x| {
                    Variable::Array(vec![Variable::Num(x.0 as f32), x.1])
                }
            );

            Variable::Array(
                array.collect()
            )
        }
        "push" => |args, _| {
            if args.len() < 2 {
                panic!(Err::MissingArgs("push".to_string()));
            }

            let mut args = args.into_iter();

            let mut array = match args.next() {
                Some(Variable::Array(arr)) => arr,
                Some(any) => panic!(Err::VarTypeMismatch(
                    Variable::Array(vec![]),
                    any
                )),
                None => panic!(Err::EOF)
            };

            while let Some(var) = args.next() {
                array.push(var)
            }

            Variable::Array(array)
        }
        "eq" => |args, _| {
            let mut args = args.into_iter();
            let first = args.next().unwrap();
            while let Some(arg) = args.next() {
                if arg != first {
                    return Variable::Bool(false)
                }
            }
            Variable::Bool(true)
        }
        "cmp" => |args, _| {
            if args.len() < 2 {
                panic!(Err::MissingArgs("cmp".to_string()))
            }

            let mut args = args.into_iter();

            let to_cmp = args.next().unwrap();

            while let Some(arg) = args.next() {
                if to_cmp < arg {
                    return Variable::Bool(false)
                }
            }

            Variable::Bool(true)
        }
        "range" => |mut args, _| {
            if args.len() != 2 {
                panic!(Err::IncorrectArgCount(2, args.len()))
            }

            let num_1 = match args.remove(0) {
                Variable::Num(num) => num as i32,
                any => panic!(Err::VarTypeMismatch(
                    Variable::Num(0.0),
                    any
                ))
            };

            let num_2 = match args.remove(0) {
                Variable::Num(num) => num as i32,
                any => panic!(Err::VarTypeMismatch(
                    Variable::Num(0.0),
                    any
                ))
            };

            Variable::Array(
                (num_1..num_2).into_par_iter().map(
                    |x| Variable::Num(x as f32)
                ).collect()
            )
        }
        "print" => |args, _| {
            for i in args {
                print!("{}", i)
            }
            std::io::stdout().flush().unwrap();
            Variable::Void
        }
        "println" => |args, _| {
            for i in args {
                print!("{}", i)
            }
            print!("\n");
            std::io::stdout().flush().unwrap();
            Variable::Void
        }
        "con" => |args, _| {
            let mut args = args.into_iter();

            let mut out = match args.next() {
                Some(any) => any.to_string(),
                None => panic!(Err::MissingArgs("con".to_string()))
            };

            while let Some(var) = args.next() {
                out += &var.to_string()
            }

            Variable::Str(out)
        }
        "add" => |args, _| {
            let mut args = args.into_iter();

            let mut final_out = 0.0;

            while let Some(arg) = args.next() {
                match arg {
                    Variable::Num(num) => final_out += num,
                    any => panic!(Err::VarTypeMismatch(
                        Variable::Num(0.0),
                        any
                    ))
                }
            }

            Variable::Num(final_out)
        }
        "sub" => |args, _| {
            let mut args = args.into_iter();

            let mut final_out = match args.next() {
                Some(var) => match var {
                    Variable::Num(num) => num,
                    any => panic!(Err::VarTypeMismatch(
                        Variable::Num(0.0),
                        any
                    ))
                },
                None => panic!(Err::MissingArgs("sub".to_string()))
            };

            while let Some(arg) = args.next() {
                match arg {
                    Variable::Num(num) => final_out -= num,
                    any => panic!(Err::VarTypeMismatch(
                        Variable::Num(0.0),
                        any
                    ))
                }
            }

            Variable::Num(final_out)
        }
        "mod" => |args, _| {
            let mut args = args.into_iter();

            let mut final_out = match args.next() {
                Some(var) => match var {
                    Variable::Num(num) => num,
                    any => panic!(Err::VarTypeMismatch(
                        Variable::Num(0.0),
                        any
                    ))
                },
                None => panic!(Err::MissingArgs("sub".to_string()))
            };

            while let Some(arg) = args.next() {
                match arg {
                    Variable::Num(num) => final_out %= num,
                    any => panic!(Err::VarTypeMismatch(
                        Variable::Num(0.0),
                        any
                    ))
                }
            }

            Variable::Num(final_out)
        }
        "mult" => |args, _| {
            let mut args = args.into_iter();

            let mut final_out = match args.next() {
                Some(var) => match var {
                    Variable::Num(num) => num,
                    any => panic!(Err::VarTypeMismatch(
                        Variable::Num(0.0),
                        any
                    ))
                },
                None => panic!(Err::MissingArgs("mult".to_string()))
            };

            while let Some(arg) = args.next() {
                match arg {
                    Variable::Num(num) => final_out *= num,
                    any => panic!(Err::VarTypeMismatch(
                        Variable::Num(0.0),
                        any
                    ))
                }
            }

            Variable::Num(final_out)
        }
        "div" => |args, _| {
            let mut args = args.into_iter();

            let mut final_out = match args.next() {
                Some(var) => match var {
                    Variable::Num(num) => num,
                    any => panic!(Err::VarTypeMismatch(
                        Variable::Num(0.0),
                        any
                    ))
                },
                None => panic!(Err::MissingArgs("div".to_string()))
            };

            while let Some(arg) = args.next() {
                match arg {
                    Variable::Num(num) => final_out /= num,
                    any => panic!(Err::VarTypeMismatch(
                        Variable::Num(0.0),
                        any
                    ))
                }
            }

            Variable::Num(final_out)
        }
        "xor" => |args, _| {
            let mut args = args.into_iter();

            let mut final_out = match args.next() {
                Some(var) => match var {
                    Variable::Num(num) => num,
                    any => panic!(Err::VarTypeMismatch(
                        Variable::Num(0.0),
                        any
                    ))
                },
                None => panic!(Err::MissingArgs("div".to_string()))
            };

            while let Some(arg) = args.next() {
                match arg {
                    Variable::Num(num) => final_out = (final_out as i32 ^ num as i32) as f32,
                    any => panic!(Err::VarTypeMismatch(
                        Variable::Num(0.0),
                        any
                    ))
                }
            }

            Variable::Num(final_out)
        }
        "pow" => |args, _| {
            let mut args = args.into_iter();

            let mut final_out = match args.next() {
                Some(var) => match var {
                    Variable::Num(num) => num,
                    any => panic!(Err::VarTypeMismatch(
                        Variable::Num(0.0),
                        any
                    ))
                },
                None => panic!(Err::MissingArgs("div".to_string()))
            };

            while let Some(arg) = args.next() {
                match arg {
                    Variable::Num(num) => final_out = final_out.powf(num),
                    any => panic!(Err::VarTypeMismatch(
                        Variable::Num(0.0),
                        any
                    ))
                }
            }

            Variable::Num(final_out)
        }
        "not" => |mut args, _| {
            if args.len() != 1 {
                panic!(Err::IncorrectArgCount(1, args.len()))
            }

            match args.remove(0) {
                Variable::Bool(boolean) => Variable::Bool(!boolean),
                any => panic!(Err::VarTypeMismatch(
                    Variable::Bool(true),
                    any
                ))
            }
        }
        "str" => |args, _| {
            if args.len() != 1 {
                panic!(Err::IncorrectArgCount(
                    1,
                    args.len()
                ))
            }

            Variable::Str(
                args[0].to_string()
            )
        }
        "num" => |mut args, _| {
            if args.len() != 1 {
                panic!(Err::IncorrectArgCount(
                    1,
                    args.len()
                ))
            }

            Variable::Num(match args.remove(0) {
                Variable::Str(string) => match string.trim().parse() {
                    Ok(res) => res,
                    Err(_) => panic!(Err::NumParserError(Variable::Str(string)))
                },
                any => panic!(Err::VarTypeMismatch(
                    Variable::Str("".to_string()),
                    any
                ))
            })
        }
        "weak_eq" => |args, _| {
            if args.len() < 2 {
                panic!(Err::MissingArgs("weak_eq".to_string()))
            }
            let mut args = args.into_iter();

            let first = args.next().unwrap();

            while let Some(arg) = args.next() {
                if arg.to_string() != first.to_string() {
                    return Variable::Bool(false)
                }
            }

            Variable::Bool(true)
        }
        "chars" => |mut args, _| {
            if args.len() != 1 {
                panic!(Err::IncorrectArgCount(
                    1,
                    args.len()
                ))
            }

            Variable::Array(
                match args.remove(0) {
                    Variable::Str(string) => string.par_chars().map(|x| Variable::Str(String::from(x))).collect(),
                    any => panic!(Err::VarTypeMismatch(
                        Variable::Str("".to_string()),
                        any
                    ))
                }
            )
        }
        "bytes" => |mut args, _| {
            if args.len() != 1 {
                panic!(Err::IncorrectArgCount(
                    1,
                    args.len()
                ))
            }

            Variable::Array(
                match args.remove(0) {
                    Variable::Str(string) => string.par_bytes().map(|x| Variable::Num(
                        {

                            match u8::try_into(x) {
                                Ok(res) => res,
                                Err(err) => panic!(err)
                            }
                        }
                    )).collect(),
                    any => panic!(Err::VarTypeMismatch(
                        Variable::Str("".to_string()),
                        any
                    ))
                }
            )
        }
        "exit" => |args, _| {
            std::process::exit(
                match &args[0] {
                    Variable::Num(number) => *number as i32,
                    _ => 1
                }
            )
        }
        "split" => |args, _| {
            if args.len() != 2 {
                panic!(
                    Err::IncorrectArgCount(
                        2,
                        args.len()
                    )
                )
            }

            let to_split = match args.get(0).unwrap() {
                Variable::Str(string) => string,
                any => panic!(
                    Err::VarTypeMismatch(
                        Variable::Str("".to_string()),
                        any.clone()
                    )
                )
            };

            let splitter = match args.get(1).unwrap() {
                Variable::Str(string) => string,
                any => panic!(
                    Err::VarTypeMismatch(
                        Variable::Str("".to_string()),
                        any.clone()
                    )
                )
            };

            Variable::Array(to_split.split(splitter).map(
                |x| Variable::Str(x.to_string())
            ).collect())
        }
    );

    map
}

/// Provides basic HTTP request support (GET, POST)
/// No other HTTP verbs supported (due to laziness :)
pub fn internet(map: &mut HashMap<String, Variable>) {
    macro_rules! insert_fn {
        (
				$(
					$name: expr => $val: expr
				)*
			) => {
            $( map.insert("internet_".to_string() + $name, Variable::NativeFunction($val)); )*
        };
    }
    insert_fn! {
        "get" => |mut args, _| {
            if args.len() != 1 {
                panic!(Err::IncorrectArgCount(1, args.len()))
            }
            let args = match args.remove(0) {
                Variable::Str(string) => string,
                any => panic!(Err::VarTypeMismatch(Variable::Str("string".to_string()), any))
            };
            Variable::Str(
                reqwest::blocking::get(args).unwrap().text().unwrap()
            )
        }
        "post" => |mut args, _| {
            if args.len() != 2 {
                panic!(Err::IncorrectArgCount(2, args.len()))
            }

            let url = match args.remove(0) {
                Variable::Str(string) => string,
                any => panic!(Err::VarTypeMismatch(
                    Variable::Str("".to_string()),
                    any
                ))
            };

            let body = match args.remove(0) {
                Variable::Str(body) => body,
                any => panic!(Err::VarTypeMismatch(
                    Variable::Str("".to_string()),
                    any
                ))
            };

            let client = reqwest::blocking::Client::new();
            let res = client.post(url)
                .body(body)
                .send()
                .unwrap()
                .text()
                .unwrap();

            Variable::Str(res)
        }
    };
}

/// Provides basic reading and writing I/O operations.
pub fn io(map: &mut HashMap<String, Variable>) {
    macro_rules! insert_fn {
        (
				$(
					$name: expr => $val: expr
				)*
			) => {
            $( map.insert("io_".to_string() + $name, Variable::NativeFunction($val)); )*
        };
    }

    insert_fn! {
        "read" => |mut args, _| {
            if args.len() != 1 {
                panic!(Err::IncorrectArgCount(1, args.len()))
            }

            let mut file = File::open(match args.remove(0) {
                Variable::Str(string) => string,
                any => panic!(Err::VarTypeMismatch(Variable::Str("string".to_string()), any))
            }).unwrap();

            let mut out = String::new();

            file.read_to_string(&mut out).unwrap();

            Variable::Str(out)
        }
        "write" => |mut args, _| {
            if args.len() != 2 {
                panic!(Err::IncorrectArgCount(1, args.len()))
            }

            let mut file = File::open(match args.remove(0) {
                Variable::Str(string) => string,
                any => panic!(Err::VarTypeMismatch(Variable::Str("string".to_string()), any))
            }).unwrap();

            let out = match args.remove(0) {
                Variable::Str(string) => {
                    let out = string.into_bytes();
                    out
                },
                any => panic!(Err::VarTypeMismatch(Variable::Str("string".to_string()), any))
            };

            file.write(&out).unwrap();

            Variable::Void
        }
        "input" => |args, _| {
            if let Some(arg) = args.get(0) {
                print!("{}", arg.to_string());
                std::io::stdout().flush().unwrap();
            }

            let mut out = String::new();

            match std::io::stdin().read_line(&mut out) {
                Ok(_) => return Variable::Str(out.trim_end().to_string()),
                Err(_) => {}
            };

            Variable::Void
        }
    }
}

pub fn html(map: &mut HashMap<String, Variable>) {
    macro_rules! insert_elem {
        (
				$(
					$name: expr => $val: expr
				)*
			) => {
            $( map.insert("el_".to_string() + $name, Variable::NativeFunction($val)); )*
        };
    }

    macro_rules! elem {
        ($($tag: expr),*) => {
            insert_elem! {
                    $($tag => |mut args, _| {
                    if args.len() == 1 {
                        args.push(
                            Variable::Str("".to_string())
                        )
                    }

                 if args.len() != 2 {
                    panic!(
                        Err::MissingArgs("el_".to_string() + $tag)
                    )
                }

                let mut args = args.into_iter();

                Variable::Str(format!(
                    "<{} {}>{}</{}>",
                    $tag,
                    args.next_back().unwrap(),
                    args.next().unwrap(),
                    $tag
                ))
            })*
                }
        };
    }

    insert_elem! {
        "custom" => |mut args, _| {
            if args.len() == 2 {
                args.push(
                    Variable::Str("".to_string())
                );
            }
            if args.len() != 3 {
                panic!(
                    Err::MissingArgs("el_".to_string() + "custom")
                )
            }

            let mut args = args.into_iter();

            let name = args.next().unwrap();

            Variable::Str(format!(
                    "<{} {}>{}</{}>",
                    name,
                    args.next_back().unwrap(),
                    args.next().unwrap(),
                    name
            ))
        }
        // For HTML comment
        "comment" => |args, _| {
            let mut comment_content = String::new();

            let mut args = args.into_iter();

            while let Some(arg) = args.next() {
                comment_content += &arg.to_string();
            }

            Variable::Str(format!("<!-- {} -->", comment_content))
        }
    }

    elem!(
        "p", "tag", "head", "title", "meta", "h1", "h2", "h3", "h4", "h5", "h6", "div", "span",
        "header", "code", "samp", "pre", "link", "a", "img", "script", "body"
    );
}
