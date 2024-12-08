use itertools::Itertools;
use std::cmp::Ordering;
use std::{
    collections::{HashMap, HashSet},
    fmt, isize,
};

#[derive(Debug, Clone)]
struct Tower {
    x: usize,
    y: usize,
    ch: u8,
}

impl fmt::Display for Tower {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "({},{}[{}])", self.x, self.y, self.ch)
    }
}

fn find_towers(world: &[&str]) -> HashMap<u8, Vec<Tower>> {
    let mut tm: HashMap<u8, Vec<Tower>> = HashMap::new();

    for (y, line) in world.iter().enumerate() {
        for (x, ch) in line.as_bytes().iter().enumerate() {
            if *ch == b'.' {
                continue;
            }
            if let Some(v) = tm.get_mut(ch) {
                v.push(Tower { x, y, ch: *ch });
            } else {
                tm.insert(*ch, vec![Tower { x, y, ch: *ch }]);
            }
        }
    }
    tm
}

fn find_antinodes(t0: &Tower, t1: &Tower, max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
    let dx = t1.x.abs_diff(t0.x);
    let dy = t1.y.abs_diff(t0.y);

    let mut res: Vec<(isize, isize)> = vec![];

    match (t0.x.cmp(&t1.x), t0.y.cmp(&t1.y)) {
        (Ordering::Equal, Ordering::Less) => {
            let y0 = t0.y as isize - dy as isize;
            let y1 = t1.y as isize + dy as isize;
            res.push((t0.x as isize, y0));
            res.push((t0.x as isize, y1));
        }
        (Ordering::Equal, Ordering::Greater) => {
            let y0 = t1.y as isize - dy as isize;
            let y1 = t0.y as isize + dy as isize;
            res.push((t0.x as isize, y0));
            res.push((t0.x as isize, y1));
        }

        (Ordering::Less, Ordering::Equal) => {
            let x0 = t0.x as isize - dx as isize;
            let x1 = t1.x as isize + dx as isize;
            res.push((x0, t0.y as isize));
            res.push((x1, t0.y as isize));
        }
        (Ordering::Greater, Ordering::Equal) => {
            let x0 = t1.x as isize - dx as isize;
            let x1 = t0.x as isize + dx as isize;
            res.push((x0, t0.y as isize));
            res.push((x1, t0.y as isize));
        }

        (Ordering::Less, Ordering::Less) => {
            let x0 = t0.x as isize - dx as isize;
            let x1 = t1.x as isize + dx as isize;

            let y0 = t0.y as isize - dy as isize;
            let y1 = t1.y as isize + dy as isize;

            res.push((x0, y0));
            res.push((x1, y1));
        }
        (Ordering::Less, Ordering::Greater) => {
            let x0 = t0.x as isize - dx as isize;
            let x1 = t1.x as isize + dx as isize;

            let y0 = t0.y as isize + dy as isize;
            let y1 = t1.y as isize - dy as isize;

            res.push((x0, y0));
            res.push((x1, y1));
        }

        (Ordering::Greater, Ordering::Less) => {
            let x0 = t0.x as isize + dx as isize;
            let x1 = t1.x as isize - dx as isize;

            let y0 = t0.y as isize - dy as isize;
            let y1 = t1.y as isize + dy as isize;

            res.push((x0, y0));
            res.push((x1, y1));
        }
        (Ordering::Greater, Ordering::Greater) => {
            let x0 = t0.x as isize + dx as isize;
            let x1 = t1.x as isize - dx as isize;

            let y0 = t0.y as isize + dy as isize;
            let y1 = t1.y as isize - dy as isize;

            res.push((x0, y0));
            res.push((x1, y1));
        }
        (_, _) => unreachable!(),
    }

    res.iter()
        .filter_map(|(x, y)| {
            if *x >= 0 && *x < max_x as isize && *y >= 0 && *y < max_y as isize {
                return Some((*x as usize, *y as usize));
            }
            None
        })
        .collect()
}

fn find_antinodes_t(t0: &Tower, t1: &Tower, max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
    let dx = t1.x.abs_diff(t0.x);
    let dy = t1.y.abs_diff(t0.y);

    let mut res: Vec<(isize, isize)> = vec![];

    res.push((t0.x as isize, t0.y as isize));
    res.push((t1.x as isize, t1.y as isize));

    match (t0.x.cmp(&t1.x), t0.y.cmp(&t1.y)) {
        (Ordering::Equal, Ordering::Less) => {
            let mut y0 = t0.y as isize - dy as isize;
            while y0 >= 0 {
                res.push((t0.x as isize, y0));
                y0 -= dy as isize;
            }

            let mut y1 = t1.y as isize + dy as isize;
            while y1 < max_y as isize {
                res.push((t0.x as isize, y1));
                y1 += dy as isize;
            }
        }
        (Ordering::Equal, Ordering::Greater) => {
            let mut y0 = t1.y as isize - dy as isize;
            while y0 >= 0 {
                res.push((t0.x as isize, y0));
                y0 -= dy as isize;
            }

            let mut y1 = t0.y as isize + dy as isize;
            while y1 < max_y as isize {
                res.push((t0.x as isize, y1));
                y1 += dy as isize;
            }
        }

        (Ordering::Less, Ordering::Equal) => {
            let mut x0 = t0.x as isize - dx as isize;
            while x0 >= 0 {
                res.push((x0, t0.y as isize));
                x0 -= dx as isize;
            }

            let mut x1 = t1.x as isize + dx as isize;

            while x1 < max_x as isize {
                res.push((x1, t0.y as isize));
                x1 += dx as isize;
            }
        }
        (Ordering::Greater, Ordering::Equal) => {
            let mut x0 = t1.x as isize - dx as isize;

            while x0 >= 0 {
                res.push((x0, t0.y as isize));
                x0 -= dx as isize;
            }

            let mut x1 = t0.x as isize + dx as isize;
            while x1 < max_x as isize {
                res.push((x1, t0.y as isize));
                x1 += dx as isize;
            }
        }

        (Ordering::Less, Ordering::Less) => {
            let mut x0 = t0.x as isize - dx as isize;
            let mut y0 = t0.y as isize - dy as isize;

            while x0 >= 0 && y0 >= 0 {
                res.push((x0, y0));
                x0 -= dx as isize;
                y0 -= dy as isize;
            }

            let mut x1 = t1.x as isize + dx as isize;
            let mut y1 = t1.y as isize + dy as isize;

            while x1 < max_x as isize && y1 < max_y as isize {
                res.push((x1, y1));
                x1 += dx as isize;
                y1 += dy as isize;
            }
        }
        (Ordering::Less, Ordering::Greater) => {
            let mut x0 = t0.x as isize - dx as isize;
            let mut y0 = t0.y as isize + dy as isize;

            while x0 >= 0 && y0 < max_y as isize {
                res.push((x0, y0));
                x0 -= dx as isize;
                y0 += dy as isize;
            }

            let mut x1 = t1.x as isize + dx as isize;
            let mut y1 = t1.y as isize - dy as isize;

            while x1 < max_x as isize && y1 >= 0 {
                res.push((x1, y1));
                x1 += dx as isize;
                y1 -= dy as isize;
            }
        }

        (Ordering::Greater, Ordering::Less) => {
            let mut x0 = t0.x as isize + dx as isize;
            let mut y0 = t0.y as isize - dy as isize;

            while x0 < max_x as isize && y0 >= 0 {
                res.push((x0, y0));
                x0 += dx as isize;
                y0 -= dy as isize;
            }

            let mut x1 = t1.x as isize - dx as isize;
            let mut y1 = t1.y as isize + dy as isize;

            while x1 >= 0 && y1 < max_y as isize {
                res.push((x1, y1));
                x1 -= dx as isize;
                y1 += dy as isize;
            }
        }
        (Ordering::Greater, Ordering::Greater) => {
            let mut x0 = t0.x as isize + dx as isize;
            let mut y0 = t0.y as isize + dy as isize;

            while x0 < max_x as isize && y0 < max_y as isize {
                res.push((x0, y0));
                x0 += dx as isize;
                y0 += dy as isize;
            }

            let mut x1 = t1.x as isize - dx as isize;
            let mut y1 = t1.y as isize - dy as isize;

            while x1 >= 0 && y1 >= 0 {
                res.push((x1, y1));
                x1 -= dx as isize;
                y1 -= dy as isize;
            }
        }
        (_, _) => unreachable!(),
    }

    res.iter()
        .filter_map(|(x, y)| {
            if *x >= 0 && *x < max_x as isize && *y >= 0 && *y < max_y as isize {
                return Some((*x as usize, *y as usize));
            }
            None
        })
        .collect()
}

fn all_pairs(ts: &[Tower]) -> Vec<(&Tower, &Tower)> {
    ts.iter().combinations(2).map(|v| (v[0], v[1])).collect()
}

pub fn uniq_antinodes(strs: &[&str]) -> HashSet<(usize, usize)> {
    let max_x = strs[0].len();
    let max_y = strs.len();
    let ts = find_towers(strs);

    let mut uniq_antitowers: HashSet<(usize, usize)> = HashSet::new();

    ts.values().for_each(|x| {
        let ps = all_pairs(x);
        ps.iter().for_each(|(t0, t1)| {
            let a = find_antinodes(t0, t1, max_x, max_y);
            //println!("for pair: {} {}", t0, t1);
            //println!("antinodes: {:?}", a);
            for p in a {
                uniq_antitowers.insert(p);
            }
        });
    });

    uniq_antitowers
}

pub fn uniq_antinodes_t(strs: &[&str]) -> HashSet<(usize, usize)> {
    let max_x = strs[0].len();
    let max_y = strs.len();
    let ts = find_towers(strs);

    let mut uniq_antitowers: HashSet<(usize, usize)> = HashSet::new();

    ts.values().for_each(|x| {
        let ps = all_pairs(x);
        ps.iter().for_each(|(t0, t1)| {
            let a = find_antinodes_t(t0, t1, max_x, max_y);
            //println!("for pair: {} {}", t0, t1);
            //println!("antinodes: {:?}", a);
            for p in a {
                uniq_antitowers.insert(p);
            }
        });
    });

    uniq_antitowers
}

pub fn solve_part1(strs: &[&str]) -> u32 {
    uniq_antinodes(strs).len() as u32
}

pub fn solve_part2(strs: &[&str]) -> u32 {
    uniq_antinodes_t(strs).len() as u32
}

#[cfg(test)]
mod tests {
    use std::str::from_utf8;

    use super::*;

    const INPUT: &str = "
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............
    ";

    const EMPTY: &str = "
        ............
        ............
        ............
        ............
        ............
        ............
        ............
        ............
        ............
        ............
        ............
        ............

    ";

    const EXPECTED: &str = "
        ......#....#
        ...#........
        ....#.....#.
        ..#.........
        .........#..
        .#....#.....
        ...#........
        #......#....
        ............
        ............
        ..........#.
        ..........#.
    ";

    /*
        ............
        ............
        ............
        ............
        ............
        ......A.....
        ............
        ............
        ............
        .........A..
        ............
        ............
    */
    fn read_input(inp: &str) -> Vec<&str> {
        inp.lines()
            .map(str::trim)
            .filter(|x| !x.is_empty())
            .collect()
    }

    #[test]
    fn test_find_antinodes() {
        let input = read_input(INPUT);
        let max_x = input[0].len();
        let max_y = input.len();
        let t0 = Tower {
            ch: b'0',
            x: 5,
            y: 2,
        };
        let t1 = Tower {
            ch: b'0',
            x: 7,
            y: 3,
        };
        let r = find_antinodes(&t0, &t1, max_x, max_y);
    }

    #[test]
    fn test_special() {
        let input = read_input(INPUT);
        let max_x = input[0].len();
        let max_y = input.len();
        let t0 = Tower {
            ch: b'0',
            x: 6,
            y: 5,
        };
        let t1 = Tower {
            ch: b'0',
            x: 9,
            y: 9,
        };
        let r = find_antinodes(&t0, &t1, max_x, max_y);
        dbg!(r);
    }

    #[test]
    fn test_part1_example() {
        let input = read_input(INPUT);
        let ts = uniq_antinodes(&input);

        let emp = read_input(EMPTY);
        let mut emp: Vec<Vec<u8>> = emp.iter().map(|x| Vec::from(x.as_bytes())).collect();

        let ex = read_input(EXPECTED);

        for an in ts.iter() {
            emp[an.1][an.0] = b'#';
        }

        for (line, exline) in emp.iter().zip(ex) {
            let line = from_utf8(line).unwrap();
            println!("{} {}", line, exline);
            //assert_eq!(line, exline);
        }
    }

    #[test]
    fn test_part1() {
        let input = read_input(INPUT);
        let res = solve_part1(&input);
        assert_eq!(res, 14);
    }

    #[test]
    fn test_part2() {
        let input = read_input(INPUT);
        let res = solve_part2(&input);
        assert_eq!(res, 34);
    }
}
