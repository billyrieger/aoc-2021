use std::fmt;
use itertools::Itertools;

const INPUT: &str = include_str!("../../input/18.txt");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Num {
    value: u32,
    depth: usize,
}

#[derive(Clone, PartialEq, Eq)]
struct Snailfish(Vec<Num>);

impl fmt::Debug for Snailfish {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Snailfish")
            .field(&self.0.iter().map(|n| n.value).collect::<Vec<_>>())
            .finish()
    }
}

impl Snailfish {
    fn new(input: &str) -> Self {
        Self::parse(input).unwrap()
    }

    fn parse(input: &str) -> Option<Self> {
        let mut depth = 0;
        let mut nums = vec![];
        for c in input.chars() {
            match c {
                '[' => depth += 1,
                ']' => depth -= 1,
                ',' => continue,
                x => {
                    nums.push(Num {
                        value: x.to_digit(10)?,
                        depth,
                    });
                }
            }
        }
        Some(Self(nums))
    }

    fn explode_index(&self) -> Option<usize> {
        for (i, num) in self.0.iter().enumerate() {
            if num.depth >= 5 {
                return Some(i);
            }
        }
        None
    }

    fn explode(&mut self) {
        if let Some(i) = self.explode_index() {
            let lidx = i;
            let ridx = i + 1;
            let left = self.0[lidx];
            let right = self.0[ridx];
            if lidx > 0 {
                self.0[lidx - 1].value += left.value;
            }
            if ridx < self.0.len() - 1 {
                self.0[ridx + 1].value += right.value;
            }
            self.0[lidx] = Num {
                value: 0,
                depth: left.depth - 1,
            };
            self.0.remove(ridx);
        }
    }

    fn split(&mut self) {
        for i in 0..self.0.len() {
            if self.0[i].value >= 10 {
                let depth = self.0[i].depth + 1;
                let value = self.0[i].value;
                let left = Num {
                    value: value / 2,
                    depth,
                };
                let right = Num {
                    value: (value + 1) / 2,
                    depth,
                };
                self.0[i] = left;
                self.0.insert(i + 1, right);
                break;
            }
        }
    }

    fn reduce(&mut self) {
        loop {
            let prev = self.clone();
            self.explode();
            if *self != prev {
                continue;
            }
            self.split();
            if *self != prev {
                continue;
            }
            break;
        }
    }

    fn add(mut left: Self, mut right: Self) -> Self {
        left.0.iter_mut().for_each(|n| n.depth += 1);
        right.0.iter_mut().for_each(|n| n.depth += 1);
        let mut total = vec![];
        total.append(&mut left.0);
        total.append(&mut right.0);
        let mut result = Self(total);
        result.reduce();
        result
    }

    fn magnitude(mut self) -> u32 {
        loop {
            if self.0.len() == 1 {
                break;
            }
            let max_depth = self.0.iter().map(|n| n.depth).max().unwrap();
            for i in 0..(self.0.len() - 1) {
                if self.0[i].depth == max_depth && self.0[i + 1].depth == max_depth {
                    let magnitude = 3 * self.0[i].value + 2 * self.0[i + 1].value;
                    self.0[i] = Num {
                        value: magnitude,
                        depth: self.0[i].depth - 1,
                    };
                    self.0.remove(i + 1);
                    break;
                }
            }
        }
        self.0[0].value
    }
}

fn main() {
    solve().unwrap()
}

fn solve() -> Option<()> {
    let all_fish: Vec<_> = INPUT.lines().map(Snailfish::new).collect();
    let mut total = all_fish[0].clone();
    for fish in all_fish.iter().skip(1) {
        total = Snailfish::add(total, fish.clone());
    }
    println!("1: {}", total.magnitude());
    let max_sum = all_fish.iter().tuple_combinations().map(|(a, b)| {
        Snailfish::add(a.clone(), b.clone()).magnitude()
    }).max().unwrap();
    println!("2: {}", max_sum);
    Some(())
}
