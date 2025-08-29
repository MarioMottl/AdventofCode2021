use anyhow::Result;
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

fn main() {
    #[allow(unused_variables)]
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));

    let mut state: Vec<i32> = contents
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().expect("Couldnt parse &str into i32"))
        .collect();

    dbg!(&state);

    let days = 80;

    for _ in 0..days {
        for i in 0..state.len() {
            if state[i] == 0 {
                state[i] = 6;
                state.push(8);
            } else {
                state[i] -= 1;
            }
        }
    }

    println!("Len after: {}Days ==> {}", days, state.len());
}
