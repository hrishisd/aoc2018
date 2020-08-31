use itertools::iproduct;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn main() -> Result<()> {
    let input = fs::read_to_string("input")?;
    println!("{}", part1(&input)?);
    println!("{:?}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> Result<usize> {
    let mut num_with_2_repeated_chars = 0usize;
    let mut num_with_3_repeated_chars = 0usize;
    for line in input.lines() {
        let counter = char_counts(line);
        let counts: HashSet<&usize> = counter.values().collect();
        if counts.contains(&2usize) {
            num_with_2_repeated_chars += 1;
        }
        if counts.contains(&3usize) {
            num_with_3_repeated_chars += 1;
        }
    }
    Ok(num_with_2_repeated_chars * num_with_3_repeated_chars)
}

fn part2(input: &str) -> Result<String> {
    for (s1, s2) in itertools::iproduct!(input.lines(), input.lines()) {
        if let Some(common) = get_common_chars_if_diff_is_one_char(s1, s2) {
            return Ok(common);
        }
    }
    Err(From::from("didn't find pair with diff of one char"))
}

fn get_common_chars_if_diff_is_one_char(s1: &str, s2: &str) -> Option<String> {
    let common: String = s1
        .chars()
        .zip(s2.chars())
        .filter(|(c1, c2)| c1 == c2)
        .map(|t| t.0)
        .collect();

    if s1.len() == s2.len() && common.len() == s1.len() - 1 {
        Some(common)
    } else {
        None
    }
}

fn char_counts(s: &str) -> HashMap<char, usize> {
    let mut counter = HashMap::new();
    for c in s.chars() {
        let count = counter.entry(c).or_insert(0);
        *count += 1;
    }
    counter
}
