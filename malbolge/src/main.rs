use lib_malbolge::{self, VM};
use std::{char, env};
fn main() {
    let args = env::args().collect::<Vec<String>>();
    let (command, args) = args.split_first().expect("expected the command to run");

    if args.is_empty() {
        eprintln!("Usage: {command} <mabolge_file>");
    }

    let file_path = &args[0];
    let file = std::fs::read_to_string(file_path).expect("Error Reading the File");

    println!("{}", file);

    let mut vm = VM::load(file).unwrap();
    vm.exec();
}
