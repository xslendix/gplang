use std::collections::HashMap;
use std::{fmt::Debug, rc::Rc};

use crate::ast;

#[derive(Clone)]
pub enum Value {
    Integer(i64),
    String(String),
    Err,
    None,
    Function(Vec<String>, Rc<ast::Node>),
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        use Value::*;
        match self {
            Integer(x) => *x != 0,
            String(x) => x.len() != 0,
            Err | None => false,
            Function(_, _) => true,
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
            None => write!(fmt, "None"),
            Function(args, _) => write!(fmt, "Fn({:?})", args),
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
    pub fn interpret(&self, vars: &mut HashMap<String, Value>) -> Value {
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
            IfElse(expr, real, fake) => {
                if expr.interpret(vars).is_truthy() {
                    real.interpret(vars)
                } else {
                    if let Some(x) = fake {
                        x.interpret(vars)
                    } else {
                        Value::None
                    }
                }
            }

            FunctionDefinition(name, args, block) => {
                vars.insert(name.clone(), Value::Function(args.clone(), block.clone()));
                vars.get(name).unwrap().clone()
            }
            FunctionCall(name, args) => match vars.clone().get(name) {
                None => Value::Err,
                Some(val) => match val {
                    Value::Function(args_func, block) => {
                        if args.len() != args_func.len() {
                            return Value::Err;
                        }

                        let mut i = 0;
                        for arg_name in args_func {
                            let interpreted = args.get(i).unwrap().interpret(vars);
                            vars.insert(arg_name.clone(), interpreted);
                            i += 1;
                        }

                        block.interpret(vars)
                    }
                    _ => Value::Err,
                },
            },
        }
    }
}