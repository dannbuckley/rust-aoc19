use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::vec::Vec;

/// Represents an individual line segment of a wire
#[derive(Clone, Copy, Debug)]
struct WireSegment {
    /// Variable component of line segment
    ///
    /// For horiz. segments: u = x;
    /// for vert. segments: u = y
    u_range: (i32, i32),
    /// Static component of line segment
    ///
    /// For horiz. segments: v = y;
    /// for vert. segments: v = x
    v: i32,
    /// Direction of line segment
    ///
    /// For horiz. segments: true = right, false = left;
    /// For vert. segments: true = up, false = down;
    direction: bool,
    /// Steps required to reach beginning of line segment
    ///
    /// For horiz. segments: beginning = (direction ? u_range.0 : u_range.1, v);
    /// For vert. segments: beginning = (v, direction ? u_range.0 : u_range.1)
    steps_to_segment: u32,
}

/// Returns true if the value s falls within the bounds of range
fn value_in_range(range: (i32, i32), s: i32) -> bool {
    s >= range.0 && s <= range.1
}

/// Finds all crossings between the given vertical and horizontal
/// set of WireSegment objects
fn find_crossings(v: &Vec<WireSegment>, h: &Vec<WireSegment>) -> Vec<(i32, i32)> {
    let mut crossings: Vec<(i32, i32)> = Vec::<(i32, i32)>::new();

    // check all vert. segments from left to right
    for i in 0..v.len() {
        // check all horiz. segments from bottom to top
        let mut j: usize = 0;

        // ignore all horiz. segments below range of current vert. segment
        while j < h.len() && h[j].v < v[i].u_range.0 {
            j += 1;
        }

        // check all horiz. segments in range of current vert. segment
        while j < h.len() && h[j].v <= v[i].u_range.1 {
            if value_in_range(h[j].u_range, v[i].v) {
                // ignore crossing at starting location
                if v[i].v != 0 && h[j].v != 0 {
                    crossings.push((v[i].v, h[j].v));
                }
            }
            j += 1;
        }
    }

    crossings
}

/// Finds the number of steps required to reach the given intersection point
///
/// If set is of horiz. segments, orient = true;
/// If set is of vert. segments, orient = false
fn find_steps_to_point(point: (i32, i32), set: &Vec<WireSegment>, orient: bool) -> u32 {
    let mut steps: u32 = 0;
    if orient {
        // searching for horiz. segment
        for i in 0..set.len() {
            if point.1 == set[i].v && value_in_range(set[i].u_range, point.0) {
                steps = set[i].steps_to_segment;
                if set[i].direction {
                    // segment going right
                    steps += (point.0 - set[i].u_range.0) as u32;
                } else {
                    // segment going left
                    steps += (set[i].u_range.1 - point.0) as u32;
                }
                break;
            }
        }
    } else {
        // searching for vert. segment
        for i in 0..set.len() {
            if point.0 == set[i].v && value_in_range(set[i].u_range, point.1) {
                steps = set[i].steps_to_segment;
                if set[i].direction {
                    // segment going up
                    steps += (point.1 - set[i].u_range.0) as u32;
                } else {
                    // segment going down
                    steps += (set[i].u_range.1 - point.1) as u32;
                }
                break;
            }
        }
    }

    steps
}

/// Represents a wire component,
/// separated into its horizontal and vertical components
#[derive(Debug)]
struct Wire {
    /// Horizontal line segments
    h_segs: Vec<WireSegment>,
    /// Vertical line segments
    v_segs: Vec<WireSegment>,
}

/// Sorts a vector of WireSegment objects by their v component
/// using insertion sort
fn insertion_sort(a: &mut Vec<WireSegment>) {
    let n = a.len();
    for i in 1..n {
        let v = a[n - 1 - i];
        let mut j = n - i;
        while j < n && a[j].v < v.v {
            a[j - 1] = a[j];
            j += 1;
        }
        a[j - 1] = v;
    }
}

impl Wire {
    /// Builds a Wire object from the given string of path data
    fn build_from_string(data: String) -> Wire {
        let mut pos: (i32, i32) = (0, 0);
        let mut steps: u32 = 0;
        let mut h_segs: Vec<WireSegment> = Vec::<WireSegment>::new();
        let mut v_segs: Vec<WireSegment> = Vec::<WireSegment>::new();

        // parse wire path
        println!("\tParsing wire path...");
        let path: Vec<_> = data.split(',').collect();
        for param in path {
            let mut p = param.to_owned();
            let dir = p.remove(0);
            let len = p.parse::<u32>().unwrap();

            // check direction of path component
            if dir == 'L' {
                // extend wire to the left
                h_segs.push(WireSegment {
                    u_range: (pos.0 - len as i32, pos.0),
                    v: pos.1,
                    direction: false,
                    steps_to_segment: steps,
                });
                // update position
                pos.0 -= len as i32;
            } else if dir == 'R' {
                // extend wire to the right
                h_segs.push(WireSegment {
                    u_range: (pos.0, pos.0 + len as i32),
                    v: pos.1,
                    direction: true,
                    steps_to_segment: steps,
                });
                // update position
                pos.0 += len as i32;
            } else if dir == 'U' {
                // extend wire upward
                v_segs.push(WireSegment {
                    u_range: (pos.1, pos.1 + len as i32),
                    v: pos.0,
                    direction: true,
                    steps_to_segment: steps,
                });
                // update position
                pos.1 += len as i32;
            } else if dir == 'D' {
                // extend wire downward
                v_segs.push(WireSegment {
                    u_range: (pos.1 - len as i32, pos.1),
                    v: pos.0,
                    direction: false,
                    steps_to_segment: steps,
                });
                // update position
                pos.1 -= len as i32;
            }

            // update number of steps taken
            steps += len;
        }

        // sort horiz. segments by y-value
        insertion_sort(&mut h_segs);
        // sort vert. segments by x-value
        insertion_sort(&mut v_segs);

        Wire { h_segs, v_segs }
    }

    /// Finds all intersection points between this and the given wire,
    /// ignoring the intersection at the starting point
    fn find_intersection_points(&self, other: &Wire) -> Vec<(i32, i32)> {
        let mut points: Vec<(i32, i32)> = Vec::<(i32, i32)>::new();

        // find all crossings between vert. and horiz. segments
        let mut cross_a = find_crossings(&self.v_segs, &other.h_segs);
        let mut cross_b = find_crossings(&other.v_segs, &self.h_segs);

        // store crossing points in a single vector
        points.append(&mut cross_a);
        points.append(&mut cross_b);

        // return all crossing points
        points
    }

    /// Finds the smallest combination of steps required to reach an
    /// intersection on this and the given wire
    fn find_min_steps(&self, other: &Wire) -> u32 {
        let mut min_steps: u32 = 0;

        let cross_a = find_crossings(&self.v_segs, &other.h_segs);
        for crossing in cross_a {
            let total_steps = find_steps_to_point(crossing, &self.v_segs, false)
                + find_steps_to_point(crossing, &other.h_segs, true);
            if min_steps == 0 {
                // ignore comparison for first point
                min_steps = total_steps;
            } else if total_steps < min_steps {
                min_steps = total_steps;
            }
        }

        let cross_b = find_crossings(&other.v_segs, &self.h_segs);
        for crossing in cross_b {
            let total_steps = find_steps_to_point(crossing, &other.v_segs, false)
                + find_steps_to_point(crossing, &self.h_segs, true);
            if total_steps < min_steps {
                min_steps = total_steps;
            }
        }

        min_steps
    }
}

/// Finds the minimum hamming distance between the starting point
/// and each point in the given vector
fn find_min_hamming_dist(points: &Vec<(i32, i32)>) -> u32 {
    let mut min_ham: u32 = 0;

    for i in 0..points.len() {
        if i == 0 {
            // skip comparison for first point
            min_ham = (points[i].0.abs() + points[i].1.abs()) as u32;
        } else {
            let temp_ham = (points[i].0.abs() + points[i].1.abs()) as u32;

            // set min_ham = min(min_ham, temp_ham)
            if temp_ham < min_ham {
                min_ham = temp_ham;
            }
        }
    }

    min_ham
}

fn main() {
    // run first test pair
    println!("Running test pair #1:");
    // build and test first wire
    let test_first_1 = Wire::build_from_string("R8,U5,L5,D3".to_owned());
    print!("\tTesting first wire of pair #1: ");
    assert_eq!(test_first_1.h_segs[0].u_range, (0 as i32, 8 as i32));
    assert_eq!(test_first_1.h_segs[0].v, 0);
    assert_eq!(test_first_1.h_segs[1].u_range, (3 as i32, 8 as i32));
    assert_eq!(test_first_1.h_segs[1].v, 5);
    assert_eq!(test_first_1.v_segs[0].u_range, (2 as i32, 5 as i32));
    assert_eq!(test_first_1.v_segs[0].v, 3);
    assert_eq!(test_first_1.v_segs[1].u_range, (0 as i32, 5 as i32));
    assert_eq!(test_first_1.v_segs[1].v, 8);
    println!("All assertions passed!");

    // build and test second wire
    let test_second_1 = Wire::build_from_string("U7,R6,D4,L4".to_owned());
    print!("\tTesting second wire of pair #1: ");
    assert_eq!(test_second_1.h_segs[0].u_range, (2 as i32, 6 as i32));
    assert_eq!(test_second_1.h_segs[0].v, 3);
    assert_eq!(test_second_1.h_segs[1].u_range, (0 as i32, 6 as i32));
    assert_eq!(test_second_1.h_segs[1].v, 7);
    assert_eq!(test_second_1.v_segs[0].u_range, (0 as i32, 7 as i32));
    assert_eq!(test_second_1.v_segs[0].v, 0);
    assert_eq!(test_second_1.v_segs[1].u_range, (3 as i32, 7 as i32));
    assert_eq!(test_second_1.v_segs[1].v, 6);
    println!("All assertions passed!");

    // find intersection points of the two test wires
    let test_points_1 = test_first_1.find_intersection_points(&test_second_1);
    assert_eq!(test_points_1, [(3, 3), (6, 5)]);
    println!(
        "\tIntersection points of wires in pair #1: {:?}",
        test_points_1
    );

    // find minimum hamming distance for all intersections points
    let min_ham_1 = find_min_hamming_dist(&test_points_1);
    assert_eq!(min_ham_1, 6);
    println!(
        "\tMin. hamming dist. of pair #1: {} (should be 6)",
        min_ham_1
    );

    let min_steps_1 = test_first_1.find_min_steps(&test_second_1);
    assert_eq!(min_steps_1, 30);
    println!(
        "\tMin. number of steps for pair #1: {} (should be 30)",
        min_steps_1
    );

    // run second test pair
    println!("Running test pair #2:");
    let test_first_2 = Wire::build_from_string("R75,D30,R83,U83,L12,D49,R71,U7,L72".to_owned());
    let test_second_2 = Wire::build_from_string("U62,R66,U55,R34,D71,R55,D58,R83".to_owned());
    let test_points_2 = test_first_2.find_intersection_points(&test_second_2);
    let min_ham_2 = find_min_hamming_dist(&test_points_2);
    assert_eq!(min_ham_2, 159);
    println!(
        "\tMin. hamming dist. of pair #2: {} (should be 159)",
        min_ham_2
    );
    let min_steps_2 = test_first_2.find_min_steps(&test_second_2);
    assert_eq!(min_steps_2, 610);
    println!(
        "\tMin. number of steps for pair #2: {} (should be 610)",
        min_steps_2
    );

    // run third test pair
    println!("Running test pair #3:");
    let test_first_3 =
        Wire::build_from_string("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_owned());
    let test_second_3 = Wire::build_from_string("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_owned());
    let test_points_3 = test_first_3.find_intersection_points(&test_second_3);
    let min_ham_3 = find_min_hamming_dist(&test_points_3);
    assert_eq!(min_ham_3, 135);
    println!(
        "\tMin. hamming dist. of pair #3: {} (should be 135)",
        min_ham_3
    );
    let min_steps_3 = test_first_3.find_min_steps(&test_second_3);
    assert_eq!(min_steps_3, 410);
    println!(
        "\tMin. number of steps for pair #3: {} (should be 410)",
        min_steps_3
    );

    // read problem input file
    let args: Vec<String> = env::args().collect();
    let mut f = File::open(&args[1]).expect("File not found!");
    let mut input = String::new();
    f.read_to_string(&mut input)
        .expect("Something went wrong while reading the file!");
    let wires: Vec<_> = input.lines().collect();

    // build problem wires from file input
    let prob_first = Wire::build_from_string(wires[0].to_owned());
    let prob_second = Wire::build_from_string(wires[1].to_owned());

    // find minimum hamming distance
    let prob_points = prob_first.find_intersection_points(&prob_second);
    let prob_min_ham = find_min_hamming_dist(&prob_points);
    println!("Min. hamming distance for problem input: {}", prob_min_ham);

    // find minimum number of steps
    let prob_min_steps = prob_first.find_min_steps(&prob_second);
    println!("Min. number of steps for problem input: {}", prob_min_steps);
}
