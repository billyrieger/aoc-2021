use aoc::prelude::*;

fn main() -> Result<()> {
    let file = std::fs::read_to_string("input/02.txt")?;
    let lines: Vec<_> = file.lines().map(parse_line).collect();

    let (mut pos, mut depth) = (0, 0);
    for &(dir, num) in &lines {
        match dir {
            "forward" => pos += num,
            "down" => depth += num,
            "up" => depth -= num,
            _ => {}
        }
    }
    println!("1: {}", pos * depth);

    let (mut pos, mut depth, mut aim) = (0, 0, 0);
    for &(dir, num) in &lines {
        match dir {
            "forward" => {
                pos += num;
                depth += aim * num;
            }
            "down" => aim += num,
            "up" => aim -= num,
            _ => {}
        }
    }
    println!("2: {}", pos * depth);

    Ok(())
}

fn parse_line(line: &str) -> (&str, i32) {
    let (dir, num) = line.split_once(' ').unwrap();
    let num = num.parse().unwrap();
    (dir, num)
}
