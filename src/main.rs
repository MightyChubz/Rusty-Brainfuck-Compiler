pub mod compiler;

use compiler::intermediate;
use std::env;
use std::fs;
use std::ffi;
use std::process;

fn main() {
    let args: Vec<ffi::OsString> = env::args_os().collect();

    let file = fs::read_to_string(&args[1]).unwrap();
    let file: &str = file.trim();
    let mut tokens = intermediate::tokenize_input(file);
    println!("Tokenized input...");

    let intermediate_input = intermediate::generate_intermediate_data(&mut tokens);
    fs::write("out.basm", &intermediate_input).unwrap();
    println!("Generated intermediate data...");

    let source = compiler::generate_output(&intermediate_input);
    fs::write("output.c", &source).unwrap();
    println!("Generated C file for gcc...");

    // Compile the output to an exe.
    let output = process::Command::new("gcc")
                                    .args(&["output.c", "-o", "brain.exe", "-O3"])
                                    .output()
                                    .expect("Failed to execute process");
    println!("warning and error output: \n{}", String::from_utf8(output.stdout).unwrap());
    println!("Program compiled...");
}
