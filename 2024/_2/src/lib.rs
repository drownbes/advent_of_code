fn solve_report(str: &str) -> bool {
    let xs: Vec<u32> = str
        .split(" ")
        .map(|x| x.parse().expect("Failed to parse"))
        .collect();

    let xs: &[u32] = &xs;

    if xs.len() == 1 {
        return true;
    }

    let mut prev_n = xs[0];

    let mut i: usize = 1;

    let mut prev_order = prev_n > xs[1];

    while i < xs.len() {
        let diff_in_range = prev_n.abs_diff(xs[i]) >= 1 && prev_n.abs_diff(xs[i]) <= 3;
        if !diff_in_range {
            return false;
        }
        let new_order = prev_n > xs[i];

        if new_order != prev_order {
            return false;
        }
        prev_order = new_order;
        prev_n = xs[i];
        i += 1;
    }

    true
}

fn check_if_all_good(xs: &[u32]) -> bool {
    let mut i = 0;
    let mut prev_order = xs[0] < xs[1];
    while i < (xs.len() - 1) {
        let order = xs[i] < xs[i + 1];
        let diff_in_range = xs[i].abs_diff(xs[i + 1]) >= 1 && xs[i].abs_diff(xs[i + 1]) <= 3;
        if !diff_in_range || order != prev_order {
            return false;
        }
        prev_order = order;
        i += 1;
    }
    true
}

fn check_if_ok_without_one(xs: &[u32]) -> Option<usize> {
    for i in 0..xs.len() {
        let (l, r) = xs.split_at(i);
        let new_slice = [l, &r[1..]].concat();
        //println!("trying slice:{:?}", new_slice);

        if check_if_all_good(&new_slice) {
            return Some(i);
        }
    }
    None
}

fn solve_report_with_dampener(str: &str) -> bool {
    println!("case: {}", str);
    let xs: Vec<u32> = str
        .split(" ")
        .map(|x| x.parse().expect("Failed to parse"))
        .collect();

    let xs: &[u32] = &xs;

    if xs.len() <= 2 {
        return true;
    }

    if xs.len() == 3 && xs[0] != xs[1] && xs[1] != xs[2] {
        return true;
    }

    let mut skip_element: Option<usize> = None;

    let mut i = 0;
    while i + 3 < xs.len() {
        let mut arr = vec![xs[i], xs[i + 1], xs[i + 2], xs[i + 3]];

        //println!("check arr: {:?}", &arr);

        if let Some(skip) = skip_element {
            println!("skiping.. {:?} {} {}", arr, skip, i);
            if skip >= i && skip <= i + 3 {
                arr.remove(skip - i);
                println!("after skip:{:?}", &arr);
            }
        }

        let all_good = check_if_all_good(&arr);

        //println!("all_good: {:?} {}", &arr, all_good);

        if !all_good && skip_element.is_none() {
            if let Some(s) = check_if_ok_without_one(&arr) {
                println!("Possible to skip in {:?} {} i:{}", &arr, s, i);
                skip_element = Some(i + s);
            } else {
                return false;
            }
        } else if !all_good {
            return false;
        }

        i += 1;
    }

    if let Some(n) = skip_element {
        println!("saved: {} removing {:?}", str, xs[n]);
    }

    true
}

pub fn solve_part1(strs: &[&str]) -> u32 {
    strs.iter()
        .fold(0, |acc, s| if solve_report(s) { acc + 1 } else { acc })
}

pub fn solve_part2(strs: &[&str]) -> u32 {
    strs.iter().fold(0, |acc, s| {
        if solve_report_with_dampener(s) {
            acc + 1
        } else {
            println!("not ok:{}", s);
            acc
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASES: [(&str, bool); 6] = [
        ("7 6 4 2 1", true),
        ("1 2 7 8 9", false),
        ("9 7 6 2 1", false),
        ("1 3 2 4 5", false),
        ("8 6 4 4 1", false),
        ("1 3 6 7 9", true),
    ];

    const CASES_WITH_DAMPENER: [(&str, bool); 12] = [
        ("7 6 4 2 1", true),
        ("1 2 7 8 9", false),
        ("9 7 6 2 1", false),
        ("1 3 2 4 5", true),
        ("8 6 4 4 1", true),
        ("1 3 6 7 9", true),
        ("1 1 3 6 7 9", true),
        ("12 84 82 81 80", true),
        ("89 84 82 81 80", true),
        ("84 82 81 80 90", true),
        ("60 62 60 57 55", true),
        ("15 15 13 7 4 3 2", false),
    ];

    const FINAL_RESULT: u32 = 2;
    const FINAL_RESULT_WITH_DAMPENER: u32 = 9;

    #[test]
    fn test_example_cases() {
        for (s, r) in CASES {
            assert_eq!(solve_report(s), r);
        }

        assert_eq!(solve_part1(&CASES.map(|x| x.0)), FINAL_RESULT);
    }

    #[test]
    fn test_all_is_good() {
        assert!(check_if_all_good(&[7, 6, 4, 2, 1]));
        assert!(!check_if_all_good(&[8, 6, 4, 4, 1]));
    }

    #[test]
    fn test_special() {
        assert!(!solve_report_with_dampener("15 15 13 7 4 3 2"));
    }

    #[test]
    fn test_example_cases_with_with_dampener() {
        for (s, r) in CASES_WITH_DAMPENER {
            println!("case: {} {}", s, r);
            assert_eq!(solve_report_with_dampener(s), r);
        }

        assert_eq!(
            solve_part2(&CASES_WITH_DAMPENER.map(|x| x.0)),
            FINAL_RESULT_WITH_DAMPENER
        );
    }
}
