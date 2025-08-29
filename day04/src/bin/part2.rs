use anyhow::{bail, Context, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

/*
https://adventofcode.com/2021/day/04
*/

pub fn read_input(file_path: &str) -> Result<String> {
    let mut file = File::open(file_path).context("File not found")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .context("Error reading file")?;
    Ok(contents)
}

#[allow(dead_code)]
const INPUT_FILE: &str = "input.txt";
#[allow(dead_code)]
const EXAMPLE_FILE: &str = "example.txt";

#[derive(Debug, Clone)]
struct Board {
    pos: HashMap<i32, (usize, usize)>,
    marked: [[bool; 5]; 5],
    row_counts: [u8; 5],
    col_counts: [u8; 5],
    has_won: bool,
}

impl Board {
    fn new() -> Self {
        Self {
            pos: HashMap::new(),
            marked: [[false; 5]; 5],
            row_counts: [0; 5],
            col_counts: [0; 5],
            has_won: false,
        }
    }

    fn mark_and_check(&mut self, n: i32) -> bool {
        if let Some((r, c)) = self.pos.remove(&n) {
            if !self.marked[r][c] {
                self.marked[r][c] = true;
                self.row_counts[r] += 1;
                self.col_counts[c] += 1;
                if self.row_counts[r] == 5 || self.col_counts[c] == 5 {
                    self.has_won = true;
                    return true;
                }
            }
        }
        false
    }

    fn sum_unmarked(&self) -> i32 {
        self.pos.keys().copied().sum()
    }
}

fn parse_input(input: &str) -> Result<(Vec<i32>, Vec<Board>)> {
    let mut sections = input.split("\n\n");

    let draws_str = sections
        .next()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .context("Missing draws line")?;

    let draws: Vec<i32> = draws_str
        .split(',')
        .map(|s| s.trim().parse::<i32>().context("Bad number in draws"))
        .collect::<Result<Vec<_>>>()?;

    let mut boards: Vec<Board> = Vec::new();

    for sec in sections {
        let sec = sec.trim();
        if sec.is_empty() {
            continue;
        }

        let mut board = Board::new();

        let rows: Vec<&str> = sec
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .collect();
        if rows.len() != 5 {
            bail!("Board does not have 5 rows:\n{sec}");
        }

        for (r, line) in rows.iter().enumerate() {
            let toks: Vec<&str> = line.split_whitespace().collect();
            if toks.len() != 5 {
                bail!("Board row is not 5-wide:\n{sec}");
            }
            for (c, tok) in toks.iter().enumerate() {
                let n = tok
                    .parse::<i32>()
                    .with_context(|| format!("Bad board number: '{tok}'"))?;

                if let Some((pr, pc)) = board.pos.insert(n, (r, c)) {
                    bail!("Duplicate number {n} in board at ({pr},{pc}) and ({r},{c})");
                }
            }
        }

        if board.pos.len() != 25 {
            bail!(
                "Board is not 5x5 (unique numbers): found {}",
                board.pos.len()
            );
        }

        boards.push(board);
    }

    if boards.is_empty() {
        bail!("No boards parsed");
    }

    Ok((draws, boards))
}

fn play_to_last_win(mut boards: Vec<Board>, draws: &[i32]) -> Option<i32> {
    let mut remaining = boards.len();
    let mut last_score: Option<i32> = None;

    for &n in draws {
        for b in boards.iter_mut().filter(|b| !b.has_won) {
            if b.mark_and_check(n) {
                remaining -= 1;
                last_score = Some(b.sum_unmarked() * n);
                if remaining == 0 {
                    return last_score;
                }
            }
        }
    }

    last_score
}

fn main() -> Result<()> {
    let contents: String = read_input(INPUT_FILE)?;
    let (draws, boards) = parse_input(&contents)?;

    println!("draws: {} numbers", draws.len());
    println!("boards: {}", boards.len());

    if let Some(score) = play_to_last_win(boards, &draws) {
        println!("Part 2 (last winner) score: {score}");
    } else {
        println!("Part 2: no winner.");
    }

    Ok(())
}
