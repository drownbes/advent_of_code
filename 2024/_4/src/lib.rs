fn count_line(line: &str) -> u32 {
    let mut count = 0;
    if line.eq("XMAS") {
        count += 1;
    }
    if line.eq("SAMX") {
        count += 1;
    }
    count
}

fn count_in_window(window: Vec<&str>) -> u32 {
    //println!("scan window: {:#?}", window);

    let mut count: u32 = 0;

    //check diagonal \

    let line: Vec<u8> = (0..=3).map(|i| window[i].as_bytes()[i]).collect();
    let line = std::str::from_utf8(&line).expect("Failed to read as str");

    //println!("checking diagonal line \\: {} fount: {}", line, count);
    count += count_line(line);

    //check diagonal /
    let line: Vec<u8> = (0..=3).map(|i| window[i].as_bytes()[3 - i]).collect();
    let line = std::str::from_utf8(&line).expect("Failed to read as str");
    //println!("checking diagonal line /: {} fount: {}", line, count);
    count += count_line(line);

    count
}

fn count_str(str: &str) -> u32 {
    let mut count = 0;
    for x in 0..str.len() - 3 {
        let line = &str[x..x + 4];
        count += count_line(line);
    }
    count
}

fn count_horizontal(strs: &[&str]) -> u32 {
    let mut count = 0;
    for line in strs {
        count += count_str(line);
    }
    count
}

fn count_vertical(strs: &[&str]) -> u32 {
    let mut count = 0;
    let max_x = strs[0].len();
    for x in 0..max_x {
        let line: Vec<u8> = strs.iter().map(|str| str.as_bytes()[x]).collect();
        let line = std::str::from_utf8(&line).expect("Failed to read as str");
        count += count_str(line);
    }
    count
}

fn count_diagonal(strs: &[&str]) -> u32 {
    //dbg!(strs);
    let mut count = 0;
    for y in 0..strs.len() - 3 {
        let max_x = strs[y].len();
        //dbg!(max_x);
        for x in 0..max_x - 3 {
            let window: Vec<&str> = (y..y + 4).map(|k| &strs[k][x..x + 4]).collect();
            count += count_in_window(window);
        }
    }

    count
}

pub fn solve_part1(strs: &[&str]) -> u32 {
    count_horizontal(strs) + count_vertical(strs) + count_diagonal(strs)
}

fn check_mas(line: &str) -> bool {
    line.eq("MAS") || line.eq("SAM")
}

fn check_for_xmas(window: Vec<&str>) -> bool {
    //println!("scan window: {:#?}", window);

    //check diagonal \

    let line: Vec<u8> = (0..=2).map(|i| window[i].as_bytes()[i]).collect();
    let line = std::str::from_utf8(&line).expect("Failed to read as str");

    //println!("checking diagonal line \\: {} fount: {}", line, count);

    let diag0 = check_mas(line);

    //check diagonal /
    let line: Vec<u8> = (0..=2).map(|i| window[i].as_bytes()[2 - i]).collect();
    let line = std::str::from_utf8(&line).expect("Failed to read as str");
    //println!("checking diagonal line /: {} fount: {}", line, count);

    let diag1 = check_mas(line);

    diag0 && diag1
}

pub fn solve_part2(strs: &[&str]) -> u32 {
    let mut count = 0;
    for y in 0..strs.len() - 2 {
        let max_x = strs[y].len();
        //dbg!(max_x);
        for x in 0..max_x - 2 {
            let window: Vec<&str> = (y..y + 3).map(|k| &strs[k][x..x + 3]).collect();
            if check_for_xmas(window) {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
    ";

    const INPUT2: &str = "
       .M.S......
       ..A..MSMS.
       .M.S.MAA..
       ..A.ASMSM.
       .M.S.M....
       ..........
       S.S.S.S.S.
       .A.A.A.A..
       M.M.M.M.M.
       ..........
    ";

    #[test]
    fn test_part1() {
        let input: Vec<&str> = INPUT
            .lines()
            .map(str::trim)
            .filter(|x| !x.is_empty())
            .collect();

        println!("{:?}", input);

        let res = solve_part1(&input);

        println!("{}", res);

        assert_eq!(res, 18);
    }

    #[test]
    fn test_part2() {
        let input: Vec<&str> = INPUT2
            .lines()
            .map(str::trim)
            .filter(|x| !x.is_empty())
            .collect();

        println!("{:?}", input);

        let res = solve_part2(&input);

        println!("{}", res);

        assert_eq!(res, 9);
    }
}
