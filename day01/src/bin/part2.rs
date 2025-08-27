use anyhow::Result;
use std::fs::File;
use std::i32;
use std::io::Read;

/*
https://adventofcode.com/2024/day/1
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
    println!("{}", contents);

    let mut prev = i32::MAX;
    let mut result = 0;
    let report = contents
        .lines()
        .map(|x| {
            x.trim()
                .parse::<i32>()
                .expect("Couldnt parse line into i32")
        })
        .collect::<Vec<i32>>();

    for index in 0..report.len() - 2 {
        let sliding_window = report[index] + report[index + 1] + report[index + 2];
        if prev < sliding_window {
            result += 1;
        }
        prev = sliding_window;
    }

    println!("Result: {}", result);
}
