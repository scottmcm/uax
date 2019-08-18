#[derive(Debug, Copy, Clone)]
pub struct LookupTable<'a, K, V> {
    starts: &'a [K],
    ends: &'a [K],
    values: &'a [V],
}

impl<'a, K, V> LookupTable<'a, K, V> {
    #[inline]
    pub const fn new(starts: &'a [K], ends: &'a [K], values: &'a [V]) -> Self {
        Self { starts, ends, values }
    }
}

impl<'a, K: Ord, V: Copy> LookupTable<'a, K, V> {
    #[cfg(test)]
    #[inline]
    pub fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    #[cfg(test)]
    #[inline]
    pub fn get(&self, key: &K) -> Option<V> {
        self.get_or(key, None)
    }

    #[inline]
    pub fn get_or<F>(&self, key: &K, fallback: F) -> F
        where V: Into<F>
    {
        assert!(self.starts.len() == self.ends.len());
        assert!(self.starts.len() == self.values.len());

        if self.starts.len() == 0 || key < &self.starts[0] {
            return fallback;
        }

        let mut width = self.starts.len();
        let mut low = 0;
        while width > 1 {
            let mid = low + width / 2;
            let mid_element = unsafe { self.starts.get_unchecked(mid) };
            if mid_element <= key {
                low = mid;
            }
            width -= width / 2;
        }
        let end = unsafe { self.ends.get_unchecked(low) };
        if key <= end {
            let value = unsafe { *self.values.get_unchecked(low) };
            value.into()
        } else {
            fallback
        }
    }
}

#[cfg(test)]
impl<'a, K: Copy + Ord + std::fmt::Debug, V: Copy + Eq> LookupTable<'a, K, V>
    where std::ops::Range<K>: Iterator
{
    pub fn validate(&self) {
        assert!(self.starts.len() == self.ends.len());
        assert!(self.starts.len() == self.values.len());

        if self.starts.is_empty() { return }

        let ranges = Iterator::zip(
            self.starts.iter().copied(),
            self.starts.iter().copied(),
        ).map(|(a, b)| a..=b);
        let mut pairs = ranges.zip(self.values.iter().copied());

        let mut prev = pairs.next().unwrap();
        assert!(prev.0.start() <= prev.0.end());
        for this in pairs {
            assert!(this.0.start() <= this.0.end());
            assert!(prev.0.end() < this.0.start());

            let r = (*prev.0.end())..(*this.0.start());
            assert!(prev.1 != this.1 || r.clone().count() > 1,
                "ranges should have been joined; {:?} are too close", r);

            prev = this;
        }
    }
}

/// This uses rvalue-static promotion on the internal slices,
/// which marks them as `one_only`, so the `LookupTable` itself
/// can just be put in a `const` since its metadata is fine to duplicate.
#[macro_export]
macro_rules! lookup_table {
    ($($tuple:expr,)+) => {
        LookupTable::new(
            &[
                $( ($tuple).0, )+
            ],
            &[
                $( ($tuple).1, )+
            ],
            &[
                $( ($tuple).2, )+
            ],
        )
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    static TABLE: LookupTable<u32, i8> = lookup_table![
        (1, 2, 3),
        (4, 5, 6),
        (7, 7, 7),
    ];

    #[test]
    fn it_works() {
        assert_eq!(TABLE.get_or(&0, 0), 0);
        assert_eq!(TABLE.get_or(&1, 0), 3);
        assert_eq!(TABLE.get_or(&2, 0), 3);
        assert_eq!(TABLE.get_or(&3, 0), 0);
        assert_eq!(TABLE.get_or(&4, 0), 6);
        assert_eq!(TABLE.get_or(&5, 0), 6);
        assert_eq!(TABLE.get_or(&6, 0), 0);
        assert_eq!(TABLE.get_or(&7, 0), 7);
        assert_eq!(TABLE.get_or(&8, 0), 0);
    }

    #[test]
    fn can_validate() {
        TABLE.validate();
    }
}
