use aoc::prelude::*;
use aoc::{parse_point, Point2D};

fn main() -> Result<()> {
    let file = std::fs::read_to_string("input/13.txt")?;
    let mut lines: Vec<&str> = file.lines().collect();
    let _first_blank = lines.iter().position(|&line| line.is_empty()).unwrap();
    let _instructions =
        lines.split_off(lines.iter().position(|&line| line.is_empty()).unwrap() + 1);
    lines.pop();

    let mut points: HashSet<Point2D> = lines.into_iter().map(parse_point).try_collect()?;

    let fold_x = |points: &HashSet<Point2D>, fold: i32| -> HashSet<Point2D> {
        points
            .iter()
            .map(|&(x, y)| if x < fold { (x, y) } else { (2 * fold - x, y) })
            .collect()
    };

    points = fold_x(&points, 655);
    points = fold_y(&points, 447);
    points = fold_x(&points, 327);
    points = fold_y(&points, 223);
    points = fold_x(&points, 163);
    points = fold_y(&points, 111);
    points = fold_x(&points, 81);
    points = fold_y(&points, 55);
    points = fold_x(&points, 40);
    points = fold_y(&points, 27);
    points = fold_y(&points, 13);
    points = fold_y(&points, 6);

    for y in 0..6 {
        for x in 0..40 {
            print!("{}", if points.contains(&(x, y)) { "O" } else { " " });
        }
        println!();
    }

    Ok(())
}

fn fold_y(points: &HashSet<Point2D>, fold: i32) -> HashSet<Point2D> {
    points
        .iter()
        .map(|&(x, y)| if y < fold { (x, y) } else { (x, 2 * fold - y) })
        .collect()
}
