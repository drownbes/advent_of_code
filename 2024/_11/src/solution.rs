use std::{cell::RefCell, collections::HashMap, iter::repeat_n, rc::Rc};

use rayon::{iter::repeat, prelude::*};

fn blink(n: &u64) -> Vec<u64> {
    if *n == 0 {
        vec![1]
    } else if n.to_string().len() % 2 == 0 {
        let s = n.to_string();
        let (l, r) = s.split_at(s.len() / 2);
        vec![l.parse().unwrap(), r.parse().unwrap()]
    } else {
        vec![n * 2024]
    }
}

#[derive(Debug)]
struct Expand {
    v: u64,
    count: u64,
}

impl Expand {
    fn new(v: u64, count: u64) -> Expand {
        Expand { v, count }
    }
}

fn go(numbers: &Vec<u64>, memo: &mut HashMap<u64, Expand>) {
    let mut before: HashMap<u64, u64> = HashMap::new();
    for e in memo.values() {
        before.insert(e.v, e.count);
    }
    //dbg!(&before);
    for n in numbers {
        let v = blink(n);
        let cnt = before.get(n).unwrap();
        //println!("n:{} splits to: {:?} times:{}", n, v, cnt);
        for x in v {
            memo.entry(x)
                .and_modify(|e| e.count += cnt)
                .or_insert(Expand::new(x, *cnt));
        }
    }
    for (k, v) in before.iter() {
        memo.get_mut(k).unwrap().count -= v;
    }
}

fn solve(numbers: Vec<u64>, max_n: usize) -> u64 {
    let mut memo: HashMap<u64, Expand> = HashMap::new();
    for n in numbers {
        memo.insert(n, Expand::new(n, 1));
    }
    for i in 0..max_n {
        //dbg!(&memo);
        let ns: Vec<u64> = memo.values().filter(|x| x.count > 0).map(|x| x.v).collect();
        go(&ns, &mut memo);
        //let vs : Vec<u64> = memo.values().filter(|x| x.count > 0).flat_map(|e| repeat_n(e.v, e.count as usize).collect::<Vec<u64>>()).collect();
        //println!("gen:{} {:?} |{}| ", i+1, vs, vs.len())
    }

    //dbg!(&memo);
    memo.values().map(|x| x.count).sum()
}

pub fn solve_part1(str: &str) -> u64 {
    let numbers: Vec<u64> = str.split(" ").flat_map(|x| x.parse()).collect();
    solve(numbers, 25)
}

pub fn solve_part2(str: &str) -> u64 {
    let numbers: Vec<u64> = str.split(" ").flat_map(|x| x.parse()).collect();
    solve(numbers, 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "125 17";

    #[test]
    fn test_part1() {
        let res = solve_part1(INPUT);
        assert_eq!(res, 55312);
        dbg!(res);
    }

    #[test]
    fn test_one() {
        let res = solve(vec![125], 75);
        dbg!(res);
    }
}
