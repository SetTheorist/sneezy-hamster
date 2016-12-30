use std::cmp::{max};
use std::env;
use std::fs::File;
use std::io::prelude::*;

extern crate regex;
use regex::Regex;


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
        let re = Regex::new(r"([0-9]+)\s+([0-9]+)\s+([0-9]+)").unwrap();
        let c = re
            .captures_iter(&s)
            .filter(|m| {
                let n1 = m.at(1).unwrap().parse::<i32>().unwrap();
                let n2 = m.at(2).unwrap().parse::<i32>().unwrap();
                let n3 = m.at(3).unwrap().parse::<i32>().unwrap();
                let s = n1 + n2 + n3;
                let m = max(n1,max(n2,n3));
                s>2*m
                })
            .count();
        println!("Part 1: {}", c);
    }
    {
        // ugly solution
        let re = Regex::new(
                r"([0-9]+)\s+([0-9]+)\s+([0-9]+)\s+([0-9]+)\s+([0-9]+)\s+([0-9]+)\s+([0-9]+)\s+([0-9]+)\s+([0-9]+)"
            ).unwrap();
        let c : i32 = re
            .captures_iter(&s)
            .map(|m| {
                let a1 = m.at(1).unwrap().parse::<i32>().unwrap();
                let a2 = m.at(4).unwrap().parse::<i32>().unwrap();
                let a3 = m.at(7).unwrap().parse::<i32>().unwrap();
                let sa = a1 + a2 + a3;
                let ma = max(a1,max(a2,a3));
                let b1 = m.at(2).unwrap().parse::<i32>().unwrap();
                let b2 = m.at(5).unwrap().parse::<i32>().unwrap();
                let b3 = m.at(8).unwrap().parse::<i32>().unwrap();
                let sb = b1 + b2 + b3;
                let mb = max(b1,max(b2,b3));
                let c1 = m.at(3).unwrap().parse::<i32>().unwrap();
                let c2 = m.at(6).unwrap().parse::<i32>().unwrap();
                let c3 = m.at(9).unwrap().parse::<i32>().unwrap();
                let sc = c1 + c2 + c3;
                let mc = max(c1,max(c2,c3));
                (if sa>2*ma {1}else{0}) + (if sb>2*mb {1}else{0}) + (if sc>2*mc {1}else{0})
                })
            .sum();
        println!("Part 2: {}", c);
    }
}
