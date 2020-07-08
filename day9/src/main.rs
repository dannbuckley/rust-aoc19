extern crate day9;

use day9::intcode::IntcodeProgram;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // read in problem input
    println!("Running problem program...");
    let args: Vec<String> = env::args().collect();
    let mut f = File::open(&args[1]).expect("File not found!");
    let mut data = String::new();
    f.read_to_string(&mut data)
        .expect("Something went wrong while reading the file!");

    // create and run program for problem input
    let mut prg = IntcodeProgram::new(&data, None).unwrap();
    prg.run().unwrap();
}
