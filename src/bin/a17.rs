use aoc::prelude::*;
use itertools::iproduct;
use regex::Regex;

const INPUT: &str = include_str!("../../input/17.txt");

#[derive(Debug)]
struct Region {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl Region {
    fn simulate(&self, mut x_vel: i32, mut y_vel: i32) -> bool {
        let (mut x_pos, mut y_pos) = (0, 0);
        while x_pos <= self.x_max && self.y_min <= y_pos {
            if self.x_min <= x_pos && y_pos <= self.y_max {
                return true;
            }
            x_pos += x_vel;
            y_pos += y_vel;
            x_vel = (x_vel - 1).max(0);
            y_vel -= 1;
        }
        false
    }
}

fn main() -> Result<()> {
    solve().unwrap();
    Ok(())
}

fn solve() -> Option<()> {
    let re = Regex::new(r"^target area: x=(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-?\d+)$").ok()?;
    let captures = re.captures(INPUT.trim())?;
    let region = Region {
        x_min: captures.get(1)?.as_str().parse().ok()?,
        x_max: captures.get(2)?.as_str().parse().ok()?,
        y_min: captures.get(3)?.as_str().parse().ok()?,
        y_max: captures.get(4)?.as_str().parse().ok()?,
    };
    println!("1: {}", (-region.y_min) * (-region.y_min + 1) / 2);
    // solution to x*(x+1)/2 >= x_min
    let x_vel_min = ((-1.0 + (8.0 * (region.x_min as f32) + 1.0).sqrt()) / 2.0).ceil() as i32;
    let x_vel_max = region.x_max;
    let y_vel_min = region.y_min;
    let y_vel_max = -region.y_min - 1;
    let total = iproduct!(x_vel_min..=x_vel_max, y_vel_min..=y_vel_max)
        .filter(|&(x_vel, y_vel)| region.simulate(x_vel, y_vel))
        .count();
    println!("2: {}", total);

    Some(())
}
