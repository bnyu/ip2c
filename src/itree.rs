use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry::{Vacant, Occupied};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Interval<T: Ord + Copy> {
    Range(T, T),
    Point(T),
}

impl<T: Ord + Copy> Display for Interval<T> where T: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Interval::Range(x, y) => f.write_fmt(format_args!("[{}, {})", x, y)),
            Interval::Point(x) => f.write_fmt(format_args!("{{{}}}", x)),
        }
    }
}

impl<T> PartialOrd for Interval<T> where T: Ord + Copy {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Not Transitive Equal; Equal only means: intersection is not empty set
impl<T> Ord for Interval<T> where T: Ord + Copy {
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum IntervalError<K: Ord + Copy> {
    Invalid([Interval<K>; 2]),
    Conflict([Interval<K>; 2]),
}

impl<K: Ord + Copy + Display> Display for IntervalError<K> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IntervalError::Invalid(kk) => f.write_fmt(format_args!("key should be left close right open interval, {} ---> {}", &kk[0], &kk[1])),
            IntervalError::Conflict(kk) => f.write_fmt(format_args!("key interval range is conflicted with other, {} <--> {}", &kk[0], &kk[1])),
        }
    }
}

impl<K: Ord + Copy + Display> Debug for IntervalError<K> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl<K: Ord + Copy + Display> Error for IntervalError<K> {}

pub struct IntervalTreeMap<K: Ord + Copy, V> {
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

    pub fn query(&self, point: K) -> Option<&V> {
        let key = Interval::Point(point);
        self.map.get(&key)
    }

    pub fn get_key_value(&self, point: K) -> Option<(&Interval<K>, &V)> {
        let key = Interval::Point(point);
        self.map.get_key_value(&key)
    }

    pub fn insert_interval(&mut self, key: Interval<K>, value: V) -> Result<(), IntervalError<K>> {
        match key {
            Interval::Range(a, b) if a >= b => Err(IntervalError::Invalid([key, if a == b { Interval::Point(a) } else { Interval::Range(b, a) }]))?,
            _ => {}
        };
        self._insert(key, value)
    }

    pub fn insert(&mut self, left: K, right: K, value: V) -> Result<(), IntervalError<K>> {
        let key = if left < right {
            Interval::Range(left, right)
        } else if left == right {
            Interval::Point(left)
        } else {
            Err(IntervalError::Invalid([Interval::Range(left, right), Interval::Range(right, left)]))?
        };

        self._insert(key, value)
    }

    fn _insert(&mut self, key: Interval<K>, value: V) -> Result<(), IntervalError<K>> {
        match self.map.entry(key) {
            Vacant(e) => e.insert(value),
            Occupied(e) => {
                let k1 = e.key();
                Err(IntervalError::Conflict([key, *k1]))?
            }
        };
        Ok(())
    }

    pub fn remove_interval(&mut self, key: &Interval<K>) -> Option<V> {
        let (k, _) = self.map.get_key_value(key)?;
        if k == key {
            return self._remove(key);
        }
        None
    }

    fn _remove(&mut self, key: &Interval<K>) -> Option<V> {
        self.map.remove(&key)
    }

    pub fn tree(&self) -> &BTreeMap<Interval<K>, V> {
        &self.map
    }
}


#[cfg(test)]
mod tests {
    use crate::Interval;
    use super::IntervalTreeMap;

    #[test]
    fn put_get() {
        let mut map = IntervalTreeMap::new();
        let r = map.insert(100, 200, 'A');
        assert_eq!(r, Ok(()));
        let r = map.insert(200, 300, 'S');
        assert_eq!(r, Ok(()));
        let r = map.query(110);
        assert_eq!(r, Some(&'A'));
        let r = map.query(200);
        assert_eq!(r, Some(&'S'));
        let r = map.query(299);
        assert_eq!(r, Some(&'S'));
        let r = map.query(300);
        assert_eq!(r, None);
        let r = map.insert(450, 330, 'D');
        assert_ne!(r, Ok(()));
        let r = map.insert(150, 230, 'F');
        assert_ne!(r, Ok(()));
        let r = map.insert(500, 600, 'A');
        assert_eq!(r, Ok(()));
        let r = map.query(501);
        assert_eq!(r, Some(&'A'));
    }

    #[test]
    fn remove() {
        let mut map = IntervalTreeMap::new();
        let _ = map.insert_interval(Interval::Range(10, 20), true);
        let _ = map.insert_interval(Interval::Range(30, 80), true);
        let _ = map.insert_interval(Interval::Point(100), true);
        let r = map.remove_interval(&Interval::Range(15, 50));
        assert_eq!(r, None);
        assert_eq!(map.query(15), Some(&true));
        let r = map.remove_interval(&Interval::Range(10, 20));
        assert_eq!(r, Some(true));
        assert_eq!(map.query(15), None);
        let r = map.remove_interval(&Interval::Range(100, 101));
        assert_eq!(r, None);
        assert_eq!(map.query(100), Some(&true));
        let r = map.remove_interval(&Interval::Point(100));
        assert_eq!(r, Some(true));
        assert_eq!(map.query(100), None);
    }
}
