mod kopt;
pub mod point;

use rand::Rng;
use std::borrow::Borrow;
use std::collections::HashSet;
use std::time;

pub trait Metrizable {
    fn distance(&self, other: &Self) -> f64;

    fn nearest_neighbor<'a>(
        &self,
        others: &Vec<IndexedT<&'a Self>>,
        visited: &mut HashSet<usize>,
    ) -> Option<&'a Self>
    where
        Self: Sized + Clone,
    {
        let mut nearest = std::f64::MAX;
        let mut nearest_node = None;

        for other in others {
            let dist = self.distance(&other.value);
            if dist < nearest && !visited.contains(&other.index) {
                nearest = dist;
                nearest_node = Some(other);
            }
        }

        if let Some(nearest_node) = nearest_node {
            visited.insert(nearest_node.index);
            Some(nearest_node.value)
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct IndexedT<T> {
    index: usize,
    value: T,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Path<T: Metrizable> {
    pub len: f64,
    pub path: Vec<T>,
}

impl<T: Metrizable + Clone + Borrow<T>> Path<T> {
    pub fn new(nodes: &Vec<T>) -> Path<T>
    where
        T: Clone,
    {
        Path {
            len: length(nodes),
            path: (*nodes).clone(),
        }
    }

    pub fn solve_kopt(&mut self, timeout: time::Duration) {
        const MAX_ITER_WITHOUT_IMPR: u32 = 100;
        let mut iter_without_impr = 0;
        let mut previous_length: f64 = std::f64::MAX;
        let start_time = time::Instant::now();
        loop {
            match crate::kopt::two_opt(&mut self.path) {
                Some(x) => {
                    iter_without_impr = 0;
                    previous_length = x;
                }
                None => {
                    iter_without_impr += 1;
                    if iter_without_impr < MAX_ITER_WITHOUT_IMPR {
                        iter_without_impr = 0;
                        crate::kopt::n_opt(4, &mut self.path, previous_length);
                    }
                }
            }
            if start_time.elapsed() > timeout {
                break;
            }
        }
        self.len = length(&self.path);
    }

    pub fn solve_nn(&mut self)
    where
        T: Metrizable + Clone,
    {
        let mut path = Vec::new();
        let nodes = index_path(&self.path);
        let mut visited = HashSet::new();

        let start_index: usize = rand::thread_rng().gen_range(0, nodes.len());
        let mut curr = &nodes[start_index].value.clone();
        path.push(curr.clone());
        visited.insert(nodes[start_index].index);

        loop {
            match curr.nearest_neighbor(&nodes, &mut visited) {
                Some(next) => {
                    path.push(next.clone());
                    curr = next;
                }
                None => {
                    if visited.len() == nodes.len() {
                        break;
                    }
                }
            };
        }

        self.len = length(&path);
        self.path = path;
    }
}

#[inline]
fn index_path<T>(path: &Vec<T>) -> Vec<IndexedT<&T>> {
    path.iter()
        .enumerate()
        .map(|(index, value)| IndexedT { index, value })
        .collect()
}

#[inline]
pub fn length<T>(v: &Vec<T>) -> f64
where
    T: Metrizable,
{
    if v.len() <= 0 {
        return 0.;
    }

    let mut sum = 0.;
    let mut prev = v.last().unwrap();
    for curr in v {
        sum += prev.distance(curr);
        prev = curr;
    }
    sum
}
