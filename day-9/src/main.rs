use std::env;
use std::fs::{File};
use std::io::{Read};

extern crate regex;
use regex::Regex;

//  ( A x B )  .
// ^ out            ^ out
//   ^^^ a
//       ^^^ b
//            ^^^ c
#[derive(Debug)]
enum S { Out, A(usize), B(usize,usize), C(usize,usize,String) }
impl S {
    fn step(&mut self, c: char, output: &mut usize) {
        if c.is_whitespace() { return; }
        let news = match *self {
            S::Out => {
                if c=='(' { S::A(0) }
                else { *output += 1; S::Out }
            }
            S::A(a) => {
                if c=='x' { S::B(a, 0) }
                else { let d = ((c as u8) - ('0' as u8)) as usize; S::A(a*10 + d) }
            }
            S::B(a, b) => {
                if c==')' { S::C(a, b, String::new()) }
                else { let d = ((c as u8) - ('0' as u8)) as usize; S::B(a, b*10 + d) }
            }
            S::C(a, b, ref mut ss) => {
                ss.push(c);
                let a = a - 1;
                if a==0 { *output += b * ss.len(); S::Out }
                else { S::C(a, b, ss.clone()) }
            }
        };
        *self = news;
    }
}

#[derive(Debug)]
enum Chunk {
    Simple(usize), // length
    Repeat(usize,Vec<Chunk>), // n x tail
}
impl Chunk {
    fn size(&self) -> usize {
        match *self {
            Chunk::Simple(k) => { k }
            Chunk::Repeat(n, ref v) => {
                let k : usize = v.iter().map(|c|c.size()).sum();
                n * k
            }
        }
    }
    // this assumes repeat blocks are nested
    fn build(s: &String) -> Vec<Chunk> {
        //println!("build('{}') ==>" , s);
        let re = Regex::new(r"([A-Z]*)(\((\d+)x(\d+)\))?").unwrap();
        let mut v = Vec::new();
        let mut tail : &str = &s;
        while tail.len()!=0 {
            //println!("\ttail='{}'", &tail);
            if let Some(m) = re.captures(tail) {
                let all = m.at(0).unwrap();
                let pre = m.at(1).unwrap();
                if pre.len()>0 { v.push(Chunk::Simple(pre.len())); }
                if m.at(3)==None {
                    tail = &tail[all.len()..];
                } else {
                    let a = m.at(3).unwrap();
                    let b = m.at(4).unwrap();
                    tail = &tail[all.len()..];
                    let n = a.parse().unwrap();
                    let iv = Chunk::build(&tail[0..n].to_string());
                    tail = &tail[n..];
                    let ch = Chunk::Repeat(b.parse().unwrap(), iv);
                    v.push(ch);
                };
            } else {
                break;
            }
        }
        //println!("build('{}') ==> {:?}", s, v);
        v
    }
}


fn main() {
    let s = {
        let args : Vec<_> = env::args().collect();
        let mut f = File::open(&args[1]).unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        let s = s.trim().to_string();
        s
    };
    //println!("{}", s);
    {
        let mut state = S::Out;
        let mut output = 0;
        for c in s.chars() { state.step(c, &mut output); }
        println!("Part 1: {}", output);
    }
    {
        let v = Chunk::build(&s);
        let n : usize = v.iter().map(|c|c.size()).sum();
        println!("Part 2: {}", n);
    }
}
