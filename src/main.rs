#![feature(let_chains)]
use std::collections::HashMap;
use std::fs::read_to_string;

use std::{
    env::args,
    io::{self, BufRead, Write},
};

use lalrpop_util::lalrpop_mod;
use regex::Regex;

lalrpop_mod!(gp);
pub mod ast;
pub mod builtins;
pub mod interpreter;

use interpreter::*;

use crate::builtins::add_builtins;

fn main() {
    let args: Vec<String> = args().collect();

    let mut vars: HashMap<String, Value> = HashMap::new();
    add_builtins(&mut vars);
    let reg = Regex::new(r#""[^"]*""#).unwrap();

    if args.len() == 1 {
        print!("GP REPL.\n> ");
        let _ = io::stdout().flush();
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let parser = gp::GPParser::new();
            let line = &line.unwrap();
            let line = reg.replace_all(line, "");
            let root = parser.parse(&line);

            print!(
                "{:?}\n> ",
                root.unwrap().interpret(&mut vars, &Vec::<Value>::new())
            );
            let _ = io::stdout().flush();
        }
    } else {
        let str = read_to_string(args.get(1).unwrap()).unwrap();
        let str = reg.replace_all(&str, "");
        let parser = gp::GPParser::new();
        let root = parser.parse(&str);
        let root = root.unwrap();
        let val = root.interpret(&mut vars, &Vec::<Value>::new());
        //print!("{:?}\n", val);
    }
}
