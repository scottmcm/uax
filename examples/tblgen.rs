#![allow(dead_code)]

use std::collections::HashSet;
use std::fs::File;
use std::io::Write;

use regex::Regex;

// Eytzinger

fn main() {
    let mut properties = Vec::new();
    let mut values = HashSet::new();

    let data = include_str!("../data/WordBreakProperty.txt");
    let re = Regex::new(r#"(?x)
        (?P<start>[[:xdigit:]]{4,6})
        (?:
            \.\.
            (?P<end>[[:xdigit:]]{4,6})
        )?
        \s+
        ;
        \s+
        (?P<value>\w+)
        \s+
        \#
        \s+
        (?P<comment>.+)
    "#).unwrap();
    for x in data.lines() {
        if x == "" || x.starts_with("#") { continue }
        let captures = re.captures(x).unwrap_or_else(|| panic!(x));
        let start = parse_char(&captures["start"]);
        let end = captures.name("end")
            .map(|x| parse_char(x.as_str()))
            .unwrap_or(start);
        let value = captures["value"].to_owned();
        values.insert(value.clone());
        let comment = captures["comment"].to_owned();
        properties.push((start, end, value, comment));
    }
    properties.sort_by_key(|x| x.0);
    let mut properties = properties.into_iter().peekable();

    let mut values = values.into_iter().collect::<Vec<_>>();
    values.sort();

    let mut f = File::create("src/properties/word_break.rs").unwrap();
    macro_rules! w {
        () => { w!("") };
        ($($x:tt)+) => {
            writeln!(f, $($x)+).unwrap()
        };
    }

    w!("use crate::lookup_table::LookupTable;");
    w!();
    w!("#[allow(non_camel_case_types)] // Whatever unicode says, we use");
    w!("#[derive(Debug, Copy, Clone, Eq, PartialEq)]");
    w!("pub enum Word_Break {{");
    w!("    Other,");
    for v in values {
        w!("    {},", v);
    }
    w!("}}");
    w!("use Word_Break::*;");
    w!();
    w!("impl From<char> for Word_Break {{");
    w!("    #[inline]");
    w!("    fn from(c: char) -> Self {{");
    w!("        if c < ROW0_LIMIT {{");
    w!("            return ROW0_TABLE.get_or(&(c as u8), Other);");
    w!("        }}");
    w!("        if c < PLANE0_LIMIT {{");
    w!("            return PLANE0_TABLE.get_or(&(c as u16), Other);");
    w!("        }}");
    w!("        return SUPPLEMENTARY_TABLE.get_or(&c, Other);");
    w!("    }}");
    w!("}}");
    w!();
    w!("const ROW0_TABLE: LookupTable<u8, Word_Break> = lookup_table![");
    if properties.peek().unwrap().0 as u32 > 0 {
        let p = properties.peek().unwrap();
        w!("    // So every possible input is always found in the table");
        w!("    ({:#04X}, {:#04X}, Other),", 0, p.1 as u32 - 1);
    }
    while properties.peek().unwrap().1 as u32 <= 0xFF {
        let p = properties.next().unwrap();
        w!("    // {}", p.3);
        w!("    ({:#04X}, {:#04X}, {}),", p.0 as u32, p.1 as u32, p.2);
    }
    w!("];");
    let row0_limit = char::min('\u{100}', properties.peek().unwrap().0);
    w!("const ROW0_LIMIT: char = '{}';", row0_limit.escape_unicode());
    w!("const PLANE0_TABLE: LookupTable<u16, Word_Break> = lookup_table![");
    if properties.peek().unwrap().0 as u32 > 0x100 {
        let p = properties.peek().unwrap();
        w!("    // So every possible input is always found in the table");
        w!("    ({:#06X}, {:#06X}, Other),", 0x100, p.1 as u32 - 1);
    }
    while properties.peek().unwrap().1 as u32 <= 0xFFFF {
        let p = properties.next().unwrap();
        w!("    // {}", p.3);
        w!("    ({:#06X}, {:#06X}, {}),", p.0 as u32, p.1 as u32, p.2);
    }
    w!("];");
    let plane0_limit = char::min('\u{10000}', properties.peek().unwrap().0);
    w!("const PLANE0_LIMIT: char = '{}';", plane0_limit.escape_unicode());
    w!("const SUPPLEMENTARY_TABLE: LookupTable<char, Word_Break> = lookup_table![");
    if properties.peek().unwrap().0 as u32 > 0x10000 {
        let p = properties.peek().unwrap();
        w!("    // So every possible input is always found in the table");
        w!("    ({:#06X}, {:#06X}, Other),", 0x10000, p.1 as u32 - 1);
    }
    for p in properties {
        w!("    // {}", p.3);
        w!("    ('{}', '{}', {}),",
            p.0.escape_unicode(), p.1.escape_unicode(), p.2);
    }
    w!("];");
}

fn prev_char(c: char) -> char {
    std::char::from_u32(c as u32 - 1).unwrap()
}

fn parse_char(hex: &str) -> char {
    let x = u32::from_str_radix(hex, 16).unwrap();
    std::char::from_u32(x).unwrap()
}
