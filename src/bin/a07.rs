use itertools::Itertools;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::read_to_string("input/07.txt")?;
    let positions: Vec<i32> = file.trim().split(',').map(str::parse).try_collect()?;
    solve(&positions);
    Ok(())
}

fn solve(positions: &[i32]) -> Option<()> {
    let min_pos = *positions.iter().min()?;
    let max_pos = *positions.iter().max()?;
    let fuel_used = |cost: &dyn Fn(i32) -> i32| -> Option<i32> {
        (min_pos..=max_pos)
            .map(|i| positions.iter().map(|pos| cost(pos - i)).sum())
            .min()
    };
    let cost1 = |delta: i32| delta.abs();
    let cost2 = |delta: i32| delta.abs() * (delta.abs() + 1) / 2;
    println!("1: {}", fuel_used(&cost1)?);
    println!("2: {}", fuel_used(&cost2)?);
    Some(())
}
