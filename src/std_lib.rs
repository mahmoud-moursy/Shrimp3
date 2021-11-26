use std::collections::HashMap;

use crate::data_types::Variable;
use crate::nodes::Node;

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
        "hello_world" => |args, vars| {
            println!("Hello world!");
            Variable::Void
        }
        "print" => |args, vars| {
            for i in args {
                print!("{}", i)
            }
            Variable::Void
        }
    );

    map
}
