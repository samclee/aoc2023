use std::collections::HashSet;
use std::f32::consts::TAU;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

use my_vec::MyVec;

#[cfg(test)]
mod tests;

mod my_vec;

enum ParseState {
    Idle,
    ValidInProgress(u32),
    InvalidInProgress(u32)
}

enum CharType {
    Digit(u32),
    Terminator,
}

fn change_state(state: &ParseState, new_char: CharType, symbol_adjacent: bool) -> (ParseState, Option<u32>)  {
    match (state, new_char, symbol_adjacent) {
        (ParseState::Idle, CharType::Digit(digit), false)                         => (ParseState::InvalidInProgress(digit), Option::None),
        (ParseState::Idle, CharType::Digit(digit), true)                          => (ParseState::ValidInProgress(digit), Option::None),
        (ParseState::Idle, CharType::Terminator, _)                                    => (ParseState::Idle, Option::None),

        (ParseState::ValidInProgress(val), CharType::Digit(digit), _)       => (ParseState::ValidInProgress(val * 10 + digit), Option::None),
        (ParseState::ValidInProgress(val), CharType::Terminator, _)              => (ParseState::Idle, Some(*val)),

        (ParseState::InvalidInProgress(val), CharType::Digit(digit), false) => (ParseState::InvalidInProgress(val * 10 + digit), Option::None),
        (ParseState::InvalidInProgress(val), CharType::Digit(digit), true)  => (ParseState::ValidInProgress(val * 10 + digit), Option::None),
        (ParseState::InvalidInProgress(_), CharType::Terminator, _)                    => (ParseState::Idle, Option::None),
    }
}

fn is_symbol(c: char) -> bool {
    !c.is_numeric() && (c != '.')
}

fn get_adjacent_coords(x: i32, y: i32) -> Vec<MyVec> {
    (0..8).map(|i| {
        let angle = (i as f32 / 8_f32)  * TAU;
        let (csin, ccos) = angle.sin_cos();
        let dx = ccos.round() as i32;
        let dy = csin.round() as i32;
        MyVec::new(x + dx, y + dy)
    })
    .collect::<Vec<_>>()
}

fn get_symbol_coords(fname: &str) -> HashSet<MyVec> {
    let file = File::open(fname).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.unwrap()
                .char_indices()
                .filter_map(|(x, c)| {
                    if is_symbol(c) {
                        Some(MyVec::new(x as i32, y as i32))
                    }
                    else {
                        None
                    }
                })
                .collect::<HashSet<MyVec>>()
        })
        .collect::<HashSet<MyVec>>()
}

fn part1(fname: &str) -> u32 {
    let symbol_coords = get_symbol_coords(fname);

    let file = File::open(fname).unwrap();
    let reader = BufReader::new(file);
    let mut state = ParseState::Idle;
    let mut sum = 0_u32;
    for (y, line) in reader.lines().enumerate() {
        for (x, c) in line.unwrap().char_indices() {
            let new_char = if c.is_numeric() {
                CharType::Digit(c.to_digit(10).unwrap())
            }
            else {
                CharType::Terminator
            };
            let symbol_adjacent = get_adjacent_coords(x as i32, y as i32)
                .iter()
                .any(|coord| symbol_coords.contains(coord));

            let (new_state, valid_val) = change_state(&state, new_char, symbol_adjacent);
            state = new_state;

            if let Some(val) = valid_val {
                sum += val;
            }
        }

        // This is disgusting. EOL case
        let (new_state, valid_val) = change_state(&state, CharType::Terminator, false);
        state = new_state;

        if let Some(val) = valid_val {
            sum += val;
        }
    }
    // Also disgusting. EOF case
    let (_, valid_val) = change_state(&state, CharType::Terminator, false);
    if let Some(val) = valid_val {
        sum += val;
    }
    sum
}

fn main() -> Result<()> {
    let fname = "input.txt";
    let sum = part1(fname);
    println!("{}", sum);

    Ok(())
}
