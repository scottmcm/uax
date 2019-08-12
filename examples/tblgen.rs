#![allow(dead_code)]

use std::fs::File;
use std::io::Write;

use regex::Regex;

// Eytzinger

fn main() {
    let mut properties = Vec::new();

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
        let comment = captures["comment"].to_owned();
        properties.push((start, end, value, comment));
    }
    properties.sort_by_key(|x| x.0);

    if properties[0].0 > '\0' {
        let p = prev_char(properties[0].0);
        properties.insert(0, ('\0', p, "Other".to_owned(),
            "So every possible input is always found in the table".to_owned()))
    }

    let mut f = File::create("src/word_break_table.rs").unwrap();
    macro_rules! w {
        () => { w!("") };
        ($($x:tt)+) => {
            writeln!(f, $($x)+).unwrap()
        };
    }

    w!("use crate::property_enums::Word_Break;");
    w!("use crate::lookup_table::LookupTable;");
    w!("use Word_Break::*;");
    w!();
    w!("impl From<char> for Word_Break {{");
    w!("    #[inline]");
    w!("    fn from(c: char) -> Self {{");
    w!("        return CHAR_TABLE.get_or(&c, Other);");
    w!("    }}");
    w!("}}");
    w!();
    w!("static CHAR_TABLE: LookupTable<char, Word_Break> = lookup_table![");
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
