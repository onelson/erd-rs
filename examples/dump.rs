//! Little program that reads an ERD file and spits out the tree of nodes that
//! have been parsed.

use std::fs::File;
use std::io::Read;

fn main() {
    let fp = std::env::args().nth(1).unwrap();
    let mut erd_file = File::open(&fp).unwrap();
    let mut buf = String::new();
    erd_file.read_to_string(&mut buf).unwrap();
    let er = erd_rs::parser::parse(&buf).unwrap();
    println!("{:?}", er);
}
