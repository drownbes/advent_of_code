use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    usize,
};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Move {
    cost: usize,
    p: (usize, usize),
}

impl Ord for Move {
    fn cmp(&self, other: &Move) -> Ordering {
        // Notice that the we flip the ordering here
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Move) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Move {
    fn new(to: (usize, usize)) -> Move {
        Move { cost: 1, p: to }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn get_shift(&self) -> (isize, isize) {
        match self {
            Dir::Up => (0, -1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
            Dir::Down => (0, 1),
        }
    }
}

#[derive(Debug)]
struct World {
    grid: Vec<Vec<char>>,
    x_range: std::ops::Range<usize>,
    y_range: std::ops::Range<usize>,
    free_loc_char: char,
}

impl World {
    fn new(x_width: usize, y_width: usize, free_loc_char: char) -> World {
        let x_range = 0..x_width;
        let y_range = 0..y_width;
        let grid = vec![vec![free_loc_char; x_width]; y_width];
        World {
            grid,
            x_range,
            y_range,
            free_loc_char,
        }
    }

    fn from_grid(strs: &[&str], free_loc_char: char) -> World {
        let max_y = strs.len();
        let max_x = strs[0].len();
        let grid = strs.iter().map(|s| s.chars().collect()).collect();
        let x_range = 0..max_x;
        let y_range = 0..max_y;
        World {
            grid,
            x_range,
            y_range,
            free_loc_char,
        }
    }

    fn is_in_bounds(&self, x: usize, y: usize) -> bool {
        self.x_range.contains(&x) && self.y_range.contains(&y)
    }

    fn get_char(&self, p: (usize, usize)) -> char {
        self.grid[p.1][p.0]
    }

    fn set_char(&mut self, p: &(usize, usize), v: char) {
        self.grid[p.1][p.0] = v;
    }

    fn checked_shift(&self, from: &(usize, usize), dir: &Dir) -> Option<(usize, usize)> {
        let (shift_x, shift_y) = dir.get_shift();
        let new_x = from.0.checked_add_signed(shift_x)?;
        let new_y = from.1.checked_add_signed(shift_y)?;
        if self.is_in_bounds(new_x, new_y) && (self.get_char((new_x, new_y)) == self.free_loc_char)
        {
            Some((new_x, new_y))
        } else {
            None
        }
    }

    fn possible_moves(&self, p: (usize, usize)) -> impl Iterator<Item = Move> + '_ {
        let moves = [Dir::Up, Dir::Down, Dir::Left, Dir::Right];
        moves
            .into_iter()
            .filter_map(move |dir| self.checked_shift(&p, &dir))
            .map(Move::new)
    }
    fn scan_iter(&self) -> impl Iterator<Item = (&char, usize, usize)> + '_ {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(y, line)| line.iter().enumerate().map(move |(x, ch)| (ch, x, y)))
    }

    fn empty_locs_iter(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.scan_iter().filter_map(|(ch, x, y)| {
            if *ch == self.free_loc_char {
                Some((x, y))
            } else {
                None
            }
        })
    }

    fn shortes_path(&self, from: (usize, usize), to: (usize, usize)) -> Option<usize> {
        let mut dist: HashMap<(usize, usize), usize> = HashMap::new();
        for loc in self.empty_locs_iter() {
            dist.insert(loc, usize::MAX);
        }
        dist.insert(from, 0);

        let mut heap = BinaryHeap::new();
        heap.push(Move { cost: 0, p: from });

        while let Some(Move { cost, p }) = heap.pop() {
            if p == to {
                return Some(cost);
            }
            if cost > *dist.get(&p).unwrap() {
                continue;
            }

            for mv in self.possible_moves(p) {
                let next = Move {
                    cost: cost + mv.cost,
                    p: mv.p,
                };
                if next.cost < *dist.get(&next.p).unwrap() {
                    heap.push(next);
                    dist.insert(next.p, next.cost);
                }
            }
        }
        None
    }
}

fn parse_fall(strs: &[&str]) -> Vec<(usize, usize)> {
    strs.iter()
        .map(|s| {
            let v: Vec<usize> = s.split(",").map(|x| x.parse().unwrap()).collect();
            (v[0], v[1])
        })
        .collect()
}

pub fn solve_part1(strs: &[&str]) -> usize {
    let fall = parse_fall(strs);
    let mut w = World::new(71, 71, '.');
    for loc in fall.iter().take(1024) {
        w.set_char(&(loc.0, loc.1), '#');
    }
    let res = w.shortes_path((0, 0), (70, 70));
    res.unwrap()
}
pub fn solve_part2(strs: &[&str]) -> (usize, usize) {
    let fall = parse_fall(strs);
    let mut w = World::new(71, 71, '.');
    for loc in fall.iter().take(1024) {
        w.set_char(&(loc.0, loc.1), '#');
    }

    fall.into_iter()
        .skip(1024)
        .find(|loc| {
            w.set_char(&(loc.0, loc.1), '#');
            let res = w.shortes_path((0, 0), (70, 70));
            res.is_none()
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
        ...#...
        ..#..#.
        ....#..
        ...#..#
        ..#..#.
        .#..#..
        #.#....";

    fn read_input(inp: &str) -> Vec<&str> {
        inp.strip_prefix("\n")
            .unwrap()
            .lines()
            .map(str::trim)
            .collect()
    }

    #[test]
    fn test_name() {
        let input = read_input(EXAMPLE);
        let w = World::from_grid(&input, '.');
        let res = w.shortes_path((0, 0), (6, 6));
        assert_eq!(res, Some(22));
    }
}
