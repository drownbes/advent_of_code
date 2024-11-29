use std::{
    fs,
    io::{self, BufRead},
    path::Path,
};

use _2023_1::solve;

fn main() -> io::Result<()> {
    let dir = Path::new(file!()).parent().unwrap();
    let input_path = dir.join("../input");
    let input = fs::File::open(input_path)?;
    let r = io::BufReader::new(input).lines();
    let v: Vec<String> = r.map(|x| x.expect("Failed to read file")).collect();
    let v = v.iter().map(String::as_str).collect();
    let res = solve(v);
    println!("{}", res);
    Ok(())
}
