use aoc::*;

// `state[i]` is the number of lanternfish with an internal timer of `i`.
type State = [i64; 9];

fn main() -> Result<()> {
    let file = std::fs::read_to_string("input/06.txt")?;
    let input: Vec<usize> = file.trim().split(',').map(str::parse).try_collect()?;
    let mut initial_state = State::default();
    // `state[i]` is the number of lanternfish with an internal timer of `i`.
    input.iter().for_each(|timer| initial_state[*timer] += 1);

    let step_once = |prev: &State| -> Option<State> {
        let mut next = State::default();
        // decrease the timer of each lanternfish
        (1..prev.len()).for_each(|i| next[i - 1] = prev[i]);
        // reset completed timers
        next[6] += prev[0];
        // create new lanternfish
        next[8] += prev[0];
        Some(next)
    };

    let total = |steps: usize| -> i64 {
        std::iter::successors(Some(initial_state), step_once)
            .nth(steps)
            .unwrap()
            .iter()
            .sum()
    };

    println!("1: {}", total(80));
    println!("2: {}", total(256));
    Ok(())
}
