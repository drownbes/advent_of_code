use std::collections::{HashMap, HashSet};

fn build_order_rules(strs: &[&str]) -> HashMap<u32, HashSet<u32>> {
    let mut m: HashMap<u32, HashSet<u32>> = HashMap::new();

    for line in strs {
        let mut iter = line.split("|").map(|x| x.parse().expect("Failed to parse"));
        let before: u32 = iter.next().expect("no before number");
        let after: u32 = iter.next().expect("no after number");
        if let Some(v) = m.get_mut(&before) {
            v.insert(after);
        } else {
            m.insert(before, HashSet::from([after]));
        }
    }
    m
}

fn check(rules: &HashMap<u32, HashSet<u32>>, page: &str) -> Option<u32> {
    let page: Vec<u32> = page
        .split(",")
        .map(|x| x.parse().expect("failed to parse page number"))
        .collect();

    //println!("checking page: {:?}", page);

    for i in 0..page.len() {
        let n = page[i];
        //check before
        for j in 0..i {
            let before = page[j];
            if let Some(r) = rules.get(&n) {
                if r.get(&before).is_some() {
                    return None;
                }
            }
        }

        for j in i..page.len() {
            let after = page[j];
            if let Some(r) = rules.get(&after) {
                if r.get(&n).is_some() {
                    return None;
                }
            }
        }
    }

    let res = page[page.len() / 2];

    //println!("ok middle: {}", res);

    Some(res)
}

fn check_wrong(rules: &HashMap<u32, HashSet<u32>>, page: &str) -> Option<u32> {
    if check(rules, page).is_some() {
        return None;
    }

    let page: Vec<u32> = page
        .split(",")
        .map(|x| x.parse().expect("failed to parse page number"))
        .collect();

    let mut page: Vec<(u32, usize)> = page
        .iter()
        .enumerate()
        .map(|(i, x)| {
            let a = &page[..i];
            let b = &page[i..];
            let v = [a, b].concat();
            if let Some(r) = rules.get(x) {
                let n = v.iter().filter_map(|l| r.get(l)).count();
                (*x, n)
            } else {
                (*x, 0)
            }
        })
        .collect();

    page.sort_by(|(_a, an), (_b, bn)| bn.cmp(an));

    let page: Vec<u32> = page.iter().map(|x| x.0).collect();

    let res = page[page.len() / 2];

    Some(res)
}

pub fn solve_part1(strs: &[&str]) -> u32 {
    let mut i = strs.split(|x| x.is_empty());
    let rules_str = i.next().expect("cannot find rules");
    let pages = i.next().expect("cannot find pages");

    let rules = build_order_rules(rules_str);

    let res = pages.iter().filter_map(|p| check(&rules, p)).sum();
    res
}

pub fn solve_part2(strs: &[&str]) -> u32 {
    let mut i = strs.split(|x| x.is_empty());
    let rules_str = i.next().expect("cannot find rules");
    let pages = i.next().expect("cannot find pages");

    let rules = build_order_rules(rules_str);

    let res = pages.iter().filter_map(|p| check_wrong(&rules, p)).sum();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_example() {
        let input: Vec<&str> = INPUT.lines().map(str::trim).collect();

        let res = solve_part1(&input);

        dbg!(res);
        assert_eq!(res, 143);
    }
    #[test]
    fn test_example_part2() {
        let input: Vec<&str> = INPUT.lines().map(str::trim).collect();

        let res = solve_part2(&input);

        dbg!(res);
        assert_eq!(res, 123);
    }
}
