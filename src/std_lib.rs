use std::collections::HashMap;
use std::io::Write;

use crate::data_types::*;

use crate::errors::Err;

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
        "eq" => |args, _| {
            let mut args = args.into_iter();
            let mut last = args.next().unwrap();
            while let Some(arg) = args.next() {
                if arg != last {
                    return Variable::Bool(false)
                }
                last = arg;
            }
            Variable::Bool(true)
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
                panic!("Incorrect argument count")
            }
            let args = match args.remove(0) {
                Variable::Str(string) => string,
                any => panic!("Incorrect type: {}", any)
            };
            Variable::Str(
                reqwest::blocking::get(args).unwrap().text().unwrap()
            )
        }
        "post" => |mut args, _| {
            if args.len() != 2 {
                panic!("Incorrect argument count")
            }

            let url = match args.remove(0) {
                Variable::Str(string) => string,
                any => panic!("Argument mismatch")
            };

            let body = match args.remove(0) {
                Variable::Str(body) => body,
                any => panic!("Argument mismatch")
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
