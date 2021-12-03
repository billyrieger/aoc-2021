use itertools::Itertools;

fn main() {
    let file = std::fs::read_to_string("input/03").unwrap();
    let lines: Vec<Vec<char>> = file.lines().map(|line| line.chars().collect()).collect();
    part1(&lines);
    part2(&lines);
}

fn part1(lines: &[Vec<char>]) {
    let line_len = lines[0].len();

    let count = |lines: &[Vec<char>], a: char, b: char| -> String {
        (0..line_len)
            .map(|i| {
                let tally = lines.iter().map(|line| line[i]).counts();
                let keep = if tally[&'1'] >= tally[&'0'] { a } else { b };
                keep
            })
            .collect()
    };

    let gamma = i32::from_str_radix(&count(lines, '1', '0'), 2).unwrap();
    let epsilon = i32::from_str_radix(&count(lines, '0', '1'), 2).unwrap();
    println!("1: {}", gamma * epsilon);
}

fn part2(lines: &[Vec<char>]) {
    let line_len = lines[0].len();

    let reduce = |mut lines: Vec<Vec<char>>, a: char, b: char| -> String {
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

    let oxygen = i32::from_str_radix(&reduce(Vec::from(lines), '1', '0'), 2).unwrap();
    let co2 = i32::from_str_radix(&reduce(Vec::from(lines), '0', '1'), 2).unwrap();
    println!("2: {}", oxygen * co2);
}
