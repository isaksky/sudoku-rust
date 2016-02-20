use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::error::Error;
use std::fmt;

struct Puzzle {
    cells: [u32; 81]
}

impl Clone for Puzzle {
    fn clone(&self) -> Self {
        Puzzle { cells: self.cells }
    }
}

fn print_puzzle(puzz: &[u32; 81]) -> String {
    let mut s = String::new();
    for i in 0..81 {
        let after = if i % 9 == 8 { "\n" } else { "" };
        s.push_str(&(format!("{} {}", puzz[i], after)));
    }
    s
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", print_puzzle(&self.cells))
    }
}

fn main() {
    let f = match File::open("sudoku.txt") {
        Ok(file) => file,
        Err(e) => panic!("Couldn't open file: {}", Error::description(&e))
    };
    
    let reader = BufReader::new(f);
    let mut puzzle : [u32; 81] = [0; 81];
    let mut puzzles : Vec<Puzzle> = Vec::new();
    let mut i = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with("Grid") {
            println!("Parsing {}", line);
            if i == 81 {
                puzzles.push(Puzzle { cells : puzzle });
                puzzle = [0; 81];
                i = 0;
            }
        } else {
            for c in line.chars() {
                match c {
                    ' ' => {},
                    _ => {
                        puzzle[i] = c.to_digit(10).unwrap();
                        i += 1;
                    }
                }
            }
        }
    }
    if i == 81 { puzzles.push(Puzzle { cells : puzzle }); }

    println!("Parsed {} puzzles!\n", puzzles.len());
    for p in puzzles.iter_mut() {
        println!("Puzzle:\n{}", p);
        match solve(p.to_owned()) {
            Some(b) => {
                println!("Solution: \n{}", b);
            },
            None => {
                println!("Failed to solve puzzle!");
                break;
            }
        }
    }
}

fn row_vals(puzz: &Puzzle, idx: usize) -> [bool; 9] {
    let mut ret : [bool; 9] = [false; 9];
    let row = idx / 9;
    for col in 0..9 {
        let idx = col + (row * 9);
        let v = puzz.cells[idx as usize];
        if v > 0 {
            ret[(v - 1) as usize] = true;
        }
    }
    ret
}

fn col_vals(puzz: &Puzzle, idx: usize) -> [bool; 9] {
    let mut ret : [bool; 9] = [false; 9];
    let col = idx % 9;
    for row in 0..9 {
        let idx = col + (row * 9);
        let v = puzz.cells[idx as usize];
        if v > 0 {
            ret[(v - 1) as usize] = true;
        }
    }
    ret
}

fn subgrid_vals(puzz: &Puzzle, idx: usize) -> [bool; 9] {
    let mut ret : [bool; 9] = [false; 9];
    let col = idx % 9;
    let row = idx / 9;
    let scol = col / 3 * 3;
    let srow = row / 3 * 3;

    for r in 0..3 {
        for c in 0..3 {
            let idx = scol + c + (srow + r) * 9;
            let v = puzz.cells[idx];
            if v > 0 {
                ret[(v - 1) as usize] = true;
            }   
        }
    }
    ret
}

fn poss_for_idx(puzz: &Puzzle, idx: usize) -> Vec<u32> {
    let colvals = col_vals(puzz, idx);
    let rowvals = row_vals(puzz, idx);
    let subgridvals = subgrid_vals(puzz, idx);

    let mut poss : Vec<u32> = Vec::new();
    for i in 0..9 {
        let taken = colvals[i] || rowvals[i] || subgridvals[i];
        if !taken {
            poss.push((i + 1) as u32);
        }
    }
    poss    
}

fn solve(puzz: Puzzle) -> Option<Puzzle> {
    let mut puzz = puzz;
    let (mut best_idx, mut best_idx_poss) = (0, vec![1u32,2,3,4,5,6,7,8,9]);
    loop {
        let mut filled_square = false;
        let mut any_empty = false;        
        for idx in 0..81 {
            if puzz.cells[idx] == 0 {
                let poss = poss_for_idx(&puzz, idx);
                match poss.len() {
                    0 => { return None; },                
                    1 => {
                        puzz.cells[idx] = poss[0];
                        filled_square = true;
                    },
                    _ => {
                        any_empty = true;
                        if poss.len() < best_idx_poss.len() {
                            best_idx_poss = poss;
                            best_idx = idx;                            
                        }
                    }
                }
            }
        }
        if !any_empty { return Some(puzz); }
        if !filled_square { break; }
    }

    for v in best_idx_poss {
        let mut puzz_copy = puzz.clone();
        puzz_copy.cells[best_idx] = v;
        let res = solve(puzz_copy);
        match res {
            Some(p) => { return Some(p); },
            None => { }            
        }
    }
    None
}
