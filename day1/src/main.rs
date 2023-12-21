use std::fs::File;
use std::io::Result;
use std::io::{BufRead, BufReader};

pub fn line_to_number(line: &str) -> u32 {
    let digits = line
        .chars()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    digits.iter().next().unwrap() * 10 + digits.iter().rev().next().unwrap()
}

fn main() -> Result<()> {
    let file = File::open("input.txt")?;
    // Using a buffer just for funsies
    let reader = BufReader::new(file);

    let sum = reader.lines()
        .fold(0, |acc, line| {
            acc + line_to_number(line.unwrap().as_str())
        });
    println!("{sum}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_to_number() {
        assert_eq!(12, line_to_number("1abc2"));
        assert_eq!(38, line_to_number("pqr3stu8vwx"));
        assert_eq!(15, line_to_number("a1b2c3d4e5f"));
        assert_eq!(77, line_to_number("treb7uchet"));
    }
}
