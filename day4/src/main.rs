use array_tool::vec::Intersect;
use std::vec::Vec;

/// Validates the given password according to the problem specifications
fn validate_password(p: String) -> bool {
    // separate password into pairs
    let p_pairs: Vec<u8> = vec![
        p[0..2].parse::<u8>().unwrap(),
        p[1..3].parse::<u8>().unwrap(),
        p[2..4].parse::<u8>().unwrap(),
        p[3..5].parse::<u8>().unwrap(),
        p[4..6].parse::<u8>().unwrap(),
    ];

    // intersection(password, forbidden) must equal the empty set
    // i.e., the digits in a pair (from left to right) cannot decrease
    let forbidden: Vec<u8> = vec![
        10, 21, 20, 32, 31, 30, 43, 42, 41, 40, 54, 53, 52, 51, 50, 65, 64, 63, 62, 61, 60, 76, 75,
        74, 73, 72, 71, 70, 87, 86, 85, 84, 83, 82, 81, 80, 98, 97, 96, 95, 94, 93, 92, 91, 90,
    ];
    let desc = p_pairs.intersect(forbidden);
    if desc.len() > 0 {
        return false;
    }

    // intersection(password, doubles) must contain at least one item
    // i.e., one pair must contain repeated digits
    let doubles: Vec<u8> = vec![0, 11, 22, 33, 44, 55, 66, 77, 88, 99];
    let two_same = p_pairs.intersect(doubles);
    if two_same.len() == 0 {
        return false;
    }

    // separate password into triplets
    let p_triplets: Vec<u16> = vec![
        p[0..3].parse::<u16>().unwrap(),
        p[1..4].parse::<u16>().unwrap(),
        p[2..5].parse::<u16>().unwrap(),
        p[3..6].parse::<u16>().unwrap(),
    ];

    // the two adjacent matching digits cannot be part
    // of a larger group
    let triples: Vec<u16> = vec![0, 111, 222, 333, 444, 555, 666, 777, 888, 999];
    let three_same = p_triplets.intersect(triples);
    if three_same.len() == 2 {
        // cannot have two blocks of three matching digits
        return false;
    } else if three_same.len() == 1 {
        // separate password into quads
        let p_quads: Vec<u16> = vec![
            p[0..4].parse::<u16>().unwrap(),
            p[1..5].parse::<u16>().unwrap(),
            p[2..6].parse::<u16>().unwrap(),
        ];
        let quads: Vec<u16> = vec![0, 1111, 2222, 3333, 4444, 5555, 6666, 7777, 8888, 9999];
        let four_same = p_quads.intersect(quads);

        if two_same.len() == 1 && four_same.len() == 1 {
            // cannot have a contiguous block of four matching digits
            // without an additional matching pair somewhere in the password
            return false;
        } else if two_same.len() == 1 && four_same.len() == 0 {
            // cannot have a contiguous block of three matching digits
            // without an additional matching pair somewhere in the password
            return false;
        }
    }

    true
}

/// Validates all passwords in the range (lower, upper)
fn find_num_valid_passwords(lower: u32, upper: u32) -> u32 {
    let mut num_valid = 0;

    for i in lower..(upper + 1) {
        if validate_password(i.to_string()) {
            num_valid += 1;
        }
    }

    num_valid
}

fn main() {
    // test validate_password function
    print!("Testing password validation...");
    assert_eq!(validate_password("111111".to_owned()), false);
    assert_eq!(validate_password("223450".to_owned()), false);
    assert_eq!(validate_password("123789".to_owned()), false);
    assert_eq!(validate_password("112233".to_owned()), true);
    assert_eq!(validate_password("123444".to_owned()), false);
    assert_eq!(validate_password("111122".to_owned()), true);
    println!("\tAll assertions passed!");
    // find valid passwords in problem input range
    let lower: u32 = 248345;
    let upper: u32 = 746315;
    println!(
        "Number of valid passwords in range ({}-{}): {}",
        lower,
        upper,
        find_num_valid_passwords(lower, upper)
    );
}
