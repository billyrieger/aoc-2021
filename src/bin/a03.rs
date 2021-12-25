use aoc::prelude::*;

fn main() -> Result<()> {
    let file = std::fs::read_to_string("input/03.txt")?;
    let input: Vec<BitVec> = file
        .lines()
        .map(|line| line.chars().map(|c| c == '1').collect())
        .collect();
    solve(&input).ok_or(AocError::Logic)?;
    Ok(())
}

fn solve(input: &[BitVec]) -> Option<()> {
    let gamma = counts(&input, true)?;
    let epsilon = counts(&input, false)?;
    println!("1: {}", gamma * epsilon);

    let oxygen = reduces(&input, false)?;
    let co2 = reduces(&input, true)?;
    println!("2: {}", oxygen * co2);
    Some(())
}

fn counts(input: &[BitVec], invert: bool) -> Option<i32> {
    let mut most_common = BitVec::new();
    for i in 0..input.first()?.len() {
        let tally = input.iter().map(|bits| bits[i]).counts();
        most_common.push((tally[&true] >= tally[&false]) ^ invert);
    }
    Some(to_binary(&most_common))
}

fn reduces(input: &[BitVec], invert: bool) -> Option<i32> {
    let mut input = Vec::from(input);
    for i in 0..input.first()?.len() {
        if input.len() == 1 {
            break;
        }
        let tally = input.iter().map(|line| line[i]).counts();
        let keep = (tally[&true] >= tally[&false]) ^ invert;
        input.retain(|line| line[i] == keep);
    }
    Some(to_binary(input.first()?))
}

fn to_binary(digits: &BitSlice) -> i32 {
    digits
        .iter()
        .fold(0, |total, bit| 2 * total + if *bit { 1 } else { 0 })
}
