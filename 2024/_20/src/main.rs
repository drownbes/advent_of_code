use std::{
    fs,
    io::{self, BufRead},
    path::Path,
};

mod solution;

use solution::{solve_part1, solve_part2};

fn main() -> io::Result<()> {
    let dir = Path::new(file!()).parent().unwrap();
    let input_path = dir.join("../input");
    let input = fs::File::open(input_path)?;
    let r = io::BufReader::new(input).lines();
    let v: Vec<String> = r.map(|x| x.expect("Failed to read file")).collect();
    let v: Vec<&str> = v.iter().map(String::as_str).collect();
    println!("Part1:");
    let res = solve_part1(&v);
    println!("{}", res);

    let res = solve_part2(&v);

    println!("Part2:");
    println!("{:?}", res);

    Ok(())
}
