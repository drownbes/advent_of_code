use std::{isize, ops::RangeBounds, thread::sleep, time::Duration, usize};

#[derive(Debug, Clone)]
struct Robot {
    x: isize,
    y: isize,
    vx: isize,
    vy: isize,
}

fn parse_robot(s: &&str) -> Robot {
    let mut si = s.split_whitespace();
    let mut xy = si
        .next()
        .unwrap()
        .strip_prefix("p=")
        .unwrap()
        .split(",")
        .filter_map(|x| x.parse::<isize>().ok());
    let mut vxvy = si
        .next()
        .unwrap()
        .strip_prefix("v=")
        .unwrap()
        .split(",")
        .filter_map(|x| x.parse::<isize>().ok());

    Robot {
        x: xy.next().unwrap(),
        y: xy.next().unwrap(),

        vx: vxvy.next().unwrap(),
        vy: vxvy.next().unwrap(),
    }
}

pub fn simulate_robot(r: &Robot, n: usize, max_x: isize, max_y: isize) -> Robot {
    let mut x = r.x;
    let mut y = r.y;
    for _i in 0..n {
        x += r.vx;
        y += r.vy;
        if x < 0 {
            x += max_x;
        } else if x >= max_x {
            x -= max_x;
        }

        if y < 0 {
            y += max_y;
        } else if y >= max_y {
            y -= max_y;
        }
    }
    Robot {
        x,
        y,
        vx: r.vx,
        vy: r.vy,
    }
}

pub fn solve(robots: &[Robot], n: usize, max_x: isize, max_y: isize) -> isize {
    let new_robots: Vec<Robot> = robots
        .iter()
        .map(|r| simulate_robot(r, n, max_x, max_y))
        .collect();
    let per_q: Vec<isize> = new_robots.iter().fold(vec![0, 0, 0, 0], |mut acc, r| {
        match (r.x, r.y) {
            (x, y) if (0..max_x / 2).contains(&x) && (0..max_y / 2).contains(&y) => acc[0] += 1,
            (x, y) if (0..max_x / 2).contains(&x) && (max_y / 2 + 1..max_y).contains(&y) => {
                acc[1] += 1
            }
            (x, y) if (max_x / 2 + 1..max_x).contains(&x) && (0..max_y / 2).contains(&y) => {
                acc[2] += 1
            }
            (x, y)
                if (max_x / 2 + 1..max_x).contains(&x) && (max_y / 2 + 1..max_y).contains(&y) =>
            {
                acc[3] += 1
            }
            (_, _) => {}
        }
        acc
    });

    per_q.iter().product()
}

fn draw_robots(robots: &[Robot], max_x: usize, max_y: usize) {
    let mut world: Vec<Vec<char>> = (0..max_y)
        .map(|_i| std::iter::repeat('.').take(max_x).collect())
        .collect();
    for r in robots {
        world[r.y as usize][r.x as usize] = '#';
    }

    for line in world {
        println!("{}", String::from_iter(line));
    }
}

fn simulate_and_draw(robots: &[Robot], max_n: usize, max_x: isize, max_y: isize) {
    let mut new_robots: Vec<Robot> = robots.to_vec();
    for i in 0..max_n {
        println!("Gen {}", i);
        new_robots = new_robots
            .iter()
            .map(|r| simulate_robot(r, 1, max_x, max_y))
            .collect::<Vec<Robot>>();
        draw_robots(&new_robots, max_x as usize, max_y as usize);
    }
}

pub fn parse_robots(strs: &[&str]) -> Vec<Robot> {
    strs.iter().map(parse_robot).collect()
}

pub fn solve_part1(strs: &[&str]) -> isize {
    let robots = parse_robots(strs);
    solve(&robots, 100, 101, 103)
}

pub fn solve_part2(strs: &[&str]) -> usize {
    let robots = parse_robots(strs);
    simulate_and_draw(&robots, 10000, 101, 103);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE0: &str = "
        p=0,4 v=3,-3
        p=6,3 v=-1,-3
        p=10,3 v=-1,2
        p=2,0 v=2,-1
        p=0,0 v=1,3
        p=3,0 v=-2,-2
        p=7,6 v=-1,-3
        p=3,0 v=-1,-2
        p=9,3 v=2,3
        p=7,3 v=-1,2
        p=2,4 v=2,-3
        p=9,5 v=-3,-3
    ";

    fn read_input(inp: &str) -> Vec<&str> {
        inp.lines()
            .map(str::trim)
            .filter(|x| !x.is_empty())
            .collect()
    }

    #[test]
    fn test_name() {
        let input = read_input(EXAMPLE0);
        let robots = parse_robots(&input);

        let res = solve(&robots, 100, 11, 7);
        dbg!(res);
        assert_eq!(12, res);
    }

    #[test]
    fn test_single() {
        let robot = Robot {
            x: 7,
            y: 3,
            vx: -1,
            vy: 2,
        };

        draw_robots(&vec![robot.clone()], 11, 7);

        let new_robot = simulate_robot(&robot, 5, 11, 7);

        draw_robots(&vec![new_robot.clone()], 11, 7);
    }
}
