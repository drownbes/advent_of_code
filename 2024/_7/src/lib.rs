use itertools::Itertools;
use rayon::prelude::*;

fn add(a: u64, b: u64) -> u64 {
    a + b
}

fn mul(a: u64, b: u64) -> u64 {
    a * b
}

fn concat(a: u64, b: u64) -> u64 {
    let nn = format!("{}{}", a, b);
    nn.parse().unwrap()
}

type OpFn = fn(u64, u64) -> u64;

fn check_possible<const N: usize>(value: u64, numbers: Vec<u64>, ops: [OpFn; N]) -> Option<u64> {
    let l = numbers.len() - 1;

    let variations: Vec<Vec<OpFn>> = (0..l).map(|_| ops).multi_cartesian_product().collect();

    for comb in variations {
        let mut res = numbers[0];
        for (i, op) in comb.iter().enumerate() {
            res = op(res, numbers[i + 1]);
        }
        if res == value {
            return Some(res);
        }
    }

    None
}

fn parse_puzzle(str: &str) -> (u64, Vec<u64>) {
    let mut s = str.split(":");
    let value: u64 = s
        .next()
        .expect("cannot find value")
        .parse()
        .expect("cannot parse value");
    let numbers: Vec<u64> = s
        .next()
        .unwrap()
        .trim()
        .split(" ")
        .map(str::trim)
        .map(|x| str::parse(x).unwrap())
        .collect();
    (value, numbers)
}

pub fn solve_part1(strs: &[&str]) -> u64 {
    strs.iter()
        .map(|x| parse_puzzle(x))
        .filter_map(|(v, ns)| check_possible(v, ns, [add, mul]))
        .sum()
}

pub fn solve_part2(strs: &[&str]) -> u64 {
    strs.par_iter()
        .map(|x| parse_puzzle(x))
        .filter_map(|(v, ns)| check_possible(v, ns, [add, mul, concat]))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20
    ";

    fn read_input() -> Vec<&'static str> {
        INPUT
            .lines()
            .map(str::trim)
            .filter(|x| !x.is_empty())
            .collect()
    }

    #[test]
    fn test_example_part1() {
        let input = read_input();

        let res = solve_part1(&input);

        assert_eq!(res, 3749);
    }

    #[test]
    fn test_example_part2() {
        let input = read_input();

        let res = solve_part2(&input);

        assert_eq!(res, 11387);
    }
}
