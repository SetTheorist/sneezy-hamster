use std::collections::{HashSet};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::ops::{Add,Mul};

extern crate regex;
use regex::Regex;


// x-axis: N
// y-axis: E

#[derive(Clone,Copy,Debug,Eq,Hash,Ord,PartialEq,PartialOrd)]
struct P(i32,i32);
impl P {
    pub fn l(&self) -> P { P(self.1, -self.0) }
    pub fn r(&self) -> P { P(-self.1, self.0) }
}
impl Add<P> for P {
    type Output = P;
    fn add(self, other:P) -> P { P(self.0+other.0, self.1+other.1) }
}
impl Mul<i32> for P {
    type Output = P;
    fn mul(self, other:i32) -> P { P(self.0*other, self.1*other) }
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
    {
        let re = Regex::new(r"(L([0-9]+))|(R([0-9]+))").unwrap();
        let mut dir = P(1,0);
        let mut pos = P(0,0);
        let mut hs = HashSet::new();
        let mut f = true;
        hs.insert(pos);
        for m in re.captures_iter(&s) {
            let n;
            match (m.at(2),m.at(4)) {
                (Some(s),None) => { dir = dir.l(); n = s.parse().unwrap(); }
                (None,Some(s)) => { dir = dir.r(); n = s.parse().unwrap(); }
                (_,_) => { panic!("Unexpected"); }
            }
            // pos = pos + dir*n;
            for _ in 0..n {
                pos = pos + dir;
                if f && hs.contains(&pos) {
                    f = false;
                    println!("Part 2: {}", pos.0.abs() + pos.1.abs());
                }
                hs.insert(pos);
            }
        }
        println!("Part 1: {}", pos.0.abs() + pos.1.abs());
    }
}
