use std::{
    fmt,
    iter,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Location {
    pub file: String,
    pub line: u64,
    pub column: u64,
}

impl fmt::Display for Location {

    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "in {} ({}:{})", self.file, self.line, self.column)
    }

}

#[derive(Clone, Debug, Eq)]
pub struct Node<T> {
    pub val: T,
    pub loc: Location,
}

impl<T: fmt::Display> fmt::Display for Node<T> {

    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{} {}", self.val, self.loc)
    }

}

impl<T: PartialEq> PartialEq for Node<T> {

    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }

}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AstNode {
    Increment(u64),
    Decrement(u64),
    Next(u64),
    Previous(u64),
    PutChar(),
    GetChar(),
    Loop(Vec<Node<AstNode>>),
}

impl fmt::Display for AstNode {

    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", match *self {
            AstNode::Increment(n) =>
                iter::repeat(INC_CHAR as char)
                .take(n as usize)
                .collect::<String>(),
            AstNode::Decrement(n) =>
                iter::repeat(DEC_CHAR as char)
                .take(n as usize)
                .collect::<String>(),
            AstNode::Next(n) =>
                iter::repeat(NEXT_CHAR as char)
                .take(n as usize)
                .collect::<String>(),
            AstNode::Previous(n) =>
                iter::repeat(PREV_CHAR as char)
                .take(n as usize)
                .collect::<String>(),
            AstNode::PutChar() => (PUTC_CHAR as char).to_string(),
            AstNode::GetChar() => (GETC_CHAR as char).to_string(),
            AstNode::Loop(ref lp) => {
                let mut ops = String::new();
                for op in lp {
                    ops += &op.to_string();
                }
                (LOOP_START_CHAR as char).to_string()
                + &ops
                + &(LOOP_END_CHAR as char).to_string()
            },
        })
    }

}

pub const INC_CHAR: u8 = b'+';
pub const DEC_CHAR: u8 = b'-';
pub const NEXT_CHAR: u8 = b'>';
pub const PREV_CHAR: u8 = b'<';
pub const PUTC_CHAR: u8 = b'.';
pub const GETC_CHAR: u8 = b',';
pub const LOOP_START_CHAR: u8 = b'[';
pub const LOOP_END_CHAR: u8 = b']';

