use std::collections::HashSet;
use std::fs;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn main() -> Result<()> {
    let input = fs::read_to_string("input")?;
    println!("net frequency is {}", calculate_net_frequency(&input)?);
    println!("first duplicate frequency is {}", first_dup(&input)?);
    Ok(())
}

fn calculate_net_frequency(s: &str) -> Result<i32> {
    let mut freq = 0;
    for diff_str in s.lines() {
        let diff: i32 = diff_str.parse()?;
        freq += diff;
    }
    Ok(freq)
}

fn first_dup(s: &str) -> Result<i32> {
    let mut freq = 0;
    let mut seen = HashSet::new();
    seen.insert(0);
    for diff_str in s.lines().cycle() {
        let diff: i32 = diff_str.parse()?;
        freq += diff;
        if !seen.insert(freq) {
            return Ok(freq);
        }
    }
    Ok(0)
}
