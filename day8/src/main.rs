use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::str;
use std::vec::Vec;

#[derive(Debug)]
struct SpaceImageLayer {
    rows: Vec<String>,
    frequencies: HashMap<char, usize>,
}

fn compute_layer_frequencies(rows: &Vec<String>) -> Option<HashMap<char, usize>> {
    if rows.len() == 0 {
        return None;
    }

    // compute frequency for each character in set of rows
    let mut frequencies: HashMap<char, usize> = HashMap::new();
    for row in rows {
        let row_data = row.as_str();
        for c in row_data.chars() {
            let freq = frequencies.entry(c).or_insert(0);
            *freq += 1;
        }
    }

    Some(frequencies)
}

fn stack_layer_rows(top: &String, bottom: &String) -> Option<String> {
    let top_data: Vec<_> = top.chars().collect();
    let bottom_data: Vec<_> = bottom.chars().collect();

    // stack row data
    let mut stacked_data: String = String::new();
    for i in 0..top_data.len() {
        if top_data[i] != '2' {
            // top pixel is black or white
            stacked_data.push(top_data[i]);
        } else {
            // top pixel is transparent
            stacked_data.push(bottom_data[i]);
        }
    }

    Some(stacked_data)
}

impl SpaceImageLayer {
    fn new(shape: (usize, usize), data: &str) -> Option<SpaceImageLayer> {
        let mut rows: Vec<String> = Vec::new();

        // separate data into rows based on layer size
        for i in 0..shape.1 {
            let row_data: String = data[(shape.0 * i)..(shape.0 * (i + 1))].to_owned();
            rows.push(row_data.to_string());
        }

        // compute frequencies of each unique character in this layer
        let frequencies = compute_layer_frequencies(&rows).unwrap();

        Some(SpaceImageLayer { rows, frequencies })
    }

    fn combine(top: &SpaceImageLayer, bottom: &SpaceImageLayer) -> Option<SpaceImageLayer> {
        // stack each row
        let mut rows: Vec<String> = Vec::new();
        for i in 0..top.rows.len() {
            rows.push(stack_layer_rows(&top.rows[i], &bottom.rows[i]).unwrap());
        }

        // compute frequencies of each unique character in resulting layer
        let frequencies = compute_layer_frequencies(&rows).unwrap();
        Some(SpaceImageLayer { rows, frequencies })
    }
}

#[derive(Debug)]
struct SpaceImage {
    /// Shape of each image layer
    ///
    /// shape.0 = layer width
    /// shape.1 = layer height
    shape: (usize, usize),
    /// Layers of space image
    layers: Vec<SpaceImageLayer>,
}

impl SpaceImage {
    fn new(shape: (usize, usize), data: &String) -> Option<SpaceImage> {
        let mut layers: Vec<SpaceImageLayer> = Vec::new();

        let layer_size: usize = shape.0 * shape.1;
        let n: usize = data.len() / layer_size;

        // separate data into layers based on image size
        for i in 0..n {
            layers.push(
                SpaceImageLayer::new(shape, &data[(layer_size * i)..(layer_size * (i + 1))])
                    .unwrap(),
            );
        }

        Some(SpaceImage { shape, layers })
    }

    fn render(&mut self) {        
        if self.layers.len() < 2 {
            return;
        }

        // stack image layers
        while self.layers.len() > 1 {
            let top = self.layers.remove(0);
            let bottom = self.layers.remove(0);
            self.layers
                .insert(0, SpaceImageLayer::combine(&top, &bottom).unwrap());
        }
    }
}

fn main() {
    // read in problem input
    println!("Running problem program...");
    let args: Vec<String> = env::args().collect();
    let mut f = File::open(&args[1]).expect("File not found!");
    let mut data = String::new();
    f.read_to_string(&mut data)
        .expect("Something went wrong while reading the file!");

    // create problem image
    let mut prob_image = SpaceImage::new((25, 6), &data).unwrap();

    // find layer with fewest zeros
    let mut min_zeros: usize = 0;
    let mut min_ind: usize = 0;
    for i in 0..prob_image.layers.len() {
        if i == 0 {
            min_zeros = *prob_image.layers[i].frequencies.get(&'0').unwrap();
        } else {
            let cur_zeros = *prob_image.layers[i].frequencies.get(&'0').unwrap();
            if cur_zeros < min_zeros {
                min_zeros = cur_zeros;
                min_ind = i;
            }
        }
    }
    println!("Layer with fewest zeros:");
    println!("{:?}", prob_image.layers[min_ind]);

    // render problem image
    prob_image.render();
    for row in &prob_image.layers[0].rows {
        println!("{}", row.replace("0", " "));
    }
}
