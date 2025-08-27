use anyhow::Result;
use std::fs::File;
use std::io::Read;

/*
https://adventofcode.com/2021/day/03
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

    let mut it = contents.lines();

    let width = it.next().expect("No lines").len();

    let report: Vec<u32> = contents
        .lines()
        .map(|x| u32::from_str_radix(x, 2).expect("Couldnt transfrom line into u32"))
        .collect();

    let mut gamma = 0u32;
    let mut epsilon = 0u32;

    for i in (0..width).rev() {
        let mask = 1 << i;
        let ones = report.iter().filter(|&&v| (v & mask) != 0).count();

        if ones * 2 >= report.len() {
            gamma |= mask;
        } else {
            epsilon |= mask;
        }
    }

    println!("{}", gamma * epsilon);
}
