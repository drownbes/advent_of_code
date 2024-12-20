use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

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
struct RacePath {
    path: Vec<(usize, usize)>,
    dist: HashMap<(usize, usize), usize>,
}

impl RacePath {
    fn new(path: Vec<(usize, usize)>) -> RacePath {
        let mut dist = HashMap::new();
        for (d, loc) in path.iter().enumerate() {
            dist.insert(*loc, d);
        }
        RacePath { path, dist }
    }

    fn get_path_dist(&self, p: (usize, usize)) -> Option<usize> {
        self.dist.get(&p).copied()
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Cheat {
    start: (usize, usize),
    start_d: usize,
    end: (usize, usize),
    end_d: usize,
}

impl Hash for Cheat {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.start.hash(state);
        self.end.hash(state);
    }
}

impl Cheat {
    fn time_saved(&self) -> usize {
        let d = self.start.0.abs_diff(self.end.0) + (self.start.1).abs_diff(self.end.1) - 1;
        self.end_d - (self.start_d + d + 1)
    }
}

#[derive(Debug)]
struct World {
    grid: Vec<Vec<char>>,
    x_range: std::ops::Range<usize>,
    y_range: std::ops::Range<usize>,
}

impl World {
    fn from_grid(strs: &[&str]) -> World {
        let max_y = strs.len();
        let max_x = strs[0].len();
        let grid = strs.iter().map(|s| s.chars().collect()).collect();
        let x_range = 0..max_x;
        let y_range = 0..max_y;
        World {
            grid,
            x_range,
            y_range,
        }
    }

    fn is_in_bounds(&self, x: usize, y: usize) -> bool {
        self.x_range.contains(&x) && self.y_range.contains(&y)
    }

    fn get_char(&self, p: (usize, usize)) -> char {
        self.grid[p.1][p.0]
    }

    fn checked_shift(&self, from: &(usize, usize), dir: &Dir) -> Option<(usize, usize)> {
        let (shift_x, shift_y) = dir.get_shift();
        let new_x = from.0.checked_add_signed(shift_x)?;
        let new_y = from.1.checked_add_signed(shift_y)?;
        if self.is_in_bounds(new_x, new_y) && (self.get_char((new_x, new_y)) != '#') {
            Some((new_x, new_y))
        } else {
            None
        }
    }

    fn possible_moves(&self, p: (usize, usize)) -> impl Iterator<Item = (usize, usize)> + '_ {
        let moves = [Dir::Up, Dir::Down, Dir::Left, Dir::Right];
        moves
            .into_iter()
            .filter_map(move |dir| self.checked_shift(&p, &dir))
    }

    fn scan_iter(&self) -> impl Iterator<Item = (&char, usize, usize)> + '_ {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(y, line)| line.iter().enumerate().map(move |(x, ch)| (ch, x, y)))
    }

    fn find_char(&self, c: char) -> Option<(usize, usize)> {
        self.scan_iter()
            .find_map(|(ch, x, y)| if *ch == c { Some((x, y)) } else { None })
    }

    fn race_path(&self, from: (usize, usize), to: (usize, usize)) -> RacePath {
        let mut stack: Vec<(usize, usize)> = Vec::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        stack.push(from);
        visited.insert(from);
        let mut path = vec![];
        while let Some(p) = stack.pop() {
            path.push(p);
            if p == to {
                break;
            }

            for m in self.possible_moves(p) {
                if !visited.contains(&m) {
                    visited.insert(m);
                    stack.push(m);
                }
            }
        }
        RacePath::new(path)
    }
}

pub fn solve(strs: &[&str], cheat_n: usize) -> usize {
    let w = World::from_grid(strs);
    let start = w.find_char('S').unwrap();
    let end = w.find_char('E').unwrap();
    let rt = w.race_path(start, end);

    let mut cheats = vec![];
    for (i, f) in rt.path.iter().enumerate() {
        for t in rt.path.iter().skip(i) {
            if t == f {
                continue;
            }
            let f_d = rt.get_path_dist(*f);
            let t_d = rt.get_path_dist(*t);
            let ch = Cheat {
                start: *f,
                start_d: f_d.unwrap(),
                end: *t,
                end_d: t_d.unwrap(),
            };

            //let dist = w.shortes_path(*f, *t).unwrap();
            let dist = f.0.abs_diff(t.0) + (f.1).abs_diff(t.1);

            if ch.time_saved() >= 2 && dist <= cheat_n {
                //println!("{:?} {:?} = {:?}, {:?} {:?} {}", f, t, dist, f_d, t_d, ch.time_saved());
                cheats.push(ch);
            }
        }
    }

    let mut buckets: HashMap<usize, HashSet<Cheat>> = HashMap::new();

    for cheat in cheats {
        let ts = cheat.time_saved();
        if ts > 0 {
            buckets
                .entry(ts)
                .and_modify(|x| {
                    x.insert(cheat.clone());
                })
                .or_insert(HashSet::from([cheat]));
        }
    }

    let mut c = 0;

    for (k, v) in buckets.iter() {
        //println!("{} {}", k, v.len());
        if *k >= 100 {
            c += v.len();
        }
    }
    c
}

pub fn solve_part1(strs: &[&str]) -> usize {
    solve(strs, 2)
}

pub fn solve_part2(strs: &[&str]) -> usize {
    solve(strs, 20)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

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
        solve_part1(&input);
    }
}
