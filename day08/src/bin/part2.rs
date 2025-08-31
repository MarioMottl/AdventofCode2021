use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

/*
https://adventofcode.com/2021/day/8
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

fn mask(s: &str) -> u8 {
    s.bytes().fold(0u8, |m, b| m | 1 << (b - b'a'))
}

fn deduce_mapping(patterns: &[u8]) -> HashMap<u8, u8> {
    let one = *patterns.iter().find(|&&m| m.count_ones() == 2).unwrap();
    let seven = *patterns.iter().find(|&&m| m.count_ones() == 3).unwrap();
    let four = *patterns.iter().find(|&&m| m.count_ones() == 4).unwrap();
    let eight = *patterns.iter().find(|&&m| m.count_ones() == 7).unwrap();

    let sixes: Vec<u8> = patterns
        .iter()
        .copied()
        .filter(|m| m.count_ones() == 6)
        .collect();
    let fives: Vec<u8> = patterns
        .iter()
        .copied()
        .filter(|m| m.count_ones() == 5)
        .collect();

    let nine = *sixes.iter().find(|&&m| (m & four) == four).unwrap();
    let zero = *sixes
        .iter()
        .find(|&&m| m != nine && (m & one) == one)
        .unwrap();
    let six = *sixes.iter().find(|&&m| m != nine && m != zero).unwrap();

    let three = *fives.iter().find(|&&m| (m & one) == one).unwrap();
    let five = *fives
        .iter()
        .find(|&&m| m != three && (m & six) == m)
        .unwrap();
    let two = *fives.iter().find(|&&m| m != three && m != five).unwrap();

    HashMap::from([
        (zero, 0),
        (one, 1),
        (two, 2),
        (three, 3),
        (four, 4),
        (five, 5),
        (six, 6),
        (seven, 7),
        (eight, 8),
        (nine, 9),
    ])
}

fn decode_output(mapping: &HashMap<u8, u8>, outs: &[u8]) -> u32 {
    outs.iter().fold(0, |acc, &m| acc * 10 + mapping[&m] as u32)
}

fn main() {
    #[allow(unused_variables)]
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));

    let mut total: u64 = 0;

    for line in contents.lines() {
        let mut parts = line.split('|');
        let patterns: Vec<u8> = parts.next().unwrap().split_whitespace().map(mask).collect();
        let outputs: Vec<u8> = parts.next().unwrap().split_whitespace().map(mask).collect();

        let mapping = deduce_mapping(&patterns);
        total += decode_output(&mapping, &outputs) as u64;
    }

    println!("Total: {}", total);
}
