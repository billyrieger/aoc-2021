use itertools::Itertools;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let file = std::fs::read_to_string("input/03.txt")?;
    let lines: Vec<Vec<char>> = file.lines().map(|line| line.chars().collect()).collect();
    let line_len = lines[0].len();

    let count = |a: char, b: char| -> String {
        (0..line_len)
            .map(|i| {
                let tally = lines.iter().map(|line| line[i]).counts();
                let keep = if tally[&'1'] >= tally[&'0'] { a } else { b };
                keep
            })
            .collect()
    };

    let gamma = i32::from_str_radix(&count('1', '0'), 2)?;
    let epsilon = i32::from_str_radix(&count('0', '1'), 2)?;
    println!("1: {}", gamma * epsilon);

    let reduce = |a: char, b: char| -> String {
        let mut lines = lines.clone();
        for i in 0..line_len {
            if lines.len() == 1 {
                break;
            }
            let tally = lines.iter().map(|line| line[i]).counts();
            let keep = if tally[&'1'] >= tally[&'0'] { a } else { b };
            lines.retain(|line| line[i] == keep);
        }
        lines[0].iter().collect()
    };

    let oxygen = i32::from_str_radix(&reduce('1', '0'), 2)?;
    let co2 = i32::from_str_radix(&reduce('0', '1'), 2)?;
    println!("2: {}", oxygen * co2);

    Ok(())
}
