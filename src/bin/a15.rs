use aoc::prelude::*;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::ops::{Index, IndexMut};

const INPUT: &str = include_str!("../../input/15.txt");

struct Grid<T> {
    rows: i32,
    cols: i32,
    grid: Vec<T>,
}

impl<T> Grid<T> {
    fn inbounds(&self, (r, c): Point) -> bool {
        0 <= r && r < self.rows && 0 <= c && c < self.cols
    }

    fn orthogonal_neighbors(&self, (r, c): Point) -> Vec<Point> {
        [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)]
            .into_iter()
            .filter(|&point| self.inbounds(point))
            .collect()
    }
}

type Point = (i32, i32);

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, (r, c): Point) -> &T {
        let i = r * self.cols as i32 + c;
        &self.grid[i as usize]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, (r, c): Point) -> &mut T {
        let i = r * self.cols as i32 + c;
        &mut self.grid[i as usize]
    }
}

fn main() -> Result<()> {
    solve().unwrap();
    Ok(())
}

fn solve() -> Option<()> {
    let rows = INPUT.lines().count() as i32;
    let cols = INPUT.lines().next()?.chars().count() as i32;
    let grid = INPUT
        .lines()
        .flat_map(|line| line.chars())
        .map(|c| Some(c.to_digit(10)? as i32))
        .collect::<Option<Vec<_>>>()?;
    let grid = Grid { rows, cols, grid };
    println!("1: {}", dijkstra(&grid)?);

    let big_rows = rows * 5;
    let big_cols = cols * 5;
    let mut big_grid = Grid {
        rows: big_rows,
        cols: big_cols,
        grid: vec![i32::MAX; (big_rows * big_cols) as usize],
    };
    for big_r in 0..5 {
        for big_c in 0..5 {
            let delta = big_r + big_c;
            for r in 0..rows {
                for c in 0..cols {
                    let orig = grid[(r, c)];
                    let new = (orig + delta - 1) % 9 + 1;
                    big_grid[(big_r * rows + r, big_c * cols + c)] = new;
                }
            }
        }
    }
    println!("2: {}", dijkstra(&big_grid)?);
    Some(())
}

fn dijkstra(grid: &Grid<i32>) -> Option<i32> {
    let mut risks = Grid {
        rows: grid.rows,
        cols: grid.cols,
        grid: vec![i32::MAX; (grid.rows * grid.cols) as usize],
    };
    let start = (0, 0);
    let end = (grid.rows - 1, grid.cols - 1);
    risks[start] = 0;
    let mut frontier = BinaryHeap::new();
    frontier.push((Reverse(0), start));

    while let Some((Reverse(risk), point)) = frontier.pop() {
        if risk > risks[point] {
            continue;
        }
        for neighbor in risks.orthogonal_neighbors(point) {
            let new_risk = risk + grid[neighbor];
            if new_risk < risks[neighbor] {
                risks[neighbor] = new_risk;
                frontier.push((Reverse(new_risk), neighbor));
            }
        }
        if point == end {
            break;
        }
    }

    Some(risks[end])
}
