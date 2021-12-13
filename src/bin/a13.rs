use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::read_to_string("input/13.txt")?;
    let mut lines: Vec<&str> = file.lines().collect();
    let _instructions = lines.split_off(lines.iter().position(|&line| line.is_empty()).unwrap() + 1);
    lines.pop();

    let mut points: HashSet<(i32, i32)> = lines
        .iter()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

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
        for x in 0..100 {
            print!("{}", if points.contains(&(x, y)) { "O" } else { " " });
        }
        println!();
    }

    Ok(())
}

fn fold_x(points: &HashSet<(i32, i32)>, fold: i32) -> HashSet<(i32, i32)> {
    points
        .into_iter()
        .map(|&(x, y)| if x < fold { (x, y) } else { (2 * fold - x, y) })
        .collect()
}

fn fold_y(points: &HashSet<(i32, i32)>, fold: i32) -> HashSet<(i32, i32)> {
    points
        .into_iter()
        .map(|&(x, y)| if y < fold { (x, y) } else { (x, 2 * fold - y) })
        .collect()
}
