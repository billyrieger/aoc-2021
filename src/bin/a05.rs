use aoc::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn min_max(a: i32, b: i32) -> (i32, i32) {
    (a.min(b), a.max(b))
}

fn main() -> Result<()> {
    let parse_line = |line: &str| -> Option<(Point, Point)> {
        let (a, b) = line.split_once(" -> ")?;
        let (x0, y0) = a.split_once(',')?;
        let (x1, y1) = b.split_once(',')?;
        let p0 = Point::new(x0.parse().ok()?, y0.parse().ok()?);
        let p1 = Point::new(x1.parse().ok()?, y1.parse().ok()?);
        Some((p0, p1))
    };
    let file = std::fs::read_to_string("input/05.txt")?;
    let endpoints: Vec<(Point, Point)> = file.lines().filter_map(parse_line).collect();
    let mut covered = HashMap::<Point, u32>::new();

    for &(p0, p1) in &endpoints {
        let (x_min, x_max) = min_max(p0.x, p1.x);
        let (y_min, y_max) = min_max(p0.y, p1.y);
        if p0.x == p1.x {
            for y in y_min..=y_max {
                *covered.entry(Point::new(p0.x, y)).or_insert(0) += 1
            }
        } else if p0.y == p1.y {
            for x in x_min..=x_max {
                *covered.entry(Point::new(x, p0.y)).or_insert(0) += 1
            }
        }
    }
    println!("1: {}", covered.values().filter(|&&n| n >= 2).count());

    for &(p0, p1) in &endpoints {
        let (x_min, x_max) = min_max(p0.x, p1.x);
        let (y_min, y_max) = min_max(p0.y, p1.y);
        let (delta_x, delta_y) = (p0.x - p1.x, p0.y - p1.y);
        if delta_x == delta_y {
            for (x, y) in (x_min..=x_max).zip(y_min..=y_max) {
                *covered.entry(Point::new(x, y)).or_insert(0) += 1
            }
        } else if delta_x == -delta_y {
            for (x, y) in (x_min..=x_max).zip((y_min..=y_max).rev()) {
                *covered.entry(Point::new(x, y)).or_insert(0) += 1
            }
        }
    }
    println!("2: {}", covered.values().filter(|&&n| n >= 2).count());

    Ok(())
}
