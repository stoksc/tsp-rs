use std::collections::HashSet;

use crate::{IndexedT, Metrizable};

#[inline]
pub(crate) fn nearest_neighbor<'a, T>(
    node: &T,
    others: &Vec<IndexedT<&'a T>>,
    visited: &mut HashSet<usize>,
) -> Option<&'a T>
where
    T: Metrizable,
{
    let mut nearest = std::f64::MAX;
    let mut nearest_node = None;

    for other in others {
        let dist = node.cost(&other.value);
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
