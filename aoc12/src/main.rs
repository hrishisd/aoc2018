// This solution could have been a lot shorter if I used raw &str instead of making a bunch of types
use std::collections::HashMap;
use std::str::FromStr;
use std::{error, result};

type BoxedError = Box<dyn error::Error>;
type Result<T> = result::Result<T, BoxedError>;
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum PotState {
    Plant,
    Empty,
}

#[derive(Debug)]
struct Rule {
    surrounding_pots: SurroundingPots,
    result_state: PotState,
}
#[derive(Debug, Eq, PartialEq, Hash)]
struct SurroundingPots([PotState; 5]);

#[derive(Debug, Clone)]
// I probably should have used a deque here instead of two vectors
struct Pots {
    nonnegative: Vec<PotState>,
    negative: Vec<PotState>,
}

fn main() -> Result<()> {
    let initial_state = "######....##.###.#..#####...#.#.....#..#.#.##......###.#..##..#..##..#.##..#####.#.......#.....##..";
    let mut pots = Pots::from_initial_state(
        initial_state
            .chars()
            .map(|c| match c {
                '#' => PotState::Plant,
                _ => PotState::Empty,
            })
            .collect(),
    );
    let input = std::fs::read_to_string("input")?;
    let rule_map: HashMap<SurroundingPots, PotState> = {
        let mut rules = HashMap::with_capacity(32);
        for line in input.lines() {
            let rule: Rule = line.parse()?;
            rules.insert(rule.surrounding_pots, rule.result_state);
        }
        rules
    };
    for i in 0..20 {
        pots = pots.next_generation(&rule_map);
    }
    let sum: i32 = pots.pots_with_plants().sum();
    println!("{}", sum);
    Ok(())
}

impl Pots {
    fn from_initial_state(initial_state: Vec<PotState>) -> Pots {
        Pots {
            nonnegative: initial_state,
            negative: vec![PotState::Empty; 4],
        }
    }

    fn next_generation(&mut self, rules: &HashMap<SurroundingPots, PotState>) -> Pots {
        let mut result = self.clone();
        // This is really ugly
        let (l, r) = (
            std::cmp::min(0, -(self.negative.len() as i32 - 1)),
            (self.nonnegative.len() - 1) as i32,
        );
        for i in (l - 2)..=r + 2 {
            let surrounding_pots = self.get_surrounding(i);
            result.put(i, *rules.get(&surrounding_pots).unwrap_or(&PotState::Empty));
        }
        result
    }

    fn get(&mut self, pot_number: i32) -> PotState {
        let idx = pot_number.abs() as usize;
        if pot_number >= 0 {
            if idx >= self.nonnegative.len() {
                self.nonnegative.resize(idx + 1, PotState::Empty);
            }
            self.nonnegative[idx]
        } else {
            if idx >= self.negative.len() {
                self.negative.resize(idx + 1, PotState::Empty);
            }
            self.negative[idx]
        }
    }

    fn get_surrounding(&mut self, pot_number: i32) -> SurroundingPots {
        SurroundingPots([
            self.get(pot_number - 2),
            self.get(pot_number - 1),
            self.get(pot_number),
            self.get(pot_number + 1),
            self.get(pot_number + 2),
        ])
    }

    fn put(&mut self, pot_number: i32, state: PotState) {
        let idx = pot_number.abs() as usize;
        if pot_number >= 0 {
            if idx >= self.nonnegative.len() {
                self.nonnegative.resize(idx + 1, PotState::Empty)
            }
            self.nonnegative[idx] = state;
        } else {
            if idx >= self.negative.len() {
                self.negative.resize(idx + 1, PotState::Empty);
            }
            self.negative[idx] = state;
        }
    }

    fn pots_with_plants<'a>(&'a self) -> impl Iterator<Item = i32> + 'a {
        let negative_pot_numbers = self
            .negative
            .iter()
            .enumerate()
            .filter(|(_, &state)| state == PotState::Plant)
            .map(|(idx, _)| -(idx as i32));
        let nonnegative_pot_numbers = self
            .nonnegative
            .iter()
            .enumerate()
            .filter(|(_, &state)| state == PotState::Plant)
            .map(|(idx, _)| idx as i32);

        negative_pot_numbers.chain(nonnegative_pot_numbers)
    }
}

impl FromStr for PotState {
    type Err = BoxedError;

    fn from_str(s: &str) -> Result<Self> {
        match s.chars().next() {
            Some('#') => Ok(PotState::Plant),
            Some('.') => Ok(PotState::Empty),
            _ => Err("error".into()),
        }
    }
}

impl FromStr for Rule {
    type Err = BoxedError;
    fn from_str(s: &str) -> Result<Self> {
        let mut iter = s.split_whitespace();
        let lhs = iter.next().unwrap();
        let result_state = iter.next_back().unwrap();

        let lhs: Vec<&str> = lhs.split_terminator("").skip(1).collect();
        Ok(Rule {
            surrounding_pots: SurroundingPots([
                lhs[0].parse()?,
                lhs[1].parse()?,
                lhs[2].parse()?,
                lhs[3].parse()?,
                lhs[4].parse()?,
            ]),
            result_state: result_state.parse()?,
        })
    }
}
