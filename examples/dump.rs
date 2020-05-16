//! Little program that reads an ERD file and spits out the tree of nodes that
//! have been parsed.

use erd_rs::{ErdParser, Parser, Rule};
use std::fs::File;
use std::io::Read;

fn main() {
    let fp = std::env::args().nth(1).unwrap();
    let mut erd_file = File::open(&fp).unwrap();
    let mut buf = String::new();
    erd_file.read_to_string(&mut buf);
    let mut root_pairs = ErdParser::parse(Rule::document, &buf).unwrap();

    for pair in root_pairs.next().unwrap().into_inner() {
        println!("Rule: {:?}", pair.as_rule());
        println!("{:?}", &pair);
    }
}
