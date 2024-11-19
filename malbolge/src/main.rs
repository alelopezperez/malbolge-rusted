use std::{char, env};
fn main() {
    let args = env::args().collect::<Vec<String>>();
    let (command, args) = args.split_first().expect("expected the command to run");

    if args.is_empty() {
        eprintln!("Usage: {command} <mabolge_file>");
    }

    let file_path = &args[0];
    let file = std::fs::read_to_string(file_path).expect("Error Reading the File");

    let chars = file
        .chars()
        .filter(|a| !a.is_whitespace())
        .collect::<Vec<_>>();
    process_characters(&chars);
    let letter = TABLE.chars().nth(40 - 33).unwrap();
    println!(" El char es {}", char::from_u32((40 - 33) % 94).unwrap());
    println!("{}", letter);
}

fn process_characters(chars: &Vec<char>) {
    for c in chars {
        if (*c as usize) < 33 || (*c as usize) > 126 {
            println!("{} NOOOO", c);
        } else {
            println!("{}", *c as usize)
        }
    }
}
fn transform_char(char: &char) -> Result<char, ()> {
    let char = TABLE.chars().nth(40 - 33).unwrap();
    todo!()
}
