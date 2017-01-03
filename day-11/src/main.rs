use std::collections::{HashSet};
use std::env;
use std::fs::{File};
use std::io::{Read};
use std::ops::{Add,Sub};

#[derive(Clone,Copy,Debug,Eq,PartialEq,Ord,PartialOrd,Hash)]
struct Floor(u16);
impl Floor {
    fn new() -> Floor { Floor(0) }
    fn chips(&self) -> u8 { (self.0&0xFF) as u8 }
    fn gens(&self) -> u8 { ((self.0>>8)&0xFF) as u8 }
    fn okay(&self) -> bool { !((self.chips()&!self.gens()!=0) && (self.gens()!=0)) }
}
impl Add<Floor> for Floor {
    type Output=Floor;
    fn add(self, other:Floor) -> Floor { Floor(self.0|other.0) }
}
impl Sub<Floor> for Floor {
    type Output=Floor;
    fn sub(self, other:Floor) -> Floor { Floor(self.0&!other.0) }
}

#[derive(Clone,Copy,Debug,Eq,PartialEq,Ord,PartialOrd,Hash)]
struct State {
    floors: [Floor; 4],
    current: usize,
}
impl State {
    fn new() -> State {
        State { floors:[Floor::new(); 4], current:0 }
    }
    fn next(&self) -> Vec<State> {
        let mut nexts = Vec::new();
        let mut items = Vec::new();
        for i in 0..16 { if (self.floors[self.current].0&(1<<i))!=0 { items.push(1<<i); } }
        let mut sets = vec![0];
        for i in 0..items.len() { for j in i..items.len() { sets.push(items[i]|items[j]); } }
        if self.current>0 { // go down
            for &s in &sets {
                let fs = Floor(s);
                let mf = self.floors[self.current-1] + fs;
                let cf = self.floors[self.current] - fs;
                if mf.okay() && cf.okay() {
                    let mut n = self.clone();
                    n.floors[self.current-1] = mf;
                    n.floors[self.current] = cf;
                    n.current -= 1;
                    nexts.push(n);
                }
            }
        }
        if self.current<3 { // go up
            for &s in &sets {
                let fs = Floor(s);
                let mf = self.floors[self.current+1] + fs;
                let cf = self.floors[self.current] - fs;
                if mf.okay() && cf.okay() {
                    let mut n = self.clone();
                    n.floors[self.current+1] = mf;
                    n.floors[self.current] = cf;
                    n.current += 1;
                    nexts.push(n);
                }
            }
        }
        nexts
    }
}


fn main() {
    let s = {
        let args = env::args().collect::<Vec<_>>();
        let mut f = File::open(&args[1]).unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        let s = s.trim().to_string();
        s
    };
    {
        let mut state = State::new();
        state.floors[0].0 = 0x0301;
        let mut seen = HashSet::new();
        seen.insert(state);
        println!("{:?}", state);
        println!("{:?}", seen);
        let n = state.next();
        println!("Part 1: {:?}", n);
    }
    {
        println!("Part 2: {}", s);
    }
}
