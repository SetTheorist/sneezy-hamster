use std::collections::{HashMap, HashSet};

extern crate pathlib;
use pathlib::path;
use pathlib::priority_queue;

#[derive(Debug,Default,Clone,Copy,Eq,PartialEq,Ord,PartialOrd,Hash)]
struct L(i32,i32);

#[derive(Debug,Clone,Copy,Eq,PartialEq,Ord,PartialOrd)]
enum Cell {
    Free, Blocked,
}

struct Grid {
    size: (i32,i32),
    cells: HashMap<L,Cell>,
}
impl Grid {
    fn new(x:i32, y:i32) -> Grid {
        let mut grid = Grid { size:(x,y), cells:HashMap::new() };
        for i in 0..x {
            for j in 0..y {
                grid.cells.insert(L(i,j),
                    if i==0||i==(x-1)||j==0||j==(y-1) {Cell::Blocked}
                    else {Cell::Free}
                );
            }
        }
        grid
    }
    fn show(&self) {
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                match self.cells[&L(x,y)] {
                    Cell::Free => print!("."),
                    Cell::Blocked => print!("#"),
                }
            }
            println!("");
        }
    }
    fn neighbors(&self, l:L) -> Vec<L> {
        let mut v = Vec::new();
        for &(dx,dy) in &[(1,0),(-1,0),(0,1),(0,-1)] {
            let l2 = L(l.0+dx,l.1+dy);
            match self.cells[&l2] {
                Cell::Free => v.push(l2),
                Cell::Blocked => (),
            }
        }
        v
    }
}


fn main() {
    let mut g = Grid::new(10,10);
    g.cells.insert(L(2,2), Cell::Blocked);
    g.cells.insert(L(4,1), Cell::Blocked);
    g.cells.insert(L(1,4), Cell::Blocked);
    g.show();

    {
        println!("--------------------");
        let mut res = HashMap::new();
        path::dijkstra(
            L(1,1),
            |_,_|1,
            |l|g.neighbors(l),
            &mut res,
            1000000000
            );
        println!("{:?}", res);
    }

    {
        println!("--------------------");
        let mut res = HashMap::new();
        let path = path::astar(
            L(1,1),
            L(1,8),
            |_,_|1,
            |l|g.neighbors(l),
            |l|g.neighbors(l),
            |l1,l2|((l1.0-l2.0).abs()+(l1.1-l2.1).abs()) as i64,
            &mut res
            );
        println!("{:?}", path);
        println!("{:?}", res);
    }
}

