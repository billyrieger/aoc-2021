use aoc::*;

fn min_max(a: i32, b: i32) -> (i32, i32) {
    (a.min(b), a.max(b))
}

fn main() -> Result<()> {
    let file = std::fs::read_to_string("input/05.txt")?;
    let endpoints: Vec<(IVec2, IVec2)> = file
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(" -> ").ok_or(AocError::Parse)?;
            Ok((aoc::parse_point2(a)?, aoc::parse_point2(b)?))
        })
        .collect::<Result<_>>()?;
    let mut covered = HashMap::<IVec2, u32>::default();

    for &(p0, p1) in &endpoints {
        let (x_min, x_max) = min_max(p0.x, p1.x);
        let (y_min, y_max) = min_max(p0.y, p1.y);
        if p0.x == p1.x {
            for y in y_min..=y_max {
                *covered.entry([p0.x, y].into()).or_insert(0) += 1
            }
        } else if p0.y == p1.y {
            for x in x_min..=x_max {
                *covered.entry([x, p0.y].into()).or_insert(0) += 1
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
                *covered.entry([x, y].into()).or_insert(0) += 1
            }
        } else if delta_x == -delta_y {
            for (x, y) in (x_min..=x_max).zip((y_min..=y_max).rev()) {
                *covered.entry([x, y].into()).or_insert(0) += 1
            }
        }
    }
    println!("2: {}", covered.values().filter(|&&n| n >= 2).count());

    Ok(())
}
