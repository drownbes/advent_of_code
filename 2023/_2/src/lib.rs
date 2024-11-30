use std::cmp::max;

#[derive(PartialEq, Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl Round {
    fn empty() -> Round {
        Round {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

#[derive(PartialEq, Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

pub struct Bag {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

fn parse_round(str: &str) -> Round {
    str.split(",").fold(Round::empty(), |mut round, draw| {
        let r: Option<Round> = (|| {
            let mut s = draw.trim().split(" ");
            let n: u32 = s.next()?.parse().ok()?;
            match s.next()? {
                "blue" => round.blue = n,
                "red" => round.red = n,
                "green" => round.green = n,
                _ => {}
            }
            Some(round)
        })();
        r.expect("failed to parse draw")
    })
}

fn parse_game(str: &str) -> Option<Game> {
    let mut s = str.split(":");
    let id: u32 = s.next()?.split(" ").last()?.parse().ok()?;
    let rounds = s.next()?.split(";").map(parse_round).collect();
    Some(Game { id, rounds })
}

fn is_round_possible(bag: &Bag, round: &Round) -> bool {
    bag.red >= round.red && bag.blue >= round.blue && bag.green >= round.green
}

pub fn solve_one(bag: &Bag, str: &str) -> Option<u32> {
    let g = parse_game(str).expect("Failed to parse game");
    for round in g.rounds {
        if !is_round_possible(bag, &round) {
            return None;
        }
    }
    Some(g.id)
}

pub fn min_bag_power(str: &str) -> u32 {
    let g = parse_game(str).expect("Failed to parse game");

    let mut b = Bag {
        red: 0,
        green: 0,
        blue: 0,
    };
    for round in g.rounds {
        b.red = max(round.red, b.red);
        b.green = max(round.green, b.green);
        b.blue = max(round.blue, b.blue);
    }
    bag_power(&b)
}

fn bag_power(b: &Bag) -> u32 {
    b.red * b.green * b.blue
}

pub fn solve(strs: &Vec<&str>) -> u32 {
    let bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };
    strs.iter().filter_map(|s| solve_one(&bag, s)).sum()
}

pub fn solve_part2(strs: &Vec<&str>) -> u32 {
    strs.iter().map(|s| min_bag_power(s)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_game() {
        let game_str: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        let g = parse_game(game_str).unwrap();

        let expected = Game {
            id: 1,
            rounds: vec![
                Round {
                    red: 4,
                    blue: 3,
                    green: 0,
                },
                Round {
                    red: 1,
                    blue: 6,
                    green: 2,
                },
                Round {
                    red: 0,
                    blue: 0,
                    green: 2,
                },
            ],
        };

        assert_eq!(expected, g);
    }
}
