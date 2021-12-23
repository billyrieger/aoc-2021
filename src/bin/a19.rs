use aoc::prelude::*;
use itertools::iproduct;
use std::collections::VecDeque;

const INPUT: &str = include_str!("../../input/19.txt");

type Point = [i32; 3];

struct Scanner {
    beacons: HashMap<Point, HashSet<i32>>,
}

impl Scanner {
    fn new(input: Vec<Point>) -> Option<Self> {
        let mut beacons: HashMap<_, _> = input.iter().map(|&p| (p, HashSet::default())).collect();
        for (&u, &v) in input.iter().tuple_combinations() {
            let dist = norm_sq(u, v);
            beacons.get_mut(&u)?.insert(dist);
            beacons.get_mut(&v)?.insert(dist);
        }
        Self { beacons }.into()
    }

    fn merge(&mut self, other: &Self) -> Option<Point> {
        let mut common: Vec<(Point, Point)> = vec![];
        for ((&u, dists0), (&v, dists1)) in iproduct!(&self.beacons, &other.beacons) {
            let similarity = dists0.iter().filter(|d| dists1.contains(d)).count();
            if similarity >= 11 {
                common.push((u, v));
            }
        }
        if common.len() < 11 {
            return None;
        }
        type Func = Box<dyn Fn(Point) -> Point>;
        let orientations: Vec<Func> = iproduct!([-1, 1], [-1, 1], [-1, 1])
            .flat_map(|(i, j, k)| {
                [
                    Box::new(move |[x, y, z]: Point| [i * x, j * y, k * z]) as Func,
                    Box::new(move |[x, y, z]: Point| [i * x, j * z, k * y]) as Func,
                    Box::new(move |[x, y, z]: Point| [i * y, j * x, k * z]) as Func,
                    Box::new(move |[x, y, z]: Point| [i * y, j * z, k * x]) as Func,
                    Box::new(move |[x, y, z]: Point| [i * z, j * x, k * y]) as Func,
                    Box::new(move |[x, y, z]: Point| [i * z, j * y, k * x]) as Func,
                ]
            })
            .collect();

        for func in orientations {
            if common
                .iter()
                .copied()
                .map(|(u, v)| (u, func(v)))
                .map(|(u, v)| [v[0] - u[0], v[1] - u[1], v[2] - u[2]])
                .all_equal()
            {
                let (u, mut v) = common[0];
                v = func(v);
                let delta: Point = [u[0] - v[0], u[1] - v[1], u[2] - v[2]];
                for (&point, dists) in &other.beacons {
                    let mut new = func(point);
                    new = [new[0] + delta[0], new[1] + delta[1], new[2] + delta[2]];
                    self.beacons
                        .entry(new)
                        .or_insert_with(|| HashSet::default())
                        .extend(dists);
                }
                let mut s = func([0, 0, 0]);
                s = [s[0] + delta[0], s[1] + delta[1], s[2] + delta[2]];
                return Some(s);
            }
        }

        None
    }
}

fn norm_sq(u: Point, v: Point) -> i32 {
    u.into_iter().zip(v).map(|(i, j)| (i - j).pow(2)).sum()
}

fn solve() -> Option<()> {
    let mut scanners: Vec<Scanner> = INPUT
        .lines()
        .group_by(|line| line.is_empty())
        .into_iter()
        // skip the empty lines between scanners
        .step_by(2)
        .map(|(_, group)| {
            Scanner::new(
                group
                    // skip the header line
                    .skip(1)
                    .map(|line| {
                        // parse each digit
                        let (x, y, z) = line
                            .split(',')
                            .map(|num| num.parse().ok())
                            .collect_tuple()?;
                        [x?, y?, z?].into()
                    })
                    .collect::<Option<_>>()?,
            )
        })
        .collect::<Option<_>>()?;

    let (all, others) = scanners.split_first_mut()?;
    let mut queue = VecDeque::from_iter(others);
    let mut centers = vec![[0; 3]];
    while let Some(scanner) = queue.pop_front() {
        if let Some(center) = all.merge(&scanner) {
            centers.push(center);
        } else {
            queue.push_back(scanner);
        }
    }
    println!("1: {}", all.beacons.len());
    println!(
        "2: {}",
        centers
            .into_iter()
            .tuple_combinations()
            .map(|(p, q)| p
                .iter()
                .zip(q.iter())
                .map(|(x0, x1)| (x0 - x1).abs())
                .sum::<i32>())
            .max()?
    );
    Some(())
}

fn main() {
    solve().unwrap();
}
