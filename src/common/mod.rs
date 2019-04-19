use std::borrow::Borrow;

use crate::metrizable::Metrizable;

#[derive(Debug, Clone, PartialEq)]
pub struct Tour<T: Metrizable> {
    pub path: Vec<T>,
}

impl<T: Metrizable + Clone + Borrow<T>> Tour<T> {
    pub fn new() -> Tour<T> {
        Tour {
            path: Vec::new() as Vec<T>,
        }
    }

    pub fn from(nodes: &Vec<T>) -> Tour<T>
    where
        T: Clone,
    {
        Tour {
            path: (*nodes).clone(),
        }
    }

    pub fn path_len(&self) -> f64 {
        if self.path.len() <= 0 {
            return 0.;
        }

        let mut sum = 0.;
        let mut prev = self.path.last().unwrap();
        for curr in &self.path {
            sum += prev.cost(&curr);
            prev = &curr;
        }
        sum
    }
}

#[derive(Clone)]
pub struct IndexedT<T> {
    pub index: usize,
    pub value: T,
}

#[inline]
pub fn index_path<T>(path: &Vec<T>) -> Vec<IndexedT<&T>> {
    path.iter()
        .enumerate()
        .map(|(index, value)| IndexedT { index, value })
        .collect()
}
