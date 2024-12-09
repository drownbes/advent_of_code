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
    content: Vec<u8>,
    id: usize,
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: &str = std::str::from_utf8(&self.content).unwrap();
        write!(f, "{}", s)
    }
}

fn checksum(chunk: &Vec<u8>, start: usize) -> usize {
    let mut c: usize = 0;
    for (i, digit) in chunk.iter().enumerate() {
        c += as_number(*digit) * (start + i);
    }
    c
}

impl File {
    fn new(i: usize, size: usize) -> File {
        let id = i / 2;
        let id_str = format!("{}", id);
        let content = id_str.repeat(size).as_bytes().to_vec();
        File { id, content }
    }

    fn content_left(&self) -> usize {
        self.content.len()
    }

    fn checksum(&self, start: usize) -> usize {
        checksum(&self.content, start)
    }

    fn move_n_digits(&mut self, start: usize, n: usize) -> usize {
        let chunk: Vec<u8> = self.content.iter().rev().take(n).copied().collect();
        self.content.truncate(self.content.len() - n);
        checksum(&chunk, start)
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

pub fn p1(s: &str) -> usize {
    let mut nums = parse(s);
    let (mut left, mut right) = (0, nums.len() - 1);
    while left < right {
        while nums[left].is_some() {
            left += 1;
        }
        while nums[right].is_none() {
            right -= 1;
        }
        if left < right {
            nums.swap(left, right);
        }
        left += 1;
        right -= 1;
    }
    nums.into_iter()
        .enumerate()
        .filter_map(|(i, opt)| opt.map(|v| v * i))
        .sum()
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
    p1(s)
}

pub fn solve_part1_(s: &str) -> u64 {
    let s = s.as_bytes();
    let mut i: usize = 0; // skip first file
    let mut j: usize = s.len() - 1;
    let mut checksum: u64 = 0;
    let mut virtual_i = 0;

    let mut free_memory_left = 0;
    let mut file_to_move = File::new(j, as_number(s[j]));

    while i < j && i != s.len() {
        println!(" ");
        println!(
            "---- i:{} j:{} vi: {} fm: {} id: {} lm: {}-----",
            i,
            j,
            virtual_i,
            free_memory_left,
            file_to_move.id,
            file_to_move.content_left()
        );
        if is_file(i) {
            let f = File::new(i, as_number(s[i]));
            let a = f.checksum(virtual_i);
            println!(
                "Reached file: {} [{}, {}]",
                f,
                virtual_i,
                virtual_i + f.content_left() - 1
            );
            println!("File checksum: {}", a);
            checksum += a as u64;
            println!("Total checksum: {}", checksum);
            i += 1;
            if i >= j {
                break;
            }
            free_memory_left = as_number(s[i]);
            virtual_i += f.content_left();
        } else if free_memory_left >= file_to_move.content_left() {
            println!("free_memory_left >= left_to_move");

            let start = virtual_i;
            let end = virtual_i + file_to_move.content_left() - 1;
            free_memory_left -= file_to_move.content_left();
            println!(
                "Filling [{},{}] with file file:{}",
                start, end, file_to_move
            );
            let a = file_to_move.move_n_digits(start, file_to_move.content_left());
            checksum += a as u64;
            println!("checksum addition: {}, checksum after: {}", a, checksum);
            virtual_i = end + 1;
            if free_memory_left == 0 {
                i += 1;
            }
            j -= 2;
            if i >= j {
                break;
            }

            file_to_move = File::new(j, as_number(s[j]));
        } else {
            println!("free_memory_left < left_to_move");
            let start = virtual_i;
            let end = virtual_i + free_memory_left - 1;
            println!("Filling [{},{}] with file: {}", start, end, file_to_move);
            let a = file_to_move.move_n_digits(start, free_memory_left);
            checksum += a as u64;
            println!("checksum addition: {}, checksum after: {}", a, checksum);
            virtual_i += free_memory_left;
            free_memory_left = 0;
            i += 1;
        }
    }
    dbg!(virtual_i, i, j);
    checksum += file_to_move.checksum(virtual_i) as u64;
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
