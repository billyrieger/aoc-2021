#![feature(array_windows)]
use aoc::prelude::*;

use std::collections::HashMap;

type Pair = [char; 2];

const INPUT: &'static str = include_str!("../../input/14.txt");

fn main() -> Result<()> {
    println!("{}", solve(40).unwrap());
    Ok(())
}

fn solve(n: usize) -> Option<u64> {
    let mut template: Vec<char> = INPUT.lines().next()?.chars().collect();
    template.insert(0, '_');
    template.push('_');
    let rules = INPUT
        .lines()
        .skip(2)
        .map(|line| {
            let (pair, insertion) = line.split_once(" -> ")?;
            let pair = [pair.chars().nth(0)?, pair.chars().nth(1)?];
            let insertion = insertion.chars().nth(0)?;
            Some((pair, [[pair[0], insertion], [insertion, pair[1]]]))
        })
        .collect::<Option<HashMap<Pair, [Pair; 2]>>>()?;
    let mut state = HashMap::new();
    for pair in template.array_windows::<2>() {
        *state.entry(pair).or_insert(0) += 1;
    }
    println!("{:?}", state);

    for _ in 0..n {
        let mut new_state = HashMap::new();
        for (pair, count) in state {
            if let Some([new0, new1]) = rules.get(pair) {
                *new_state.entry(new0).or_insert(0) += count;
                *new_state.entry(new1).or_insert(0) += count;
            } else {
                *new_state.entry(pair).or_insert(0) += count;
            }
        }
        state = new_state;
    }

    let mut elements = HashMap::new();
    for ([a, b], count) in state {
        *elements.entry(a).or_insert(0) += count;
        *elements.entry(b).or_insert(0) += count;
    }
    elements.remove(&'_');
    let max = elements.values().max()? / 2;
    let min = elements.values().min()? / 2;
    dbg!(&elements);

    Some(max - min)
}
