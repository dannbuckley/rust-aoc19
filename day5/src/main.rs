extern crate day5;

use day5::intcode::IntcodeProgram;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn main() {
    // read in problem input
    let args: Vec<String> = env::args().collect();
    let mut f = File::open(&args[1]).expect("File not found!");
    let mut data = String::new();
    f.read_to_string(&mut data).expect("Something went wrong while reading the file!");

    // execute problem program
    let mut prg = IntcodeProgram::new(&data).unwrap_or_else(|err| {
        eprintln!("Problem creating Intcode program from input: {}", err);
        process::exit(1);
    });
    if let Err(e) = prg.run() {
        eprintln!("Error occured during program execution: {}", e);
        process::exit(1);
    }
}
