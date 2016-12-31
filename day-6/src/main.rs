use std::collections::{HashMap};
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn zhm() -> HashMap<char, i32> {
    let mut hm = HashMap::new();
    for c in ('a' as u8)..(('z' as u8)+1) { hm.insert(c as char,0); }
    hm
}
// yes, this should just be a fold
fn most(hm: &HashMap<char, i32>) -> char {
    let mut c = '?';
    let mut n = i32::min_value();
    for (k,v) in hm.iter() { if *v>n { c=*k; n=*v; } }
    c
}
// yes, this should just be a fold, and why the duplication?!
fn lest(hm: &HashMap<char, i32>) -> char {
    let mut c = '?';
    let mut n = i32::max_value();
    for (k,v) in hm.iter() { if *v<n { c=*k; n=*v; } }
    c
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
        let mut hms : Vec<HashMap<char,i32>> = (0..8).map(|_|zhm()).collect();
        for l in s.lines() {
            let b : Vec<char> = l.chars().collect();
            if b.len()<8 { continue; }
            for i in 0..8 {
                *hms[i].get_mut(&b[i]).unwrap() += 1;
            }
        }
        let mc : String = hms.iter().map(|hm|most(hm)).collect();
        println!("Part 1: {}", mc);
        let lc : String = hms.iter().map(|hm|lest(hm)).collect();
        println!("Part 2: {}", lc);
    }
}
