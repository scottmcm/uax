#![cfg_attr(test, allow(dead_code))]

mod property_enums;
use property_enums::*;

mod word_break_table;

#[inline]
pub(crate) fn table_lookup<T: Ord, V: Copy>(starts: &[T], needle: &T, values: &[V]) -> V {
    assert!(starts.len() == values.len());

    let mut width = starts.len();
    let mut low = 0;
    while width > 1 {
        let mid = low + width/2;
        let mid_element = unsafe { starts.get_unchecked(mid) };
        if mid_element <= needle {
            low = mid;
        }
        width -= width/2;
    }
    unsafe { *values.get_unchecked(low) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Word_Break::from(' '), Word_Break::WSegSpace);
        assert_eq!(Word_Break::from('\r'), Word_Break::CR);
        assert_eq!(Word_Break::from('\n'), Word_Break::LF);
        assert_eq!(Word_Break::from('\u{0374}'), Word_Break::ALetter);
        assert_eq!(Word_Break::from('\u{05D0}'), Word_Break::Hebrew_Letter);
        assert_eq!(Word_Break::from('\u{05D1}'), Word_Break::Hebrew_Letter);
        assert_eq!(Word_Break::from('\u{05E9}'), Word_Break::Hebrew_Letter);
        assert_eq!(Word_Break::from('\u{05EA}'), Word_Break::Hebrew_Letter);

        assert_eq!(Word_Break::from('\u{00}'), Word_Break::Other);
        assert_eq!(Word_Break::from('\u{7F}'), Word_Break::Other);
        assert_eq!(Word_Break::from('\u{80}'), Word_Break::Other);
        assert_eq!(Word_Break::from('\u{E01F0}'), Word_Break::Other);
        assert_eq!(Word_Break::from('\u{E00FF}'), Word_Break::Other);
        assert_eq!(Word_Break::from('\u{10FFFF}'), Word_Break::Other);
    }
}
