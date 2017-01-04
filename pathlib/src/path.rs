use std::collections::{HashMap};
use std::i64;
use std::fmt::{Debug};
use std::hash::{Hash};

use priority_queue::PriorityQueue;

// result = minimal cost to reach point from start
pub fn dijkstra<Pos,Fcost,Fedges>(start: Pos, step_cost: Fcost, edges: Fedges, result: &mut HashMap<Pos,i64>, bound: i64)
    where
        Fcost:Fn(Pos,Pos)->i64,
        Fedges:Fn(Pos)->Vec<Pos>,
        Pos:Debug+Default+Clone+Copy+Eq+PartialEq+Ord+PartialOrd+Hash
{
    let mut q = PriorityQueue::<Pos,i64>::new(256);
    q.push(start, 0);
    result.insert(start, 0);
    while let Some(z) = q.pop() {
        let &r_z = result.get(&z).unwrap(); // else logic error
        for &nz in &edges(z) {
            let or_nz = match result.get(&nz) { Some(&r)=>r, None=>i64::MAX };
            let r_nz = r_z + step_cost(z, nz);
            if (r_nz<or_nz) && (r_nz<bound) {
                result.insert(nz, r_nz);
                q.push(nz, r_nz); // TODO: check if already in q and just decrease cost!
            }
        }
    }
}

pub fn astar
    <Pos,Fcost,Fedges,Fbackedges,Festimate>(
        start: Pos,
        goal: Pos,
        step_cost: Fcost,
        edges: Fedges,
        back_edges: Fbackedges,
        estimate: Festimate,
        result: &mut HashMap<Pos, i64>
    ) -> Vec<Pos>
    where
        Fcost:Fn(Pos,Pos)->i64,
        Fedges:Fn(Pos)->Vec<Pos>,
        Fbackedges:Fn(Pos)->Vec<Pos>,
        Festimate:Fn(Pos,Pos)->i64,
        Pos:Debug+Default+Clone+Copy+Eq+PartialEq+Ord+PartialOrd+Hash
{
    let mut q = PriorityQueue::<Pos,i64>::new(256);
    q.push(start, 0);
    result.insert(start, 0);
    while let Some(z) = q.pop() {
        let &r_z = result.get(&z).unwrap(); // else logic error
        if z==goal { break; }
        for &nz in &edges(z) {
            let or_nz = match result.get(&nz) { Some(&r)=>r, None=>i64::MAX };
            let r_nz = r_z + step_cost(z, nz);
            let est_nz = r_nz + estimate(nz, goal);
            if r_nz<or_nz {
                result.insert(nz, r_nz);
                q.push(nz, est_nz); // TODO: should check if nz in q already &c.
            }
        }
    }
    let mut path = vec![goal];
    let mut x = goal;
    while x!=start {
        let mut min_res = i64::MAX;
        let mut px = x;
        for &pz in &back_edges(x) {
            let r = match result.get(&pz) { Some(&r)=>r, None=>i64::MAX };
            if r<min_res {
                px = pz;
                min_res = *result.get(&pz).unwrap();
            }
        }
        if min_res==i64::MAX { break; } // got stuck!
        x = px;
        path.push(x);
    }
    path.reverse();
    path
}


