use std::env;
use std::fs::{File};
use std::io::{Read};

extern crate regex;
use regex::{Regex};

struct Lcd {
    screen: Vec<Vec<bool>>,
}
impl Lcd {
    fn new(r:usize, c:usize) -> Lcd {
        Lcd { screen: (0..r).map(|_|vec![false;c]).collect() }
    }
    fn rect(&mut self, a:usize, b:usize) {
        for x in 0..a {
            for y in 0..b {
                self.screen[x][y] = true;
            }
        }
    }
    fn row(&mut self, a:usize, b:usize) {
        let l = self.screen.len();
        let b = b % l;
        for _ in 0..b {
            let t = self.screen[l-1][a];
            for i in (0..(l-1)).rev() {
                self.screen[i+1][a] = self.screen[i][a];
            }
            self.screen[0][a] = t;
        }
    }
    fn col(&mut self, a:usize, b:usize) {
        let b = b % self.screen[a].len();
        for _ in 0..b {
            let t = self.screen[a].pop().unwrap();
            self.screen[a].insert(0, t);
        }
    }
    fn show(&self) {
        for y in 0..self.screen[0].len() {
            for x in 0..self.screen.len() {
                print!("{}", if self.screen[x][y] {'#'} else {'.'});
            }
            println!("");
        }
    }
}

fn main() {
    let s = {
            let args : Vec<_> = env::args().collect();
            let mut f = File::open(&args[1]).unwrap();
            let mut s = String::new();
            f.read_to_string(&mut s).unwrap();
            s
        };
    {
        let mut lcd = Lcd::new(50, 6);
        let re_rect = Regex::new(r"rect (\d+)x(\d+)").unwrap();
        let re_row = Regex::new(r"rotate row y=(\d+) by (\d+)").unwrap();
        let re_col = Regex::new(r"rotate column x=(\d+) by (\d+)").unwrap();
        for l in s.lines() {
            //println!("{}", l);
            if let Some(m) = re_rect.captures(l) {
                let a : usize = m.at(1).unwrap().parse().unwrap();
                let b : usize = m.at(2).unwrap().parse().unwrap();
                lcd.rect(a, b);
            } else if let Some(m) = re_row.captures(l) {
                let a : usize = m.at(1).unwrap().parse().unwrap();
                let b : usize = m.at(2).unwrap().parse().unwrap();
                lcd.row(a, b);
            } else if let Some(m) = re_col.captures(l) {
                let a : usize = m.at(1).unwrap().parse().unwrap();
                let b : usize = m.at(2).unwrap().parse().unwrap();
                lcd.col(a, b);
            }
            //lcd.show();
        }
        println!("----- -----");
        lcd.show();
        let n : usize = lcd.screen.iter().map(|x|x.iter().filter(|&&v|v).count()).sum();
        println!("Part 1: {}", n);
    }
    {
        println!("Part 2: {}", "CFLELOYFCS"); // TODO: do this correctly
    }
}
