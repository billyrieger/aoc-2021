// `state[i]` is the number of lanternfish with an internal timer of `i`.
type State = [i64; 9];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::read_to_string("input/06.txt")?;

    let mut initial_state = State::default();
    for timer in file.trim().split(',').map(str::parse::<usize>) {
        initial_state[timer?] += 1;
    }

    let step_once = |prev: &State| -> State {
        let mut next = State::default();
        // decrease the timer of each lanternfish
        for i in 1..prev.len() {
            next[i - 1] = prev[i];
        }
        // reset completed timers
        next[6] += prev[0];
        // create new lanternfish
        next[8] += prev[0];
        next
    };

    let total = |steps: usize| -> i64 {
        std::iter::successors(Some(initial_state), |state| Some(step_once(state)))
            .nth(steps)
            .unwrap()
            .iter()
            .sum()
    };

    println!("1: {}", total(80));
    println!("2: {}", total(256));
    Ok(())
}
