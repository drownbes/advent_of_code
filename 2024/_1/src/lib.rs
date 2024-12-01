use std::collections::HashMap;

fn read_input(strs: &Vec<&str>) -> (Vec<u32>, Vec<u32>) {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];

    for line in strs {
        let mut s = line.split("   ");
        let left_d: u32 = s
            .next()
            .expect("Failed to split")
            .parse()
            .expect("Failed to parse");
        let right_d: u32 = s
            .next()
            .expect("Failed to split")
            .parse()
            .expect("Failed to parse");
        left.push(left_d);
        right.push(right_d);
    }
    (left, right)
}

pub fn solve_part1(strs: &Vec<&str>) -> u32 {
    let (mut left, mut right) = read_input(strs);
    let mut sum = 0;

    left.sort();
    right.sort();

    for (l, r) in left.iter().zip(right.iter()) {
        sum += l.abs_diff(*r);
    }

    sum
}

pub fn solve_part2(strs: &Vec<&str>) -> u32 {
    let (left, right) = read_input(strs);

    let right_times: HashMap<u32, u32> = right.iter().fold(HashMap::new(), |mut acc, n| {
        if let Some(v) = acc.get_mut(n) {
            *v += 1;
        } else {
            acc.insert(*n, 1);
        }
        acc
    });

    left.iter().fold(0, |acc, n| {
        if let Some(v) = right_times.get(n) {
            acc + n * v
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let case: Vec<&str> = vec!["3   4", "4   3", "2   5", "1   3", "3   9", "3   3"];

        let expected_res = 11;
        let actual_res = solve_part1(&case);

        assert_eq!(expected_res, actual_res);
    }

    #[test]
    fn test_part2() {
        let case: Vec<&str> = vec!["3   4", "4   3", "2   5", "1   3", "3   9", "3   3"];

        let expected_res = 31;
        let actual_res = solve_part2(&case);

        assert_eq!(expected_res, actual_res);
    }
}
