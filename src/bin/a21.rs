use aoc::prelude::*;

const INPUT: &str = include_str!("../../input/21.txt");

fn solve() -> Option<()> {
    let (p1, p2): (Option<i64>, Option<i64>) = INPUT
        .lines()
        .map(|line| line.split_once(':')?.1.trim().parse().ok())
        .collect_tuple()?;
    let (p0, p1) = (p1?, p2?);
    part1(p0, p1)?;

    let (a, b) = Universe::new(p0, p1).count_winning();
    println!("2: {}", a.max(b));
    Some(())
}

#[derive(Clone, Copy)]
struct Universe {
    rolls: usize,
    positions: [i64; 2],
    scores: [i64; 2],
    player: usize,
    threshold: i64,
}

impl Universe {
    fn new(p0: i64, p1: i64) -> Self {
        Self {
            rolls: 0,
            positions: [p0, p1],
            scores: [0, 0],
            player: 0,
            threshold: 21,
        }
    }

    fn step(&mut self, sum: i64) {
        self.rolls += 3;
        self.positions[self.player] = (self.positions[self.player] + sum - 1) % 10 + 1;
        self.scores[self.player] += self.positions[self.player];
        self.player = 1 - self.player;
    }

    fn count_winning(&self) -> (i64, i64) {
        let mut wins = (0, 0);
        let stats = [[3, 1], [4, 3], [5, 6], [6, 7], [7, 6], [8, 3], [9, 1]];
        for [sum, freq] in stats {
            let mut universe = *self;
            universe.step(sum);
            if universe.scores[0] >= self.threshold {
                wins.0 += freq;
            } else if universe.scores[1] >= self.threshold {
                wins.1 += freq;
            } else {
                let new_wins = universe.count_winning();
                wins.0 += freq * new_wins.0;
                wins.1 += freq * new_wins.1;
            }
        }
        wins
    }
}

fn part1(mut p0: i64, mut p1: i64) -> Option<()> {
    let (mut score0, mut score1) = (0, 0);
    let mut dice = (1..=100).cycle();
    let threshold = 1000;
    let mut rolls = 0;
    loop {
        // player one rolls dice
        let (d0, d1, d2) = dice.next_tuple()?;
        rolls += 3;
        p0 = (p0 + d0 + d1 + d2 - 1) % 10 + 1;
        score0 += p0;
        if score0 >= threshold {
            // player 2 lost
            println!("1: {}", score1 * rolls);
            break;
        }

        // player two rolls dice
        let (d0, d1, d2) = dice.next_tuple()?;
        rolls += 3;
        p1 = (p1 + d0 + d1 + d2 - 1) % 10 + 1;
        score1 += p1;
        if score1 >= threshold {
            // player 1 lost
            println!("1: {}", score0 * rolls);
            break;
        }
    }

    Some(())
}

fn main() {
    solve().unwrap();
}
