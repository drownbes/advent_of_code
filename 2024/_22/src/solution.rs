use std::{
    collections::{BTreeMap, HashMap, HashSet},
    isize, u64, usize,
};

fn mix(secret: u64, number: u64) -> u64 {
    number ^ secret
}

fn prune(secret: u64) -> u64 {
    secret % 16777216
}

fn next_secret(secret: u64) -> u64 {
    let s0 = secret * 64;
    let s0 = prune(mix(secret, s0));

    let s1 = s0 / 32;
    let s1 = prune(mix(s0, s1));

    let s2 = s1 * 2048;
    prune(mix(s1, s2))
}

fn calc_nth_secret(secret: u64, n: u64) -> u64 {
    if n == 0 {
        secret
    } else {
        let next_secret = next_secret(secret);
        calc_nth_secret(next_secret, n - 1)
    }
}

fn price_from_secret(secret: u64) -> isize {
    (secret % 10) as isize
}

fn prices(secret: u64, n: usize) -> Vec<(isize, isize)> {
    let mut prev_secret = secret;
    let mut prev_price = price_from_secret(prev_secret);
    let mut res = vec![];
    for _i in 1..n {
        let secret = next_secret(prev_secret);
        let price = price_from_secret(secret);
        let delta: isize = price - prev_price;
        res.push((price, delta));
        prev_secret = secret;
        prev_price = price;
    }
    res
}

#[derive(Debug, Clone)]
struct Prepare {
    seq_price_hm: HashMap<Vec<isize>, isize>,
}

impl Prepare {
    fn new() -> Prepare {
        Prepare {
            seq_price_hm: HashMap::new(),
        }
    }
}

fn prices_with_seqs(secret: u64) -> Prepare {
    let prs = prices(secret, 2000);
    prs.windows(4).fold(Prepare::new(), |mut acc, v| {
        let p = v[3];
        let seq: Vec<isize> = v.iter().map(|(_, d)| *d).collect();
        acc.seq_price_hm.entry(seq.clone()).or_insert_with(|| p.0);
        acc
    })
}

fn merge(p0: &Prepare, p1: &Prepare) -> Prepare {
    let mut res: HashMap<Vec<isize>, isize> = HashMap::new();

    for (seq, price) in p0.seq_price_hm.iter() {
        let mut sum = *price;
        //println!("price:{} seq:{}", price, seq);
        if let Some(price1) = p1.seq_price_hm.get(seq) {
            sum += price1;
        }

        res.insert(seq.clone(), sum);
    }

    for (seq, price) in p1.seq_price_hm.iter() {
        let mut sum = *price;
        //println!("price:{} seq:{}", price, seq);
        if let Some(price0) = p0.seq_price_hm.get(seq) {
            sum += price0;
        }

        res.insert(seq.clone(), sum);
    }

    Prepare { seq_price_hm: res }
}

pub fn solve_part1(strs: &[&str]) -> u64 {
    let secrets: Vec<u64> = strs.iter().map(|x| x.parse().unwrap()).collect();
    secrets.iter().map(|x| calc_nth_secret(*x, 2000)).sum()
}

pub fn solve_part2(strs: &[&str]) -> isize {
    let secrets: Vec<u64> = strs.iter().map(|x| x.parse().unwrap()).collect();

    let v: Vec<Prepare> = secrets
        .iter()
        .map(|secret| prices_with_seqs(*secret))
        .collect();

    let v = v.iter().fold(v[0].clone(), |acc, x| merge(&acc, x));

    let max = v.seq_price_hm.values().max();
    *max.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_secret() {
        let secret = 123;

        let next_secret = next_secret(secret);

        assert_eq!(next_secret, 15887950);

        let tenth_secret = calc_nth_secret(secret, 10);

        assert_eq!(tenth_secret, 5908254);
    }

    #[test]
    fn test_prices() {
        let secret = 1;
        let p0 = prices_with_seqs(1);
        let p1 = prices_with_seqs(2);
        let p2 = prices_with_seqs(3);
        let p4 = prices_with_seqs(2024);

        let max = merge(&p4, &merge(&p2, &merge(&p0, &p1)));

        let prs = max.seq_price_hm.values().max();

        dbg!(max.seq_price_hm.len());
        dbg!(prs);
    }
}
