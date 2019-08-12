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
    #[inline]
    pub fn get_or(&self, key: &K, fallback: V) -> V {
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
            unsafe { *self.values.get_unchecked(low) }
        } else {
            fallback
        }
    }
}

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
}
