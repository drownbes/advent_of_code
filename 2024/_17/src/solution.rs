use rayon::prelude::*;

#[derive(Debug)]
struct Literal(isize);

#[derive(Debug)]
enum Combo {
    Literal(isize),
    RegA,
    RegB,
    RegC,
}

impl Combo {
    fn new(oper: isize) -> Combo {
        match oper {
            0..=3 => Combo::Literal(oper),
            4 => Combo::RegA,
            5 => Combo::RegB,
            6 => Combo::RegC,
            _ => panic!("Unknow operand: {}", oper),
        }
    }

    fn get_value(&self, r: &Runtime) -> isize {
        match self {
            Combo::Literal(v) => *v,
            Combo::RegA => r.a,
            Combo::RegB => r.b,
            Combo::RegC => r.c,
        }
    }
}

#[derive(Debug)]
struct Ignore(isize);

#[derive(Debug)]
enum Instruction {
    Adv(Combo),
    Bxl(Literal),
    Bst(Combo),
    Jnz(Literal),
    Bxc(Ignore),
    Out(Combo),
    Bdv(Combo),
    Cdv(Combo),
}

impl Instruction {
    fn new(opc: isize, oper: isize) -> Instruction {
        match opc {
            0 => Instruction::Adv(Combo::new(oper)),
            1 => Instruction::Bxl(Literal(oper)),
            2 => Instruction::Bst(Combo::new(oper)),
            3 => Instruction::Jnz(Literal(oper)),
            4 => Instruction::Bxc(Ignore(oper)),
            5 => Instruction::Out(Combo::new(oper)),
            6 => Instruction::Bdv(Combo::new(oper)),
            7 => Instruction::Cdv(Combo::new(oper)),
            _ => panic!("Unknown opcode! {}", opc),
        }
    }

    fn run(&self, r: &mut Runtime) {
        //println!("Running: {:?} r: {:?}", self, r);
        match self {
            Instruction::Adv(x) => Self::run_adv(r, x),
            Instruction::Bxl(x) => Self::run_bxl(r, x),
            Instruction::Bst(x) => Self::run_bst(r, x),
            Instruction::Jnz(x) => Self::run_jnz(r, x),
            Instruction::Bxc(x) => Self::run_bxc(r, x),
            Instruction::Out(x) => Self::run_out(r, x),
            Instruction::Bdv(x) => Self::run_bdv(r, x),
            Instruction::Cdv(x) => Self::run_cdv(r, x),
        }
    }

    fn run_adv(r: &mut Runtime, oper: &Combo) {
        let numenator = r.a;
        let base: isize = 2;
        let denominator: isize = base.pow(oper.get_value(r).try_into().unwrap());
        r.a = numenator / denominator;
        r.pc += 2;
    }

    fn run_bxl(r: &mut Runtime, oper: &Literal) {
        r.b ^= oper.0;
        r.pc += 2;
    }

    fn run_bst(r: &mut Runtime, oper: &Combo) {
        let mask = (1 << 3) - 1;
        let v = (oper.get_value(r) % 8) & mask;
        r.b = v;
        r.pc += 2;
    }

    fn run_jnz(r: &mut Runtime, oper: &Literal) {
        if r.a != 0 {
            r.pc = oper.0 as usize;
        } else {
            r.pc += 2;
        }
    }

    fn run_bxc(r: &mut Runtime, _oper: &Ignore) {
        r.b ^= r.c;
        r.pc += 2;
    }

    fn run_out(r: &mut Runtime, oper: &Combo) {
        r.out.push(oper.get_value(r) % 8);
        r.pc += 2;
    }

    fn run_bdv(r: &mut Runtime, oper: &Combo) {
        let numenator = r.a;
        let base: isize = 2;
        let denominator: isize = base.pow(oper.get_value(r).try_into().unwrap());
        r.b = numenator / denominator;
        r.pc += 2;
    }

    fn run_cdv(r: &mut Runtime, oper: &Combo) {
        let numenator = r.a;
        let base: isize = 2;
        let denominator: isize = base.pow(oper.get_value(r).try_into().unwrap());
        r.c = numenator / denominator;
        r.pc += 2;
    }
}

#[derive(Debug, Clone)]
struct Runtime {
    a: isize,
    b: isize,
    c: isize,
    program: Vec<isize>,
    out: Vec<isize>,
    pc: usize,
}

impl Runtime {
    fn run(&mut self) {
        while self.pc + 1 < self.program.len() {
            let opc = self.program[self.pc];
            let oper = self.program[self.pc + 1];
            let inst = Instruction::new(opc, oper);
            inst.run(self);
        }
    }
}

fn parse_register(reg_name: char, s: &str) -> Option<isize> {
    s.strip_prefix(format!("Register {}:", reg_name).as_str())?
        .trim()
        .parse()
        .ok()
}

fn parse_program(s: &str) -> Option<Vec<isize>> {
    s.strip_prefix("Program:")?
        .trim()
        .split(",")
        .map(|x| x.parse::<isize>().expect("failed to parse number"))
        .collect::<Vec<isize>>()
        .into()
}

fn parse_runtime(strs: &[&str]) -> Option<Runtime> {
    let mut s = strs.split(|x| x.is_empty());
    let mut regs = s.next()?.iter();
    Some(Runtime {
        a: parse_register('A', regs.next()?)?,
        b: parse_register('B', regs.next()?)?,
        c: parse_register('C', regs.next()?)?,
        program: parse_program(s.next()?.first()?)?,
        out: vec![],
        pc: 0,
    })
}

fn to_string(v: &Vec<isize>) -> String {
    let out: Vec<String> = v.iter().map(|x| x.to_string()).collect();
    out.join(",")
}

pub fn solve_part1(strs: &[&str]) -> String {
    let mut r = parse_runtime(strs).unwrap();
    r.run();
    to_string(&r.out)
}

fn try_a_reg(a_val: isize, r: &Runtime) -> Option<isize> {
    let expect_out = to_string(&r.program);
    let mut rtm = r.clone();
    rtm.a = a_val;
    rtm.run();
    let got = to_string(&rtm.out);
    println!("{} | {}", got.len(), expect_out.len());
    println!("{} | {}", got, expect_out);
    if got == expect_out {
        Some(a_val)
    } else {
        None
    }
}

//Program: 2,4,1,7,7,5,4,1,1,4,5,5,0,3,3,0
pub fn solve_part2(strs: &[&str]) -> isize {
    let r = parse_runtime(strs).unwrap();
    let expect_out = to_string(&r.program);

    let mut _start: isize = 109685330781408;
    //let _end : isize =  140_737_489_266_832;
    let _end = 109_685_364_266_832;
    //let _start_ :isize = 281_474_976_556_256;

    while _start < _end {
        let mut rtm = r.clone();
        rtm.a = _start;
        rtm.run();
        if rtm.out == r.program {
            break;
        }
        // let last_4 = &rtm.out[rtm.out.len() - 16 ..];
        // if last_4 == &[2,4,1,7,7,5, 4,1,1,4,5,5,0,3,3,0] {
        //     dbg!(rtm.out);
        //     break;
        // }
        _start += 1;
    }
    dbg!(_start);

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
    Register A: 729
    Register B: 0
    Register C: 0

    Program: 0,1,5,4,3,0";

    fn read_input(inp: &str) -> Vec<&str> {
        inp.strip_prefix("\n")
            .unwrap()
            .lines()
            .map(str::trim)
            .collect()
    }

    #[test]
    fn test_name() {
        let input = read_input(EXAMPLE);
        let mut r = parse_runtime(&input).unwrap();
        r.run();
        let out: Vec<String> = r.out.iter().map(|x| x.to_string()).collect();
        let s = out.join(",");
        dbg!(s);
    }
}
