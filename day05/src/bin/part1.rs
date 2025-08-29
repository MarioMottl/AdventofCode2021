use anyhow::{bail, Context, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

/*
https://adventofcode.com/2021/day/05
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

struct LineSegment {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

fn parse_line(line: &str) -> Result<LineSegment> {
    // format: "x1,y1 -> x2,y2"
    let (lhs, rhs) = line.split_once("->").context("Missing '->' in line")?;

    let (x1s, y1s) = lhs.trim().split_once(',').context("Bad left coord")?;
    let (x2s, y2s) = rhs.trim().split_once(',').context("Bad right coord")?;

    let x1 = x1s.trim().parse::<i32>().context("Bad x1")?;
    let y1 = y1s.trim().parse::<i32>().context("Bad y1")?;
    let x2 = x2s.trim().parse::<i32>().context("Bad x2")?;
    let y2 = y2s.trim().parse::<i32>().context("Bad y2")?;

    Ok(LineSegment { x1, y1, x2, y2 })
}

fn parse_input(input: &str) -> Result<Vec<LineSegment>> {
    let mut segments = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let seg = parse_line(line).with_context(|| format!("On line {}: '{line}'", i + 1))?;
        segments.push(seg);
    }
    if segments.is_empty() {
        bail!("No segments parsed");
    }
    Ok(segments)
}

fn overlaps_hv(segments: &Vec<LineSegment>) -> usize {
    let mut hits: HashMap<(i32, i32), u32> = HashMap::new();
    for &LineSegment { x1, y1, x2, y2 } in segments {
        if x1 == x2 {
            //vertical
            let (a, b) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };
            for y in a..=b {
                *hits.entry((x1, y)).or_insert(0) += 1;
            }
        } else if y1 == y2 {
            //horizontal
            let (a, b) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
            for x in a..=b {
                *hits.entry((x, y1)).or_insert(0) += 1;
            }
        } else {
            // ignore diagonal
            continue;
        }
    }
    hits.values().filter(|&&c| c >= 2).count()
}

fn main() {
    #[allow(unused_variables)]
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));
    let segments = parse_input(&contents).expect("Couldnt parse input");

    let result = overlaps_hv(&segments);
    println!("{}", result)
}
