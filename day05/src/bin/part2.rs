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
        let dx_total = x2 - x1;
        let dy_total = y2 - y1;

        let hv = dx_total == 0 || dy_total == 0;
        let diag45 = dx_total.abs() == dy_total.abs();

        if !(hv || diag45) {
            continue;
        }

        let dx = dx_total.signum();
        let dy = dy_total.signum();
        let steps = dx_total.abs().max(dy_total.abs());

        for i in 0..=steps {
            let x = x1 + i * dx;
            let y = y1 + i * dy;
            *hits.entry((x, y)).or_insert(0) += 1;
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
