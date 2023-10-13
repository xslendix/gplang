use std::{collections::HashMap, io::Write, rc::Rc};

use crate::{ast, interpreter::Value};

fn built_in_print(args: Vec<Value>) -> Value {
    for i in args {
        print!("{} ", i);
    }
    let _ = std::io::stdout().flush();
    println!("");

    Value::None
}

fn built_in_dprint(args: Vec<Value>) -> Value {
    for i in args {
        print!("{:?} ", i);
    }
    let _ = std::io::stdout().flush();
    println!("");

    Value::None
}

fn built_in_readlnn(args: Vec<Value>) -> Value {
    if args.len() >= 1 {
        if let Value::String(s) = args.get(0).unwrap() {
            let _ = std::io::stdout().write(s.as_bytes());
            let _ = std::io::stdout().flush();
        } else {
            panic!("Invalid argument to readn.");
        }
    }

    let mut input_line = String::new();
    std::io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read input line");
    let x: i64 = input_line
        .trim()
        .parse()
        .expect("Invalid input (not a integer)");

    Value::Integer(x)
}

pub fn add_builtins(vars: &mut HashMap<String, Value>) {
    vars.insert(
        String::from("print"),
        Value::Function(vec![], Rc::new(ast::Node::BuiltInNode(built_in_print))),
    );
    vars.insert(
        String::from("dprint"),
        Value::Function(vec![], Rc::new(ast::Node::BuiltInNode(built_in_dprint))),
    );
    vars.insert(
        String::from("readlnn"),
        Value::Function(vec![], Rc::new(ast::Node::BuiltInNode(built_in_readlnn))),
    );
}
