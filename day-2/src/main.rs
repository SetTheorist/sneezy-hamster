use std::collections::{HashMap};
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args : Vec<_> = env::args().collect();
    let s = {
        let mut f = File::open(&args[1]).unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        s.trim();
        s
    };

    // 123
    // 456
    // 789
    let hm1 = {
        let mut hm = HashMap::new();
        hm.insert('U', [0, 1,2,3, 1,2,3, 4,5,6]);
        hm.insert('D', [0, 4,5,6, 7,8,9, 7,8,9]);
        hm.insert('L', [0, 1,1,2, 4,4,5, 7,7,8]);
        hm.insert('R', [0, 2,3,3, 5,6,6, 8,9,9]);
        hm
    };
    print!("Part 1: ");
    let mut n = 5;
    for l in s.lines() {
        for c in l.trim().chars() {
            n = hm1.get(&c).unwrap()[n];
        }
        print!("{}", n);
    }
    println!("");

    //   1
    //  234
    // 56789
    //  ABC
    //   D
    let hm2 = {
        let mut hm = HashMap::new();
        hm.insert('U', [0, 1, 2,1,4, 5, 2, 3, 4,9,  6, 7, 8, 11]);
        hm.insert('D', [0, 3, 6,7,8, 5,10,11,12,9, 10,11,12, 13]);
        hm.insert('L', [0, 1, 2,2,3, 5, 5, 6, 7,8, 10,10,11, 13]);
        hm.insert('R', [0, 1, 3,4,4, 6, 7, 8, 9,9, 11,12,12, 13]);
        hm
    };
    print!("Part 2: ");
    let mut n = 5;
    for l in s.lines() {
        for c in l.trim().chars() {
            n = hm2.get(&c).unwrap()[n];
        }
        print!("{:X}", n);
    }
    println!("");
}
