use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry::{Vacant, Occupied};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

/// [Interval] is a closed range.
/// `Scope(0,4)` contains `[0,4]`, can be `from((0,4))`
/// The one element interval, eg:`{0}` also can be `from(0)`
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Interval<T: Ord + Copy>(pub T, pub T);

impl<T: Copy + Ord> From<T> for Interval<T> {
    fn from(t: T) -> Self {
        Interval(t, t)
    }
}

impl<T: Copy + Ord> From<(T, T)> for Interval<T> {
    fn from(t: (T, T)) -> Self {
        Interval(t.0, t.1)
    }
}

impl<T: Ord + Copy> Display for Interval<T> where T: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0 == self.1 {
            f.write_fmt(format_args!("{{{}}}", self.0))
        } else {
            f.write_fmt(format_args!("[{}, {}]", self.0, self.1))
        }
    }
}

impl<T> PartialOrd for Interval<T> where T: Ord + Copy {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// `==` Eq is Transitive Equal. But `Ord::cmp` return `Ordering::Equal` only means that intersection is non empty set
impl<T> Ord for Interval<T> where T: Ord + Copy {
    fn cmp(&self, that: &Self) -> Ordering {
        debug_assert_eq!(self.0 <= self.1, true);
        debug_assert_eq!(that.0 <= that.1, true);
        if self.1 < that.0 { Ordering::Less } else if self.0 > that.1 { Ordering::Greater } else { Ordering::Equal }
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
            IntervalError::Invalid(kk) => f.write_fmt(format_args!("interval invalid. consider replace {} with {}", &kk[0], &kk[1])),
            IntervalError::Conflict(kk) => f.write_fmt(format_args!("interval conflict. consider insert {} after delete {}", &kk[0], &kk[1])),
        }
    }
}

impl<K: Ord + Copy + Display> Debug for IntervalError<K> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl<K: Ord + Copy + Display> Error for IntervalError<K> {}

/// An ordered interval tree map.
/// base on [BTreeMap]
// the inner map base on `BTreeMap` struct may change
pub struct IntervalTreeMap<K: Ord + Copy, V> {
    map: BTreeMap<Interval<K>, V>,
}

impl<K: Ord + Copy, V> IntervalTreeMap<K, V> {
    pub fn new() -> Self {
        IntervalTreeMap {
            map: BTreeMap::new()
        }
    }

    /// Returns the number of interval items in the map.
    ///
    /// # Examples
    /// Basic usage:
    ///
    /// ```
    /// use ip2c::IntervalTreeMap;
    ///
    /// let mut map = IntervalTreeMap::new();
    /// let _ = map.insert_scope(0, 4, "a");
    /// assert_eq!(map.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns a reference to the value corresponding to the point.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// use ip2c::IntervalTreeMap;
    ///
    /// let mut map = IntervalTreeMap::new();
    /// let _ = map.insert_scope(0, 4, "a");
    /// assert_eq!(map.query(0), Some(&"a"));
    /// assert_eq!(map.query(3), Some(&"a"));
    /// assert_eq!(map.query(4), Some(&"a"));
    /// assert_eq!(map.query(5), None);
    /// ```
    pub fn query(&self, point: K) -> Option<&V> {
        let key = Interval(point, point);
        self.map.get(&key)
    }

    /// Returns the key-value pair corresponding to the point.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// use ip2c::{Interval, IntervalTreeMap};
    ///
    /// let mut map = IntervalTreeMap::new();
    /// let _ = map.insert_scope(0, 4, "a");
    /// assert_eq!(map.get_key_value(3), Some((&Interval(0, 4), &"a")));
    /// ```
    pub fn get_key_value(&self, point: K) -> Option<(&Interval<K>, &V)> {
        let key = Interval(point, point);
        self.map.get_key_value(&key)
    }

    /// Inserts an interval-value pair into the map.
    ///
    /// If the interval is invalid(eg. `Interval(5, 3)`), [IntervalError::Invalid]` is returned
    /// If the new interval has conflict with others that already in the map, [IntervalError::Conflict] is returned
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// use ip2c::{Interval, IntervalTreeMap};
    ///
    /// let mut map = IntervalTreeMap::new();
    /// let _ = map.insert(Interval(0, 4), "a");
    /// assert_eq!(map.get_key_value(3), Some((&Interval(0, 4), &"a")));
    /// ```
    pub fn insert(&mut self, key: Interval<K>, value: V) -> Result<(), IntervalError<K>> {
        if key.0 > key.1 {
            Err(IntervalError::Invalid([key, Interval(key.1, key.0)]))?
        }
        self._insert(key, value)
    }

    /// insert a left close right close interval key of value
    pub fn insert_scope(&mut self, left: K, right: K, value: V) -> Result<(), IntervalError<K>> {
        self.insert(Interval(left, right), value)
    }

    /// insert a single point interval key of value
    pub fn insert_point(&mut self, point: K, value: V) -> Result<(), IntervalError<K>> {
        self.insert(Interval(point, point), value)
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

    /// Removes an interval from the map, returning the value of the interval if the exactly same interval was previously in the map.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// use ip2c::{Interval, IntervalTreeMap};
    ///
    /// let mut map = IntervalTreeMap::new();
    /// let _ = map.insert(Interval(0, 4), "a");
    /// let _ = map.insert(Interval(6, 7), "b");
    /// assert_eq!(map.remove(&Interval(2, 9)), None);
    /// assert_eq!(map.remove(&Interval(6, 6)), None);
    /// assert_eq!(map.query(6), Some(&"b"));
    /// assert_eq!(map.remove(&Interval(6, 7)), Some("b"));
    /// assert_eq!(map.query(6), None);
    /// ```
    pub fn remove(&mut self, key: &Interval<K>) -> Option<V> {
        let (k, _) = self.map.get_key_value(key)?;
        if k == key {
            return self._remove(key);
        }
        None
    }

    fn _remove(&mut self, key: &Interval<K>) -> Option<V> {
        self.map.remove(&key)
    }

    /// Return the reference of the inner map, so you can iterate on it.
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
        let r = map.insert_scope(100, 200, 'A');
        assert_eq!(r, Ok(()));
        let r = map.query(200);
        assert_eq!(r, Some(&'A'));
        let r = map.query(201);
        assert_eq!(r, None);
        let r = map.insert_scope(200, 300, 'B');
        assert_ne!(r, Ok(()));
        let r = map.insert_scope(201, 300, 'S');
        assert_eq!(r, Ok(()));
        let r = map.query(200);
        assert_eq!(r, Some(&'A'));
        let r = map.query(300);
        assert_eq!(r, Some(&'S'));
        let r = map.insert_scope(450, 330, 'D');
        assert_ne!(r, Ok(()));
        let r = map.insert_scope(150, 230, 'F');
        assert_ne!(r, Ok(()));
        let r = map.insert_scope(500, 600, 'A');
        assert_eq!(r, Ok(()));
        let r = map.query(501);
        assert_eq!(r, Some(&'A'));
    }

    #[test]
    fn remove() {
        let mut map = IntervalTreeMap::new();
        let _ = map.insert(Interval(10, 20), true);
        let _ = map.insert(Interval(30, 80), true);
        let _ = map.insert(Interval(100, 100), true);
        let r = map.remove(&Interval(15, 50));
        assert_eq!(r, None);
        assert_eq!(map.query(15), Some(&true));
        let r = map.remove(&Interval(10, 20));
        assert_eq!(r, Some(true));
        assert_eq!(map.query(15), None);
        let r = map.remove(&Interval(100, 101));
        assert_eq!(r, None);
        assert_eq!(map.query(100), Some(&true));
        let r = map.remove(&Interval(100, 100));
        assert_eq!(r, Some(true));
        assert_eq!(map.query(100), None);
    }
}
