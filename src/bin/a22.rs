use aoc::prelude::*;

type IVec3 = na::Vector3<i32>;

fn main() -> Result<()> {
    let file = std::fs::read_to_string("./input/22.txt")?;
    let mut points: HashMap<IVec3, bool> = HashMap::default();
    let mut regions = 0;
    for line in file.lines() {
        let (turn_on, rest) = line.split_once(' ').ok_or(AocError::Parse)?;
        let turn_on = match turn_on {
            "on" => true,
            "off" => false,
            _ => Err(AocError::Parse)?,
        };
        let delimiters = &['x', 'y', 'z', '.', ',', '='];
        let ranges: Vec<i32> = rest
            .split(delimiters)
            .filter_map(|s| if s.is_empty() { None } else { Some(s.parse()) })
            .try_collect()?;
        let [x0, x1, y0, y1, z0, z1]: [i32; 6] = ranges.try_into().map_err(|_| AocError::Parse)?;
        for (x, y, z) in iproduct!(x0..=x1, y0..=y1, z0..=z1) {
            *points.entry([x, y, z].into()).or_insert(false) = turn_on;
        }
        regions += 1;
        if regions >= 20 {
            break;
        }
    }
    println!("1: {}", points.values().filter(|&&on| on).count());
    Ok(())
}
