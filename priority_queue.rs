use std::fmt::{Debug,Display};

struct Elem<T,P>
    where T:Clone+Copy+Debug+Eq+PartialEq, P:Clone+Copy+Debug+Eq+PartialEq+Ord+PartialOrd
{
    data: T,
    priority: P,
}

pub struct PriorityQueue<T,P>
    where T:Clone+Copy+Debug+Eq+PartialEq, P:Clone+Copy+Debug+Eq+PartialEq+Ord+PartialOrd
{
    num: usize,
    allocated: usize,
    buffer: Vec<Elem<T,P>>,
}
impl<T,P> PriorityQueue<T,P>
    where T:Clone+Copy+Debug+Eq+PartialEq, P:Clone+Copy+Debug+Eq+PartialEq+Ord+PartialOrd
{
    pub fn new(size: usize) -> Self {
        let size = if size<4 {4} else {size};
        let mut pq = PriorityQueue { num:1, allocated:size, buffer:Vec::new() };
        pq.buffer.reserve(1);
        pq
    }
    fn resize(&mut self, newalloc: usize) {
        //buffer.reserve(newalloc);
        //buffer.length = newalloc;
        //allocated = newalloc;
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
}


/*
    bool change_priority(T data, P priority) {
        if (!remove(data))
            return false;
        push(data, priority);
        return true;
    }
    bool remove(T data) {
        int i = find(data);
        if (!i) return false;
        --num;
        int n = bubble_down(i, buffer[num].priority);
        buffer[n] = buffer[num];
        return true;
    }
    void push(T data, P priority) {
        if (num >= allocated)
            resize(allocated*2);
        // append at end, then up heap
        int n = bubble_up(num++, priority);
        buffer[n].data = data;
        buffer[n].priority = priority;
    }
    P top_priority() const {
        return buffer[(num == 1) ? 0 : 1].priority;
    }
    T top() const {
        P priority;
        return top(priority);
    }
    T top(ref P priority) const {
        if (num == 1) {
            priority = buffer[0].priority;
            return cast(T)buffer[0].data;
        } else {
            priority = buffer[1].priority;
            return cast(T)buffer[1].data;
        }
    }
    T pop() {
        P priority;
        return pop(priority);
    }
    T pop(ref P priority) {
        if (num==1) return buffer[0].data;
        T out_data = buffer[1].data;
        priority = buffer[1].priority;
        // pull last item to top, then down heap
        --num;
        int n = bubble_down(1, buffer[num].priority);
        buffer[n] = buffer[num];
        return out_data;
    }

    public int opApply(int delegate(ref T) dg) {
        foreach (e; buffer[1..num])
            if (int res = dg(e.data))
                return res;
        return 0;
    }

    public int opApply(int delegate(const ref T) dg) const {
        foreach (e; buffer[1..num])
            if (int res = dg(e.data))
                return res;
        return 0;
    }

    public int opApply(int delegate(ref T, const ref P) dg) {
        foreach (e; buffer[1..num])
            if (int res = dg(e.data, e.priority))
                return res;
        return 0;
    }

    public int opApply(int delegate(const ref T, const ref P) dg) const {
        foreach (e; buffer[1..num])
            if (int res = dg(e.data, e.priority))
                return res;
        return 0;
    }
}
*/

fn main() {
    println!(".");
}
