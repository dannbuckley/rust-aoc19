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
    let mut k: i32 = 4;
    let mut k_ind: usize = 0;

    // find largest element
    while k >= 0 {
        // find index of element within vector
        for i in 0..5 {
            if elements[i] == k {
                k_ind = i;
                break;
            }
        }

        // check if element is mobile
        if directions[k as usize] {
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

    if k == -1 {
        // no mobile elements exist
        return None;
    }

    Some(k_ind)
}

/// Generates Johnson-Trotter permutations of phase settings
fn johnson_trotter() -> Vec<PhaseSettingPermutation> {
    // initialize permutation and direction vectors
    let mut p = PhaseSettingPermutation {
        elements: vec![0, 1, 2, 3, 4],
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
        if directions[k_val as usize] {
            // swap with element to the right
            swap(&mut p.elements, k, k + 1);
        } else {
            // swap with element to the left
            swap(&mut p.elements, k - 1, k);
        }

        // reverse direction of all elements larger than k_val
        for i in (k_val + 1)..5 {
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

    let mut max_o: i32 = 0;
    let mut max_ind: usize = 0;
    for i in 0..120 {
        let mut o: i32 = 0;
        for j in 0..5 {
            let input: Vec<i32> = vec![phase_configs[i].elements[j], o];
            let mut prg =
                IntcodeProgram::new(&data.to_owned(), Some(input)).unwrap_or_else(|err| {
                    eprintln!("Problem creating Intcode program from input: {}", err);
                    process::exit(1);
                });
            if let Err(e) = prg.run() {
                eprintln!("Error occured during program execution: {}", e);
                process::exit(1);
            }
            o = prg.output[0];
        }
        if o > max_o {
            max_o = o;
            max_ind = i;
        }
    }
    println!("Max output: {}", max_o);
    println!("Max configuration: {:?}", phase_configs[max_ind]);
}
