use std::collections::HashMap;
use std::hash::Hash;

use itertools::Itertools;

/// Frequency distribution maintains the frequency of T values.
#[derive(Clone, Debug, Default)]
pub struct FrequencyDistribution<T>
where
    T: Eq + Hash,
{
    values: HashMap<T, usize>,
    total: usize,
}

impl<T: Eq + Hash> FrequencyDistribution<T> {
    pub fn total(&self) -> usize {
        return self.total;
    }

    pub fn unique(&self) -> usize {
        return self.values.len();
    }

    pub fn frequencies(&self) -> FrequencyDistribution<usize> {
        let mut fs = FrequencyDistribution::default();
        for count in self.values.values() {
            fs += *count
        }
        fs
    }
}

impl<T: Eq + Hash + Ord> FrequencyDistribution<T> {
    pub fn quantiles(&self, q: usize) -> Vec<&T> {
        if q == 0 || self.total == 0 {
            return vec![];
        }

        let mut it = self.values.iter().sorted_by_key(|e| e.0);
        let mut total_count = 0;
        let mut last_value;
        let mut result = Vec::new();

        if let Some((value, count)) = it.next() {
            total_count += count;
            last_value = value;
        } else {
            return vec![];
        }
        result.push(last_value);

        for k in 1..=q {
            let limit = ((self.total as f64 * k as f64) / q as f64).round() as usize;
            while total_count < limit {
                if let Some((value, count)) = it.next() {
                    total_count += count;
                    last_value = value;
                } else {
                    break;
                }
            }
            result.push(last_value);
        }

        result
    }
}

impl<T> std::ops::AddAssign<T> for FrequencyDistribution<T>
where
    T: Eq + Hash,
{
    fn add_assign(&mut self, rhs: T) {
        *self.values.entry(rhs).or_default() += 1;
        self.total += 1;
    }
}

impl<T> std::ops::AddAssign<&Self> for FrequencyDistribution<T>
where
    T: Eq + Hash + Clone,
{
    fn add_assign(&mut self, rhs: &Self) {
        for (value, count) in &rhs.values {
            *self.values.entry(value.clone()).or_default() += count;
        }
        self.total += rhs.total;
    }
}
