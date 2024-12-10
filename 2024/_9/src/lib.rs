use std::{fmt, iter, usize};

fn as_number(x: u8) -> usize {
    (x - b'0') as usize
}

fn seq_sum(start: usize, end: usize) -> usize {
    (end + 1 - start) * (start + end) / 2
}

fn is_file(n: usize) -> bool {
    n % 2 == 0
}

#[derive(Debug)]
struct File {
    content: usize,
    id: usize,
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self.content)
    }
}

fn checksum(id: usize, start: usize, end: usize) -> usize {
    id * seq_sum(start, end)
}

impl File {
    fn new(i: usize, size: usize) -> File {
        let id = i / 2;
        let content = size;
        File { id, content }
    }

    fn content_left(&self) -> usize {
        self.content
    }

    fn checksum(&self, start: usize) -> usize {
        checksum(self.id, start, start + self.content - 1)
    }

    fn move_n_digits(&mut self, start: usize, n: usize) -> usize {
        self.content -= n;
        checksum(self.id, start, start + n - 1)
    }
}

fn parse(s: &str) -> Vec<Option<usize>> {
    let mut is_file = true;
    let mut id = 0;
    let mut res = vec![];
    for b in s.trim().bytes() {
        let len = as_number(b);
        if is_file {
            res.extend(iter::repeat_n(Some(id), len));
            id += 1;
        } else {
            res.extend(iter::repeat_n(None, len));
        }
        is_file = !is_file;
    }
    res
}

pub fn p2(s: &str) -> usize {
    let mut nums = parse(s);
    let mut indices: Vec<(usize, Option<usize>)> = nums.clone().into_iter().enumerate().collect();
    let mut skip = 0;
    loop {
        let Some((right, len)) = indices
            .chunk_by(|a, b| a.1 == b.1)
            .filter(|p| p[0].1.is_some())
            .rev()
            .nth(skip)
            .map(|p| (p[0].0, p.len()))
        else {
            break;
        };
        if right == 0 {
            break;
        }
        let Some(left) = indices
            .chunk_by(|a, b| a.1 == b.1)
            .find(|p| p.len() >= len && p[0].1.is_none() && p[0].0 < right)
            .map(|p| p[0].0)
        else {
            skip += 1;
            continue;
        };
        for i in 0..len {
            nums.swap(left + i, right + i);
        }
        indices = nums.clone().into_iter().enumerate().collect();
    }
    nums.into_iter()
        .enumerate()
        .filter_map(|(i, opt)| opt.map(|v| v * i))
        .sum()
}

pub fn solve_part1(s: &str) -> usize {
    let s = s.as_bytes();
    let mut i: usize = 0; // skip first file
    let mut j: usize = s.len() - 1;
    let mut checksum: usize = 0;
    let mut virtual_i = 0;

    let mut free_memory_left = 0;
    let mut file_to_move = File::new(j, as_number(s[j]));

    while i < j && i != s.len() {
        if is_file(i) {
            let f = File::new(i, as_number(s[i]));
            let a = f.checksum(virtual_i);
            checksum += a;
            i += 1;
            free_memory_left = as_number(s[i]);
            virtual_i += f.content_left();
        } else if free_memory_left >= file_to_move.content_left() {
            let start = virtual_i;
            let end = virtual_i + file_to_move.content_left() - 1;
            free_memory_left -= file_to_move.content_left();
            let a = file_to_move.move_n_digits(start, file_to_move.content_left());
            checksum += a;
            virtual_i = end + 1;
            if free_memory_left == 0 {
                i += 1;
            }
            j -= 2;
            file_to_move = File::new(j, as_number(s[j]));
        } else {
            let start = virtual_i;
            let a = file_to_move.move_n_digits(start, free_memory_left);
            checksum += a;
            virtual_i += free_memory_left;
            free_memory_left = 0;
            i += 1;
        }
    }
    checksum += file_to_move.checksum(virtual_i);
    checksum
}

pub fn solve_part2(str: &str) -> usize {
    p2(str)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402";
    const INPUT2: &str = "12345";

    const INPUT3: &str = "233313312141413140258172424";

    #[test]
    fn test_file() {
        let f = File::new(1242, 4);
        dbg!(f);
    }

    #[test]
    fn test_parse() {
        let r = parse(INPUT3);
        dbg!(r);
    }

    #[test]
    fn test_example0() {
        let res = solve_part1(INPUT2);
        assert_eq!(res, 60);
    }

    #[test]
    fn test_example1() {
        dbg!(seq_sum(4, 4));
        let res = solve_part1(INPUT);
        assert_eq!(res, 1928);
    }
}
