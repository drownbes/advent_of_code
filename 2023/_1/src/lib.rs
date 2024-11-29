fn as_number(x: u8) -> u8 {
    x - b'0'
}

pub fn solve_one(str: &str) -> u8 {
    let chs = str.as_bytes();
    let mut i = 0;
    let mut k = str.len() - 1;
    let mut res_a: Option<u8> = None;
    let mut res_b: Option<u8> = None;
    while res_a.is_none() || res_b.is_none() {
        let a = chs[i];
        let b = chs[k];
        if a.is_ascii_digit() {
            res_a = Some(as_number(a));
        }
        if b.is_ascii_digit() {
            res_b = Some(as_number(b));
        }
        if res_a.is_none() {
            i += 1;
        }
        if res_b.is_none() && k > 0 {
            k -= 1;
        }
    }

    res_a.unwrap() * 10 + res_b.unwrap()
}

pub fn solve(strs: Vec<&str>) -> u32 {
    strs.iter().map(|x| solve_one(x) as u32).sum()
}
