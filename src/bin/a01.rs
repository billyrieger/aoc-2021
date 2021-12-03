#![feature(array_windows)]

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file = std::fs::read_to_string("input/01")?;
    let lines: Vec<i32> = file
        .lines()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;

    let count_increasing = |lines: &[i32]| lines.array_windows().filter(|[a, b]| a < b).count();
    println!("1: {:?}", count_increasing(&lines));

    let summed = lines
        .array_windows()
        .map(|[a, b, c]| a + b + c)
        .collect::<Vec<_>>();
    println!("2: {:?}", count_increasing(&summed));

    Ok(())
}
