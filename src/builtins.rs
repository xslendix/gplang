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

pub fn add_builtins(vars: &mut HashMap<String, Value>) {
    vars.insert(
        String::from("print"),
        Value::Function(vec![], Rc::new(ast::Node::BuiltInNode(built_in_print))),
    );
    vars.insert(
        String::from("dprint"),
        Value::Function(vec![], Rc::new(ast::Node::BuiltInNode(built_in_dprint))),
    );
}
