use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone,Copy,Debug,Eq,Hash,Ord,PartialEq,PartialOrd)]
struct S {
    c_3 : char,
    c_2 : char,
    c_1 : char,
    f : bool,
    a_in : bool,
    a_out : bool,
    is_in : bool,
}
impl S {
    fn new() -> Self {
        S { c_3:'*', c_2:'*', c_1:'*', f:false, a_in:false, a_out:false, is_in:false, }
    }
    fn step(&self, c: char) -> S {
        let f = self.c_3==c && self.c_2==self.c_1 && self.c_3!=self.c_2;
        S {
            c_3 : self.c_2,
            c_2 : self.c_1,
            c_1 : c,
            f : f,
            is_in : (self.is_in || c=='[') && c!=']',
            a_in : self.a_in || (self.is_in && f),
            a_out : self.a_out || (!self.is_in && f),
        }
    }
}

fn main() {
    let args : Vec<_> = env::args().collect();
    let s = {
        let mut f = File::open(&args[1]).unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        s.trim();
        s
    };
    let n : usize = s
        .lines()
        .filter(|l| {
            let s = l.chars().fold(S::new(), |s,c|s.step(c));
            s.a_out && !s.a_in
            })
        .count();
    println!("Part 1: {}", n);
}
