use aoc::*;

const OPENING: [char; 4] = ['(', '[', '{', '<'];
const CLOSING: [char; 4] = [')', ']', '}', '>'];

fn matches(opening: char, closing: char) -> bool {
    OPENING.iter().position(|&c| c == opening) == CLOSING.iter().position(|&c| c == closing)
}

fn score(close: char) -> i64 {
    match close {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!(),
    }
}

fn score_opening(open: char) -> i64 {
    match open {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!(),
    }
}

fn main() -> Result<()> {
    let file = std::fs::read_to_string("input/10.txt")?;
    let mut corrupted = 0;
    let mut scores = vec![];
    'outer: for line in file.lines() {
        let mut stack = vec![];
        for c in line.chars() {
            if OPENING.contains(&c) {
                stack.push(c);
            } else {
                if let Some(prev) = stack.pop() {
                    if !matches(prev, c) {
                        corrupted += score(c);
                        continue 'outer;
                    }
                } else {
                    corrupted += score(c);
                    continue 'outer;
                }
            }
        }
        scores.push(
            stack
                .into_iter()
                .rfold(0, |score, c| 5 * score + score_opening(c)),
        );
    }
    scores.sort();
    println!("1: {}", corrupted);
    println!("2: {}", scores[scores.len() / 2]);
    Ok(())
}
