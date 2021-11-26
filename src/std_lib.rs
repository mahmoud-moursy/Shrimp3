use std::collections::HashMap;

use crate::data_types::*;
use crate::nodes::Node;
use crate::tokens::Token;

use anyhow::bail;

use std::io::Write;

use crate::errors::Err;

pub fn construct_lib() -> HashMap<String, Variable> {
    let mut map: HashMap<String, Variable> = HashMap::new();

    map.insert("void".to_string(), Variable::Void);
    map.insert("true".to_string(), Variable::Bool(true));
    map.insert("false".to_string(), Variable::Bool(false));

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
        "decl" => |mut args, vars| {
            if args.get(0).is_some() && args.get(1).is_some() {
                return if let Some(var) = vars.insert(
                    match args.remove(0) {
                        Variable::Ident(id) => id,
                        any => todo!("{}", any)
                    },
                    args.remove(0)
                ) { var } else { Variable::Void }
            }
            panic!("{}", Err::MissingArgs("ident".to_string()))
        }
        "add" => |args, _| {
            let mut args = args.into_iter();

            let mut final_out = 0.0;

            while let Some(arg) = args.next() {
                match arg {
                    Variable::Num(num) => final_out += num,
                    any => todo!("{}", any)
                }
            }

            Variable::Num(final_out)
        }
        "sub" => |args, _| {
            let mut args = args.into_iter();

            let mut final_out = match args.next() {
                Some(var) => match var {
                    Variable::Num(num) => num,
                    any => todo!("{}", any)
                },
                None => panic!("{}", Err::MissingArgs("sub".to_string()))
            };

            while let Some(arg) = args.next() {
                match arg {
                    Variable::Num(num) => final_out -= num,
                    any => todo!("{}", any)
                }
            }

            Variable::Num(final_out)
        }
        "mult" => |args, _| {
            let mut args = args.into_iter();

            let mut final_out = match args.next() {
                Some(var) => match var {
                    Variable::Num(num) => num,
                    any => todo!("{}", any)
                },
                None => panic!("{}", Err::MissingArgs("mult".to_string()))
            };

            while let Some(arg) = args.next() {
                match arg {
                    Variable::Num(num) => final_out *= num,
                    any => todo!("{}", any)
                }
            }

            Variable::Num(final_out)
        }
        "div" => |args, _| {
            let mut args = args.into_iter();

            let mut final_out = match args.next() {
                Some(var) => match var {
                    Variable::Num(num) => num,
                    any => todo!("{}", any)
                },
                None => panic!("{}", Err::MissingArgs("div".to_string()))
            };

            while let Some(arg) = args.next() {
                match arg {
                    Variable::Num(num) => final_out /= num,
                    any => todo!("{}", any)
                }
            }

            Variable::Num(final_out)
        }
        "pow" => |args, _| {
            let mut args = args.into_iter();

            let mut final_out = match args.next() {
                Some(var) => match var {
                    Variable::Num(num) => num,
                    any => todo!("{}", any)
                },
                None => panic!("{}", Err::MissingArgs("div".to_string()))
            };

            while let Some(arg) = args.next() {
                match arg {
                    Variable::Num(num) => final_out = final_out.powf(num),
                    any => todo!("{}", any)
                }
            }

            Variable::Num(final_out)
        }
    );

    map
}
