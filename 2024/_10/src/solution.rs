use std::collections::{HashSet, VecDeque};

fn as_number(x: u8) -> usize {
    (x - b'0') as usize
}

fn possible_moves(x: usize, y: usize, world: &[&str]) -> Vec<(usize, usize)> {
    let v = as_number(world[y].as_bytes()[x]);
    let x: isize = x as isize;
    let y: isize = y as isize;
    let max_x = world[0].len() as isize;
    let max_y = world.len() as isize;
    let moves = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    moves
        .iter()
        .filter_map(|(shift_x, shift_y)| {
            let new_x = x + shift_x;
            let new_y = y + shift_y;
            if (0..max_x).contains(&new_x) && (0..max_y).contains(&new_y) {
                let new_x = new_x as usize;
                let new_y = new_y as usize;
                if as_number(world[new_y].as_bytes()[new_x]) == v + 1 {
                    return Some((new_x, new_y));
                }
            }
            None
        })
        .collect()
}

fn trace_bfs(x: usize, y: usize, world: &[&str]) -> Vec<(usize, usize)> {
    //println!("trail head: ({}, {})", x, y);
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    queue.push_back((x, y));
    visited.insert((x, y));

    let mut nines: Vec<(usize, usize)> = vec![];

    while !queue.is_empty() {
        let l = queue.pop_front().unwrap();
        let v = as_number(world[l.1].as_bytes()[l.0]);
        if v == 9 {
            nines.push((l.0, l.1));
        }
        let mvs = possible_moves(l.0, l.1, world);
        for m in mvs {
            if !visited.contains(&m) {
                visited.insert(m);
                queue.push_back(m);
            }
        }
    }
    nines
}

fn path_hash(path: &Vec<(usize, usize)>) -> String {
    let v: Vec<String> = path
        .iter()
        .map(|p| format!("({},{}),", p.0, p.1).to_string())
        .collect();
    v.join(",")
}

fn count_uniq_paths(trail_head: (usize, usize), peak: (usize, usize), world: &[&str]) -> usize {
    //println!("({:?}) -> ({:?})", trail_head, peak);
    let mut stack: Vec<(usize, usize)> = vec![trail_head];
    let mut path = vec![];
    let mut uniq_paths: HashSet<String> = HashSet::new();
    let mut c = 0;
    while let Some(l) = stack.pop() {
        path.push(l);
        let mvs = possible_moves(l.0, l.1, world);
        if l == peak {
            uniq_paths.insert(path_hash(&path));
            //println!("path {:?}", path);
            c += 1;
            path.clear();
            path.push(trail_head);
        }

        for m in mvs {
            stack.push(m);
        }
    }
    c
}

pub fn solve_part1(strs: &[&str]) -> usize {
    let mut res = 0;
    for (y, line) in strs.iter().enumerate() {
        for (x, ch) in line.as_bytes().iter().enumerate() {
            if as_number(*ch) == 0 {
                let nines = trace_bfs(x, y, strs);
                res += nines.len();
            }
        }
    }
    res
}

pub fn solve_part2(strs: &[&str]) -> usize {
    let mut res = 0;
    for (y, line) in strs.iter().enumerate() {
        for (x, ch) in line.as_bytes().iter().enumerate() {
            if as_number(*ch) == 0 {
                let uniq_nines = trace_bfs(x, y, strs);
                let a: usize = uniq_nines
                    .iter()
                    .map(|n| count_uniq_paths((x, y), *n, strs))
                    .sum();
                //println!("{:?} {:?}", (x,y), uniq_nines);
                dbg!(a);
                res += a;
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732
    ";

    fn read_input(inp: &str) -> Vec<&str> {
        inp.lines()
            .map(str::trim)
            .filter(|x| !x.is_empty())
            .collect()
    }

    #[test]
    fn test_name() {
        let input = read_input(INPUT);
        let res = solve_part1(&input);
        assert_eq!(res, 36);
    }

    #[test]
    fn test_count_paths() {
        let input = read_input(INPUT);
        count_uniq_paths((2, 0), (0, 3), &input);
    }

    #[test]
    fn test_example1() {
        let input = read_input(INPUT);
        let res = solve_part2(&input);
        assert_eq!(res, 81);
    }
}
