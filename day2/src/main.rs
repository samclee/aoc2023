use std::fs::File;
use std::io::Read;
use std::io::Result;

// --- part 1 ---
pub fn is_game_good(line: &str) -> bool {
    !line.rsplit(":").next().unwrap()
        .split(";") // round = "3 blue, 4 red
        .any(|round| is_round_bad(round.trim()))
}

pub fn is_round_bad(round: &str) -> bool {
    round
        .split(",")
        .any(|pull| is_pull_bad(pull.trim()))
}

pub fn is_pull_bad(pull: &str) -> bool {
    let mut pulls_iter = pull.split(" ");
    let pulled_amount = pulls_iter.next().unwrap().parse::<u32>().unwrap();
    let color_name = pulls_iter.next().unwrap();

    pulled_amount > string_to_limit(color_name)
}

pub fn string_to_limit(color_name: &str) -> u32 {
    match color_name {
        "red" => 12,
        "green" => 13,
        "blue" => 14,
        _ => panic!("what the HELL")
    }
}

fn part1(buf: &String) {
    let sum_of_good_games = buf.lines()
        .enumerate()
        .filter(|(_i, line)| is_game_good(line))
        .map(|(i, _line)| 1 + i as u64)
        .sum::<u64>();
    println!("{:?}", sum_of_good_games);
}
// --- end part 1 ---

fn main() -> Result<()> {
    let mut file = File::open("input.txt")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    part1(&buf);
    Ok(())
}
