use lib_malbolge::{self, VM};
use std::{
    char, env,
    io::{Read, Write},
    process::Output,
};
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
    while let Ok(state) = vm.exec() {
        match state {
            lib_malbolge::ExecState::Output(output) => {
                let mut out = std::io::stdout();
                out.write_all(&[output])
                    .expect("Failed to write to buffer.");
                let _ = out.flush();
            }
            lib_malbolge::ExecState::Input => {
                let mut buf: [u8; 1] = Default::default();
                let mut stdin = std::io::stdin();
                let handle = stdin.read(&mut buf).unwrap();
                let input = buf[0];

                if buf.is_empty() {
                    vm.input_instruction(input, 0)
                } else {
                    vm.input_instruction(input, handle);
                }
            }
            lib_malbolge::ExecState::Finished => {
                break;
            }
            _ => {}
        }
    }
}
