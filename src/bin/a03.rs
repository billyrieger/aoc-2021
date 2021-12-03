use itertools::Itertools;

fn part1(lines: &[Vec<char>]) {
    let line_len = lines[0].len();
    let mut gamma = vec!['?'; line_len];
    let mut epsilon = vec!['?'; line_len];
    for i in 0..line_len {
        let tally = lines.iter().map(|line| line[i]).counts();
        gamma[i] = if tally[&'0'] > tally[&'1'] { '0' } else { '1' };
        epsilon[i] = if gamma[i] == '0' { '1' } else { '0' };
    }
    let gamma = gamma.into_iter().collect::<String>();
    let gamma = i32::from_str_radix(&gamma, 2).unwrap();
    let epsilon = epsilon.into_iter().collect::<String>();
    let epsilon = i32::from_str_radix(&epsilon, 2).unwrap();
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
    let oxygen = reduce(Vec::from(lines), '1', '0');
    let co2 = reduce(Vec::from(lines), '0', '1');
    let oxygen = i32::from_str_radix(&oxygen, 2).unwrap();
    let co2 = i32::from_str_radix(&co2, 2).unwrap();
    println!("2: {}", oxygen * co2);
}

fn main() {
    let file = std::fs::read_to_string("input/03").unwrap();
    let lines: Vec<Vec<char>> = file.lines().map(|line| line.chars().collect()).collect();
    part1(&lines);
    part2(&lines);
}
