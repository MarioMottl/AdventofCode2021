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

    let mut items: Vec<(u32, u32)> = freq.into_iter().collect();
    items.sort_unstable_by_key(|&(pos, _)| pos);

    let total: u64 = items.iter().map(|&(_, c)| c as u64).sum();
    let mut acc: u64 = 0;
    let mut median_pos: u32 = 0;
    for (pos, count) in &items {
        acc += *count as u64;
        if acc * 2 >= total {
            median_pos = *pos;
            break;
        }
    }

    let mut fuel: u64 = 0;
    for (pos, count) in &items {
        fuel += (median_pos.abs_diff(*pos) as u64) * (*count as u64);
    }

    println!("{}", fuel);
}
