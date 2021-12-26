use aoc::*;

fn main() -> Result<()> {
    let file = std::fs::read_to_string("input/13.txt")?;
    let first_blank = file
        .lines()
        .position(str::is_empty)
        .ok_or(AocError::Parse)?;
    let mut points: HashSet<IVec2> = file
        .lines()
        .take(first_blank)
        .map(aoc::parse_point2)
        .try_collect()?;

    for (i, line) in file.lines().skip(first_blank + 1).enumerate() {
        let (line, num) = line.split_once('=').ok_or(AocError::Parse)?;
        let axis = num.parse()?;
        points = if line.ends_with('x') {
            fold_x(&points, axis)
        } else {
            fold_y(&points, axis)
        };
        if i == 0 {
            println!("1: {}", points.len());
        }
    }

    let (x_min, x_max) = points
        .iter()
        .map(|point| point.x)
        .minmax()
        .into_option()
        .ok_or(AocError::Parse)?;
    let (y_min, y_max) = points
        .iter()
        .map(|point| point.y)
        .minmax()
        .into_option()
        .ok_or(AocError::Parse)?;
    println!("2:");
    for y in y_min..=y_max {
        let row: String = (x_min..=x_max)
            .map(|x| {
                if points.contains(&[x, y].into()) {
                    "#"
                } else {
                    " "
                }
            })
            .collect();
        println!("{}", row);
    }

    Ok(())
}

fn fold_x(points: &HashSet<IVec2>, fold: i32) -> HashSet<IVec2> {
    points
        .iter()
        .map(|&p| {
            let pivot = IVec2::from([fold, p.y]);
            let delta = IVec2::from([p.x - fold, 0]);
            pivot - delta.abs()
        })
        .collect()
}

fn fold_y(points: &HashSet<IVec2>, fold: i32) -> HashSet<IVec2> {
    points
        .iter()
        .map(|&point| {
            let pivot: IVec2 = [point.x, fold].into();
            let delta: IVec2 = [0, point.y - fold].into();
            pivot - delta.abs()
        })
        .collect()
}
