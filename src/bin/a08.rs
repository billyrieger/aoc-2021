use itertools::Itertools;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::read_to_string("input/08.txt")?;
    println!(
        "1: {}",
        file.lines()
            .map(|line| {
                line.split_once('|')
                    .unwrap()
                    .1
                    .trim()
                    .split_whitespace()
                    .map(|word| word.len())
                    .filter(|len| [2, 3, 4, 7].contains(len))
                    .count()
            })
            .sum::<usize>()
    );
    let mut total = 0;
    for line in file.lines() {
        let (first, second) = line.split_once('|').unwrap();
        let digits: Vec<_> = first
            .split_whitespace()
            .map(|word| word.as_bytes())
            .collect();
        let num: Vec<_> = second
            .split_whitespace()
            .map(|word| word.as_bytes())
            .collect();
        let tally = digits.iter().flat_map(|&word| word).counts();
        let b = tally.iter().find(|(_, &v)| v == 6).unwrap().0;
        let e = tally.iter().find(|(_, &v)| v == 4).unwrap().0;
        let f = tally.iter().find(|(_, &v)| v == 9).unwrap().0;
        let one = digits.into_iter().find(|word| word.len() == 2).unwrap();
        let c = one.iter().filter(|segment| segment != f).next().unwrap();
        let parse_digit = |segments: &[u8]| -> i32 {
            match segments.len() {
                2 => return 1,
                4 => return 4,
                3 => return 7,
                7 => return 8,
                _ => {}
            }
            match (
                segments.contains(&b),
                segments.contains(&e),
                segments.contains(&f),
            ) {
                (false, true, false) => return 2,
                (false, false, true) => return 3,
                (true, false, true) if segments.len() == 5 => return 5,
                (true, false, true) if segments.len() == 6 => return 9,
                _ => {}
            }
            if segments.contains(c) {
                0
            } else {
                6
            }
        };
        let mut sum = 0;
        for digit in num {
            sum = sum * 10 + parse_digit(digit);
        }
        total += sum;
    }
    println!("2: {}", total);
    Ok(())
}
