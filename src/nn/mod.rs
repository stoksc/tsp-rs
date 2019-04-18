use std::borrow::Borrow;
use std::collections::HashSet;

use rand::Rng;

use crate::common::{self, Path};
use crate::metrizable::Metrizable;

impl<T: Metrizable + Clone + Borrow<T>> Path<T> {
    pub fn solve_nn(&mut self)
    where
        T: Metrizable + Clone,
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
