use std::collections::{BTreeMap, btree_map::{self, Entry}};
use std::iter::Peekable;
use std::ops::Range;

#[derive(Debug)]
pub struct RangeMap<K, V, M>(V, BTreeMap<K, (V, Vec<M>)>);

impl<K: Ord, V: Eq + Clone, M> RangeMap<K, V, M> {
    pub fn new(d: V) -> Self {
        RangeMap(d, BTreeMap::default())
    }
    pub fn set(&mut self, r: Range<K>, v: V, m: impl Into<Option<M>>) {
        let m = m.into();
        let mut post_meta = vec![];
        match self.1.entry(r.end) {
            Entry::Vacant(x) => {
                x.insert((self.0.clone(), vec![]));
            }
            Entry::Occupied(x) => {
                if x.get().0 == v {
                    post_meta = x.remove().1;
                }
            }
        }
        if let Some(prev) = self.1.range_mut(..&r.start).next_back() {
            if (prev.1).0 == v {
                (prev.1).1.extend(m);
                (prev.1).1.extend(post_meta);
                self.1.remove(&r.start);
                return;
            }
        }
        let mut m = m.into_iter().collect::<Vec<_>>();
        m.extend(post_meta);
        self.1.insert(r.start, (v, m));
    }
    pub fn iter(&self) -> <&Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
}

pub struct Iter<'a, K, V, M> {
    iter: Peekable<btree_map::Iter<'a, K, (V, Vec<M>)>>,
    default: &'a V,
}

impl<'a, K, V: Eq, M> Iterator for Iter<'a, K, V, M> {
    type Item = (Range<&'a K>, &'a V, &'a [M]);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let first = self.iter.next()?;
            let second = self.iter.peek()?;
            let value = first.1;
            if &value.0 != self.default {
                return Some((first.0 .. second.0, &value.0, &value.1));
            }
        }
    }
}

impl<'a, K, V: Eq, M> IntoIterator for &'a RangeMap<K, V, M> {
    type IntoIter = Iter<'a, K, V, M>;
    type Item = <Self::IntoIter as Iterator>::Item;
    fn into_iter(self) -> Self::IntoIter {
        Iter {
            iter: self.1.iter().peekable(),
            default: &self.0,
        }
    }
}