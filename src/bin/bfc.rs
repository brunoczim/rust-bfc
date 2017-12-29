extern crate bfc;

use std::env::{
    args
};
use std::process::{
    exit,
};

fn main() {
    exit(bfc::main(args()))
}
