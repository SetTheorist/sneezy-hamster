use std::collections::{HashMap};
use std::env;
use std::fs::{File};
use std::io::{Read};

extern crate regex;
use regex::Regex;

#[derive(Clone,Copy,Debug)]
enum Dest {
    Unknown, Output(usize), Bot(usize)
}
#[derive(Debug)]
struct Bot {
    values: [usize; 2],
    hi: Dest,
    lo: Dest,
    dirty: bool,
}
impl Bot {
    fn new() -> Bot { Bot { values:[0,0], hi:Dest::Unknown, lo:Dest::Unknown, dirty:false, } }
    fn add(&mut self, v: usize) {
        if self.values[0]==0 {
            self.values[0] = v;
        } else {
            if self.values[0]<v {
                if self.values[1] != v {
                    self.values[1] = v;
                    self.dirty = true;
                }
            } else if self.values[0]>v {
                self.values[1] = self.values[0];
                self.values[0] = v;
                self.dirty = true;
            } else {
            }
        }
    }
}

fn main() {
    let s = {
        let args : Vec<_> = env::args().collect();
        let fname = &args[1];
        let mut f = File::open(fname).unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        let s = s.trim().to_string();
        s
    };
    let re_val = Regex::new(r"value (\d+) goes to bot (\d+)").unwrap();
    let re_gives = Regex::new(r"bot (\d+) gives low to ((bot (\d+))|(output (\d+))) and high to ((bot (\d+))|(output (\d+)))").unwrap();
    {
        let mut bots = HashMap::<usize,Bot>::new();
        for l in s.lines() {
            if let Some(m) = re_val.captures(l) {
                let v = m.at(1).unwrap().parse().unwrap();
                let a = m.at(2).unwrap();
                let b : usize = a.parse().unwrap();
                if !bots.contains_key(&b) { bots.insert(b, Bot::new()); }
                bots.get_mut(&b).unwrap().add(v);
            } else if let Some(m) = re_gives.captures(l) {
                let a = m.at(1).unwrap();
                let b : usize = a.parse().unwrap();
                if !bots.contains_key(&b) { bots.insert(b, Bot::new()); }
                let b = bots.get_mut(&b).unwrap();
                let dlo = 
                    if let Some(x) = m.at(4) {
                        Dest::Bot(x.parse().unwrap())
                    } else if let Some(x) = m.at(6) {
                        Dest::Output(x.parse().unwrap())
                    } else {
                        panic!("Unexpected");
                    };
                let dhi = 
                    if let Some(x) = m.at(9) {
                        Dest::Bot(x.parse().unwrap())
                    } else if let Some(x) = m.at(11) {
                        Dest::Output(x.parse().unwrap())
                    } else {
                        panic!("Unexpected");
                    };
                b.lo = dlo;
                b.hi = dhi;
            }
        }
        let mut changed = true;
        let mut iters : i32 = 0;
        let mut outputs = HashMap::<usize,usize>::new();
        while changed {
            iters += 1;
            changed = false;
            let mut changes = Vec::new();
            for (_,v) in bots.iter_mut() {
                if v.dirty {
                    match v.lo {
                        Dest::Bot(x) => { changes.push((x, v.values[0])); }
                        Dest::Output(x) => { outputs.insert(x, v.values[0]); }
                        Dest::Unknown => {}
                    }
                    match v.hi {
                        Dest::Bot(x) => { changes.push((x, v.values[1])); }
                        Dest::Output(x) => { outputs.insert(x, v.values[1]); }
                        Dest::Unknown => {}
                    }
                    v.dirty = false;
                    changed = true;
                }
            }
            for (b,v) in changes { bots.get_mut(&b).unwrap().add(v); }
        }
        for (k,v) in bots.iter() {
            if v.values == [17,61] {
                println!("Part 1: {:?} {:?} ({})", k, v, iters);
            }
        }
        println!("Part 2: {:?} {:?} {:?} {}",
            outputs.get(&0), outputs.get(&1), outputs.get(&2),
            outputs.get(&0).unwrap()*outputs.get(&1).unwrap()*outputs.get(&2).unwrap());
    }
}
