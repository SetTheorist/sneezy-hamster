use std::collections::{HashSet};
use std::env;
use std::fs::File;
use std::io::prelude::*;
//use std::ops::{Add,Mul};

extern crate crypto;
use crypto::digest::Digest;
use crypto::md5::Md5;

extern crate regex;
use regex::Regex;

fn main() {
    let args : Vec<_> = env::args().collect();
    let s = {
        let mut f = File::open(&args[1]).unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        s.trim().to_string()
    };
    let mut n = 0;
    print!("Part 1: ");
    for salt in 0..100 {
        let mut m = Md5::new();
        m.input_str(&s);
        m.input_str(&salt.to_string());
        let h = m.result_str();
        if &h[0..5] == "00000" {
            print!("{}", &h[5..6]);
            n += 1;
            if n>=8 { break; }
        }
    }
    println!("");
}
