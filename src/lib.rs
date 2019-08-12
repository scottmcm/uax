#![cfg_attr(test, allow(dead_code))]

#[macro_use]
mod lookup_table;

mod property_enums;
use property_enums::*;

mod word_break_table;

pub fn demo(c: char) -> usize {
    Word_Break::from(c) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Word_Break::from(' '), Word_Break::WSegSpace);
        assert_eq!(Word_Break::from('\r'), Word_Break::CR);
        assert_eq!(Word_Break::from('\n'), Word_Break::LF);
        assert_eq!(Word_Break::from('\u{00FF}'), Word_Break::ALetter);
        assert_eq!(Word_Break::from('\u{0100}'), Word_Break::ALetter);
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
