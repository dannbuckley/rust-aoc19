use math::round;

use std::cmp;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::vec::Vec;

/**
 * Calculates the amount of fuel required for a module of the given mass.
 */
fn get_fuel(mass: u32) -> u32 {
    // convert mass to fuel
    let m = mass as f64;
    let f = round::floor(m / 3.0, 0) as u32;
    // ensure return value is positive number
    cmp::max(f as i32 - 2, 0) as u32
}

/**
 * Calculates the amount of fuel required for a module of the given mass
 *  accounting for added fuel.
 */
fn get_fuel_with_added(mass: u32) -> u32 {
    let mut fuels: Vec<u32> = Vec::<u32>::new();
    let mut last: u32 = get_fuel(mass);

    // calculate additional fuel required
    while last > 0 {
        fuels.push(last);
        last = get_fuel(last);
    }

    // sum all fuel values
    let mut sum: u32 = 0;
    for fuel in fuels {
        sum += fuel;
    }
    sum
}

fn main() {
    // test get_fuel function
    println!("Testing get_fuel function...");
    assert_eq!(get_fuel(12), 2);
    assert_eq!(get_fuel(14), 2);
    assert_eq!(get_fuel(1969), 654);
    assert_eq!(get_fuel(100756), 33583);

    // test get_fuel_with_added function
    println!("Testing get_fuel_with_added function...");
    assert_eq!(get_fuel_with_added(1969), 966);
    assert_eq!(get_fuel_with_added(100756), 50346);

    println!("All assertions passed!");

    // read in problem input
    println!("Reading in problem data...");
    let args: Vec<String> = env::args().collect();
    let mut f = File::open(&args[1]).expect("File not found!");
    let mut lines = String::new();
    f.read_to_string(&mut lines)
        .expect("Something went wrong while reading the file!");
    let masses: Vec<_> = lines.lines().collect();

    // convert masses to fuels and append to sums
    let mut fuel_sum: u32 = 0;
    let mut fuel_w_add_sum: u32 = 0;
    for mass in masses {
        let m = mass.parse::<u32>().unwrap();
        // part one
        fuel_sum += get_fuel(m);
        // part two
        fuel_w_add_sum += get_fuel_with_added(m);
    }

    // print results
    println!("Sum of the fuel requirements: {}", fuel_sum);
    println!(
        "Sum of the fuel requirements(with added): {}",
        fuel_w_add_sum
    );
}
