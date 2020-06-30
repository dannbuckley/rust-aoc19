use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::vec::Vec;

#[derive(Debug)]
struct IntcodeProgram {
    data: Vec<u32>,
}

impl IntcodeProgram {
    /**
     * Executes an Intcode program.
     */
    fn run(&mut self) {
        // initialize instruction pointer to 0
        let mut ip: usize = 0;
        loop {
            if self.data[ip] == 99 {
                // break on exit opcode
                break;
            }

            // extract addresses from current instruction
            let op_l = self.data[ip + 1] as usize;
            let op_r = self.data[ip + 2] as usize;
            let addr = self.data[ip + 3] as usize;

            if self.data[ip] == 1 {
                // add values of left and right parameters
                // and store sum at addr
                let sum = self.data[op_l] + self.data[op_r];
                self.data[addr] = sum;
            } else if self.data[ip] == 2 {
                // multiply values of left and right parameters
                // and store product at addr
                let prod = self.data[op_l] * self.data[op_r];
                self.data[addr] = prod;
            }

            // advance to next instruction
            ip += 4;
        }
    }
}

/**
 * Constructs an IntcodeProgram object from the given program string.
 */
fn build_intcode_program(prg: String) -> IntcodeProgram {
    // split program into vector of values
    let values: Vec<_> = prg.split(',').collect();
    let mut data: Vec<u32> = Vec::<u32>::new();
    for value in values {
        // parse value strings as 32-bit unsigned ints
        // and push to program data vector
        let v = value.parse::<u32>().unwrap();
        data.push(v);
    }

    // create IntcodeProgram object with parsed program data
    IntcodeProgram { data }
}

/**
 * Tests the given program against the given starting and ending conditions
 *  to ensure proper functionality of the build_intcode_program() and
 *  IntcodeProgram::run() functions.
 */
fn test_intcode_program(prg: String, initial_state: &[u32], final_state: &[u32]) {
    let mut test_prg = build_intcode_program(prg);
    println!("Test program: {:?}", test_prg);

    println!("\tVerifying initial program data...");
    assert_eq!(test_prg.data, initial_state);

    println!("\tRunning program...");
    test_prg.run();

    println!("\tVerifying final program state...");
    assert_eq!(test_prg.data, final_state);

    println!("\tSuccess!");
}

/**
 * Finds the noun and verb pair (using brute force) that produces 19690720 when
 *  the problem input is executed.
 */
fn find_input(prg_data: Vec<u32>) -> (u32, u32) {
    for i in 0..100 {
        for j in 0..100 {
            // make a new program object with the given data
            let mut temp_prg = IntcodeProgram {
                data: prg_data.to_vec(),
            };

            // set i as noun and j as verb and run program
            temp_prg.data[1] = i;
            temp_prg.data[2] = j;
            temp_prg.run();

            if temp_prg.data[0] == 19690720 {
                // if value is correct, return noun-verb pair
                return (i, j);
            }
        }
    }

    // pair not found
    (0, 0)
}

fn main() {
    // run test programs
    test_intcode_program(
        "1,9,10,3,2,3,11,0,99,30,40,50".to_owned(),
        &[1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
        &[3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
    );
    test_intcode_program(
        "1,0,0,0,99".to_owned(),
        &[1, 0, 0, 0, 99],
        &[2, 0, 0, 0, 99],
    );
    test_intcode_program(
        "2,3,0,3,99".to_owned(),
        &[2, 3, 0, 3, 99],
        &[2, 3, 0, 6, 99],
    );
    test_intcode_program(
        "2,4,4,5,99,0".to_owned(),
        &[2, 4, 4, 5, 99, 0],
        &[2, 4, 4, 5, 99, 9801],
    );
    test_intcode_program(
        "1,1,1,4,99,5,6,0,99".to_owned(),
        &[1, 1, 1, 4, 99, 5, 6, 0, 99],
        &[30, 1, 1, 4, 2, 5, 6, 0, 99],
    );

    // read problem input file
    let args: Vec<String> = env::args().collect();
    let mut f = File::open(&args[1]).expect("File not found!");
    let mut prg = String::new();
    f.read_to_string(&mut prg)
        .expect("Something went wrong while reading the file!");

    // build and run Intcode program from problem input
    let mut int_prg = build_intcode_program(prg);

    // part two
    let (noun, verb) = find_input(int_prg.data.to_vec());
    println!(
        "Noun-verb pair that produces 19690720: ({}, {})",
        noun, verb
    );
    println!("100 * noun + verb: {}", (100 * noun) + verb);

    // part one
    int_prg.data[1] = 12;
    int_prg.data[2] = 2;
    int_prg.run();
    println!(
        "Value at position 0 with noun-verb pair (12, 2): {}",
        int_prg.data[0]
    );
}
