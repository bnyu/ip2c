use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry::{Vacant, Occupied};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Eq)]
pub enum Interval<T: Ord> {
    Range(T, T),
    Point(T),
}

impl<T> PartialOrd for Interval<T> where T: Ord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Not Transitive Equal; Equal only means: intersection is not empty set
impl<T> Ord for Interval<T> where T: Ord {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Interval::Point(x), Interval::Point(y)) => x.cmp(y),
            (Interval::Range(x0, x1), Interval::Range(y0, y1)) => {
                assert_eq!(x0 < x1, true);
                assert_eq!(y0 < y1, true);
                if x1 <= y0 { Ordering::Less } else if x0 >= y1 { Ordering::Greater } else { Ordering::Equal }
            }
            (Interval::Range(r0, r1), Interval::Point(p)) => {
                assert_eq!(r0 < r1, true);
                if r1 <= p { Ordering::Less } else if r0 > p { Ordering::Greater } else { Ordering::Equal }
            }
            (Interval::Point(p), Interval::Range(r0, r1)) => {
                assert_eq!(r0 < r1, true);
                if r1 <= p { Ordering::Less } else if r0 > p { Ordering::Greater } else { Ordering::Equal }.reverse()
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyError {
    Invalid,
    Conflict,
}

impl Display for KeyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyError::Invalid => f.write_str("right key must not less than left"),
            KeyError::Conflict => f.write_str("key interval has conflict with other"),
        }
    }
}

impl Error for KeyError {}

pub struct IntervalTreeMap<K: Ord, V> {
    map: BTreeMap<Interval<K>, V>,
}

impl<K: Ord + Copy, V> IntervalTreeMap<K, V> {
    pub fn new() -> Self {
        IntervalTreeMap {
            map: BTreeMap::new()
        }
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn get(&self, point: K) -> Option<&V> {
        let key = Interval::Point(point);
        self.map.get(&key)
    }

    pub fn get_key_value(&self, point: K) -> Option<(&Interval<K>, &V)> {
        let key = Interval::Point(point);
        self.map.get_key_value(&key)
    }

    pub fn put(&mut self, left: K, right: K, value: V) -> Result<(), KeyError> {
        let key = if left < right {
            Interval::Range(left, right)
        } else if left == right {
            Interval::Point(left)
        } else {
            Err(KeyError::Invalid)?
        };

        match self.map.entry(key) {
            Vacant(e) => e.insert(value),
            Occupied(_) => Err(KeyError::Conflict)?
        };
        Ok(())
    }

    pub fn get_tree(&self) -> &BTreeMap<Interval<K>, V> {
        &self.map
    }
}
