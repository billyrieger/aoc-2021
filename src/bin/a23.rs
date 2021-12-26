use aoc::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    const fn all() -> [Self; 4] {
        [Self::Amber, Self::Bronze, Self::Copper, Self::Desert]
    }

    fn room(&self) -> usize {
        match self {
            Self::Amber => 2,
            Self::Bronze => 4,
            Self::Copper => 6,
            Self::Desert => 8,
        }
    }

    fn cost(&self) -> i32 {
        match self {
            Self::Amber => 1,
            Self::Bronze => 10,
            Self::Copper => 100,
            Self::Desert => 1000,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Space {
    Hallway(Option<Amphipod>),
    Room(Vec<Amphipod>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct State {
    total_cost: i32,
    room_depth: usize,
    spaces: [Space; 11],
}

impl State {
    fn new([amber, bronze, copper, desert]: [Vec<Amphipod>; 4]) -> Self {
        Self {
            total_cost: 0,
            room_depth: amber.len(),
            spaces: [
                Space::Hallway(None),
                Space::Hallway(None),
                Space::Room(amber),
                Space::Hallway(None),
                Space::Room(bronze),
                Space::Hallway(None),
                Space::Room(copper),
                Space::Hallway(None),
                Space::Room(desert),
                Space::Hallway(None),
                Space::Hallway(None),
            ],
        }
    }

    fn _get_hallway(&mut self, index: usize) -> Option<&Option<Amphipod>> {
        match self.spaces.get(index)? {
            Space::Hallway(space) => Some(space),
            Space::Room(_) => None,
        }
    }

    const fn hallways() -> [usize; 7] {
        [0, 1, 3, 5, 7, 9, 10]
    }

    fn is_uniform(&self, room: Amphipod) -> bool {
        match &self.spaces[room.room()] {
            Space::Room(occupants) => occupants.iter().all(|&pod| pod == room),
            Space::Hallway(_) => false,
        }
    }

    fn open_between(&self, start: usize, dest: usize) -> bool {
        let (i, j) = if start < dest {
            (start + 1, dest)
        } else {
            (dest, start - 1)
        };
        self.spaces[i..=j]
            .iter()
            .all(|space| !matches!(space, Space::Hallway(Some(_))))
    }

    fn enter_room(&mut self, from: usize) -> Option<()> {
        let mover = match self.spaces.get_mut(from)? {
            Space::Hallway(space) => space.take()?,
            Space::Room(_) => None?,
        };
        self.total_cost += mover.cost() * ((from as i32) - (mover.room() as i32));
        match self.spaces.get_mut(mover.room())? {
            Space::Room(occupants) => {
                self.total_cost += mover.cost() * (self.room_depth - occupants.len()) as i32;
                occupants.push(mover);
            }
            Space::Hallway(_) => None?,
        };
        Some(())
    }

    fn leave_room(&mut self, from: Amphipod, to: usize) -> Option<()> {
        let mover = match self.spaces.get_mut(from.room())? {
            Space::Room(occupants) => {
                let mover = occupants.pop()?;
                self.total_cost += mover.cost() * (self.room_depth - occupants.len()) as i32;
                mover
            }
            _ => None?,
        };
        match self.spaces.get_mut(to)? {
            Space::Hallway(space @ None) => space.replace(mover),
            _ => None?,
        };
        self.total_cost += mover.cost() * ((from.room() as i32) - (to as i32)).abs();
        Some(())
    }

    fn moves(&self) -> Option<Vec<Self>> {
        // The rules imply an amphipod can make exactly two moves:
        //  - Out of a home to the hallway
        //  - From the hallway to its correct home
        // `(Amphipod, usize)`
        // Each home functions like a stack where the cost to add or remove an
        // amphipod is proportional to the number of vacancies in the home.

        let mut moves = vec![];
        for start in Self::hallways() {
            match self.spaces.get(start)? {
                Space::Hallway(Some(pod)) => {
                    if self.open_between(start, pod.room()) && self.is_uniform(*pod) {
                        let can_enter = match self.spaces.get(pod.room())? {
                            Space::Room(occupants) => occupants.iter().all_equal(),
                            _ => false,
                        };
                        if can_enter {
                            let mut new = self.clone();
                            new.enter_room(start);
                            moves.push(new);
                        }
                    }
                }
                Space::Hallway(None) => continue,
                _ => None?,
            };
        }
        for pod in Amphipod::all() {
            for dest in Self::hallways() {
                if self.open_between(pod.room(), dest) {
                    let mut new = self.clone();
                    new.leave_room(pod, dest);
                    moves.push(new);
                }
            }
        }
        println!("moves: {:?}", moves);
        Some(moves)
    }
}

fn main() -> Result<()> {
    let state = State {
        total_cost: 0,
        room_depth: 2,
        spaces: [
            Space::Hallway(None),
            Space::Hallway(Some(Amphipod::Amber)),
            Space::Room(vec![Amphipod::Amber; 1]),
            Space::Hallway(None),
            Space::Room(vec![Amphipod::Bronze; 2]),
            Space::Hallway(None),
            Space::Room(vec![Amphipod::Copper; 2]),
            Space::Hallway(None),
            Space::Room(vec![Amphipod::Desert; 2]),
            Space::Hallway(None),
            Space::Hallway(None),
        ],
    };
    // let state = State::new([
    // vec![Amphipod::Copper, Amphipod::Copper],
    // vec![Amphipod::Bronze, Amphipod::Bronze],
    // vec![Amphipod::Amber, Amphipod::Amber],
    // vec![Amphipod::Desert, Amphipod::Desert],
    // ]);
    let goal = State::new([
        vec![Amphipod::Amber, Amphipod::Amber],
        vec![Amphipod::Bronze, Amphipod::Bronze],
        vec![Amphipod::Copper, Amphipod::Copper],
        vec![Amphipod::Desert, Amphipod::Desert],
    ]);
    let mut queue = Vec::from([state]);
    let mut min_score = i32::MAX;
    while let Some(state) = queue.pop() {
        println!("{:?}", state.total_cost);
        if state.total_cost > min_score {
            continue;
        }
        if state == goal {
            min_score = min_score.min(state.total_cost);
            println!("SCORE: {}", min_score);
            break;
        }
        queue.extend(state.moves().unwrap());
    }
    Ok(())
}
