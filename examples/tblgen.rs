#![allow(dead_code)]

use std::collections::HashMap;
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
    let mut properties = properties.as_slice();

    let mut f = File::create("src/word_break_table.rs").unwrap();
    macro_rules! w {
        () => { w!("") };
        ($($x:tt)+) => {
            writeln!(f, $($x)+).unwrap()
        };
    }

    w!("use crate::property_enums::Word_Break;");
    w!();
    w!("impl From<char> for Word_Break {{");
    w!("    fn from(c: char) -> Self {{");
    w!("        use Word_Break::*;");
    w!("        if c.is_ascii() {{");
    w!("            return ASCII_TABLE[c as u32 as usize];");
    w!("        }}");
    w!("        return crate::table_lookup(&START_TABLE, &(c as u32), &VALUE_TABLE);");
    w!();

    w!("        const ASCII_TABLE: [Word_Break; 128] = [");
    for i in 0_u32..128 {
        let p = &properties[0];
        let c = std::char::from_u32(i).unwrap();
        if c < p.0 {
            w!("            Other,");
        } else {
            w!("            {}, // {}",
                p.2, p.3);
            if c == p.1 {
                properties = &properties[1..];
            }
        }
    }
    w!("        ];");

    let (starts, values, comments) = to_gap_tables("Other".to_owned(), properties);

    w!("        const START_TABLE: [u32; {}] = [", comments.len());
    for (start, comment) in starts.iter().zip(&comments) {
        w!("            {:#06X},{}", start, comment);
    }
    w!("        ];");
    w!("        const VALUE_TABLE: [Word_Break; {}] = [", comments.len());
    for (value, comment) in values.iter().zip(&comments) {
        w!("            {},{}", value, comment);
    }
    w!("        ];");

    w!("    }}");
    w!("}}");

    // for p in properties {
    //     w!("// {:}..{:}\t{:?}\t# {}",
    //         p.0.escape_unicode(), p.1.escape_unicode(), p.2, p.3);
    // }
}

fn parse_char(hex: &str) -> char {
    let x = u32::from_str_radix(hex, 16).unwrap();
    std::char::from_u32(x).unwrap()
}

fn to_gap_tables<T: Clone>(filler: T, properties: &[(char, char, T, String)])
    -> (Vec<u32>, Vec<T>, Vec<String>)
{
    let mut starts = Vec::new();
    let mut values = Vec::new();
    let mut comments = Vec::new();

    let mut next = 0;
    for p in properties {
        if p.0 as u32 != next {
            starts.push(next);
            values.push(filler.clone());
            comments.push(String::new());
        }

        starts.push(p.0 as u32);
        values.push(p.2.clone());
        comments.push(format!(" // {}", p.3));

        next = p.1 as u32 + 1;
    }

    starts.push(next);
    values.push(filler);
    comments.push(String::new());

    (starts, values, comments)
}

/*

    w!("            return match c {{");

    //while properties[0].1.is_ascii() {
    while properties.len() > 0 {
        let p = &properties[0];
        w!("                // {}", p.3);
        w!("                '{}'..='{}' => {:?},",
            p.0.escape_unicode(), p.1.escape_unicode(), p.2);
        properties = &properties[1..];
    }

    w!("                _ => Other,");
    w!("            }}");


*/