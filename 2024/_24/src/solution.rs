use core::panic;
use std::collections::{BTreeMap, LinkedList};


pub fn solve_part1(strs: &[&str]) -> u64 {
    let mut r = parse_input(strs);
    r.process_wiring();
    r.get_number()
}

pub fn solve_part2(strs: &[&str]) -> usize {
    let r = parse_input(strs);
    println!("graph");

    for (i, g) in r.gates.iter().enumerate() {
        println!("  {} --> {:?}{}", g.a, g.op, i);
        println!("  {} --> {:?}{}", g.b, g.op, i);
        println!("  {:?}{} --> {}", g.op, i, g.out);
    }
    0
}


#[derive(Debug)]
enum GateOp {
    And,
    Or,
    Xor
}

#[derive(Debug)]
struct Gate {
    op: GateOp,
    a: String,
    b: String,
    out: String
}


#[derive(Debug)]
struct Wiring {
    inputs:  BTreeMap<String, bool>,
    gates: LinkedList<Gate>
}

fn parse_input(strs : &[&str]) -> Wiring {
    let mut s = strs.split(|x| x.is_empty());

    let inputs : BTreeMap<String, bool> = s.next().unwrap().iter().map(|s| {
        let mut m = s.split(":").map(str::trim);
        let name =  m.next().unwrap().to_string();
        let state = match m.next().unwrap() {
            "1" => true,
            "0" => false,
            _ => panic!("not 1 or 0")
        };
        (name, state)
    }).collect();

    let gates : LinkedList<Gate> = s.next().unwrap().iter().map(|s| {
        let mut m = s.split_whitespace();
        let a = m.next().unwrap().to_string();
        let op = m.next().unwrap();
        let b = m.next().unwrap().to_string();
        m.next();
        let out = m.next().unwrap().to_string();
        
        match op {
            "AND" => Gate { a, b, out, op: GateOp::And },
            "OR" => Gate { a, b, out, op: GateOp::Or },
            "XOR" => Gate { a, b, out, op: GateOp::Xor },
            _ => panic!("Unknown gate")
        }
    }).collect();

    Wiring {
        inputs,
        gates
    }
}

impl Wiring {
    fn process_wiring(&mut self) {
        while let Some(gate) = self.gates.pop_back() {
            let a = self.inputs.get(&gate.a);
            let b = self.inputs.get(&gate.b);
            if let (Some(a), Some(b)) = (a, b) {
                let out = match gate.op {
                    GateOp::And => *a && *b,
                    GateOp::Or => *a || *b,
                    GateOp::Xor => *a ^ *b
                };
                self.inputs.insert(gate.out, out);
            } else {
                self.gates.push_front(gate);
            }
        }
    }

    fn get_number(&self) -> u64 {
        let mut c : u64 = 0;
        let z_vals : Vec<u64> = self.inputs.iter().filter_map(|(k, v)| {
            if k.starts_with("z") {
                Some(*v as u64)
            } else {
                None
            }
        }).collect();
        for (i, v) in z_vals.into_iter().enumerate() {
            let i : u64 = i.try_into().unwrap();
            c+= v * (2_u64.pow(i.try_into().unwrap()));
        }
        c
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
        x00: 1
        x01: 0
        x02: 1
        x03: 1
        x04: 0
        y00: 1
        y01: 1
        y02: 1
        y03: 1
        y04: 1
        
        ntg XOR fgs -> mjb
        y02 OR x01 -> tnw
        kwq OR kpj -> z05
        x00 OR x03 -> fst
        tgd XOR rvg -> z01
        vdt OR tnw -> bfw
        bfw AND frj -> z10
        ffh OR nrd -> bqk
        y00 AND y03 -> djm
        y03 OR y00 -> psh
        bqk OR frj -> z08
        tnw OR fst -> frj
        gnj AND tgd -> z11
        bfw XOR mjb -> z00
        x03 OR x00 -> vdt
        gnj AND wpb -> z02
        x04 AND y00 -> kjc
        djm OR pbm -> qhw
        nrd AND vdt -> hwm
        kjc AND fst -> rvg
        y04 OR y02 -> fgs
        y01 AND x02 -> pbm
        ntg OR kjc -> kwq
        psh XOR fgs -> tgd
        qhw XOR tgd -> z09
        pbm OR djm -> kpj
        x03 XOR y03 -> ffh
        x00 XOR y04 -> ntg
        bfw OR bqk -> z06
        nrd XOR fgs -> wpb
        frj XOR qhw -> z04
        bqk OR frj -> z07
        y03 OR x01 -> nrd
        hwm AND bqk -> z03
        tgd XOR rvg -> z12
        tnw OR pbm -> gnj";

    fn read_input(inp: &str) -> Vec<&str> {
        inp.strip_prefix("\n")
            .unwrap()
            .lines()
            .map(str::trim)
            .collect()
    }

    #[test]
    fn test_parsing() {
        let input = read_input(EXAMPLE);

        let mut r = parse_input(&input);
        r.process_wiring();
        dbg!(r.inputs.len());
        let res = r.get_number();
        dbg!(res);
    }
}
