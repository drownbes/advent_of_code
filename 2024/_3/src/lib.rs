use regex::Regex;

fn parse_regex(s: &str) -> Vec<(u32, u32)> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let finds: Vec<(u32, u32)> = re
        .captures_iter(s)
        .map(|m| {
            let n0: u32 = m.get(1).unwrap().as_str().parse().unwrap();
            let n1: u32 = m.get(2).unwrap().as_str().parse().unwrap();
            (n0, n1)
        })
        .collect();
    finds
}

fn parse_program(s: &str) -> Vec<(u32, u32)> {
    let mut s = s;

    let mut res = vec![];
    while let Some(s_pos) = s.find("mul(") {
        s = &s[s_pos + 4..];

        //println!("After mul(: pos: {} left: {}", s_pos, s);

        let possible_number = if s.len() >= 4 { &s[0..4] } else { s };

        //println!("scanning possible number0 location: {}", possible_number);

        if let Some(comma_pos) = possible_number.find(",") {
            //println!("comma found at: {}", comma_pos);

            let n0 = &possible_number[0..comma_pos];

            //println!("possible number0: {}", n0);

            let n0: u32 = match n0.parse() {
                Ok(n) => n,
                _ => continue,
            };

            //println!("found number0: {}", n0);

            s = &s[comma_pos + 1..];

            //println!("After number0: {}", s);

            let possible_number = if s.len() >= 4 { &s[0..4] } else { s };

            //println!("scanning possible number1 location: {}", possible_number);

            if let Some(bracket_pos) = possible_number.find(")") {
                let n1 = &possible_number[0..bracket_pos];

                //println!("possible number1: {}", n1);

                let n1: u32 = match n1.parse() {
                    Ok(n) => n,
                    _ => continue,
                };

                //println!("found number1: {}", n1);

                s = &s[bracket_pos + 1..];

                //println!("After number1: {}", s);

                res.push((n0, n1));

                //println!("Found: {} x {}", n0, n1);
            }
        }
    }

    res
}

fn count_program(s: &str) -> u32 {
    let p = parse_regex(s);
    p.iter().map(|n| n.0 * n.1).sum()
}

pub fn solve_part1(str: &str) -> u32 {
    count_program(str)
}

fn get_enabled_blocks(s: &str) -> Vec<&str> {
    //println!("{}", s);
    let mut res: Vec<&str> = vec![];

    let mut s = s;

    let dont_pat = "don't()";
    let do_pat = "do()";

    let mut search_dont = true;

    while !s.is_empty() {
        if search_dont {
            if let Some(dont_pos) = s.find(dont_pat) {
                //println!("found don't at: {}", dont_pos);
                let en = &s[0..dont_pos];
                if !en.is_empty() {
                    res.push(en);
                }
                //println!("enabled part: {}", en);
                s = &s[dont_pos + dont_pat.len()..];
                //println!("left string: {}", s);
                search_dont = false;
            } else {
                //println!("no dont's left {}", s);
                if !s.is_empty() {
                    res.push(s);
                }
                break;
            }
        } else if let Some(do_pos) = s.find(do_pat) {
            //println!("found do at: {}", do_pos);
            let enable_start = do_pos + do_pat.len();
            s = &s[enable_start..];
            //println!("left string after do: {}", s);
            search_dont = true;
        } else {
            break;
        }
    }

    res
}

fn solve_one(str: &str) -> u32 {
    get_enabled_blocks(str)
        .iter()
        .map(|x| count_program(x))
        .sum()
}

pub fn solve_part2(str: &str) -> u32 {
    solve_one(str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_program() {
        let case: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let expected: Vec<(u32, u32)> = vec![(2, 4), (5, 5), (11, 8), (8, 5)];

        assert_eq!(parse_program(case), expected);
    }

    #[test]
    fn test_name() {
        let case: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let expected: u32 = 161;

        assert_eq!(solve_part1(case), expected);
    }

    #[test]
    fn test_get_enabled_blocks_0() {
        let case: &str =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let expected: Vec<&str> = vec!["xmul(2,4)&mul[3,7]!^", "?mul(8,5))"];

        assert_eq!(get_enabled_blocks(case), expected);
    }
    #[test]
    fn test_get_enabled_blocks_no_dont() {
        let case: &str = "xmul(2,4)&do()mul[3,7]!^mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let expected: Vec<&str> =
            vec!["xmul(2,4)&do()mul[3,7]!^mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"];

        assert_eq!(get_enabled_blocks(case), expected);
    }
    #[test]
    fn test_get_enabled_blocks_no_do_and_dont() {
        let case: &str = "xmul(2,4)&mul[3,7]!^mul(5,5)+mul(32,64](mul(11,8)un?mul(8,5))";

        let expected: Vec<&str> =
            vec!["xmul(2,4)&mul[3,7]!^mul(5,5)+mul(32,64](mul(11,8)un?mul(8,5))"];

        assert_eq!(get_enabled_blocks(case), expected);
    }
    #[test]
    fn test_get_enabled_blocks_dont_at_start() {
        let case: &str = "don't()xmul(2,4)&mul[3,7]!^mul(5,5)+mul(32,64](mul(11,8)un?mul(8,5))";

        let expected: Vec<&str> = vec![];

        assert_eq!(get_enabled_blocks(case), expected);
    }
    #[test]
    fn test_get_enabled_blocks_dont_at_end() {
        let case: &str = "xmul(2,4)&mul[3,7]!^mul(5,5)+mul(32,64](mul(11,8)un?mul(8,5))don't()";

        let expected: Vec<&str> =
            vec!["xmul(2,4)&mul[3,7]!^mul(5,5)+mul(32,64](mul(11,8)un?mul(8,5))"];

        assert_eq!(get_enabled_blocks(case), expected);
    }
    #[test]
    fn test_get_enabled_multiple() {
        let case: &str = "do()_mul(5,5)+mul(32,64]don't()(mul(11,8)undo()?mul(8,5))do()mul(11,8)";

        let expected: Vec<&str> = vec!["do()_mul(5,5)+mul(32,64]", "?mul(8,5))do()mul(11,8)"];

        dbg!(solve_one(case));

        assert_eq!(get_enabled_blocks(case), expected);
    }

    #[test]
    fn test_get_enabled_close() {
        let case: &str = "aaadon't()do()bbb";

        let expected: Vec<&str> = vec!["aaa", "bbb"];

        dbg!(solve_one(case));

        assert_eq!(get_enabled_blocks(case), expected);
    }

    #[test]
    fn test_get_enabled_close_2() {
        let case: &str = "aaado()don't()bbb";

        let expected: Vec<&str> = vec!["aaado()"];

        dbg!(solve_one(case));

        assert_eq!(get_enabled_blocks(case), expected);
    }

    #[test]
    fn test_final() {
        let case = include_str!("./case0.txt");
        parse_regex(case);

        println!("RESULT: {:#?}", get_enabled_blocks(case));
    }
    #[test]
    fn test_final_1() {
        let case = include_str!("./case6.txt");
        parse_regex(case);

        //println!("RESULT: {:#?}", get_enabled_blocks(case));
    }

    #[test]
    fn test_final_2() {
        let case = include_str!("./case1.txt");

        println!("RESULT: {:#?}", get_enabled_blocks(case));
    }

    #[test]
    fn test_final_3() {
        let case = include_str!("./case2.txt");

        println!("RESULT: {:#?}", get_enabled_blocks(case));
    }

    #[test]
    fn test_final_4() {
        let case = include_str!("./case3.txt");

        println!("RESULT: {:#?}", get_enabled_blocks(case));
    }

    #[test]
    fn test_final_5() {
        let case = include_str!("./case4.txt");

        println!("RESULT: {:#?}", get_enabled_blocks(case));
    }
}
