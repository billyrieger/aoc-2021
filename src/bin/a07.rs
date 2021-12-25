use aoc::prelude::*;

fn min_cost(positions: &[i32], fuel: impl Fn(i32) -> i32) -> Option<i32> {
    let (&min_pos, &max_pos) = positions.iter().minmax().into_option()?;
    (min_pos..=max_pos)
        .map(|i| positions.iter().map(|pos| fuel((pos - i).abs())).sum())
        .min()
}

fn main() -> Result<()> {
    let file = std::fs::read_to_string("input/07.txt")?;
    let input: Vec<i32> = file.trim().split(',').map(str::parse).try_collect()?;

    let fuel = |delta: i32| delta;
    println!("1: {}", min_cost(&input, fuel).ok_or(AocError::Logic)?);

    let fuel = |delta: i32| delta * (delta + 1) / 2;
    println!("2: {}", min_cost(&input, fuel).ok_or(AocError::Logic)?);
    Ok(())
}
