#![feature(let_chains)]

use std::collections::HashMap;
use std::fs::read_to_string;

use std::{
    env::args,
    fmt::Debug,
    io::{self, BufRead, Write},
};

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(gp);
pub mod ast;

#[derive(Clone)]
enum Value {
    Integer(i64),
    String(String),
    Err,
}

impl Value {
    fn is_truthy(&self) -> bool {
        use Value::*;
        match self {
            Integer(x) => *x != 0,
            String(x) => x.len() != 0,
            Err => false,
        }
    }
}

impl Debug for Value {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Value::*;
        match self {
            Integer(v) => write!(fmt, "{:?}", v),
            String(v) => write!(fmt, "{:?}", v),
            Err => write!(fmt, "Err"),
        }
    }
}

macro_rules! binary_op {
    ($oper_2: tt, $l: expr, $r: expr, $vars: expr) => {
        {
            let first = $l.interpret($vars);
            let second = $r.interpret($vars);

            if let Value::Integer(x) = first && let Value::Integer(y) = second {
                Value::Integer(x $oper_2 y)
            } else {
                Value::Err
            }
        }

    }
}

impl ast::Node {
    fn interpret(&self, vars: &mut HashMap<String, Value>) -> Value {
        use ast::Node::*;
        use ast::Operation::*;
        match self {
            BinaryOp(l, op, r) => match op {
                Add => binary_op!(+, l, r, vars),
                Sub => binary_op!(-, l, r, vars),
                Mul => binary_op!(*, l, r, vars),
                Div => binary_op!(/, l, r, vars),
                Mod => binary_op!(%, l, r, vars),
            },
            UnaryNegate(v) => v.interpret(vars),
            Integer(v) => Value::Integer(*v),
            String(v) => Value::String(v.to_string()),
            Identifier(v) => vars.get(v).unwrap().clone(),
            Block(lst) => {
                let mut last_result = Value::Err;
                for i in lst {
                    last_result = i.interpret(vars);
                }
                last_result
            }
            VarSet(name, v) => {
                let interpreted = v.interpret(vars);
                vars.insert(name.to_string(), interpreted.clone());
                interpreted
            }
            While(cond, block) => {
                let mut val = cond.interpret(vars);
                let mut condv = val.is_truthy();

                let mut last_result = Value::Err;
                while condv == true {
                    last_result = block.interpret(vars);
                    val = cond.interpret(vars);
                    condv = val.is_truthy();
                }
                last_result
            }
        }
    }
}

fn main() {
    let args: Vec<String> = args().collect();

    let mut vars: HashMap<String, Value> = HashMap::new();

    if args.len() == 1 {
        print!("GP REPL.\n> ");
        let _ = io::stdout().flush();
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let parser = gp::GPParser::new();
            let line = line.unwrap();
            let root = parser.parse(&line);

            print!("{:?}\n> ", root.unwrap().interpret(&mut vars));
            let _ = io::stdout().flush();
        }
    } else {
        let str = read_to_string(args.get(1).unwrap()).unwrap();
        let parser = gp::GPParser::new();
        let root = parser.parse(&str);
        let root = root.unwrap();
        println!("Parsed: {:?}", root);
        let val = root.interpret(&mut vars);
        print!("{:?}\n", val);
    }
}
