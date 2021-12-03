use anyhow::{anyhow, bail, Result};

fn parse_instruction(line: &str) -> Result<(&str, i32)> {
    let (dir, num) = line.split_once(' ').ok_or(anyhow!("bad line"))?;
    let num = num.parse()?;
    Ok((dir, num))
}

fn part1(lines: &[(&str, i32)]) -> Result<()> {
    let mut pos = 0;
    let mut depth = 0;
    for &(dir, num) in lines {
        match dir {
            "forward" => pos += num,
            "down" => depth += num,
            "up" => depth -= num,
            _ => bail!("bad direction"),
        }
    }
    println!("1: {:?}", pos * depth);
    Ok(())
}

fn part2(lines: &[(&str, i32)]) -> Result<()> {
    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;
    for &(dir, num) in lines {
        match dir {
            "forward" => {
                pos += num;
                depth += aim * num;
            }
            "down" => aim += num,
            "up" => aim -= num,
            _ => bail!("bad direction"),
        }
    }
    println!("2: {:?}", pos * depth);
    Ok(())
}

fn main() -> Result<()> {
    let file = std::fs::read_to_string("input/02")?;
    let lines: Vec<_> = file.lines().flat_map(parse_instruction).collect();
    part1(&lines)?;
    part2(&lines)?;
    Ok(())
}
