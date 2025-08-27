use anyhow::Result;
use std::fs::File;
use std::io::Read;

/*
https://adventofcode.com/2021/day/02
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

    let mut vertical = 0;
    let mut horizontal = 0;
    let mut aim = 0;

    for line in contents.lines() {
        let mut parts = line.trim().split_ascii_whitespace();
        let direction = parts.next().expect("Missing direction");
        let steps: i32 = parts
            .next()
            .expect("missing steps")
            .parse()
            .expect("steps not a number");

        match direction {
            "forward" => {
                horizontal += steps;
                vertical += aim * steps;
            }
            "down" => {
                aim += steps;
            }
            "up" => {
                aim -= steps;
            }
            _ => panic!("Unknown direction"),
        }
    }

    println!("Result: {}", vertical * horizontal)
}
