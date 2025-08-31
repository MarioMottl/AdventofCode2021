use anyhow::Result;
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

fn main() {
    #[allow(unused_variables)]
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));

    let mut input_values: Vec<Vec<&str>> = Vec::new();
    let mut output_values: Vec<Vec<&str>> = Vec::new();

    for line in contents.lines() {
        let mut it = line.split('|');
        input_values.push(it.next().unwrap().split_ascii_whitespace().collect());
        output_values.push(it.next().unwrap().split_ascii_whitespace().collect());
    }

    let mut unique_count = 0;
    for row in output_values {
        for digit in row {
            match digit.len() {
                2 | 3 | 4 | 7 => unique_count += 1,
                _ => {}
            }
        }
    }

    println!("{}", unique_count);
}
