use crate::metrizable::Metrizable;

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
