#![feature(array_windows)]
use aoc::*;

fn main() -> Result<()> {
    let file = std::fs::read_to_string("input/01.txt")?;
    let lines: Vec<i32> = file.lines().map(str::parse).try_collect()?;
    let summed_lines: Vec<i32> = lines.array_windows().map(|[x, y, z]| x + y + z).collect();
    let count_increasing = |list: &[i32]| list.array_windows().filter(|[a, b]| a < b).count();
    println!("1: {}", count_increasing(&lines));
    println!("2: {}", count_increasing(&summed_lines));
    Ok(())
}
