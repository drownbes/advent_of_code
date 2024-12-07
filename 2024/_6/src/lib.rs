use rayon::prelude::*;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn shift(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Guard {
    x: usize,
    y: usize,
    facing: Direction,
}

impl Guard {
    fn turn(&mut self) {
        self.facing = match self.facing {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
    }
}

fn find_guard(strs: &[&str]) -> Option<Guard> {
    let guard: Option<Guard> = None;
    for (y, line) in strs.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '^' {
                return Some(Guard {
                    x,
                    y,
                    facing: Direction::Up,
                });
            }
        }
    }
    guard
}

fn unique_locations(guard: &Guard, strs: &[&str]) -> HashSet<(usize, usize)> {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut guard = guard.clone();

    let max_x: isize = strs[0].len() as isize;
    let max_y: isize = strs.len() as isize;

    visited.insert((guard.x, guard.y));
    loop {
        let shift = guard.facing.shift();
        let new_x = guard.x as isize + shift.0;
        let new_y = guard.y as isize + shift.1;
        if new_x < 0 || new_x >= max_x || new_y < 0 || new_y >= max_y {
            break;
        }
        let new_x = new_x as usize;
        let new_y = new_y as usize;
        let c = strs[new_y].as_bytes()[new_x];
        if c == b'#' {
            guard.turn();
        } else {
            guard.x = new_x;
            guard.y = new_y;
            visited.insert((guard.x, guard.y));
        }
    }
    visited
}

fn has_cycle(guard: &Guard, strs: &[Vec<u8>]) -> bool {
    let mut visited: HashSet<Guard> = HashSet::new();
    let mut guard = guard.clone();

    let max_x: isize = strs[0].len() as isize;
    let max_y: isize = strs.len() as isize;

    visited.insert(guard.clone());
    loop {
        let shift = guard.facing.shift();
        let new_x = guard.x as isize + shift.0;
        let new_y = guard.y as isize + shift.1;
        if new_x < 0 || new_x >= max_x || new_y < 0 || new_y >= max_y {
            break;
        }
        let new_x = new_x as usize;
        let new_y = new_y as usize;
        let c = strs[new_y][new_x];
        if c == b'#' {
            guard.turn();
        } else {
            guard.x = new_x;
            guard.y = new_y;
            if visited.contains(&guard) {
                return true;
            }
            visited.insert(guard.clone());
        }
    }
    false
}

pub fn solve_part1(strs: &[&str]) -> usize {
    let guard = find_guard(strs).expect("Failed to find guard");
    unique_locations(&guard, strs).len()
}

pub fn solve_part2(strs: &[&str]) -> usize {
    let guard = find_guard(strs).expect("Failed to find guard");
    let un_locs = unique_locations(&guard, strs);
    let map_v: Vec<Vec<u8>> = strs
        .to_vec()
        .clone()
        .iter()
        .map(|x| Vec::from(x.as_bytes()))
        .collect();

    un_locs
        .par_iter()
        .map(|(x, y)| {
            let mut new_map: Vec<Vec<u8>> = map_v.clone();
            new_map[*y][*x] = b'#';

            if has_cycle(&guard, &new_map) {
                1
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    /*      0123456789
           0....#.....
           1.........#
           2..........
           3..#.......
           4.......#..
           5..........
           6.#.O^.....
           7......OO#.
           8#.........
           9......#...
    */

    const INPUT: &str = "
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
    ";

    fn read_input() -> Vec<&'static str> {
        INPUT
            .lines()
            .map(str::trim)
            .filter(|x| !x.is_empty())
            .collect()
    }

    #[test]
    fn test_find_guard() {
        let input = read_input();

        let g = find_guard(&input).unwrap();

        assert_eq!(g.x, 4);
        assert_eq!(g.y, 6);
    }

    #[test]
    fn test_part1() {
        let input = read_input();
        let res = solve_part1(&input);

        assert_eq!(res, 41);
    }

    #[test]
    fn test_part2() {
        let input = read_input();
        let res = solve_part2(&input);

        assert_eq!(res, 6);
    }
}
