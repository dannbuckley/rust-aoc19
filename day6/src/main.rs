use matrix::prelude::*;
use priority_queue::PriorityQueue;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::vec::Vec;

/// Applies Dijkstra's algorithm for single-source shortest paths on the given graph
/// using s as index of the source vertex
fn dijkstra(g: &Compressed<u8>, s: usize) -> (Vec<u16>, Vec<isize>) {
    // initialize priority queue to empty
    let mut q: PriorityQueue<usize, u16> = PriorityQueue::new();

    // initialize vectors for penultimate vertices and shortest distances
    let mut p: Vec<isize> = Vec::new();
    let mut d: Vec<u16> = Vec::new();

    for v in 0..g.rows() {
        d.push(65535);
        p.push(-1);

        // initialize vertex priority in the priority queue
        q.push(v, 65535 - d[v]);
    }

    // set priority of source vertex to 0
    d[s] = 0;
    q.push_increase(s, 65535 - d[s]);

    // initialize vector of considered vertices
    let mut v_t: Vec<usize> = Vec::new();

    for _ in 0..g.rows() {
        // delete the minimum priority element
        let min = q.pop().unwrap();
        let u_star = min.0;
        v_t.push(u_star);

        for u in 0..g.columns() {
            // for every vertex u in V - v_t that is adjacent to u_star
            if !v_t.contains(&u) && g.get((u_star, u)) != 0 {
                let cur_val = d[u_star] + g.get((u_star, u)) as u16;
                if cur_val < d[u] {
                    d[u] = cur_val;
                    p[u] = u_star as isize;
                    q.push_increase(u, 65535 - d[u]);
                }
            }
        }
    }

    (d, p)
}

#[derive(Debug)]
struct OrbitMap {
    /// Planets contained in map
    planets: HashMap<String, usize>,
    /// Collection of orbits between the planets in the map
    orbits: Vec<(usize, usize)>,
}

impl OrbitMap {
    /// Creates a new OrbitMap object using the given orbit data
    fn new(data: &Vec<&str>) -> OrbitMap {
        // initialize planets (a.k.a. graph vertices) hashmap with the
        // center of mass as the first planet
        let mut i: usize = 0;
        let mut planets: HashMap<String, usize> = HashMap::new();
        planets.insert("COM".to_owned(), i);
        i += 1;

        // initialize vector of orbits (a.k.a. graph edges)
        let mut orbits: Vec<(usize, usize)> = Vec::new();

        // parse each orbit
        for orbit in data {
            // split orbit string into orbited and orbiting planets
            let orbit_planets: Vec<_> = orbit.split(')').collect();
            let p_orbited: String = orbit_planets[0].to_owned();
            let p_orbiting: String = orbit_planets[1].to_owned();

            // get ID for orbited planet
            let id_orbited: usize;
            if !planets.contains_key(&p_orbited) {
                planets.insert(p_orbited, i);
                id_orbited = i;
                i += 1;
            } else {
                id_orbited = *planets.get(&p_orbited).unwrap();
            }

            // get ID for orbiting planet
            let id_orbiting: usize;
            if !planets.contains_key(&p_orbiting) {
                planets.insert(p_orbiting, i);
                id_orbiting = i;
                i += 1;
            } else {
                id_orbiting = *planets.get(&p_orbiting).unwrap();
            }

            // add orbit to vector
            orbits.push((id_orbited, id_orbiting));
        }

        OrbitMap { planets, orbits }
    }

    /// Computes the orbit count checksum for this orbit map
    fn compute_orbit_count_checksum(&self) -> u32 {
        // initialize n x n graph of orbits
        let n = self.planets.len();
        let mut orbit_map_graph = Compressed::<u8>::zero((n, n));

        // set weight of all orbits to 1
        for orbit in self.orbits.to_vec() {
            orbit_map_graph.set(orbit, 1);
        }

        // apply Dijkstra's algorithm to orbit graph
        // (use center of mass as source vertex)
        let (d, _) = dijkstra(&orbit_map_graph, 0);

        // compute orbit count checksum
        let mut checksum: u32 = 0;
        for value in self.planets.values() {
            checksum += d[*value] as u32;
        }
        checksum
    }
}

fn main() {
    // compute checksum for test orbit map
    let test_lines: Vec<&str> = vec![
        "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
    ];
    let test_map = OrbitMap::new(&test_lines);
    let test_checksum = test_map.compute_orbit_count_checksum();
    assert_eq!(test_checksum, 42);
    println!("Test map: {:?}", test_map);
    println!("Test checksum: {}", test_checksum);

    // read in problem input
    let args: Vec<String> = env::args().collect();
    let mut f = File::open(&args[1]).expect("File not found!");
    let mut prob_data = String::new();
    f.read_to_string(&mut prob_data)
        .expect("Something went wrong while reading the file!");
    // compute problem checksum
    println!("Creating map for problem input...");
    let prob_map = OrbitMap::new(&prob_data.lines().collect());
    println!("Computing checksum for problem input...");
    let prob_checksum = prob_map.compute_orbit_count_checksum();
    println!("Problem checksum: {}", prob_checksum);
}
