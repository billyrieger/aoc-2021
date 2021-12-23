use aoc::prelude::*;
use itertools::iproduct;
use std::collections::HashMap;

const INPUT: &str = include_str!("../../input/20.txt");

#[derive(Clone)]
struct InfiniteGrid {
    grid: HashMap<(i32, i32), bool>,
    boundary: bool,
}

impl InfiniteGrid {
    fn get(&self, point: (i32, i32)) -> bool {
        self.grid.get(&point).copied().unwrap_or(self.boundary)
    }

    fn set(&mut self, point: (i32, i32), value: bool) {
        *self.grid.entry(point).or_insert(false) = value;
    }

    fn update(&self, rules: &[bool]) -> Option<Self> {
        let mut new_grid = Self {
            grid: HashMap::new(),
            boundary: if self.boundary { rules[511] } else { rules[0] },
        };
        let (min_row, max_row) = self.grid.keys().map(|(r, _)| r).minmax().into_option()?;
        let (min_col, max_col) = self.grid.keys().map(|(_, c)| c).minmax().into_option()?;
        for (row, col) in iproduct!((min_row - 1)..=(max_row + 1), (min_col - 1)..=(max_col + 1)) {
            let value = iproduct!(-1..=1, -1..=1)
                .map(|(dr, dc)| self.get((row + dr, col + dc)))
                .fold(0, |sum, digit| 2 * sum + if digit { 1 } else { 0 });
            new_grid.set((row, col), rules[value]);
        }
        Some(new_grid)
    }

    fn count_true(&self) -> usize {
        self.grid.values().filter(|&&x| x).count()
    }
}

fn main() {
    solve().unwrap();
}

fn solve() -> Option<()> {
    let rules: Vec<bool> = INPUT.lines().next()?.chars().map(|c| c == '#').collect();
    // There should be exactly 512 = 2^9 rules.
    assert_eq!(rules.len(), 512);

    let grid: HashMap<(i32, i32), bool> = INPUT
        .lines()
        .skip(2)
        .zip(0..)
        .flat_map(|(line, row)| {
            line.chars()
                .zip(0..)
                .map(move |(c, col)| ((row, col), c == '#'))
        })
        .collect();

    let mut grid = InfiniteGrid {
        grid,
        boundary: false,
    };

    for _ in 0..2 {
        grid = grid.update(&rules)?;
    }
    println!("1: {}", grid.count_true());

    for _ in 2..50 {
        grid = grid.update(&rules)?;
    }
    println!("2: {}", grid.count_true());

    Some(())
}
