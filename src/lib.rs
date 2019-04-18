mod common;
mod kopt;
pub mod metrizable;
pub mod point;

use rand::Rng;
use std::borrow::Borrow;
use std::collections::HashSet;
use std::time;

#[derive(Debug, Clone, PartialEq)]
pub struct Path<T: metrizable::Metrizable> {
    pub len: f64,
    pub path: Vec<T>,
}

impl<T: metrizable::Metrizable + Clone + Borrow<T>> Path<T> {
    pub fn new(nodes: &Vec<T>) -> Path<T>
    where
        T: Clone,
    {
        Path {
            len: common::length(nodes),
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
        self.len = common::length(&self.path);
    }

    pub fn solve_nn(&mut self)
    where
        T: metrizable::Metrizable + Clone,
    {
        let mut path = Vec::new();
        let nodes = common::index_path(&self.path);
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

        self.len = common::length(&path);
        self.path = path;
    }
}
