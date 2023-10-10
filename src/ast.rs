use std::fmt::{Debug, Formatter};
use std::rc::Rc;

pub enum Node {
    BinaryOp(Rc<Node>, Operation, Rc<Node>),
    UnaryNegate(Rc<Node>),
    Integer(i64),
    String(String),
    Identifier(String),
    Block(Vec<Rc<Node>>),

    VarSet(String, Rc<Node>),
    While(Rc<Node>, Rc<Node>),
    IfElse(Rc<Node>, Rc<Node>, Option<Rc<Node>>),
    FunctionDefinition(String, Vec<String>, Rc<Node>),
    FunctionCall(String, Vec<Rc<Node>>),
}

impl Debug for Node {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        use self::Node::*;
        match self {
            BinaryOp(l, op, r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            UnaryNegate(v) => write!(fmt, "-{:?}", v),
            Integer(v) => write!(fmt, "{:?}", v),
            String(v) => write!(fmt, "{:?}", v),
            Identifier(v) => write!(fmt, "{}", v),
            Block(v) => write!(fmt, "Block{:?}", v),
            VarSet(name, v) => write!(fmt, "{} = {:?}", name, v),
            While(v, block) => write!(fmt, "While({:?})->{:?}", v, block),
            IfElse(a, b, c) => write!(fmt, "If({:?})->{:?}->{:?}", a, b, c),
            FunctionDefinition(name, args, block) => {
                write!(fmt, "Fn({}, {:?}, {:?})", name, args, block)
            }
            FunctionCall(a, b) => write!(fmt, "{}({:?})", a, b),
        }
    }
}

pub enum Operation {
    Add,
    Sub,

    Mul,
    Div,
    Mod,
}

impl Debug for Operation {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        use self::Operation::*;
        match self {
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Mod => write!(fmt, "%"),
        }
    }
}
