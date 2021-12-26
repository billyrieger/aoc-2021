use aoc::*;
use itertools::multizip;

const LEN: usize = 10;

#[derive(Clone, Debug)]
struct Grid([u32; LEN * LEN]);

#[derive(Clone, Copy, Debug)]
struct Coords(i32, i32);

impl Grid {
    fn inbounds(&self, coords: Coords) -> bool {
        0 <= coords.0 && coords.0 < 10 && 0 <= coords.1 && coords.1 < 10
    }

    fn get_mut(&mut self, coords: Coords) -> &mut u32 {
        assert!(self.inbounds(coords));
        &mut self.0[coords.0 as usize * 10 + coords.1 as usize]
    }

    fn neighbors(&self, coords: Coords) -> Vec<Coords> {
        let mut neighbors = vec![];
        for r in -1..=1 {
            for c in -1..=1 {
                if r == 0 && c == 0 {
                    continue;
                }
                let neighbor = Coords(coords.0 + r, coords.1 + c);
                if self.inbounds(neighbor) {
                    neighbors.push(neighbor);
                }
            }
        }
        neighbors
    }

    fn step(&mut self) -> u32 {
        let mut new_grid = self.clone();
        new_grid.0.iter_mut().for_each(|o| *o += 1);
        let mut flashed = [false; 100];
        while let Some((i, _, flash)) = multizip(((0..100), &new_grid.0, &mut flashed))
            .find(|(_, &val, &mut flashed)| !flashed && val >= 10)
        {
            *flash = true;
            for neighbor in self.neighbors(Coords(i / 10, i % 10)) {
                *new_grid.get_mut(neighbor) += 1;
            }
        }
        new_grid.0.iter_mut().for_each(|val| {
            if *val >= 10 {
                *val = 0
            }
        });
        *self = new_grid;
        flashed.into_iter().filter(|&i| i).count() as u32
    }
}

fn main() -> Result<()> {
    let file = std::fs::read_to_string("input/11.txt")?;
    let grid: Vec<u32> = file
        .lines()
        .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap()))
        .collect();
    let mut grid = Grid(grid.try_into().unwrap());
    let mut total = 0;
    for _ in 0..100 {
        total += grid.step();
    }
    println!("1: {}", total);
    for i in 100.. {
        if grid.step() == 100 {
            println!("2: {}", i + 1);
            break;
        }
    }
    Ok(())
}
