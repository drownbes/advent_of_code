use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    usize,
};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Move {
    cost: usize,
    p: Pos,
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
    fn new(from: &Pos, to: Pos) -> Move {
        let cost = from.dir.rotation_cost(&to.dir) * 1000 + 1;
        Move { cost, p: to }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Pos {
    x: usize,
    y: usize,
    dir: Dir,
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

    fn rotation_cost(&self, to_dir: &Dir) -> usize {
        match (self, to_dir) {
            (Dir::Up, Dir::Up) => 0,
            (Dir::Up, Dir::Down) => 2,
            (Dir::Up, Dir::Left) => 1,
            (Dir::Up, Dir::Right) => 1,
            (Dir::Down, Dir::Up) => 2,
            (Dir::Down, Dir::Down) => 0,
            (Dir::Down, Dir::Left) => 1,
            (Dir::Down, Dir::Right) => 1,
            (Dir::Left, Dir::Up) => 1,
            (Dir::Left, Dir::Down) => 1,
            (Dir::Left, Dir::Left) => 0,
            (Dir::Left, Dir::Right) => 2,
            (Dir::Right, Dir::Up) => 1,
            (Dir::Right, Dir::Down) => 1,
            (Dir::Right, Dir::Left) => 1,
            (Dir::Right, Dir::Right) => 0,
        }
    }
}

fn possible_moves(p: &Pos, grid: &[Vec<char>]) -> Vec<Move> {
    let moves = [Dir::Up, Dir::Down, Dir::Left, Dir::Right];
    moves
        .iter()
        .filter_map(|dir| {
            let (shift_x, shift_y) = dir.get_shift();
            let new_x = p.x.checked_add_signed(shift_x)?;
            let new_y = p.y.checked_add_signed(shift_y)?;
            match grid[new_y][new_x] {
                '#' => None,
                _ => Some(Move::new(
                    p,
                    Pos {
                        dir: *dir,
                        x: new_x,
                        y: new_y,
                    },
                )),
            }
        })
        .collect()
}

fn empty_locs(grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    grid.iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .flat_map(|(x, ch)| if *ch != '#' { Some((x, y)) } else { None })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect()
}

fn find_char(c: char, grid: &[Vec<char>]) -> Option<(usize, usize)> {
    grid.iter().enumerate().find_map(|(y, line)| {
        line.iter()
            .enumerate()
            .find_map(|(x, ch)| if *ch == c { Some((x, y)) } else { None })
    })
}

fn trace_paths(parent: &HashMap<Pos, Vec<Pos>>, target: Pos, start: Pos) -> usize {
    let mut stack: Vec<Pos> = vec![target];
    let mut paths: Vec<Vec<Pos>> = vec![];
    let mut path: Vec<Pos> = vec![];
    let mut uniq_locs: HashSet<(usize, usize)> = HashSet::new();
    while let Some(p) = stack.pop() {
        path.push(p);
        if p == start {
            uniq_locs.extend(path.iter().map(|l| (l.x, l.y)));
            paths.push(path.clone());
            path.clear();
        }
        if let Some(parents) = parent.get(&p) {
            for parent in parents {
                stack.push(*parent);
            }
        }
    }
    uniq_locs.len()
}

fn shortes_path(start: Pos, goal: (usize, usize), grid: &[Vec<char>]) -> Option<(usize, usize)> {
    let mut dist: HashMap<Pos, usize> = HashMap::new();
    let locs = empty_locs(grid);
    for loc in locs {
        dist.insert(
            Pos {
                dir: Dir::Up,
                x: loc.0,
                y: loc.1,
            },
            usize::MAX,
        );
        dist.insert(
            Pos {
                dir: Dir::Down,
                x: loc.0,
                y: loc.1,
            },
            usize::MAX,
        );
        dist.insert(
            Pos {
                dir: Dir::Left,
                x: loc.0,
                y: loc.1,
            },
            usize::MAX,
        );
        dist.insert(
            Pos {
                dir: Dir::Right,
                x: loc.0,
                y: loc.1,
            },
            usize::MAX,
        );
    }
    dist.insert(start, 0);
    let mut heap = BinaryHeap::new();
    heap.push(Move { cost: 0, p: start });
    let mut parents: HashMap<Pos, Vec<Pos>> = HashMap::new();

    while let Some(Move { cost, p }) = heap.pop() {
        if (p.x, p.y) == goal {
            let k = trace_paths(&parents, p, start);
            return Some((cost, k));
        }
        if cost > *dist.get(&p).unwrap() {
            continue;
        }

        for mv in possible_moves(&p, grid) {
            let next = Move {
                cost: cost + mv.cost,
                p: mv.p,
            };
            if next.cost < *dist.get(&next.p).unwrap() {
                heap.push(next);
                dist.insert(next.p, next.cost);
                parents.insert(next.p, vec![p]);
            } else if next.cost == *dist.get(&next.p).unwrap() {
                parents
                    .entry(next.p)
                    .and_modify(|x| x.push(p))
                    .or_insert(vec![p]);
            }
        }
    }
    None
}

pub fn solve_part1(strs: &[&str]) -> usize {
    let grid: Vec<Vec<char>> = strs.iter().map(|s| s.chars().collect()).collect();
    let (x, y) = find_char('S', &grid).unwrap();
    let start = Pos {
        dir: Dir::Right,
        x,
        y,
    };
    let goal = find_char('E', &grid).unwrap();
    let res = shortes_path(start, goal, &grid);
    res.unwrap().0
}
pub fn solve_part2(strs: &[&str]) -> usize {
    let grid: Vec<Vec<char>> = strs.iter().map(|s| s.chars().collect()).collect();
    let (x, y) = find_char('S', &grid).unwrap();
    let start = Pos {
        dir: Dir::Right,
        x,
        y,
    };
    let goal = find_char('E', &grid).unwrap();
    let res = shortes_path(start, goal, &grid);
    res.unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
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

        let res = solve_part1(&input);
        dbg!(res);
    }
}
