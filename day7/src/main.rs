extern crate day7;

use day7::intcode::IntcodeProgram;
use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::mem;
use std::process;
use std::vec::Vec;

fn swap<T>(x: &mut Vec<T>, i: usize, j: usize) {
    // get proper ordering of elements
    let (lo, hi) = match i.cmp(&j) {
        Ordering::Less => (i, j),
        Ordering::Greater => (j, i),
        _ => return,
    };

    // swap elements
    let (init, tail) = x.split_at_mut(hi);
    mem::swap(&mut init[lo], &mut tail[0]);
}

#[derive(Debug)]
struct PhaseSettingPermutation {
    elements: Vec<i32>,
}

/// Finds the largest mobile element for a Johnson-Trotter permutation
fn find_largest_mobile_element(elements: &Vec<i32>, directions: &Vec<bool>) -> Option<usize> {
    // initialize index of largest element
    let mut k: i32 = 9;
    let mut k_ind: usize = 0;

    // find largest element
    while k >= 5 {
        // find index of element within vector
        for i in 0..5 {
            if elements[i] == k {
                k_ind = i;
                break;
            }
        }

        // check if element is mobile
        if directions[(k - 5) as usize] {
            // element going right
            if k_ind < 4 && elements[k_ind] > elements[k_ind + 1] {
                break;
            }
        } else {
            // element going left
            if k_ind > 0 && elements[k_ind] > elements[k_ind - 1] {
                break;
            }
        }

        // current element not mobile
        // check next largest element
        k -= 1;
    }

    if k == 4 {
        // no mobile elements exist
        return None;
    }

    Some(k_ind)
}

/// Generates Johnson-Trotter permutations of phase settings
fn johnson_trotter() -> Vec<PhaseSettingPermutation> {
    // initialize permutation and direction vectors
    let mut p = PhaseSettingPermutation {
        elements: vec![5, 6, 7, 8, 9],
    };
    let mut directions: Vec<bool> = vec![false; 5];

    // initialize list of permutations
    let mut permutations: Vec<PhaseSettingPermutation> = Vec::new();
    permutations.push(PhaseSettingPermutation {
        elements: p.elements.to_vec(),
    });

    loop {
        // find current largest mobile element
        let k = match find_largest_mobile_element(&p.elements, &directions) {
            Some(k_ind) => k_ind,
            // if None, no more mobile elements
            None => break,
        };

        // swap k with adjacent element (indicated by k's direction)
        let k_val = p.elements[k];
        if directions[(k_val - 5) as usize] {
            // swap with element to the right
            swap(&mut p.elements, k, k + 1);
        } else {
            // swap with element to the left
            swap(&mut p.elements, k - 1, k);
        }

        // reverse direction of all elements larger than k_val
        for i in (k_val - 4)..5 {
            directions[i as usize] = !directions[i as usize];
        }

        // add new permutation to list
        permutations.push(PhaseSettingPermutation {
            elements: p.elements.to_vec(),
        });
    }

    permutations
}

fn main() {
    // generate all possible permutations of phase settings
    let phase_configs = johnson_trotter();

    // read in problem input
    println!("Running problem program...");
    let args: Vec<String> = env::args().collect();
    let mut f = File::open(&args[1]).expect("File not found!");
    let mut data = String::new();
    f.read_to_string(&mut data)
        .expect("Something went wrong while reading the file!");

    // define closure to handle IntcodeProgram errors
    let handle_except = |err| {
        eprintln!("Problem creating Intcode program from input: {}", err);
        process::exit(1);
    };

    // define closure to handle input injection errors
    let inject_prg_input = |prg: &mut IntcodeProgram, value: i32| {
        if let Err(e) = prg.inject_input(value) {
            eprintln!("{}", e);
        }
    };

    // define closure to handle run-until-input-operation errors
    let run_prg_until_input = |prg: &mut IntcodeProgram| {
        if let Err(e) = prg.run_until_input() {
            eprintln!("{}", e);
        }
    };

    // current output, maximum final output, and index of max permutation
    let mut max_final: i32 = 0;
    let mut max_ind: usize = 0;

    // find maximum output permutation
    for i in 0..120 {
        // initialize amplifier programs with the current phase setting permutation
        let mut amp_a = IntcodeProgram::new(&data, Some(vec![])).unwrap_or_else(handle_except);
        inject_prg_input(&mut amp_a, phase_configs[i].elements[0]);
        run_prg_until_input(&mut amp_a);

        let mut amp_b = IntcodeProgram::new(&data, Some(vec![])).unwrap_or_else(handle_except);
        inject_prg_input(&mut amp_b, phase_configs[i].elements[1]);
        run_prg_until_input(&mut amp_b);

        let mut amp_c = IntcodeProgram::new(&data, Some(vec![])).unwrap_or_else(handle_except);
        inject_prg_input(&mut amp_c, phase_configs[i].elements[2]);
        run_prg_until_input(&mut amp_c);

        let mut amp_d = IntcodeProgram::new(&data, Some(vec![])).unwrap_or_else(handle_except);
        inject_prg_input(&mut amp_d, phase_configs[i].elements[3]);
        run_prg_until_input(&mut amp_d);

        let mut amp_e = IntcodeProgram::new(&data, Some(vec![])).unwrap_or_else(handle_except);
        inject_prg_input(&mut amp_e, phase_configs[i].elements[4]);
        run_prg_until_input(&mut amp_e);

        // run feedback loop to completion
        let mut cur_o: i32 = 0;
        while amp_e.active {
            // initially, send signal of 0 to amplifier A
            // after the first iteration:
            // send amplifier E's output as amplifier A's input
            inject_prg_input(&mut amp_a, cur_o);
            run_prg_until_input(&mut amp_a);
            cur_o = amp_a.output[amp_a.output.len() - 1];

            // send amplifier A's output as amplifier B's input
            inject_prg_input(&mut amp_b, cur_o);
            run_prg_until_input(&mut amp_b);
            cur_o = amp_b.output[amp_b.output.len() - 1];

            // send amplifier B's output as amplifier C's input
            inject_prg_input(&mut amp_c, cur_o);
            run_prg_until_input(&mut amp_c);
            cur_o = amp_c.output[amp_c.output.len() - 1];

            // send amplifier C's output as amplifier D's input
            inject_prg_input(&mut amp_d, cur_o);
            run_prg_until_input(&mut amp_d);
            cur_o = amp_d.output[amp_d.output.len() - 1];

            // send amplifier D's output as amplifier E's input
            inject_prg_input(&mut amp_e, cur_o);
            run_prg_until_input(&mut amp_e);
            cur_o = amp_e.output[amp_e.output.len() - 1];
        }

        // check if current output is greater than the current maximum final output
        if cur_o > max_final {
            max_final = cur_o;
            max_ind = i;
        }
    }

    println!("Maximum final output: {}", max_final);
    println!(
        "Phase setting permutation for maximum output: {:?} (index {})",
        phase_configs[max_ind], max_ind
    );
}
