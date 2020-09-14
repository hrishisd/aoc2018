use serde_scan::scan;

#[derive(Debug)]
struct State {
    current_marble: Marble,
    current_marble_idx: usize,
    next_marble: Marble,
    num_players: usize,
    circle: Vec<Marble>,
    points: Vec<usize>,
}

type Marble = usize;
type Result<T> = std::result::Result<T, std::boxed::Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input")?;
    let input_str = input.as_str();
    let (num_players, last_marble): (usize, usize) =
        scan!("{} players; last marble is worth {} points" <- input_str)?;
    let state = State::initial_state(num_players);
    println!("{}", part1(state, last_marble)?);
    Ok(())
}

fn part1(mut state: State, last_marble: Marble) -> Result<usize> {
    while state.current_marble != last_marble {
        state.place_marble();
    }
    state
        .points
        .iter()
        .cloned()
        .max()
        .ok_or_else(|| std::boxed::Box::from("error"))
}

impl State {
    fn initial_state(num_players: usize) -> Self {
        State {
            current_marble: 0,
            current_marble_idx: 0,
            next_marble: 1,
            num_players,
            circle: vec![0],
            points: vec![0; num_players],
        }
    }

    fn place_marble(&mut self) {
        if self.next_marble % 23 == 0 {
            let current_player = (self.next_marble - 1) % self.num_players;
            let remove_marble_idx =
                (self.current_marble_idx as i32 - 7).rem_euclid(self.circle.len() as i32) as usize;
            let removed_marble_value = self.circle.remove(remove_marble_idx);
            self.points[current_player] += self.next_marble + removed_marble_value;
            self.current_marble_idx = remove_marble_idx % self.circle.len();
            self.current_marble = self.circle[remove_marble_idx % self.circle.len()];
        } else {
            let next_marble_idx = (self.current_marble_idx + 2) % self.circle.len();
            self.circle.insert(next_marble_idx, self.next_marble);
            self.current_marble = self.next_marble;
            self.current_marble_idx = next_marble_idx;
        }
        self.next_marble += 1;
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "current marble: {}, current_idx: {}, circle: {:?}",
            self.current_marble, self.current_marble_idx, self.circle
        )
    }
}
