use anyhow::{Context, Result};
use std::fs::File;
use std::io::Read;

/*
https://adventofcode.com/2021/day/06
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

fn parse_timers(s: &str) -> Result<[u128; 9]> {
    let mut counts = [0u128; 9];
    for tok in s.trim().split(',') {
        if tok.is_empty() {
            continue;
        }
        let t: usize = tok.trim().parse().context("Bad timer")?;
        if t > 8 {
            anyhow::bail!("Timer out of range: {t}");
        }
        counts[t] += 1;
    }
    Ok(counts)
}

fn simulate(mut counts: [u128; 9], days: u32) -> u128 {
    for _ in 0..days {
        let births = counts[0];
        counts.rotate_left(1); // 1->0, 2->1, ..., 8->7, births go to index 8
        counts[6] += births; // parents reset to 6
    }
    counts.iter().sum()
}

fn main() {
    #[allow(unused_variables)]
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));

    let state = parse_timers(&contents).expect("Couldnt parse input");
    let days = 256;

    println!("Len after: {}Days ==> {}", days, simulate(state, days));
}
