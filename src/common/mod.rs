use std::borrow::Borrow;

use crate::metrizable::Metrizable;

#[derive(Debug, Clone, PartialEq)]
pub struct Path<T: Metrizable> {
    pub len: f64,
    pub path: Vec<T>,
}

impl<T: Metrizable + Clone + Borrow<T>> Path<T> {
    pub fn new() -> Path<T> {
        Path {
            len: 0.,
            path: Vec::new() as Vec<T>,
        }
    }

    pub fn from(nodes: &Vec<T>) -> Path<T>
    where
        T: Clone,
    {
        Path {
            len: length(nodes),
            path: (*nodes).clone(),
        }
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
