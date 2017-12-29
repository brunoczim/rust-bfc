pub mod bstream;
pub mod syntax;


pub use self::bstream::ByteStream;
pub use self::syntax::{
    Location,
    Node,
    AstNode
};


use utils::{
    HeadedList,
};
use std::{
    fmt,
};
use std::error::{
    Error,
};

#[derive(Clone, Debug)]
pub struct ParseError {
    message: String,
    loc: Location,
}

impl fmt::Display for ParseError {

    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}: {}", self.message, self.loc)
    }

}

impl Error for ParseError {

    fn description(&self) -> &str {
        &self.message
    }

}

impl ParseError {

    pub fn new(message: String, loc: Location) -> Self {
        Self {message, loc}
    }

    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn loc(&self) -> &Location {
        &self.loc
    }

}

#[derive(Clone, Debug)]
struct Loop {
    ops: Vec<Node<AstNode>>,
    loc: Location,
}

pub fn parse(mut stream: ByteStream) -> Result<Vec<Node<AstNode>>, Vec<ParseError>> {
    let mut errs = Vec::new();
    let mut loops = HeadedList::new(Loop {
        ops: Vec::new(),
        loc: stream.loc(),
    }, None);
    while let Some(ch) = stream.current() {
        macro_rules! repeated_op {
            ($cons:path) => {{
                let loc = stream.loc();
                let mut count = 0;
                loop {
                    count += 1;
                    stream.next();
                    if stream.current() != Some(ch) {break}
                }
                loops.val_mut().ops.push(Node {
                    val: $cons(count),
                    loc,
                });
            }};
        }
        match ch {
            syntax::INC_CHAR => repeated_op!(AstNode::Increment),
            syntax::DEC_CHAR => repeated_op!(AstNode::Decrement),
            syntax::NEXT_CHAR => repeated_op!(AstNode::Next),
            syntax::PREV_CHAR => repeated_op!(AstNode::Previous),
            syntax::PUTC_CHAR => {
                loops.val_mut().ops.push(Node {
                    val: AstNode::PutChar(),
                    loc: stream.loc(),
                });
                stream.next();
            },
            syntax::GETC_CHAR => {
                loops.val_mut().ops.push(Node {
                    val: AstNode::GetChar(),
                    loc: stream.loc(),
                });
                stream.next();
            },
            syntax::LOOP_START_CHAR => {
                loops.receive(Loop {
                    ops: Vec::new(),
                    loc: stream.loc()
                });
                stream.next();
            },
            syntax::LOOP_END_CHAR => {
                match loops.take() {
                    Some(lp) => {
                        loops.val_mut().ops.push(Node {
                            val: AstNode::Loop(lp.ops),
                            loc: lp.loc,
                        });
                    },
                    _ => errs.push(ParseError {
                        message: String::from("No loop to terminate"),
                        loc: stream.loc(),
                    }),
                }
                stream.next();
            },
            _ => {
                stream.next();
            },
        }
    }
    while let Some(lp) = loops.take() {
        errs.push(ParseError {
            message: String::from("Unterminated loop"),
            loc: lp.loc,
        });
    }
    if errs.len() > 0 {Err(errs)} else {Ok(loops.reclaim_val().ops)}
}


