use std::{
    fs,
    io::{self, BufRead},
    path::Path,
};

use _2023_2::{solve, solve_part2};

fn main() -> io::Result<()> {
    let dir = Path::new(file!()).parent().unwrap();
    let input_path = dir.join("../input");
    let input = fs::File::open(input_path)?;
    let r = io::BufReader::new(input).lines();
    let v: Vec<String> = r.map(|x| x.expect("Failed to read file")).collect();
    let v = v.iter().map(String::as_str).collect();
    let res = solve(&v);
    println!("Part 1:{}", res);
    let res = solve_part2(&v);
    println!("Part 2:{}", res);
    Ok(())
}
