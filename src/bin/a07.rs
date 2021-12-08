use itertools::Itertools;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::read_to_string("input/07.txt")?;
    let positions: Vec<i32> = file.trim().split(',').map(str::parse).try_collect()?;
    let &min = positions.iter().min().unwrap();
    let &max = positions.iter().max().unwrap();
    let fuel1 = |delta: i32| delta.abs();
    let fuel2 = |delta: i32| delta.abs() * (delta.abs() + 1) / 2;
    let fuel_used: i32 = (min..=max)
        .map(|i| positions.iter().map(|pos| fuel1(pos - i)).sum())
        .min()
        .unwrap();
    println!("1: {}", fuel_used);
    let fuel_used: i32 = (min..=max)
        .map(|i| positions.iter().map(|pos| fuel2(pos - i)).sum())
        .min()
        .unwrap();
    println!("2: {}", fuel_used);
    Ok(())
}
