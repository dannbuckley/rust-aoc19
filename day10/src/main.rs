extern crate array_tool;
extern crate math;

use array_tool::vec::Intersect;
use math::round;
use std::cmp::{max, Ordering};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::mem;
use std::process;
use std::vec::Vec;

fn swap(x: &mut Vec<usize>, i: usize, j: usize) {
    let (lo, hi) = match i.cmp(&j) {
        Ordering::Less => (i, j),
        Ordering::Greater => (j, i),

        // no swapping necessary
        _ => return,
    };

    let (init, tail) = x.split_at_mut(hi);
    mem::swap(&mut init[lo], &mut tail[0]);
}

/// Sorts the given array in nondecreasing order by using heapsort
fn heapsort(h: &mut Vec<usize>) {
    // transform array into bottom-up heap
    let heap_construct = |a: &mut Vec<usize>| {
        let na = a.len();
        let nh = round::floor(na as f64 / 2.0, 0) as usize;

        for i in 1..(nh + 1) {
            let mut k = nh - i + 1;
            let v = a[k - 1];

            let mut heap = false;
            while !heap && (2 * k) <= na {
                let mut j = 2 * k;
                if j < na {
                    // there are two children
                    if a[j - 1] < a[j] {
                        j += 1;
                    }
                }

                if v >= a[j - 1] {
                    heap = true;
                } else {
                    a[k - 1] = a[j - 1];
                    k = j;
                }
            }

            a[k - 1] = v;
        }
    };
    heap_construct(h);

    // apply root-deletion n - 1 times
    let n = h.len();
    for i in 0..(n - 1) {
        // exchange root key with last key k
        swap(h, 0, n - 1 - i);

        // verify parental dominance of k
        let mut hn = Vec::<usize>::from(&h[0..(n - 1 - i)]);
        heap_construct(&mut hn);
        for j in 0..(n - 1 - i) {
            h[j] = hn[j];
        }
    }
}

#[test]
fn test_heapsort() {
    let mut h: Vec<usize> = vec![2, 9, 7, 6, 5, 8];
    heapsort(&mut h);
    assert_eq!(h, vec![2, 5, 6, 7, 8, 9]);
}

/// Calculates the greatest common denominator of a and b
fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        return a;
    } else {
        return gcd(b, a % b);
    }
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(48, 18), 6);
    assert_eq!(gcd(18, 48), 6);
}

#[derive(Clone, Copy, Debug)]
struct Slope {
    /// Change in y
    dy: isize,
    /// Change in x
    dx: isize,
}

impl PartialEq for Slope {
    fn eq(&self, other: &Self) -> bool {
        self.dx == other.dx && self.dy == other.dy
    }
}

impl Eq for Slope {}

#[test]
fn test_slope_eq() {
    let slope_1 = Slope { dy: -1, dx: 0 };
    let slope_2 = Slope { dy: -1, dx: 0 };
    assert_eq!(slope_1, slope_2);
}

#[test]
fn test_slope_neq() {
    let slope_1 = Slope { dy: -1, dx: 0 };
    let slope_2 = Slope { dy: 0, dx: -1 };
    assert_ne!(slope_1, slope_2);
}

#[test]
fn test_vec_contains_slope() {
    let v = vec![Slope { dy: -1, dx: 0 }];
    assert!(v.contains(&Slope { dy: -1, dx: 0 }));
}

impl Slope {
    /// Calculates slope from source point to other given point in reduced form
    fn calculate(source: usize, other: usize) -> Slope {
        // define unpairing closure for asteroid coordinates
        let szudzik_unpair = |z: usize| {
            let z_sq_fl = round::floor((z as f64).sqrt(), 0) as isize;
            let cmp_l = z as isize - (z_sq_fl * z_sq_fl);
            if cmp_l >= z_sq_fl {
                return (z_sq_fl, cmp_l - z_sq_fl);
            } else {
                return (cmp_l, z_sq_fl);
            }
        };

        // unpair points
        let p_s = szudzik_unpair(source);
        let p_o = szudzik_unpair(other);

        // dy = y - y0
        let mut dy = p_o.1 - p_s.1;

        // dx = x - x0
        let mut dx = p_o.0 - p_s.0;

        // reduce slope to simplest form
        if dx == 0 {
            // vertical slope, reduce to unit vector
            if dy < 0 {
                dy = -1;
            } else {
                dy = 1;
            }
        } else if dy == 0 {
            // horizontal slope, reduce to unit vector
            if dx < 0 {
                dx = -1;
            } else {
                dx = 1;
            }
        } else {
            // other slope, divide both components by GCD
            let _gcd = gcd(dx.abs(), dy.abs());
            dx /= _gcd;
            dy /= _gcd;
        }

        Slope { dy, dx }
    }
}

#[derive(Debug)]
struct Shell {
    /// Source asteroid of shell
    source: usize,
    /// Distance of shell from source asteroid
    radius: usize,
    /// All points that lie on the shell
    points: Vec<usize>,
    /// Number of asteroids that lie on the shell
    n: usize,
    /// All asteroids on map that lie on the shell
    asteroids: Vec<usize>,
    /// Slopes from source asteroid to every asteroid on shell
    slopes: Vec<Slope>,
}

impl Shell {
    /// Constructs a Shell object using the given source asteroid,
    /// the dimensions of the asteroid map,
    /// and the radial distance from the source asteroid
    fn new(point: &(usize, usize), dim: &(usize, usize), radius: usize) -> Shell {
        // define pairing closure for asteroid coordinates
        let szudzik_pair = |x: usize, y: usize| {
            if max(x, y) == x {
                return x * (x + 1) + y;
            } else {
                return (y * y) + x;
            }
        };

        // initialize vector of paired point coordinates
        let mut points: Vec<usize> = Vec::new();

        // top and bottom sides
        for x in 0..((2 * radius as isize) + 1) {
            // check if shell point lies on map
            let x_val = point.0 as isize + x - radius as isize;
            if x_val < 0 || x_val >= dim.0 as isize {
                continue;
            }

            // add top point
            if radius <= point.1 {
                points.push(szudzik_pair(x_val as usize, point.1 - radius));
            }

            // add bottom point
            if point.1 + radius < dim.1 {
                points.push(szudzik_pair(x_val as usize, point.1 + radius));
            }
        }

        // left and right sides
        for y in 0..((2 * radius as isize) - 1) {
            // check if shell point lies on map
            let y_val = point.1 as isize + y - radius as isize + 1;
            if y_val < 0 || y_val >= dim.1 as isize {
                continue;
            }

            // add left point
            if radius <= point.0 {
                points.push(szudzik_pair(point.0 - radius, y_val as usize));
            }

            // add right point
            if point.0 + radius < dim.0 {
                points.push(szudzik_pair(point.0 + radius, y_val as usize));
            }
        }

        // sort points on shell of radius radius
        heapsort(&mut points);

        Shell {
            source: szudzik_pair(point.0, point.1),
            radius,
            points,
            n: 0,
            asteroids: Vec::new(),
            slopes: Vec::new(),
        }
    }

    /// Computes intersection of points on shell and asteroids on map
    fn compute_shell_asteroids(&mut self, asteroids: &Vec<usize>) {
        self.asteroids = asteroids.intersect(self.points.to_vec());
        self.n = self.asteroids.len();
    }

    /// Calculates slope from source asteroid to every asteroid on shell
    fn calculate_slopes(&mut self) {
        let calc_slope = |o: usize| Slope::calculate(self.source, o);
        self.slopes = self
            .asteroids
            .to_vec()
            .into_iter()
            .map(calc_slope)
            .rev()
            .collect();
    }
}

fn compute_seen_asteroids(shells: &Vec<Shell>) -> usize {
    // initialize vector of seen asteroids and slopes
    let mut seen_asteroids: Vec<usize> = Vec::new();
    let mut seen_slopes: Vec<Slope> = Vec::new();
    for shell in shells {
        if seen_asteroids.len() == 0 {
            seen_asteroids.append(&mut shell.asteroids.to_vec().as_mut());
            seen_slopes.append(&mut shell.slopes.to_vec().as_mut());
        } else {
            for m in 0..shell.n {
                if !seen_slopes.contains(&shell.slopes[m]) {
                    seen_asteroids.push(shell.asteroids[m]);
                    seen_slopes.push(shell.slopes[m]);
                }
            }
        }
    }

    seen_asteroids.len()
}

#[derive(Debug)]
struct BestAsteroid {
    /// Position of best asteroid on map
    position: (usize, usize),
    /// Number of asteroids this best asteroid is able to see
    num_seen_asteroids: usize,
}

#[derive(Debug)]
struct AsteroidMap {
    /// Paired coordinates of asteroids in map
    asteroids: Vec<usize>,
    /// Dimensions (width x height) of map
    dimensions: (usize, usize),
}

impl AsteroidMap {
    /// Constructs a new AsteroidMap object from the given map data
    fn new(data: &Vec<&str>) -> Option<AsteroidMap> {
        if data.len() == 0 {
            return None;
        }

        // define pairing closure for asteroid coordinates
        let szudzik_pair = |x: usize, y: usize| {
            if max(x, y) == x {
                return x * (x + 1) + y;
            } else {
                return (y * y) + x;
            }
        };

        // parse asteroid data
        let mut asteroids: Vec<usize> = Vec::new();
        let mut l: usize = 0;
        let mut i: usize = 0;
        for line in data {
            i = 0;
            for c in line.chars() {
                if c == '#' {
                    asteroids.push(szudzik_pair(i, l));
                }

                // advance to next character
                i += 1;
            }

            // advance to next line
            l += 1;
        }

        // create dimensions of map (width x height)
        let dimensions = (i, l);

        // sort asteroid paired values in ascending order
        heapsort(&mut asteroids);

        Some(AsteroidMap {
            asteroids,
            dimensions,
        })
    }

    /// Finds the asteroid within the map from which the most
    /// asteroids can be seen
    fn find_best_asteroid(&mut self) -> BestAsteroid {
        // define unpairing closure for asteroid coordinates
        let szudzik_unpair = |z: usize| {
            let z_sq_fl = round::floor((z as f64).sqrt(), 0) as usize;
            let cmp_l = z - (z_sq_fl * z_sq_fl);
            if cmp_l >= z_sq_fl {
                return (z_sq_fl, cmp_l - z_sq_fl);
            } else {
                return (cmp_l, z_sq_fl);
            }
        };

        // create hashmap for shells around asteroids
        let mut asteroid_shells: HashMap<usize, Vec<Shell>> = HashMap::new();

        // calculate number of seen asteroids from each asteroid
        for asteroid in &self.asteroids {
            // unpair coordinate value
            let source: (usize, usize) = szudzik_unpair(*asteroid);

            // calculate number of shells for source
            let n_shells = max(
                max(source.0, self.dimensions.0 - 1 - source.0),
                max(source.1, self.dimensions.1 - 1 - source.1),
            );

            // compute all shells for source
            for s in 1..(n_shells + 1) {
                // build new Shell object
                let mut s_t = Shell::new(&source, &self.dimensions, s);

                // compute intersection of shell points and asteroids
                s_t.compute_shell_asteroids(&self.asteroids);

                // calculate slopes from source to asteroids in shell
                s_t.calculate_slopes();

                // add shell to hashmap
                let entry = asteroid_shells.entry(*asteroid).or_insert(Vec::new());
                entry.push(s_t);
            }
        }

        // search for best asteroid
        let mut best_asteroid: usize = 0;
        let mut best_value: usize = 0;
        for (key, value) in asteroid_shells {
            let seen = compute_seen_asteroids(&value);
            if seen > best_value {
                best_asteroid = key;
                best_value = seen;
            }
        }

        // return best asteroid
        BestAsteroid {
            position: szudzik_unpair(best_asteroid),
            num_seen_asteroids: best_value,
        }
    }
}

#[test]
fn test_asteroid_map_1() {
    let test = vec![".#..#", ".....", "#####", "....#", "...##"];
    let mut asteroid_map = match AsteroidMap::new(&test) {
        Some(m) => m,
        None => process::exit(1),
    };
    let best = asteroid_map.find_best_asteroid();
    assert_eq!(best.position, (3, 4));
    assert_eq!(best.num_seen_asteroids, 8);
}

#[test]
fn test_asteroid_map_2() {
    let test = vec![
        "......#.#.",
        "#..#.#....",
        "..#######.",
        ".#.#.###..",
        ".#..#.....",
        "..#....#.#",
        "#..#....#.",
        ".##.#..###",
        "##...#..#.",
        ".#....####",
    ];
    let mut asteroid_map = match AsteroidMap::new(&test) {
        Some(m) => m,
        None => process::exit(1),
    };
    let best = asteroid_map.find_best_asteroid();
    assert_eq!(best.position, (5, 8));
    assert_eq!(best.num_seen_asteroids, 33);
}

#[test]
fn test_asteroid_map_3() {
    let test = vec![
        "#.#...#.#.",
        ".###....#.",
        ".#....#...",
        "##.#.#.#.#",
        "....#.#.#.",
        ".##..###.#",
        "..#...##..",
        "..##....##",
        "......#...",
        ".####.###.",
    ];
    let mut asteroid_map = match AsteroidMap::new(&test) {
        Some(m) => m,
        None => process::exit(1),
    };
    let best = asteroid_map.find_best_asteroid();
    assert_eq!(best.position, (1, 2));
    assert_eq!(best.num_seen_asteroids, 35);
}

#[test]
fn test_asteroid_map_4() {
    let test = vec![
        ".#..#..###",
        "####.###.#",
        "....###.#.",
        "..###.##.#",
        "##.##.#.#.",
        "....###..#",
        "..#.#..#.#",
        "#..#.#.###",
        ".##...##.#",
        ".....#.#..",
    ];
    let mut asteroid_map = match AsteroidMap::new(&test) {
        Some(m) => m,
        None => process::exit(1),
    };
    let best = asteroid_map.find_best_asteroid();
    assert_eq!(best.position, (6, 3));
    assert_eq!(best.num_seen_asteroids, 41);
}

#[test]
fn test_asteroid_map_5() {
    let test = vec![
        ".#..##.###...#######",
        "##.############..##.",
        ".#.######.########.#",
        ".###.#######.####.#.",
        "#####.##.#.##.###.##",
        "..#####..#.#########",
        "####################",
        "#.####....###.#.#.##",
        "##.#################",
        "#####.##.###..####..",
        "..######..##.#######",
        "####.##.####...##..#",
        ".#####..#.######.###",
        "##...#.##########...",
        "#.##########.#######",
        ".####.#.###.###.#.##",
        "....##.##.###..#####",
        ".#.#.###########.###",
        "#.#.#.#####.####.###",
        "###.##.####.##.#..##",
    ];
    let mut asteroid_map = match AsteroidMap::new(&test) {
        Some(m) => m,
        None => process::exit(1),
    };
    let best = asteroid_map.find_best_asteroid();
    assert_eq!(best.position, (11, 13));
    assert_eq!(best.num_seen_asteroids, 210);
}

fn main() {
    // read in problem input
    let args: Vec<String> = env::args().collect();
    let mut f = File::open(&args[1]).expect("File not found!");
    let mut data = String::new();
    f.read_to_string(&mut data)
        .expect("Something went wrong while reading the file!");

    // create map for problem input
    println!("Creating map for problem input...");
    let mut asteroid_map = match AsteroidMap::new(&data.lines().collect()) {
        Some(m) => m,
        None => process::exit(1),
    };

    // find best asteroid in problem map
    println!("Finding best asteroid in map...");
    println!("Best asteroid: {:?}", asteroid_map.find_best_asteroid());
}
