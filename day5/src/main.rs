extern crate day5;

use day5::intcode::IntcodeProgram;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn run_test_program(test_data: &String, prompt: &String) {
    println!("Running test program ({})...", prompt);
    let mut prg = IntcodeProgram::new(test_data).unwrap_or_else(|err| {
        eprintln!("Problem creating Intcode program from input: {}", err);
        process::exit(1);
    });
    if let Err(e) = prg.run() {
        eprintln!("Error occured during program execution: {}", e);
        process::exit(1);
    }
}

fn main() {
    // run test programs for part two
    run_test_program(
        &"3,9,8,9,10,9,4,9,99,-1,8".to_owned(),
        &"pos. mode equals: 1 if input = 8, else 0".to_owned(),
    );
    run_test_program(
        &"3,3,1108,-1,8,3,4,3,99".to_owned(),
        &"imm. mode equals: 1 if input = 8, else 0".to_owned(),
    );
    run_test_program(
        &"3,9,7,9,10,9,4,9,99,-1,8".to_owned(),
        &"pos. mode less than: 1 if input < 8, else 0".to_owned(),
    );
    run_test_program(
        &"3,3,1107,-1,8,3,4,3,99".to_owned(),
        &"imm. mode less than: 1 if input < 8, else 0".to_owned(),
    );
    run_test_program(
        &"3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9".to_owned(),
        &"pos. mode jump: 1 if input non-zero, else 0".to_owned(),
    );
    run_test_program(
        &"3,3,1105,-1,9,1101,0,0,12,4,12,99,1".to_owned(),
        &"imm. mode jump: 1 if input non-zero, else 0".to_owned(),
    );
    run_test_program(
        &"3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"
            .to_owned(),
        &"999 if input < 8, 1000 if input = 8, 1001 if input > 8".to_owned(),
    );

    // read in problem input
    println!("Running problem program...");
    let args: Vec<String> = env::args().collect();
    let mut f = File::open(&args[1]).expect("File not found!");
    let mut data = String::new();
    f.read_to_string(&mut data)
        .expect("Something went wrong while reading the file!");

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
