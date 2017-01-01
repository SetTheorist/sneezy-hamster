use std::env;
use std::fs::{File};
use std::io::{Read};

//  ( A x B )  .
// ^ out            ^ out
//   ^^^ a
//       ^^^ b
//            ^^^ c
#[derive(Debug)]
enum S {
    Out, A(i32), B(i32,i32), C(i32,i32,String)
}
impl S {
    fn step(&mut self, c: char, output: &mut String) {
        if c.is_whitespace() { return; }
        let news = match *self {
            S::Out => {
                if c=='(' {
                    S::A(0)
                } else {
                    output.push(c);
                    S::Out
                }
            }
            S::A(a) => {
                if c=='x' {
                    S::B(a, 0)
                } else {
                    let d = ((c as u8) - ('0' as u8)) as i32;
                    S::A(a*10 + d)
                }
            }
            S::B(a, b) => {
                if c==')' {
                    S::C(a, b, String::new())
                } else {
                    let d = ((c as u8) - ('0' as u8)) as i32;
                    S::B(a, b*10 + d)
                }
            }
            S::C(a, b, ref mut ss) => {
                ss.push(c);
                let a = a - 1;
                if a==0 {
                    for _ in 0..b { output.push_str(&ss); }
                    S::Out
                } else {
                    S::C(a, b, ss.clone())
                }
            }
        };
        *self = news;
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
        let mut state = S::Out;
        let mut output = String::new();
        for c in s.chars() {
            state.step(c, &mut output);
        }
        println!("{}", output);
        println!("Part 1: {}", output.len());
    }
}
