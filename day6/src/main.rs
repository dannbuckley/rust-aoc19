use array_tool::vec::{Intersect, Uniq};
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
    /// Graph representation of orbit map
    orbit_map_graph: Compressed<u8>,
    /// Distances to every planet from the center of mass
    distances: Vec<u16>,
    /// Second to last planet on the path to each planet
    penultimates: Vec<isize>,
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

        // construct graph of orbit map
        let mut orbit_map_graph = Compressed::<u8>::zero((i, i));
        for orbit in orbits.to_vec() {
            orbit_map_graph.set(orbit, 1);
        }

        // apply Dijkstra's algorithm to compute
        // distances and penultimate vertices
        let (distances, penultimates) = dijkstra(&orbit_map_graph, 0);

        OrbitMap {
            planets,
            orbits,
            orbit_map_graph,
            distances,
            penultimates,
        }
    }

    /// Computes the orbit count checksum for this orbit map
    fn compute_orbit_count_checksum(&self) -> u32 {
        let mut checksum: u32 = 0;
        for value in self.planets.values() {
            checksum += self.distances[*value] as u32;
        }
        checksum
    }

    /// Computes the number of orbit transfers required to get from you to santa
    fn compute_orbit_transfers(&self) -> usize {
        // get positions of you and santa
        let you_index = *self.planets.get(&"YOU".to_owned()).unwrap();
        let san_index = *self.planets.get(&"SAN".to_owned()).unwrap();

        // compute path from you to center of mass (excluding your position)
        let mut path_to_you: Vec<usize> = vec![you_index];
        loop {
            let next_planet: usize = path_to_you[path_to_you.len() - 1];
            if next_planet == 0 {
                break;
            }
            path_to_you.push(self.penultimates[next_planet] as usize);
        }
        path_to_you.remove(0);

        // compute path from santa to center of mass (excluding santa's position)
        let mut path_to_san: Vec<usize> = vec![san_index];
        loop {
            let next_planet: usize = path_to_san[path_to_san.len() - 1];
            if next_planet == 0 {
                break;
            }
            path_to_san.push(self.penultimates[next_planet] as usize);
        }
        path_to_san.remove(0);

        // compute intersection of the two paths
        let path_intersection = path_to_you.intersect(path_to_san.to_vec());

        // compute unique values in the two paths
        let you_uniq = path_to_you.uniq(path_intersection.to_vec());
        let san_uniq = path_to_san.uniq(path_intersection.to_vec());

        // # of orbit transfers = sum of path lengths
        you_uniq.len() + san_uniq.len()
    }
}

fn main() {
    // compute checksum and transfers for test orbit map
    let test_lines: Vec<&str> = vec![
        "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU",
        "I)SAN",
    ];
    let test_map = OrbitMap::new(&test_lines);
    println!("Test map: {:?}", test_map);
    let test_checksum = test_map.compute_orbit_count_checksum();
    println!("Test checksum: {}", test_checksum);
    let test_transfers = test_map.compute_orbit_transfers();
    println!("Test orbit transfers: {}", test_transfers);

    // read in problem input
    let args: Vec<String> = env::args().collect();
    let mut f = File::open(&args[1]).expect("File not found!");
    let mut prob_data = String::new();
    f.read_to_string(&mut prob_data)
        .expect("Something went wrong while reading the file!");

    // create orbit map for problem input
    println!("Creating map for problem input...");
    let prob_map = OrbitMap::new(&prob_data.lines().collect());

    // compute problem checksum and transfers
    let prob_checksum = prob_map.compute_orbit_count_checksum();
    println!("Problem checksum: {}", prob_checksum);
    let prob_transfers = prob_map.compute_orbit_transfers();
    println!("Problem transfers: {}", prob_transfers);
}
