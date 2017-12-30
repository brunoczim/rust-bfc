use std::io::{
    Error,
};
use front_end::{
    AstNode,
    Node,
};

pub enum Format {
    Asm,
    Elf,
}

pub trait Arch {

    fn generate(
        &self,
        ast: Vec<Node<AstNode>>,
        format: Format,
        out: String
    ) -> Result<usize, Error>;

}

