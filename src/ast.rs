use std::fmt::{Debug, Formatter};

pub enum Node {
    BinaryOp(Box<Node>, Operation, Box<Node>),
    UnaryNegate(Box<Node>),
    Integer(i64),
    String(String),
    Identifier(String),
    Block(Vec<Box<Node>>),

    VarSet(String, Box<Node>),
    While(Box<Node>, Box<Node>),
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
