use std::{
    fs,
    io::{self},
    path::Path,
};

use _2024_3::{solve_part1, solve_part2};

fn main() -> io::Result<()> {
    let dir = Path::new(file!()).parent().unwrap();
    let input_path = dir.join("../input");
    let v = fs::read_to_string(input_path).unwrap();

    println!("Part1:");
    let res = solve_part1(v.as_str());
    println!("{}", res);

    println!("Part2:");
    let res = solve_part2(v.as_str());
    println!("{}", res);

    Ok(())
}
