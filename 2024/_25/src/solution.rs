pub fn solve_part1(strs: &[&str]) -> usize {
    let p = parse_input(strs);
    let mut c = 0;
    for key in p.keys {
        for lock in p.locks.iter() {
            let fits = key.iter().zip(lock.iter()).all(|(k_p, l_p)| k_p + l_p <= 5);
            if fits {
                c += 1;
            }
        }
    }
    c
}

pub fn solve_part2(strs: &[&str]) -> usize {
    0
}

#[derive(Debug)]
struct Obj {
    pins: Vec<usize>,
    is_lock: bool,
}

fn parse_grid(strs: &[&str]) -> Obj {
    let first = strs.first().unwrap().chars().all(|c| c == '#');
    let n = strs[0].len();
    let mut pins = vec![];
    for i in 0..n {
        let mut c = 0;
        for line in strs {
            if line.chars().nth(i).unwrap() == '#' {
                c += 1;
            }
        }
        pins.push(c - 1);
    }
    if first {
        Obj {
            is_lock: true,
            pins,
        }
    } else {
        Obj {
            is_lock: false,
            pins,
        }
    }
}

#[derive(Debug)]
struct Parsed {
    keys: Vec<Vec<usize>>,
    locks: Vec<Vec<usize>>,
}

fn parse_input(strs: &[&str]) -> Parsed {
    let mut p = Parsed {
        keys: vec![],
        locks: vec![],
    };
    strs.split(|x| x.is_empty()).for_each(|x| {
        let obj = parse_grid(x);
        if obj.is_lock {
            p.locks.push(obj.pins);
        } else {
            p.keys.push(obj.pins);
        }
    });
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_input(inp: &str) -> Vec<&str> {
        inp.strip_prefix("\n")
            .unwrap()
            .lines()
            .map(str::trim)
            .collect()
    }

    const EXAMPLE: &str = "
        #####
        .####
        .####
        .####
        .#.#.
        .#...
        .....
        
        #####
        ##.##
        .#.##
        ...##
        ...#.
        ...#.
        .....
        
        .....
        #....
        #....
        #...#
        #.#.#
        #.###
        #####
        
        .....
        .....
        #.#..
        ###..
        ###.#
        ###.#
        #####
        
        .....
        .....
        .....
        #....
        #.#..
        #.#.#
        #####";

    #[test]
    fn test_solved() {
        let input = read_input(EXAMPLE);
        let r = solve_part1(&input);
        dbg!(r);
    }
}
