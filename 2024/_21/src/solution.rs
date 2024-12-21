use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Eq, PartialEq)]
enum Press {
    Up,
    Left,
    Right,
    Down,
    A,
}

impl Press {
    fn get_shift(&self) -> (isize, isize) {
        match self {
            Press::Up => (0, -1),
            Press::Left => (-1, 0),
            Press::Right => (1, 0),
            Press::Down => (0, 1),
            Press::A => (0, 0),
        }
    }

    fn get_press(from: (usize, usize), to: (usize, usize)) -> Press {
        let shift_x = to.0 as isize - from.0 as isize;
        let shift_y = to.1 as isize - from.1 as isize;

        match (shift_x, shift_y) {
            (0, -1) => Press::Up,
            (-1, 0) => Press::Left,
            (1, 0) => Press::Right,
            (0, 1) => Press::Down,
            _ => panic!("Illegal shift"),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Press::Up => '^',
            Press::Left => '<',
            Press::Right => '>',
            Press::Down => 'v',
            Press::A => 'A',
        }
    }
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<char>>,
    x_range: std::ops::Range<usize>,
    y_range: std::ops::Range<usize>,
}

impl Grid {
    fn from_grid(grid: Vec<Vec<char>>) -> Grid {
        let max_y = grid.len();
        let max_x = grid[0].len();
        let x_range = 0..max_x;
        let y_range = 0..max_y;
        Grid {
            grid,
            x_range,
            y_range,
        }
    }

    fn is_in_bounds(&self, x: usize, y: usize) -> bool {
        self.x_range.contains(&x) && self.y_range.contains(&y)
    }
    fn scan_iter(&self) -> impl Iterator<Item = (&char, usize, usize)> + '_ {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(y, line)| line.iter().enumerate().map(move |(x, ch)| (ch, x, y)))
    }

    fn get_char(&self, p: (usize, usize)) -> char {
        self.grid[p.1][p.0]
    }

    fn all_locs(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.scan_iter()
            .filter_map(|(ch, x, y)| if *ch != '#' { Some((x, y)) } else { None })
    }

    fn checked_shift(&self, from: &(usize, usize), press: &Press) -> Option<(usize, usize)> {
        let (shift_x, shift_y) = press.get_shift();
        let new_x = from.0.checked_add_signed(shift_x)?;
        let new_y = from.1.checked_add_signed(shift_y)?;
        if self.is_in_bounds(new_x, new_y) && (self.get_char((new_x, new_y)) != '#') {
            Some((new_x, new_y))
        } else {
            None
        }
    }

    fn possible_moves(
        &self,
        p: (usize, usize),
    ) -> impl Iterator<Item = (Press, (usize, usize))> + '_ {
        let moves = [Press::Up, Press::Down, Press::Left, Press::Right];
        moves
            .into_iter()
            .filter_map(move |dir| self.checked_shift(&p, &dir).map(|l| (dir, l)))
    }

    fn bfs_from(&self, from: (usize, usize)) -> HashMap<(usize, usize), Vec<(usize, usize)>> {
        let mut dist = HashMap::new();
        for loc in self.all_locs() {
            dist.insert(loc, usize::MAX);
        }

        let mut queue = VecDeque::new();

        let mut parents: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

        queue.push_back(from);
        dist.insert(from, 0);

        while let Some(p) = queue.pop_front() {
            for new_p in self.possible_moves(p) {
                let old_dist = *dist.get(&p).unwrap();
                let new_dist = *dist.get(&new_p.1).unwrap();
                if new_dist > old_dist + 1 {
                    dist.insert(new_p.1, old_dist + 1);
                    queue.push_back(new_p.1);
                    parents.insert(new_p.1, vec![p]);
                } else if new_dist == old_dist + 1 {
                    parents
                        .entry(new_p.1)
                        .and_modify(|x| x.push(p))
                        .or_insert(vec![p]);
                }
            }
        }
        parents
    }

    fn find_paths(
        path: &mut Vec<(usize, usize)>,
        paths: &mut Vec<Vec<(usize, usize)>>,
        parents: &HashMap<(usize, usize), Vec<(usize, usize)>>,
        from: (usize, usize),
        to: (usize, usize),
    ) {
        if from == to {
            let mut p = path.to_vec();
            p.push(from);
            p.reverse();
            paths.push(p);
            return;
        }

        for p in parents.get(&to).unwrap() {
            path.push(to);

            Self::find_paths(path, paths, parents, from, *p);

            path.pop();
        }
    }

    fn shortest_paths(&self, from: (usize, usize), to: (usize, usize)) -> Vec<Vec<(usize, usize)>> {
        let parents = self.bfs_from(from);

        let mut paths = vec![];
        let mut path = vec![];
        Self::find_paths(&mut path, &mut paths, &parents, from, to);
        paths
    }

    fn path_to_moves(path: Vec<(usize, usize)>) -> Vec<Press> {
        path.windows(2)
            .map(|v| Press::get_press(v[0], v[1]))
            .collect()
    }
}

#[derive(Debug)]
struct Pad {
    nav_table: HashMap<char, HashMap<char, Vec<Vec<char>>>>,
}

impl Pad {
    fn new(m_str: &str) -> Pad {
        let grid = Grid::from_grid(Self::read_m_str(m_str));
        let char_hm = Self::do_char_map(&grid);
        let nav_table = Self::do_nav_table(&grid, &char_hm);
        Pad { nav_table }
    }

    fn get_short_paths(&self, from: char, to: char) -> Vec<Vec<char>> {
        self.nav_table
            .get(&from)
            .unwrap()
            .get(&to)
            .unwrap()
            .to_vec()
    }

    fn read_m_str(m_str: &str) -> Vec<Vec<char>> {
        m_str
            .strip_prefix("\n")
            .unwrap()
            .lines()
            .map(str::trim)
            .map(|x| x.chars().collect())
            .collect()
    }

    fn do_char_map(grid: &Grid) -> HashMap<char, (usize, usize)> {
        let mut cm: HashMap<char, (usize, usize)> = HashMap::new();
        for (ch, x, y) in grid.scan_iter() {
            if *ch != '#' {
                cm.insert(*ch, (x, y));
            }
        }

        cm
    }

    fn find_short_paths(
        grid: &Grid,
        char_hm: &HashMap<char, (usize, usize)>,
        from: char,
        to: char,
    ) -> Vec<Vec<char>> {
        let from = char_hm.get(&from).unwrap();
        let to = char_hm.get(&to).unwrap();
        let pfs = grid.shortest_paths(*from, *to);
        pfs.into_iter()
            .map(|x| {
                let mut p = Grid::path_to_moves(x);
                p.push(Press::A);
                p.into_iter().map(|x| x.to_char()).collect()
            })
            .collect()
    }

    fn do_nav_table(
        grid: &Grid,
        char_hm: &HashMap<char, (usize, usize)>,
    ) -> HashMap<char, HashMap<char, Vec<Vec<char>>>> {
        let mut nav_table: HashMap<char, HashMap<char, Vec<Vec<char>>>> = HashMap::new();
        for a in char_hm.keys() {
            for b in char_hm.keys() {
                let pfs = Self::find_short_paths(grid, char_hm, *a, *b);
                if let Some(v) = nav_table.get_mut(a) {
                    v.insert(*b, pfs);
                } else {
                    let mut hm = HashMap::new();
                    hm.insert(*b, pfs);
                    nav_table.insert(*a, hm);
                }
            }
        }

        nav_table
    }
}

fn get_presses(
    pad: &Pad,
    dir_pad: &Pad,
    code: &Vec<char>,
    depth: usize,
    memo: &mut HashMap<String, usize>,
) -> usize {
    let hash = format!("{}_{}", String::from_iter(code), depth);

    if let Some(l) = memo.get(&hash) {
        return *l;
    }

    let mut cur_pos = 'A';
    let mut length = 0;

    for ch in code {
        let paths = pad.get_short_paths(cur_pos, *ch);
        if depth == 0 {
            length += paths[0].len();
        } else {
            length += paths
                .iter()
                .map(|p| get_presses(dir_pad, dir_pad, p, depth - 1, memo))
                .min()
                .unwrap();
        }
        cur_pos = *ch;
    }

    memo.insert(hash, length);
    length
}

fn solve(code: String, depth: usize) -> usize {
    let mut memo = HashMap::new();
    let num_pad = create_num_pad();
    let dir_pad = create_arrow_pad();
    let cc: Vec<char> = code.chars().collect();
    let l = get_presses(&num_pad, &dir_pad, &cc, depth, &mut memo);

    let mut code = code;
    code.pop();

    let n: usize = code.parse().unwrap();

    n * l
}

fn create_num_pad() -> Pad {
    let m_str = "
            789 
            456
            123
            #0A";

    Pad::new(m_str)
}

fn create_arrow_pad() -> Pad {
    let m_str = "
    #^A
    <v>";

    Pad::new(m_str)
}

pub fn solve_part1(strs: &[&str]) -> usize {
    strs.iter().map(|s| solve(s.to_string(), 2)).sum()
}

pub fn solve_part2(strs: &[&str]) -> usize {
    strs.iter().map(|s| solve(s.to_string(), 25)).sum()
}
