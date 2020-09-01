type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let polymer: Vec<char> = std::fs::read_to_string("input")?.trim().chars().collect();
    let reduced_polymer = reduce(polymer)?;
    println!("Part1: {}", reduced_polymer.len());
    println!("Part2: {}", part2(reduced_polymer)?);
    Ok(())
}

fn part2(polymer: Vec<char>) -> Result<usize> {
    (b'a'..=b'z')
        .map(|c| {
            polymer
                .iter()
                .filter(|unit| unit.to_lowercase().next().unwrap() != c as char)
                .map(|&c| c)
                .collect::<Vec<char>>()
        })
        .map(|filtered_polymer| reduce(filtered_polymer).unwrap().len())
        .min()
        .ok_or(From::from("no polymers"))
}

fn reduce(mut polymer: Vec<char>) -> Result<Vec<char>> {
    let mut previous_len;
    loop {
        previous_len = polymer.len();
        polymer = reduce_step(polymer)?;
        if polymer.len() == previous_len {
            break Ok(polymer);
        }
    }
}

fn reduce_step(mut polymer: Vec<char>) -> Result<Vec<char>> {
    let mut insert_idx = 0;
    let mut curr_idx = 0;
    while curr_idx < polymer.len() {
        if curr_idx + 1 < polymer.len() && will_react(polymer[curr_idx], polymer[curr_idx + 1])? {
            // they get destroyed
            curr_idx += 2;
        } else {
            polymer[insert_idx] = polymer[curr_idx];
            insert_idx += 1;
            curr_idx += 1;
        }
    }
    polymer.truncate(insert_idx);
    Ok(polymer)
}

fn will_react(first: char, second: char) -> Result<bool> {
    if first.is_lowercase() {
        Ok(second.is_uppercase()
            && first == second.to_lowercase().next().ok_or("invalid character")?)
    } else {
        Ok(second.is_lowercase()
            && first.to_lowercase().next().ok_or("invalid character")? == second)
    }
}
