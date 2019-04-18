use crate::common;
use crate::metrizable::Metrizable;
use itertools::Itertools;
use rand::Rng;
use std::collections::HashSet;

#[inline]
pub fn two_opt<T>(v: &mut Vec<T>) -> Option<f64>
where
    T: Metrizable,
{
    let p1: usize = rand::thread_rng().gen_range(0, v.len());
    let p2: usize = rand::thread_rng().gen_range(0, v.len());

    if p1 == p2 {
        return None;
    }

    let prev_len = common::length(&v);
    v.swap(p1, p2);

    if common::length(&v) < prev_len {
        Some(prev_len - common::length(&v))
    } else {
        v.swap(p1, p2);
        None
    }
}

#[inline]
pub fn n_opt<T>(n: usize, v: &mut Vec<T>, l: f64) -> Option<f64>
where
    T: Metrizable,
{
    let s = v.len();
    let mut unq_points: HashSet<usize> = HashSet::new();
    while unq_points.len() < n {
        let p: usize = rand::thread_rng().gen_range(0, s);
        unq_points.insert(p);
    }
    let mut swapped = Vec::new();
    for (p1, p2) in unq_points.iter().tuples() {
        swapped.push((p1.clone(), p2.clone()));
        v.swap(*p1, *p2);
    }
    let nl: f64 = common::length(v);
    if nl > l {
        for (p1, p2) in swapped.iter().rev() {
            v.swap(*p2, *p1);
        }
        return None;
    }
    return Some(nl);
}
