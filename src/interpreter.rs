use std::collections::HashMap;
use std::fmt::Display;
use std::{fmt::Debug, rc::Rc};

use crate::ast::{self, Node};

#[derive(Clone)]
pub enum Value {
    Integer(i64),
    String(String),
    Err(String),
    None,
    Function(Vec<String>, Rc<ast::Node>),
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        use Value::*;
        match self {
            Integer(x) => *x != 0,
            String(x) => x.len() != 0,
            Err(_) | None => false,
            Function(_, _) => true,
        }
    }
}

impl Debug for Value {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Value::*;
        match self {
            Integer(v) => {
                write!(fmt, "8{}D", "=".to_string().repeat(*v as usize))
            }
            String(v) => write!(fmt, "/*{}*/", v),
            Err(err) => write!(fmt, "Err({})", err),
            None => write!(fmt, "None"),
            Function(args, _) => write!(fmt, "😭({:?})", args),
        }
    }
}

impl Display for Value {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Value::*;
        match self {
            Integer(v) => write!(fmt, "{}", v),
            String(v) => write!(fmt, "{}", v),
            Err(err) => write!(fmt, "Err({})", err),
            None => write!(fmt, "None"),
            Function(args, _) => write!(fmt, "Fn({:?})", args),
        }
    }
}

macro_rules! binary_op {
    ($oper_2: tt, $l: expr, $r: expr, $vars: expr, $values: expr) => {
        {
            let first = $l.interpret($vars, $values);
            let second = $r.interpret($vars, $values);

            if let Value::Integer(x) = first && let Value::Integer(y) = second {
                Value::Integer(x $oper_2 y)
            } else {
                Value::Err("Not an integer".to_string())
            }
        }

    }
}

impl ast::Node {
    pub fn interpret(&self, vars: &mut HashMap<String, Value>, values: &Vec<Value>) -> Value {
        use ast::Node::*;
        use ast::Operation::*;
        match self {
            BinaryOp(l, op, r) => match op {
                Add => binary_op!(+, l, r, vars, values),
                Sub => binary_op!(-, l, r, vars, values),
                Mul => binary_op!(*, l, r, vars, values),
                Div => binary_op!(/, l, r, vars, values),
                Mod => binary_op!(%, l, r, vars, values),
            },
            UnaryNegate(v) => v.interpret(vars, values),
            Integer(v) => Value::Integer(*v),
            String(v) => Value::String(v.to_string()),
            Identifier(v) => {
                if v == "none" {
                    Value::None
                } else if v == "err" {
                    Value::Err("?".to_string())
                } else {
                    vars.get(v).unwrap().clone()
                }
            }
            Block(lst) => {
                let mut last_result = Value::None;
                for i in lst {
                    last_result = i.interpret(vars, values);
                }
                last_result
            }
            VarSet(name, v) => {
                let interpreted = v.interpret(vars, values);
                vars.insert(name.to_string(), interpreted.clone());
                interpreted
            }
            While(cond, block) => {
                let mut val = cond.interpret(vars, values);
                let mut condv = val.is_truthy();

                let mut last_result = Value::None;
                while condv == true {
                    last_result = block.interpret(vars, values);
                    val = cond.interpret(vars, values);
                    condv = val.is_truthy();
                }
                last_result
            }
            IfElse(expr, real, fake) => {
                if expr.interpret(vars, values).is_truthy() {
                    real.interpret(vars, values)
                } else {
                    if let Some(x) = fake {
                        x.interpret(vars, values)
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
                None => Value::Err("Invalid function name".to_string()),
                Some(val) => match val {
                    Value::Function(args_func, block) => {
                        if let Node::BuiltInNode(builtin_function) = **block {
                            let mut val = Vec::<Value>::new();
                            for i in args {
                                val.push(i.interpret(vars, &values))
                            }
                            builtin_function(val)
                        } else {
                            if args.len() != args_func.len() {
                                return Value::Err("Not the same amount of arguments".to_string());
                            }

                            let mut i = 0;
                            for arg_name in args_func {
                                let interpreted = args.get(i).unwrap().interpret(vars, values);
                                vars.insert(arg_name.clone(), interpreted);
                                i += 1;
                            }

                            block.interpret(vars, values)
                        }
                    }
                    _ => Value::Err("Invalid function value".to_string()),
                },
            },
            BuiltInNode(_) => unreachable!(),
        }
    }
}
