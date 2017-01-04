// quick and dirty implementation of priority-queue

use std::fmt::{Debug};

#[derive(Clone,Copy,Debug,Default,Eq,PartialEq,Ord,PartialOrd)]
struct Elem<T,P>
    where T:Clone+Copy+Debug+Default+Eq+PartialEq, P:Clone+Copy+Debug+Default+Eq+PartialEq+Ord+PartialOrd
{
    data: T,
    priority: P,
}

#[derive(Debug)]
pub struct PriorityQueue<T,P>
    where T:Clone+Copy+Debug+Default+Eq+PartialEq, P:Clone+Copy+Debug+Default+Eq+PartialEq+Ord+PartialOrd
{
    num: usize,
    allocated: usize,
    buffer: Vec<Elem<T,P>>,
}
impl<T,P> PriorityQueue<T,P>
    where T:Clone+Copy+Debug+Default+Eq+PartialEq, P:Clone+Copy+Debug+Default+Eq+PartialEq+Ord+PartialOrd
{
    pub fn new(size: usize) -> Self {
        let size = if size<4 {4} else {size};
        let mut pq = PriorityQueue { num:1, allocated:size, buffer:Vec::new() };
        pq.resize(size);
        pq
    }
    fn resize(&mut self, newalloc: usize) {
        while self.buffer.len()<newalloc {
            self.buffer.push(Elem{data:T::default(), priority:P::default()});
        }
        self.allocated = newalloc;
    }
    fn find(&self, data: T) -> Option<usize> {
        for i in 1..self.buffer.len() {
            if data==self.buffer[i].data {
                return Some(i);
            }
        }
        return None;
    }
    fn bubble_down(&mut self, n: usize, priority: P) -> usize {
        let mut n = n;
        while n*2<self.num {
            let mut m = n*2;
            if (m+1)<self.num && self.buffer[m].priority>self.buffer[m+1].priority {
                m += 1;
            }
            if priority<=self.buffer[m].priority {
                break;
            }
            self.buffer.swap(n, m);
            n = m;
        }
        n
    }
    fn bubble_up(&mut self, n: usize, priority: P) -> usize {
        let mut n = n;
        while n/2!=0 && priority<self.buffer[n/2].priority {
            let m = n/2;
            self.buffer.swap(n, m);
            n = m;
        }
        n
    }
    pub fn purge(&mut self) {
        self.num = 1;
    }
    pub fn size(&self) -> usize {
        self.num - 1
    }
    pub fn is_in(&self, data: T) -> bool {
        self.find(data) != None
    }
    pub fn push(&mut self, data: T, priority: P) {
        if self.num >= self.allocated { let sa=self.allocated; self.resize(sa*2); }
        // append at end, then up heap
        let sn = self.num;
        let n = self.bubble_up(sn, priority);
        self.num += 1;
        self.buffer[n].data = data;
        self.buffer[n].priority = priority;
    }
    pub fn top(&self) -> Option<T> {
        if self.num==1 {None} else {Some(self.buffer[1].data)}
    }
    pub fn top_priority(&self) -> Option<P> {
        if self.num==1 {None} else {Some(self.buffer[1].priority)}
    }
    pub fn top_entry(&self) -> Option<(T,P)> {
        if self.num==1 {None} else {Some((self.buffer[1].data, self.buffer[1].priority))}
    }
    pub fn pop(&mut self) -> Option<T> {
        self.pop_entry().map(|(t,_)|t)
    }
    pub fn pop_priority(&mut self) -> Option<P> {
        self.pop_entry().map(|(_,p)|p)
    }
    pub fn pop_entry(&mut self) -> Option<(T,P)> {
        if self.num==1 { return None; }
        let out_data = self.buffer[1].data;
        let out_priority = self.buffer[1].priority;
        self.num -= 1;
        let p = self.buffer[self.num].priority;
        let n = self.bubble_down(1, p);
        self.buffer.swap(n, self.num);
        Some((out_data, out_priority))
    }
    pub fn change_priority(&mut self, data: T, priority: P) -> bool {
        if !self.remove(data) { return false; }
        self.push(data, priority);
        true
    }
    pub fn remove(&mut self, data: T) -> bool {
        let i = match self.find(data) {
            Some(i) => i, None => { return false; }
        };
        self.num -= 1;
        let p = self.buffer[self.num].priority;
        let n = self.bubble_down(i, p);
        self.buffer.swap(n, self.num);
        true
    }
}
