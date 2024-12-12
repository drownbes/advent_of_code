use std::{
    collections::{HashMap, HashSet, VecDeque},
    usize,
};

#[derive(Debug, Clone, Eq)]
struct Plant {
    v: char,
    x: usize,
    y: usize,
    neighbors: Option<Vec<(isize, isize)>>,
}

impl Plant {
    fn get_sides(&self) -> Vec<[(usize, usize); 2]> {
        let ns = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let i: &Vec<(isize, isize)> = &self.neighbors.clone().unwrap_or_default();
        ns.iter()
            .filter(|s| !i.iter().any(|n| n == *s))
            .map(|s| {
                let n: [(usize, usize); 2] = match s {
                    (-1, 0) => [(self.x, self.y), (self.x, self.y + 1)],
                    (1, 0) => [(self.x + 1, self.y), (self.x + 1, self.y + 1)],
                    (0, -1) => [(self.x, self.y), (self.x + 1, self.y)],
                    (0, 1) => [(self.x, self.y + 1), (self.x + 1, self.y + 1)],
                    _ => unreachable!(),
                };
                n
            })
            .collect()
    }
}

impl PartialEq for Plant {
    fn eq(&self, other: &Self) -> bool {
        self.v == other.v && self.x == other.x && self.y == other.y
    }
}

impl std::hash::Hash for Plant {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.v.hash(state);
        self.x.hash(state);
        self.y.hash(state);
    }
}

struct World<'a> {
    grid: &'a [&'a str],
    max_x: usize,
    max_y: usize,
}

impl<'a> World<'a> {
    fn new(grid: &'a [&'a str]) -> World {
        World {
            grid,
            max_x: grid[0].len(),
            max_y: grid.len(),
        }
    }

    fn get_pos(&self, x: usize, y: usize) -> Option<char> {
        if (0..self.max_x).contains(&x) && (0..self.max_y).contains(&y) {
            self.grid[y].chars().nth(x)
        } else {
            None
        }
    }
}

fn possible_moves(p: &mut Plant, world: &World) -> Vec<Plant> {
    let moves = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    moves
        .iter()
        .filter_map(|s @ (shift_x, shift_y)| {
            let new_x = p.x.checked_add_signed(*shift_x)?;
            let new_y = p.y.checked_add_signed(*shift_y)?;
            match world.get_pos(new_x, new_y) {
                Some(v) if v == p.v => {
                    if let Some(ns) = &mut p.neighbors {
                        ns.push(*s);
                    } else {
                        p.neighbors = Some(vec![*s]);
                    }

                    Some(Plant {
                        x: new_x,
                        y: new_y,
                        v,
                        neighbors: None,
                    })
                }
                _ => None,
            }
        })
        .collect()
}

fn scan_region(start_p: &Plant, world: &World) -> Region {
    let mut queue: VecDeque<Plant> = VecDeque::new();
    let mut visited: HashSet<Plant> = HashSet::new();
    queue.push_back(start_p.clone());
    visited.insert(start_p.clone());
    while let Some(mut p) = queue.pop_front() {
        let mvs = possible_moves(&mut p, world);
        visited.replace(p.clone());
        for m in mvs {
            if !visited.contains(&m) {
                visited.insert(m.clone());
                queue.push_back(m);
            }
        }
    }
    Region { plants: visited }
}

#[derive(Debug)]
struct Region {
    plants: HashSet<Plant>,
}

impl Region {
    fn get_sides(&self) -> Vec<[(usize, usize); 2]> {
        self.plants.iter().flat_map(|p| p.get_sides()).collect()
    }

    fn get_number_of_sides(&self) -> usize {
        let sides = self.get_sides();

        let mut joins: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

        for side in sides {
            joins
                .entry(side[0])
                .and_modify(|x| x.push(side[1]))
                .or_insert(vec![side[1]]);

            joins
                .entry(side[1])
                .and_modify(|x| x.push(side[0]))
                .or_insert(vec![side[0]]);
        }

        let corners: usize = joins
            .iter()
            .map(|(p, v)| {
                let c0 = v[0];
                let c1 = v[1];
                let is_line = (c0.0 == p.0 && c1.0 == p.0) || (c0.1 == p.1 && c1.1 == p.1);

                if v.len() == 4 {
                    2
                } else if is_line {
                    0
                } else {
                    1
                }
            })
            .sum();

        corners
    }

    fn has_plant(&self, p: &Plant) -> bool {
        self.plants.contains(p)
    }

    fn area(&self) -> usize {
        self.plants.len()
    }

    fn perimeter(&self) -> usize {
        self.plants
            .iter()
            .map(|p| 4 - p.neighbors.as_ref().map_or(0, |x| x.len()))
            .sum()
    }

    fn fence_price(&self) -> usize {
        self.area() * self.perimeter()
    }

    fn fence_price_discount(&self) -> usize {
        self.area() * self.get_number_of_sides()
    }
}

fn scan_regions(world: &World) -> Vec<Region> {
    let mut regions: Vec<Region> = vec![];
    for (y, line) in world.grid.iter().enumerate() {
        for (x, v) in line.chars().enumerate() {
            let p = Plant {
                x,
                y,
                v,
                neighbors: None,
            };
            if !regions.iter().any(|r| r.has_plant(&p)) {
                regions.push(scan_region(&p, world));
            }
        }
    }
    regions
}

pub fn solve_part1(strs: &[&str]) -> usize {
    let w = World::new(strs);
    let regions = scan_regions(&w);

    regions.iter().map(|r| r.fence_price()).sum()
}

pub fn solve_part2(strs: &[&str]) -> usize {
    let w = World::new(strs);
    let regions = scan_regions(&w);

    regions.iter().map(|r| r.fence_price_discount()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE0: &str = "
        AAAA
        BBCD
        BBCC
        EEEC
    ";

    const EXAMPLE1: &str = "
        RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE
    ";

    const EXAMPLE2: &str = "
        AAAAAA
        AAABBA
        AAABBA
        ABBAAA
        ABBAAA
        AAAAAA
    ";

    const EXAMPLE3: &str = "
        AAAAAA
        AAA..A
        AAA..A
        A..AAA
        A..AAA
        AAAAAA
    ";

    fn read_input(inp: &str) -> Vec<&str> {
        inp.lines()
            .map(str::trim)
            .filter(|x| !x.is_empty())
            .collect()
    }

    #[test]
    fn test_example0() {
        let input = read_input(EXAMPLE0);
        let w = World::new(&input);
        let res = scan_regions(&w);
        for r in res {
            let s = r.get_number_of_sides();
            dbg!(s);
        }
    }

    #[test]
    fn test_example1() {
        let input = read_input(EXAMPLE1);
        let res = solve_part1(&input);
        assert_eq!(res, 1930);
    }

    #[test]
    fn test_example0_part2() {
        let input = read_input(EXAMPLE0);
        let res = solve_part2(&input);
        assert_eq!(res, 80);
    }

    #[test]
    fn test_example2_part2() {
        let input = read_input(EXAMPLE2);

        let w = World::new(&input);
        let res = scan_regions(&w);
        for r in res {
            let s = r.get_number_of_sides();
            dbg!(s);
        }
    }
}
