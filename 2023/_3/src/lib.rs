use core::str;
use std::collections::HashMap;

fn is_symbol(x: u8) -> bool {
    !x.is_ascii_digit() && x != b'.'
}

fn is_gear(x: u8) -> bool {
    x == b'*'
}

#[derive(Debug, Clone)]
struct Symbol {
    y: usize,
    x: usize,
    value: u8,
}

impl Symbol {
    fn is_gear(&self) -> bool {
        is_gear(self.value)
    }

    fn hash(&self) -> String {
        format!("{}_{}", self.x, self.y)
    }
}

#[derive(Debug, Clone)]
struct Number {
    value: u32,
    y: usize,
    start_pos: usize,
    end_pos: usize,
    syms: Vec<Symbol>,
}

fn check_surroundings(mut n: Number, grid: &[&str]) -> Option<Number> {
    //println!("checking number: {:?}", n);
    let max_x = grid[0].len() - 1;
    let max_y = grid.len() - 1;

    let s = if n.start_pos > 0 { n.start_pos - 1 } else { 0 };
    let e = if n.end_pos < max_x {
        n.end_pos + 1
    } else {
        max_x
    };

    let mut syms = vec![];

    //check above
    if n.y > 0 {
        for i in s..=e {
            let x = i;
            let y = n.y - 1;
            let value = grid[y].as_bytes()[x];
            if is_symbol(value) {
                syms.push(Symbol { x, y, value });
            }
        }
    }
    //check bellow
    if n.y < max_y {
        for i in s..=e {
            //println!("{} {}", n.y, max_y);
            let x = i;
            let y = n.y + 1;
            let value = grid[y].as_bytes()[x];
            if is_symbol(value) {
                syms.push(Symbol { x, y, value });
            }
        }
    }

    if n.start_pos > 0 {
        let x = n.start_pos - 1;
        let y = n.y;
        let value = grid[y].as_bytes()[x];
        if is_symbol(value) {
            syms.push(Symbol { x, y, value });
        }
    }

    if n.end_pos < max_x {
        let x = n.end_pos + 1;
        let y = n.y;
        let value = grid[y].as_bytes()[x];
        if is_symbol(value) {
            syms.push(Symbol { x, y, value });
        }
    }
    if syms.is_empty() {
        None
    } else {
        n.syms = syms;
        Some(n)
    }
}

fn find_parts(strs: &[&str]) -> Vec<Number> {
    let mut res: Vec<Number> = vec![];
    for (y, line) in strs.iter().enumerate() {
        let mut i = 0;
        while i < line.len() {
            let s = line.as_bytes();
            if s[i].is_ascii_digit() {
                let start_pos = i;
                let mut n = vec![];
                while i < line.len() && s[i].is_ascii_digit() {
                    n.push(s[i]);
                    i += 1;
                }
                let end_pos = i - 1;
                let value: u32 = std::str::from_utf8(&n)
                    .expect("failed to read")
                    .parse()
                    .expect("Failed to parse");

                let number = Number {
                    y,
                    start_pos,
                    end_pos,
                    value,
                    syms: vec![],
                };
                if let Some(n) = check_surroundings(number, strs) {
                    res.push(n);
                }
            };
            i += 1;
        }
    }
    res
}

pub fn solve_part1(strs: &[&str]) -> u32 {
    let res: Vec<Number> = find_parts(strs);
    res.iter().map(|x| x.value).sum()
}

pub fn solve_part2(strs: &[&str]) -> u32 {
    let res: Vec<Number> = find_parts(strs);

    let mut gears: HashMap<String, Vec<Number>> = HashMap::new();

    for n in res {
        for s in &n.syms {
            if s.is_gear() {
                if let Some(v) = gears.get_mut(&s.hash()) {
                    v.push(n.clone());
                } else {
                    gears.insert(s.hash(), Vec::from([n.clone()]));
                }
            }
        }
    }

    let r = gears
        .values()
        .filter(|v| v.len() == 2)
        .map(|g| g[0].value * g[1].value)
        .sum();
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_symbol() {
        let cases = [
            (b'*', true),
            (b'#', true),
            (b'+', true),
            (b'$', true),
            (b'2', false),
            (b'.', false),
            (b'@', true),
            (b'/', true),
            (b'%', true),
        ];

        for (s, e) in cases {
            assert_eq!(is_symbol(s), e)
        }
    }

    #[test]
    fn test_solve_part1() {
        let test_case: Vec<&str> = vec![
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ];
        let expected = 4361;
        let res = solve_part1(&test_case);

        assert_eq!(res, expected);
    }

    #[test]
    fn test_solve_part2() {
        let test_case: Vec<&str> = vec![
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ];
        let expected = 467835;
        let res = solve_part2(&test_case);

        assert_eq!(res, expected);
    }
}
