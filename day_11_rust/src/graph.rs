use std::hash::Hash;
use std::collections::{HashMap, VecDeque};
use std::marker::Sized;

pub trait Graph {
    fn adjacent(&self) -> Vec<Self> where Self: Sized;

    fn bfs<'a, T>(source: &'a T, end: &'a T) -> usize
        where T: Graph + Hash + Clone + Eq
    {
        let mut dist: HashMap<T, usize> = HashMap::new();
        let mut q = VecDeque::new();

        dist.insert(source.clone(), 0_usize);
        q.push_back(source.clone());

        while !q.is_empty() {
            let current = q.pop_front().unwrap();
            let current_dist = dist[&current];
            
            for n in current.adjacent() {
                let mut entry = dist.entry(n.clone()).or_insert(::std::usize::MAX);
                if *entry > current_dist + 1 {
                    *entry = current_dist + 1;
                    q.push_back(n.clone());
                }
            }
        }
        dist[end]
    }
}
