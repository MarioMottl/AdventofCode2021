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

fn rating(mut nums: Vec<u32>, width: usize, keep_most: bool) -> u32 {
    for i in (0..width).rev() {
        if nums.len() == 1 {
            break;
        }

        let mask = 1u32 << i;
        let ones = nums.iter().filter(|&&v| (v & mask) != 0).count();
        let n = nums.len();

        let desired_bit_is_one = if keep_most {
            ones * 2 >= n
        } else {
            ones * 2 < n
        };

        nums.retain(|&v| ((v & mask) != 0) == desired_bit_is_one);
    }

    nums[0]
}

fn oxygen_rating(nums: &[u32], width: usize) -> u32 {
    rating(nums.to_vec(), width, true)
}

fn co2_rating(nums: &[u32], width: usize) -> u32 {
    rating(nums.to_vec(), width, false)
}

fn main() {
    #[allow(unused_variables)]
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));

    let mut it = contents.lines();

    let width = it.next().expect("No lines").len();

    let report: Vec<u32> = contents
        .lines()
        .map(|x| u32::from_str_radix(x, 2).expect("Couldnt transfrom line into u32"))
        .collect();

    let o2 = oxygen_rating(&report, width);
    let co2 = co2_rating(&report, width);

    println!("{}", o2 * co2);
}
