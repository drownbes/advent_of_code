use core::str;

fn is_symbol(x: u8) -> bool {
    !x.is_ascii_digit() && x != b'.'
}

#[derive(Debug, Clone)]
struct FinishedPart {
    value: u32,
    line: usize,
    start_pos: usize,
    end_pos: usize,
}

#[derive(Debug, Clone)]
struct Symbol {
    line: usize,
    pos: usize,
}

impl Symbol {
    fn is_non_interesting(&self, line: usize, i: usize) -> bool {
        self.line < line && self.pos + 1 < i
    }
}

fn is_adjacent(p: &FinishedPart, s: &Symbol) -> bool {
    (p.start_pos <= s.pos && s.pos <= p.end_pos
        || p.end_pos + 1 == s.pos
        || s.pos + 1 == p.start_pos)
        && s.line.abs_diff(p.line) <= 1
}

#[derive(Debug)]
enum Obj {
    Symbol(Symbol),
    Finished(FinishedPart),
}

pub fn solve(strs: &Vec<&str>) -> u32 {
    let mut res: Vec<FinishedPart> = vec![];
    let mut prev_stack: Vec<Obj> = vec![];
    let mut stack: Vec<Obj> = vec![];
    for (line_n, line) in strs.iter().enumerate() {
        println!("{}", line);
        let mut i = 0;
        let s = line.as_bytes();

        stack.reverse();
        prev_stack = stack;
        stack = vec![];

        while i < line.len() {
            let c = s[i];
            let mut obj: Option<Obj> = None;

            if let Some(Obj::Symbol(s)) = prev_stack.last() {
                if s.is_non_interesting(line_n, i) {
                    println!("Droping non interesting symbol: {:?} at cursor at:{}", s, i);
                    prev_stack.pop();
                }
            }

            if c.is_ascii_digit() {
                let mut n = vec![];
                let start_pos = i;
                while i < line.len() && s[i].is_ascii_digit() {
                    n.push(s[i]);
                    i += 1;
                }
                i -= 1;
                let end_pos = i;
                let value: u32 = std::str::from_utf8(&n)
                    .expect("failed to read")
                    .parse()
                    .expect("Failed to parse");
                obj = Some(Obj::Finished(FinishedPart {
                    value,
                    line: line_n,
                    start_pos,
                    end_pos,
                }));
            } else if is_symbol(c) {
                obj = Some(Obj::Symbol(Symbol {
                    line: line_n,
                    pos: i,
                }));
            }

            if let Some(o) = obj {
                let mut do_push = true;

                println!("-----");
                println!("new obj: {:?}", o);
                println!("stack: {:?}", stack);
                println!("prev_stack: {:?}", prev_stack);

                match o {
                    Obj::Symbol(ref s) => {
                        // #12 || #..12
                        if let Some(Obj::Finished(p)) = stack.last() {
                            if is_adjacent(p, s) {
                                res.push(p.clone());
                                stack.pop();
                            }
                        }

                        while let Some(Obj::Finished(p)) = prev_stack.last() {
                            if is_adjacent(p, s) {
                                res.push(p.clone());
                                prev_stack.pop();
                            } else {
                                break;
                            }
                        }
                    }
                    Obj::Finished(ref p) => {
                        if let Some(Obj::Symbol(s)) = stack.last() {
                            if is_adjacent(p, s) {
                                res.push(p.clone());
                                do_push = false;
                            }
                        } else if let Some(Obj::Symbol(s)) = prev_stack.last() {
                            if is_adjacent(p, s) {
                                res.push(p.clone());
                                do_push = false;
                            }
                        }
                    }
                }
                if do_push {
                    stack.push(o);
                }

                println!("After logic");
                println!("stack: {:?}", stack);
                println!("prev_stack: {:?}", prev_stack);
                println!("res {:?}", res);
            }
            i += 1;
        }
    }
    dbg!(&res);

    res.iter().map(|v| v.value).sum()
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
    fn test_is_adjacent() {
        /*
         * ##123##
         * #######
         */

        let p = FinishedPart {
            value: 123,
            line: 0,
            start_pos: 2,
            end_pos: 4,
        };

        let lines = [
            (Symbol { line: 0, pos: 0 }, false),
            (Symbol { line: 0, pos: 1 }, true),
            (Symbol { line: 0, pos: 5 }, true),
            (Symbol { line: 0, pos: 6 }, false),
            (Symbol { line: 1, pos: 0 }, false),
            (Symbol { line: 1, pos: 1 }, true),
            (Symbol { line: 1, pos: 2 }, true),
            (Symbol { line: 1, pos: 3 }, true),
            (Symbol { line: 1, pos: 4 }, true),
            (Symbol { line: 1, pos: 5 }, true),
            (Symbol { line: 1, pos: 6 }, false),
        ];

        for (s, r) in lines {
            assert_eq!(is_adjacent(&p, &s), r);
        }
    }

    #[test]
    fn test_solve() {
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
        let res = solve(&test_case);

        assert_eq!(res, expected);
    }

    #[test]
    fn test_bottom() {
        let test_case: Vec<&str> = vec!["..25.13..", "....*...."];
        let expected = 38;
        let res = solve(&test_case);

        assert_eq!(res, expected);
    }

    #[test]
    fn test_between() {
        let test_case: Vec<&str> = vec!["..25*13..", "........."];
        let expected = 38;
        let res = solve(&test_case);

        assert_eq!(res, expected);
    }
    #[test]
    fn test_left() {
        let test_case: Vec<&str> = vec!["*25...", "*13..."];
        let expected = 38;
        let res = solve(&test_case);

        assert_eq!(res, expected);
    }

    #[test]
    fn test_right() {
        let test_case: Vec<&str> = vec!["...25*", "...13*"];
        let expected = 38;
        let res = solve(&test_case);

        assert_eq!(res, expected);
    }
    #[test]
    fn test_center() {
        let test_case: Vec<&str> = vec!["10.20", "..*...", "30.40."];
        let expected = 100;
        let res = solve(&test_case);

        assert_eq!(res, expected);
    }
    #[test]
    fn test_strange() {
        let test_case: Vec<&str> = vec!["....589..", "#..%....."];
        let expected = 100;
        let res = solve(&test_case);

        assert_eq!(res, expected);
    }

    #[test]
    fn test_part_of_input() {
        let test_case: Vec<&str> = vec![
            ".241....966...........%.........804...............589....554...307.+....#.505..........&....332...449.190......780........*322..........-540",
            ".................126.....837.@.........701.......%.......&......&.....581....*371.......753..-....................$....426.....=............",
            //"...........%.....*.......&....264.......*...................127....................562.......................379*...........786...#.-.......",
            //"....376...596......................%...511.551.........868....*....................&............224..............992............817.309....."
        ];
        let expected = 100;
        let res = solve(&test_case);

        assert_eq!(res, expected);
    }
}
