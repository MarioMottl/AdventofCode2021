use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

/*
https://adventofcode.com/2021/day/07
*/

pub fn read_input(file_path: &str) -> Result<String> {
    let mut file = File::open(file_path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    Ok(contents)
}

#[allow(dead_code)]
const INPUT_FILE: &str = "input.txt";
#[allow(dead_code)]
const EXAMPLE_FILE: &str = "example.txt";

fn tri(d: u64) -> u64 {
    d * (d + 1) / 2
}

fn total_cost(freq: &HashMap<u32, u32>, target: u32) -> u64 {
    let t = target as u64;
    let mut sum: u64 = 0;
    for (&pos, &cnt) in freq.iter() {
        let d = (pos as u64).abs_diff(t);
        sum += tri(d) * (cnt as u64);
    }
    sum
}

fn main() {
    #[allow(unused_variables)]
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));
    let mut freq: HashMap<u32, u32> = HashMap::new();

    let initial_input: Vec<u32> = contents
        .trim()
        .split(',')
        .map(|x| x.parse::<u32>().expect("Couldnt parse &str to i32"))
        .collect();

    for num in initial_input {
        *freq.entry(num).or_insert(0) += 1;
    }

    let min_pos = *freq.keys().min().unwrap();
    let max_pos = *freq.keys().max().unwrap();

    let mut best_cost = u64::MAX;
    let mut best_t = 0u32;

    for t in min_pos..=max_pos {
        let c = total_cost(&freq, t);
        if c < best_cost {
            best_cost = c;
            best_t = t;
        }
    }

    println!("target={}, fuel={}", best_t, best_cost);
}
