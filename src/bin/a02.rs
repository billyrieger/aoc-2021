use aoc::prelude::*;

type Command<'a> = (&'a str, i32);

fn main() -> Result<()> {
    let file = std::fs::read_to_string("input/02.txt")?;
    let commands: Vec<Command> = file
        .lines()
        .map(|line| {
            let (dir, num) = line.split_once(' ').ok_or(AocError::Parse)?;
            Ok((dir, num.parse()?))
        })
        .collect::<Result<_>>()?;
    part1(&commands);
    part2(&commands);
    Ok(())
}

fn part1(commands: &[Command]) {
    let (mut pos, mut depth) = (0, 0);
    for &(direction, num) in commands {
        match direction {
            "forward" => pos += num,
            "down" => depth += num,
            "up" => depth -= num,
            _ => {}
        }
    }
    println!("1: {}", pos * depth);
}

fn part2(commands: &[Command]) {
    let (mut pos, mut depth, mut aim) = (0, 0, 0);
    for &(dir, num) in commands {
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
}
