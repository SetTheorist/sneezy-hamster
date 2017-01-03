use std::collections::{HashMap};
use std::env;
use std::fs::{File};
use std::io::{Read};

extern crate regex;
use regex::Regex;


#[derive(Clone,Copy,Debug,Eq,PartialEq,Ord,PartialOrd,Hash)]
enum Register {
    A, B, C, D
}

#[derive(Clone,Copy,Debug,Eq,PartialEq,Ord,PartialOrd)]
enum V {
    Imm(i32), Reg(Register)
}

#[derive(Clone,Copy,Debug,Eq,PartialEq,Ord,PartialOrd)]
enum Op {
    Cpy(V,Register),
    Inc(Register),
    Dec(Register),
    Jnz(V,i32),
}

#[derive(Debug)]
struct CPU {
    code: Vec<Op>,
    regs: HashMap<Register,i32>,
    ip: i32,
}
impl CPU {
    fn new() -> CPU {
        let mut cpu = CPU { code:Vec::new(), regs:HashMap::new(), ip:0 };
        for &r in &[Register::A, Register::B, Register::C, Register::D] {
            cpu.regs.insert(r, 0);
        }
        cpu
    }
    fn step(&mut self) -> bool {
        if self.ip<0 || self.ip>=(self.code.len() as i32) { return false; }
        let op = self.code[self.ip as usize];
        self.ip += 1;
        match op {
            Op::Cpy(V::Imm(i),r) => { *self.regs.get_mut(&r).unwrap() = i; }
            Op::Cpy(V::Reg(s),r) => { *self.regs.get_mut(&r).unwrap() = self.regs[&s]; }
            Op::Inc(r) => { *self.regs.get_mut(&r).unwrap() += 1; }
            Op::Dec(r) => { *self.regs.get_mut(&r).unwrap() -= 1; }
            Op::Jnz(V::Imm(i),j) => { if i!=0 { self.ip += j-1; } }
            Op::Jnz(V::Reg(r),j) => { if self.regs[&r]!=0 { self.ip += j-1; } }
        }
        return true;
    }
    fn assemble(&mut self, s: &str) {
        fn c2d(c: char) -> Register {
            match c {
                'a' => Register::A,
                'b' => Register::B,
                'c' => Register::C,
                'd' => Register::D,
                _ => panic!("Oops"),
            }
        }
        let re_cpy_imm = Regex::new(r"cpy (\d+) ([a-d])").unwrap();
        let re_cpy_reg = Regex::new(r"cpy ([a-d]) ([a-d])").unwrap();
        let re_inc = Regex::new(r"inc ([a-d])").unwrap();
        let re_dec = Regex::new(r"dec ([a-d])").unwrap();
        let re_jnz_imm = Regex::new(r"jnz (\d+) (-?\d+)").unwrap();
        let re_jnz_reg = Regex::new(r"jnz ([a-d]) (-?\d+)").unwrap();
        for l in s.lines() {
            let op =
                if let Some(m) = re_cpy_imm.captures(l) {
                    Op::Cpy(V::Imm(m.at(1).unwrap().parse().unwrap()), 
                            c2d(m.at(2).unwrap().chars().nth(0).unwrap()))
                } else if let Some(m) = re_cpy_reg.captures(l) {
                    Op::Cpy(V::Reg(c2d(m.at(1).unwrap().chars().nth(0).unwrap())),
                            c2d(m.at(2).unwrap().chars().nth(0).unwrap()))
                } else if let Some(m) = re_inc.captures(l) {
                    Op::Inc(c2d(m.at(1).unwrap().chars().nth(0).unwrap()))
                } else if let Some(m) = re_dec.captures(l) {
                    Op::Dec(c2d(m.at(1).unwrap().chars().nth(0).unwrap()))
                } else if let Some(m) = re_jnz_imm.captures(l) {
                    Op::Jnz(V::Imm(m.at(1).unwrap().parse().unwrap()),
                            m.at(2).unwrap().parse().unwrap())
                } else if let Some(m) = re_jnz_reg.captures(l) {
                    Op::Jnz(V::Reg(c2d(m.at(1).unwrap().chars().nth(0).unwrap())),
                            m.at(2).unwrap().parse().unwrap())
                } else {
                    panic!("Aww, man");
                };
            self.code.push(op);
        }
    }
}

fn main() {
    let s = {
        let args : Vec<_> = env::args().collect();
        let mut f = File::open(&args[1]).unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        s.trim().to_string()
    };
    println!("{}", s);
    {
        let mut cpu = CPU::new();
        cpu.assemble(&s);
        while cpu.step() {
            //println!("{:?}", cpu);
        }
        println!("Part 1: {:?}", cpu.regs[&Register::A]);
    }
    {
        let mut cpu = CPU::new();
        *cpu.regs.get_mut(&Register::C).unwrap() = 1;
        cpu.assemble(&s);
        while cpu.step() {
            //println!("{:?}", cpu);
        }
        println!("Part 2: {:?}", cpu.regs[&Register::A]);
    }
}
