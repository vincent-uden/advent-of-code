use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Facing {
    North,
    East,
    South,
    West,
}

impl Facing {
    fn turn(&self) -> Self {
        use Facing::*;
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Guard {
    pub facing: Facing,
    pub x: i64,
    pub y: i64,
}

impl Guard {
    pub fn walk(&mut self, world: &World) {
        let mut candidate = None;
        while candidate.is_none() {
            candidate = Some(self.in_front());

            let mut is_free = true;
            for obs in &world.obstacles {
                if Some(*obs) == candidate {
                    is_free = false;
                }
            }
            if !is_free {
                self.facing = self.facing.turn();
                candidate = None;
            }
        }
        self.x = candidate.unwrap().0;
        self.y = candidate.unwrap().1;
    }

    pub fn in_front(&mut self) -> (i64, i64) {
        match self.facing {
            Facing::North => (self.x, self.y - 1),
            Facing::East => (self.x + 1, self.y),
            Facing::South => (self.x, self.y + 1),
            Facing::West => (self.x - 1, self.y),
        }
    }
}

#[derive(Debug, Clone)]
struct World {
    pub width: i64,
    pub height: i64,
    pub obstacles: Vec<(i64, i64)>,
}

impl World {
    pub fn parse_world(input: &str) -> Option<(Self, Guard)> {
        let lines: Vec<_> = input.lines().collect();
        let mut out = World {
            width: lines[0].len() as i64,
            height: lines.len() as i64,
            obstacles: vec![],
        };
        let mut guard = Guard {
            facing: Facing::East,
            x: -1,
            y: -1,
        };

        for (y, l) in lines.iter().enumerate() {
            if (l.len() as i64) != out.width {
                return None;
            }
            for (x, c) in l.chars().enumerate() {
                match c {
                    '#' => {
                        out.obstacles.push((x as i64, y as i64));
                    }
                    '^' => {
                        guard.x = x as i64;
                        guard.y = y as i64;
                        guard.facing = Facing::North;
                    }
                    'v' => {
                        guard.x = x as i64;
                        guard.y = y as i64;
                        guard.facing = Facing::South;
                    }
                    '>' => {
                        guard.x = x as i64;
                        guard.y = y as i64;
                        guard.facing = Facing::East;
                    }
                    '<' => {
                        guard.x = x as i64;
                        guard.y = y as i64;
                        guard.facing = Facing::West;
                    }
                    _ => {}
                }
            }
        }
        if guard.x == -1 {
            None
        } else {
            Some((out, guard))
        }
    }

    pub fn in_bounds(&self, guard: &Guard) -> bool {
        guard.x >= 0 && guard.y >= 0 && guard.x < self.width && guard.y < self.height
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (world, mut guard) = World::parse_world(input).unwrap();
    let mut positions = HashSet::new();

    while world.in_bounds(&guard) {
        positions.insert((guard.x, guard.y));
        guard.walk(&world);
    }

    Some(positions.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut world, guard) = World::parse_world(input).unwrap();
    let mut positions: HashSet<(i64, i64, Facing)> = HashSet::new();
    let mut cycling_obstacles: Vec<(i64, i64)> = vec![];

    for x in 0..world.width {
        for y in 0..world.height {
            let mut guard = guard.clone();
            world.obstacles.push((x, y));

            while world.in_bounds(&guard) {
                let next = (guard.x, guard.y, guard.facing);
                if positions.contains(&next) {
                    cycling_obstacles.push((x, y));
                    break;
                }
                positions.insert(next);
                guard.walk(&world);
            }

            positions.clear();
            world.obstacles.pop();
        }
    }

    Some(cycling_obstacles.len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
