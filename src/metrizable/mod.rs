use crate::common;
use std::collections::HashSet;

pub trait Metrizable {
    fn distance(&self, other: &Self) -> f64;

    fn nearest_neighbor<'a>(
        &self,
        others: &Vec<common::IndexedT<&'a Self>>,
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
