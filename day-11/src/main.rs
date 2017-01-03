use std::collections::{HashSet,VecDeque};
use std::env;
use std::fmt;
use std::fs::{File};
use std::io::{Read};
use std::ops::{Add,Sub};

#[derive(Clone,Copy,Eq,PartialEq,Ord,PartialOrd,Hash)]
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
impl fmt::Display for Floor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut sc = String::new();
        let mut sg = String::new();
        for i in 0..8 {
            if self.0&(0x0001<<i)!=0 { sc.push((('a' as u8)+i) as char); }
            if self.0&(0x0100<<i)!=0 { sg.push((('A' as u8)+i) as char); }
        }
        write!(f, "{}{}", sc, sg)
    }
}
impl fmt::Debug for Floor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut sc = String::new();
        let mut sg = String::new();
        for i in 0..8 {
            if self.0&(0x0001<<i)!=0 { sc.push((('a' as u8)+i) as char); }
            if self.0&(0x0100<<i)!=0 { sg.push((('A' as u8)+i) as char); }
        }
        write!(f, "{}{}", sc, sg)
    }
}

#[derive(Clone,Copy,Ord,PartialOrd,Hash)]
struct State {
    floors: [Floor; 4],
    current: usize,
    d: i32,
}
impl State {
    fn new() -> State {
        State { floors:[Floor::new(); 4], current:0, d:0 }
    }
    fn next(&self) -> Vec<State> {
        let mut nexts = Vec::new();
        let mut items = Vec::new();
        for i in 0..16 { if (self.floors[self.current].0&(1<<i))!=0 { items.push(1<<i); } }
        let mut sets = Vec::new();
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
                    n.d = self.d+1;
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
                    n.d = self.d+1;
                    nexts.push(n);
                }
            }
        }
        nexts
    }
}
impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[<{}>{}{}{}|{}{}{}|{}{}{}|{}{}{}]", self.d,
            (if self.current==0{'('}else{' '}), self.floors[0], (if self.current==0{')'}else{' '}),
            (if self.current==1{'('}else{' '}), self.floors[1], (if self.current==1{')'}else{' '}),
            (if self.current==2{'('}else{' '}), self.floors[2], (if self.current==2{')'}else{' '}),
            (if self.current==3{'('}else{' '}), self.floors[3], (if self.current==3{')'}else{' '}),
            )
    }
}
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[<{}>{}{}{}|{}{}{}|{}{}{}|{}{}{}]", self.d,
            (if self.current==0{'('}else{' '}), self.floors[0], (if self.current==0{')'}else{' '}),
            (if self.current==1{'('}else{' '}), self.floors[1], (if self.current==1{')'}else{' '}),
            (if self.current==2{'('}else{' '}), self.floors[2], (if self.current==2{')'}else{' '}),
            (if self.current==3{'('}else{' '}), self.floors[3], (if self.current==3{')'}else{' '}),
            )
    }
}
impl PartialEq for State {
    fn eq(&self, other: &State) -> bool {
        self.current==other.current && self.floors==other.floors
    }
}
impl Eq for State {}

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
        // example:
        let init = State { floors:[Floor(0x0003),Floor(0x0100),Floor(0x0200),Floor(0x0000)], current:0, d:0 };
        let goal = State { floors:[Floor(0),Floor(0),Floor(0),Floor(0x0303)], current:3, d:0 };
        let mut seen = HashSet::new();
        let mut queued = HashSet::new();
        let mut q = VecDeque::new();
        q.push_back(init);
        println!("init={} goal={}", init, goal);
        let mut d = 0;
        let mut maxq : usize = 0;
        let mut state;
        loop {
            if q.len()>maxq { maxq = q.len(); }
            state = q.pop_front().unwrap();
            if state.floors==goal.floors { d = state.d; break; }
            seen.insert(state);
            let ns = state.next();
            for n in ns {
                if !queued.contains(&n) {
                    q.push_back(n);
                    queued.insert(n);
                }
            }
        }
        println!("Example: {:?} ({}) {}", d, maxq, state);
    }
    {
//The first floor contains a thulium generator, a thulium-compatible microchip, a plutonium generator, and a strontium generator.
//The second floor contains a plutonium-compatible microchip and a strontium-compatible microchip.
//The third floor contains a promethium generator, a promethium-compatible microchip, a ruthenium generator, and a ruthenium-compatible microchip.
//The fourth floor contains nothing relevant.
// PlPrRuStTh
        // TODO: read from file
        let init = State { floors:[Floor(0x1910),Floor(0x0009),Floor(0x0606),Floor(0x0000)], current:0, d:0 };
        let goal = State { floors:[Floor(0),Floor(0),Floor(0),Floor(0x1F1F)], current:3, d:0 };
        let mut seen = HashSet::new();
        let mut queued = HashSet::new();
        let mut q = VecDeque::new();
        q.push_back(init);
        println!("init={} goal={}", init, goal);
        let mut d = 0;
        let mut maxq : usize = 0;
        let mut state;
        loop {
            if q.len()>maxq { maxq = q.len(); }
            state = q.pop_front().unwrap();
            if state.floors==goal.floors { d = state.d; break; }
            seen.insert(state);
            let ns = state.next();
            for n in ns {
                if !queued.contains(&n) {
                    q.push_back(n);
                    queued.insert(n);
                }
            }
        }
        println!("Part 1: {:?} ({}) {}", d, maxq, state);
    }
    {
//The first floor contains a thulium generator, a thulium-compatible microchip, a plutonium generator, and a strontium generator.
//The second floor contains a plutonium-compatible microchip and a strontium-compatible microchip.
//The third floor contains a promethium generator, a promethium-compatible microchip, a ruthenium generator, and a ruthenium-compatible microchip.
//The fourth floor contains nothing relevant.
// PlPrRuStTh
// -- extra on 1st floor: El+Di chip+gen
// PlPrRuStThDiEl
        // TODO: read from file
        let init = State { floors:[Floor(0x7970),Floor(0x0009),Floor(0x0606),Floor(0x0000)], current:0, d:0 };
        let goal = State { floors:[Floor(0),Floor(0),Floor(0),Floor(0x7F7F)], current:3, d:0 };
        let mut seen = HashSet::new();
        let mut queued = HashSet::new();
        let mut q = VecDeque::new();
        q.push_back(init);
        println!("init={} goal={}", init, goal);
        let mut d = 0;
        let mut maxq : usize = 0;
        let mut state;
        loop {
            if q.len()>maxq { maxq = q.len(); }
            state = q.pop_front().unwrap();
            if state.floors==goal.floors { d = state.d; break; }
            seen.insert(state);
            let ns = state.next();
            for n in ns {
                if !queued.contains(&n) {
                    q.push_back(n);
                    queued.insert(n);
                }
            }
        }
        println!("Part 2: {:?} ({}) {}", d, maxq, state);
    }
}
