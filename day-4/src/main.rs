use std::collections::{HashMap};
use std::env;
use std::fs::File;
use std::io::prelude::*;

extern crate regex;
use regex::Regex;

#[derive(Clone,Copy,Debug,Eq,Ord,PartialEq,PartialOrd)]
struct P(i32,char);

fn rot(s: &str, n: u32) -> String {
    s.chars().map(|c| {
        if c=='-' {
            ' '
        } else {
            char::from((((((c as u32) - ('a' as u32)) + n) % 26) + ('a' as u32)) as u8)
        }
    }).collect()
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
        let re = Regex::new(r"([a-z-]+)-([0-9]+)\[([a-z]+)\]").unwrap();
        let sum : u32 = re.captures_iter(&s).map(|m| {
            let n : u32 = m.at(2).unwrap().parse().unwrap();
            let mut hm = HashMap::new();
            for c in m.at(1).unwrap().chars().filter(|c|*c!='-') {
                if hm.contains_key(&c) {
                    let v : i32 = *hm.get(&c).unwrap();
                    hm.insert(c, v+1);
                } else {
                    hm.insert(c, 1);
                }
            }
            let mut v : Vec<_> = hm.iter().map(|(k,v)|P(-v,*k)).collect();
            v.sort();
            let v : Vec<_> = v.iter().take(5).map(|&P(_,k)|k).collect();
            let cs : Vec<_> = m.at(3).unwrap().chars().collect();
            if v==cs {
                let x = rot(m.at(1).unwrap(), n);
                if x.contains("north") {
                    println!("Part 2: {:?}", n);
                }
            }
            if v==cs { n } else { 0 }
        }).sum();
        println!("Part 1: {}", sum);
    }

    {
    }
}
