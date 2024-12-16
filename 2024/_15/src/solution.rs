use std::{
    collections::{HashSet, LinkedList},
};

#[derive(Debug)]
enum Mov {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Mov {
    fn from(c: char) -> Self {
        match c {
            '^' => Mov::Up,
            '<' => Mov::Left,
            '>' => Mov::Right,
            'v' => Mov::Down,
            _ => unreachable!(),
        }
    }
}

impl From<Mov> for (isize, isize) {
    fn from(c: Mov) -> Self {
        match c {
            Mov::Up => (0, -1),
            Mov::Left => (-1, 0),
            Mov::Right => (1, 0),
            Mov::Down => (0, 1),
        }
    }
}

fn parse_input(strs: &[&str]) -> World {
    let mut s = strs.split(|x| x.is_empty());
    let grid: Vec<Vec<char>> = s
        .next()
        .unwrap()
        .iter()
        .map(|x| x.chars().collect())
        .collect();
    let moves: LinkedList<Mov> = s
        .next()
        .unwrap()
        .join("")
        .chars()
        .map(|x| x.into())
        .collect();
    let (x, y) = find_robot(&grid);

    World {
        r: Pos { x, y },
        grid,
        moves,
    }
}

#[derive(Debug)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct World {
    r: Pos,
    grid: Vec<Vec<char>>,
    moves: LinkedList<Mov>,
}

#[derive(Debug)]
struct WorldX2 {
    r: Pos,
    grid: Vec<Vec<char>>,
    moves: LinkedList<Mov>,
}

fn find_robot(grid: &Vec<Vec<char>>) -> (usize, usize) {
    grid.iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter()
                .enumerate()
                .find_map(|(x, c)| if *c == '@' { Some((x, y)) } else { None })
        })
        .expect("Cannot find robot")
}

fn x2_world(w: World) -> WorldX2 {
    let w2_grid: Vec<Vec<char>> = w
        .grid
        .iter()
        .map(|line| {
            line.iter()
                .flat_map(|c| match c {
                    '@' => vec!['@', '.'],
                    'O' => vec!['[', ']'],
                    _ => vec![*c, *c],
                })
                .collect()
        })
        .collect();
    let (x, y) = find_robot(&w2_grid);

    WorldX2 {
        r: Pos { x, y },
        grid: w2_grid,
        moves: w.moves,
    }
}

trait HasGrid {
    fn get_grid(&self) -> &Vec<Vec<char>>;
}

trait Drawable: HasGrid {
    fn draw_grid(&self) {
        for line in self.get_grid().iter() {
            println!("{}", line.iter().collect::<String>());
        }
    }
}

trait Gps: HasGrid {
    const GPS_CHAR: char;
    fn get_gps(&self) -> usize {
        self.get_grid()
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .map(|(x, c)| if *c == Self::GPS_CHAR { 100 * y + x } else { 0 })
                    .sum::<usize>()
            })
            .sum()
    }
}

trait CanStep: Drawable {
    fn step(&mut self) -> Option<()>;
}

impl HasGrid for World {
    fn get_grid(&self) -> &Vec<Vec<char>> {
        &self.grid
    }
}

impl HasGrid for &mut World {
    fn get_grid(&self) -> &Vec<Vec<char>> {
        &self.grid
    }
}

impl Gps for World {
    const GPS_CHAR: char = 'O';
}

impl Drawable for World {}

impl CanStep for World {
    fn step(&mut self) -> Option<()> {
        let m = self.moves.pop_front()?;
        //println!("{:?}", m);
        let (shift_x, shift_y) = m.into();
        let (mut x, mut y) = (self.r.x, self.r.y);
        let mut stack: Vec<(usize, usize)> = vec![(self.r.x, self.r.y)];
        loop {
            (x, y) = (
                x.checked_add_signed(shift_x).unwrap(),
                y.checked_add_signed(shift_y).unwrap(),
            );
            let ch = self.grid[y][x];
            match ch {
                '#' => {
                    stack.clear();
                    break;
                }
                'O' => {
                    stack.push((x, y));
                }
                _ => {
                    break;
                }
            }
        }
        //dbg!(&stack);
        while let Some((x, y)) = stack.pop() {
            let (new_x, new_y) = (
                x.checked_add_signed(shift_x).unwrap(),
                y.checked_add_signed(shift_y).unwrap(),
            );
            if self.grid[y][x] == '@' {
                self.r.x = new_x;
                self.r.y = new_y;
            }
            self.grid[new_y][new_x] = self.grid[y][x];
            self.grid[y][x] = '.';
        }

        //self.draw_grid();

        Some(())
    }
}

impl HasGrid for WorldX2 {
    fn get_grid(&self) -> &Vec<Vec<char>> {
        &self.grid
    }
}

impl Drawable for WorldX2 {}

impl Gps for WorldX2 {
    const GPS_CHAR: char = '[';
}

impl WorldX2 {
    fn step_hor(&mut self) -> Option<()> {
        let m = self.moves.pop_front()?;
        //println!("{:?}", m);
        let (shift_x, shift_y) = m.into();
        let (mut x, mut y) = (self.r.x, self.r.y);
        let mut stack: Vec<(usize, usize)> = vec![(self.r.x, self.r.y)];
        loop {
            (x, y) = (
                x.checked_add_signed(shift_x).unwrap(),
                y.checked_add_signed(shift_y).unwrap(),
            );
            let ch = self.grid[y][x];
            match ch {
                '#' => {
                    stack.clear();
                    break;
                }
                '[' | ']' => {
                    stack.push((x, y));
                }
                _ => {
                    break;
                }
            }
        }
        //dbg!(&stack);
        while let Some((x, y)) = stack.pop() {
            let (new_x, new_y) = (
                x.checked_add_signed(shift_x).unwrap(),
                y.checked_add_signed(shift_y).unwrap(),
            );
            if self.grid[y][x] == '@' {
                self.r.x = new_x;
                self.r.y = new_y;
            }
            self.grid[new_y][new_x] = self.grid[y][x];
            self.grid[y][x] = '.';
        }

        //self.draw_grid();

        Some(())
    }

    fn step_vertical(&mut self) -> Option<()> {
        let m = self.moves.pop_front()?;
        let (_, shift_y) = m.into();
        let (x, mut y) = (self.r.x, self.r.y);
        let mut stack: Vec<Vec<(usize, usize)>> = vec![];
        stack.push(vec![(self.r.x, self.r.y)]);

        let new_y = y.checked_add_signed(shift_y).unwrap();
        let mut check_locs: Vec<Vec<(usize, usize)>> = vec![vec![(x, new_y)]];

        if self.grid[new_y][x] == '.' {
            self.grid[y][x] = '.';
            self.r.y = new_y;
            self.grid[new_y][x] = '@';
            return Some(());
        }

        'outer: while let Some(step_locs) = check_locs.pop() {
            let mut all_locs_free = true;
            let mut next_step_locs: HashSet<(usize, usize)> = HashSet::new();
            let mut locs_to_check : HashSet<(usize, usize)>= HashSet::from_iter(step_locs);

            let mut next_slice = HashSet::new();
            y = y.checked_add_signed(shift_y).unwrap();
            let mut d : Vec<char> = vec![];
            for l in locs_to_check.iter() {
                d.push(self.grid[l.1][l.0]);
            }
            while let Some(loc) = locs_to_check.iter().next().cloned() {
                locs_to_check.remove(&loc);
                let ch = self.grid[loc.1][loc.0];
                match ch {
                    '#' => {
                        stack.clear();
                        check_locs.clear();
                        break 'outer;
                    }
                    '[' => {
                        let next_y = loc.1.checked_add_signed(shift_y).unwrap();
                        next_step_locs.insert((loc.0, next_y));
                        next_slice.insert(loc);
                        let close = (loc.0 + 1, loc.1);
                        if !next_slice.contains(&close) {
                            locs_to_check.insert(close);
                        }
                        all_locs_free = false;
                    }
                    ']' => {
                        let next_y = loc.1.checked_add_signed(shift_y).unwrap();
                        next_step_locs.insert((loc.0, next_y));
                        next_slice.insert(loc);
                        let close = (loc.0 - 1, loc.1);
                        if !next_slice.contains(&close) {
                            locs_to_check.insert(close);
                        }
                        all_locs_free = false;
                    }
                    _ => {}
                }
            }
            if all_locs_free {
                break;
            }
            stack.push(next_slice.into_iter().collect());
            check_locs.push(next_step_locs.into_iter().collect());
        }
        if stack.len() > 1 {
            self.r.y = self.r.y.checked_add_signed(shift_y).unwrap();
        }
        while let Some(objs) = stack.pop() {
            for obj in objs {
                let new_y = obj.1.checked_add_signed(shift_y).unwrap();
                self.grid[new_y][obj.0] = self.grid[obj.1][obj.0];
                self.grid[obj.1][obj.0] = '.';
            }
        }
        Some(())
    }
}

impl CanStep for WorldX2 {
    fn step(&mut self) -> Option<()> {
        match self.moves.front() {
            Some(Mov::Right | Mov::Left) => self.step_hor(),
            Some(Mov::Up | Mov::Down) => self.step_vertical(),
            None => None,
        }
    }
}

pub fn solve_part1(strs: &[&str]) -> usize {
    let mut w = parse_input(strs);
    while w.step().is_some() {}
    w.get_gps()
}

pub fn solve_part2(strs: &[&str]) -> usize {
    let mut w = x2_world(parse_input(strs));
    let n = w.moves.len();
    for _ in 0..n {
        w.step();
    }
    w.get_gps()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_input(inp: &str) -> Vec<&str> {
        inp.strip_prefix("\n")
            .unwrap()
            .lines()
            .map(str::trim)
            .collect()
    }

    const EXAMPLE: &str = r#"
    ########
    #..O.O.#
    ##@.O..#
    #...O..#
    #.#.O..#
    #...O..#
    #......#
    ########
    
    <^^>>>vv<v>>v<<"#;

    const EXAMPLE_BIG: &str = r#"
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;

    const EXAMPLE_PART2: &str = r#"
########
#......#
#......#
#[]..[]#
#.[][].#
#..[]..#
#...@..#
########

^^^^^
"#;

    #[test]
    fn test_example() {
        let input = read_input(EXAMPLE);
        let mut w = parse_input(&input);
        w.draw_grid();
        while w.step().is_some() {}
        assert_eq!(w.get_gps(), 2028);
    }

    #[test]
    fn test_example_big() {
        let input = read_input(EXAMPLE_BIG);
        let mut w = parse_input(&input);
        w.draw_grid();
        while w.step().is_some() {}
        assert_eq!(w.get_gps(), 10092);
    }

    #[test]
    fn test_example_x2_small() {
        let input = read_input(EXAMPLE_PART2);
        let w = parse_input(&input);
        let mut w = WorldX2 {
            r: w.r,
            grid: w.grid,
            moves: w.moves,
        };

        while w.step().is_some() {
            w.draw_grid();
        }
        

        assert_eq!(w.get_gps(), 2028);
    }

    #[test]
    fn test_example_x2_big() {
        let input = read_input(EXAMPLE_BIG);
        let mut w = x2_world(parse_input(&input));
        w.draw_grid();
        while w.step().is_some() {
            w.draw_grid();
        }

        assert_eq!(w.get_gps(), 2028);
    }
}
