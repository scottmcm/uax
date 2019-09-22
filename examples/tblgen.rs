#![allow(dead_code)]

use std::collections::HashSet;
use std::fs::File;
use std::io::Write;

use regex::Regex;
use unicase::UniCase;

fn main() {
    generate_property_table(
        "Grapheme_Cluster_Break",
        include_str!("../data/GraphemeBreakProperty.txt"),
        "Other",
        "src/property_tables/grapheme_cluster_break.rs",
    );
    generate_property_table(
        "Word_Break",
        include_str!("../data/WordBreakProperty.txt"),
        "Other",
        "src/property_tables/word_break.rs",
    );
    generate_property_table(
        "Sentence_Break",
        include_str!("../data/SentenceBreakProperty.txt"),
        "Other",
        "src/property_tables/sentence_break.rs",
    );
}

fn generate_property_table(
    type_name: &str,
    property_definition: &str,
    fallback: &str,
    file_path: &str,
) {
    let mut properties = Vec::new();
    let mut values = HashSet::new();

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
    for x in property_definition.lines() {
        if x == "" || x.starts_with("#") { continue }
        let captures = re.captures(x).unwrap_or_else(|| panic!("{}", x));
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
    let mut properties = combine_adjacent(properties).into_iter().peekable();

    let mut values = values.into_iter().collect::<Vec<_>>();
    values.sort_by_cached_key(|x| UniCase::new(x.clone()));

    let mut f = File::create(file_path).unwrap();
    macro_rules! w {
        () => { w!("") };
        ($($x:tt)+) => {
            writeln!(f, $($x)+).unwrap()
        };
    }

    w!("use crate::lookup_table::LookupTable;");
    w!();
    w!("use crate::properties::{};", type_name);
    w!("use {}::*;", type_name);
    w!();
    w!("impl From<char> for {} {{", type_name);
    w!("    #[inline]");
    w!("    fn from(c: char) -> Self {{");
    w!("        if c < ROW0_LIMIT {{");
    w!("            return ROW0_TABLE.get_or(&(c as u8), {});", fallback);
    w!("        }}");
    w!("        if c < PLANE0_LIMIT {{");
    w!("            return PLANE0_TABLE.get_or(&(c as u16), {});", fallback);
    w!("        }}");
    w!("        return SUPPLEMENTARY_TABLE.get_or(&(c as u32), {});", fallback);
    w!("    }}");
    w!("}}");
    w!();
    w!("#[test]");
    w!("fn validate_tables() {{");
    w!("    use std::convert::TryInto;");
    w!("    ROW0_TABLE.validate();");
    w!("    if let Ok(x) = (ROW0_LIMIT as u32).try_into() {{ assert!(!ROW0_TABLE.contains(&x)); }}");
    w!("    PLANE0_TABLE.validate();");
    w!("    if let Ok(x) = (PLANE0_LIMIT as u32).try_into() {{ assert!(!PLANE0_TABLE.contains(&x)); }}");
    w!("    SUPPLEMENTARY_TABLE.validate();");
    w!("}}");
    w!();
    w!("const ROW0_TABLE: LookupTable<u8, {}> = lookup_table![", type_name);
    if properties.peek().unwrap().0 as u32 > 0 {
        let p = properties.peek().unwrap();
        w!("    // So every possible input is always found in the table");
        w!("    ({:#04X}, {:#04X}, {}),", 0, p.0 as u32 - 1, fallback);
    }
    while properties.peek().unwrap().1 as u32 <= 0xFF {
        let p = properties.next().unwrap();
        for comment in p.3 {
            w!("    // {}", comment);
        }
        w!("    ({:#04X}, {:#04X}, {}),", p.0 as u32, p.1 as u32, p.2);
    }
    w!("];");
    let row0_limit = char::min('\u{100}', properties.peek().unwrap().0);
    w!("const ROW0_LIMIT: char = '{}';", row0_limit.escape_unicode());
    w!("const PLANE0_TABLE: LookupTable<u16, {}> = lookup_table![", type_name);
    if properties.peek().unwrap().0 as u32 > 0x100 {
        let p = properties.peek().unwrap();
        w!("    // So every possible input is always found in the table");
        w!("    ({:#06X}, {:#06X}, {}),", 0x100, p.0 as u32 - 1, fallback);
    }
    while properties.peek().unwrap().1 as u32 <= 0xFFFF {
        let p = properties.next().unwrap();
        for comment in p.3 {
            w!("    // {}", comment);
        }
        w!("    ({:#06X}, {:#06X}, {}),", p.0 as u32, p.1 as u32, p.2);
    }
    w!("];");
    let plane0_limit = char::min('\u{10000}', properties.peek().unwrap().0);
    w!("const PLANE0_LIMIT: char = '{}';", plane0_limit.escape_unicode());
    w!("const SUPPLEMENTARY_TABLE: LookupTable<u32, {}> = lookup_table![", type_name);
    if properties.peek().unwrap().0 as u32 > 0x10000 {
        let p = properties.peek().unwrap();
        w!("    // So every possible input is always found in the table");
        w!("    ({:#08X}, {:#08X}, {}),", 0x10000, p.0 as u32 - 1, fallback);
    }
    for p in properties {
        for comment in p.3 {
            w!("    // {}", comment);
        }
        w!("    ({:#08X}, {:#08X}, {}),",
            p.0 as u32, p.1 as u32, p.2);
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

fn combine_adjacent(raw_properties: Vec<(char, char, String, String)>)
    -> Vec<(char, char, String, Vec<String>)>
{
    let mut properties = Vec::<(char, char, String, Vec<String>)>::with_capacity(raw_properties.len());
    for p in raw_properties {
        if !properties.is_empty() {
            let prev = properties.last_mut().unwrap();
            if p.2 == prev.2 && (prev.1 as u32)+1 == (p.0 as u32) {
                prev.1 = p.1;
                prev.3.push(p.3);
                continue;
            }
        }

        properties.push((p.0, p.1, p.2, vec![p.3]));
    }
    properties
}
