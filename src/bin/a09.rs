use aoc::prelude::*;
use std::collections::VecDeque;

struct Grid(Vec<Vec<u32>>);

impl Grid {
    fn get(&self, r: i32, c: i32) -> Option<u32> {
        if 0 <= r && r < self.0.len() as i32 && 0 <= c && c < self.0[0].len() as i32 {
            Some(self.0[r as usize][c as usize])
        } else {
            None
        }
    }

    fn neighbors(&self, (r, c): (i32, i32)) -> [(i32, i32); 4] {
        [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)]
    }

    fn floodfill(&self, start: (i32, i32)) -> usize {
        let mut seen = HashSet::<(i32, i32)>::default();
        let mut frontier = VecDeque::from([start]);
        while let Some(coords) = frontier.pop_front() {
            if matches!(self.get(coords.0, coords.1), None | Some(9)) {
                continue;
            }
            if !seen.contains(&coords) {
                seen.insert(coords);
                frontier.extend(self.neighbors(coords));
            }
        }
        seen.len()
    }
}

fn main() {
    let file = std::fs::read_to_string("input/09.txt").unwrap();
    let grid: Grid = Grid(
        file.lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect(),
    );
    let mut tot = 0;
    let mut low_points = vec![];
    for r in 0..grid.0.len() as i32 {
        for c in 0..grid.0[0].len() as i32 {
            let min_surrounding = grid
                .neighbors((r, c))
                .into_iter()
                .filter_map(|(r, c)| grid.get(r, c))
                .min()
                .unwrap();
            let val = grid.get(r, c).unwrap();
            if val < min_surrounding {
                tot += val + 1;
                low_points.push((r, c));
            }
        }
    }
    println!("1: {}", tot);
    let mut basin_sizes: Vec<usize> = low_points
        .into_iter()
        .map(|coords| grid.floodfill(coords))
        .collect();
    basin_sizes.sort();
    println!(
        "2: {}",
        basin_sizes.into_iter().rev().take(3).product::<usize>()
    );
}
