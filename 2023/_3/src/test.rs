use std::collections::HashMap;

use _2023_3::{find_pub_numbers, solve};

fn main() {
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

    //let part_numbers : HashMap<u32,()> = HashMap::from([
    //    (114, ()),
    //    (58, ())
    //]);

    const FINAL_RESULT: u32 = 4361;
    //let found_part_numbers = find_pub_numbers(&test_case);

    let res = solve(&test_case);

    println!(
        "{} => final result expected: {} actual {}",
        res == FINAL_RESULT,
        FINAL_RESULT,
        res
    );
}
