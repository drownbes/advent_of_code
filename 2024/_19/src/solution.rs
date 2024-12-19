use std::collections::{HashMap, VecDeque};

fn rec_match<'a>(s: &'a str, pats: &Vec<&'a str>, memo: &mut HashMap<&'a str, usize>) -> usize {
    if let Some(v) = memo.get(s) {
        return *v;
    }
    if s.is_empty() {
        1
    } else {
        let level = pats.iter().filter(|p| s.starts_with(*p));
        let c = level
            .map(|pat| rec_match(s.strip_prefix(pat).unwrap(), pats, memo))
            .sum();
        memo.insert(s, c);
        c
    }
}

fn rec_match_bool<'a>(s: &'a str, pats: &Vec<&'a str>) -> bool {
    let mut stack: Vec<&str> = vec![s];
    while let Some(s) = stack.pop() {
        if s.is_empty() {
            return true;
        }
        let level = pats.iter().filter(|p| s.starts_with(*p));
        for pat in level {
            stack.push(s.strip_prefix(pat).unwrap());
        }
    }
    false
}

fn solve_one(pats: &Vec<&str>, design: &str) -> usize {
    let pats: Vec<&str> = pats
        .iter()
        .filter(|p| design.contains(*p))
        .cloned()
        .collect();
    let mut memo = HashMap::new();
    rec_match(design, &pats, &mut memo)
}

fn solve_one_bool(pats: &Vec<&str>, design: &str) -> bool {
    let pats: Vec<&str> = pats
        .iter()
        .filter(|p| design.contains(*p))
        .cloned()
        .collect();
    rec_match_bool(design, &pats)
}

fn parse_input<'a>(strs: &'a [&'a str]) -> (Vec<&'a str>, Vec<&'a str>) {
    let mut s = strs.split(|x| x.is_empty());
    let patterns: Vec<&str> = s
        .next()
        .unwrap()
        .first()
        .unwrap()
        .split(",")
        .map(str::trim)
        .collect();
    let designs: Vec<&str> = s.next().unwrap().to_vec();
    (patterns, designs)
}

pub fn solve_part1(strs: &[&str]) -> usize {
    let (patterns, designs) = parse_input(strs);
    designs
        .iter()
        .filter(|des| solve_one_bool(&patterns, des))
        .count()
}

pub fn solve_part2(strs: &[&str]) -> usize {
    let (patterns, designs) = parse_input(strs);
    designs.iter().map(|des| solve_one(&patterns, des)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
r, wr, b, g, bwu, rb, gb, br

rrbgbr";

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
        let res = solve_part2(&input);
        dbg!(res);
        assert_eq!(res, 16);
    }
}
