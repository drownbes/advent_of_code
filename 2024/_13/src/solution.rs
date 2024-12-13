#[derive(Debug)]
struct Button {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Game {
    a: Button,
    b: Button,
    prize: (i64, i64),
}

//Button A: X+94, Y+34
//Button B: X+22, Y+67

fn parse_button(s: &str) -> Button {
    let mut s: &str = s;
    let i = s.find("X+").expect("cannot find X+") + 2;
    s = &s[i..];
    let c = s.find(",").expect("cannot find comma");
    let x: i64 = s[..c].parse().expect("cannot parse x number");
    s = &s[c + 1..];
    let i = s.find("Y+").expect("cannot find Y+") + 2;
    let y: i64 = s[i..].parse().expect("cannot parse y number");
    Button { x, y }
}

fn parse_prize(s: &str) -> (i64, i64) {
    let mut s: &str = s;
    let i = s.find("X=").expect("cannot find X+") + 2;
    s = &s[i..];
    let c = s.find(",").expect("cannot find comma");
    let x: i64 = s[..c].parse().expect("cannot parse x number");
    s = &s[c + 1..];
    let i = s.find("Y=").expect("cannot find Y+") + 2;
    let y: i64 = s[i..].parse().expect("cannot parse y number");
    (x, y)
}

fn parse_game(game_str: &[&str]) -> Game {
    Game {
        a: parse_button(game_str[0]),
        b: parse_button(game_str[1]),
        prize: parse_prize(game_str[2]),
    }
}

fn solve_game(g: Game) -> i64 {
    let bn = (g.a.y * g.prize.0 - g.a.x * g.prize.1) / (g.a.y * g.b.x - g.a.x * g.b.y);
    let an = (g.prize.1 - g.b.y * bn) / g.a.y;
    if an * g.a.x + bn * g.b.x == g.prize.0 && an * g.a.y + bn * g.b.y == g.prize.1 {
        an * 3 + bn
    } else {
        0
    }
}

pub fn solve_part1(strs: &[&str]) -> i64 {
    strs.split(|s| s.is_empty())
        .map(parse_game)
        .map(solve_game)
        .sum()
}

pub fn solve_part2(strs: &[&str]) -> i64 {
    strs.split(|s| s.is_empty())
        .map(|s| {
            let mut g = parse_game(s);
            g.prize.0 += 10000000000000;
            g.prize.1 += 10000000000000;
            solve_game(g)
        })
        .sum()
}
