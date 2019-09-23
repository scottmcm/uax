#![allow(dead_code)]

use std::fs::File;
use std::io::Write;

use regex::Regex;

mod shared;
use shared::range_map::RangeMap;

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

    generate_property_table(
        "Script",
        include_str!("../data/Scripts.txt"),
        "Unknown",
        "src/property_tables/script.rs",
    );

    generate_property_table(
        "Block",
        include_str!("../data/Blocks.txt"),
        "No_Block",
        "src/property_tables/block.rs",
    );

    generate_property_table(
        "Indic_Positional_Category",
        include_str!("../data/IndicPositionalCategory.txt"),
        "NA",
        "src/property_tables/indic_positional_category.rs",
    );
    generate_property_table(
        "Indic_Syllabic_Category",
        include_str!("../data/IndicSyllabicCategory.txt"),
        "Other",
        "src/property_tables/indic_syllabic_category.rs",
    );
}

fn generate_property_table(
    type_name: &str,
    property_definition: &str,
    fallback: &str,
    file_path: &str,
) {
    let mut properties = RangeMap::<u32, String, String>::new(fallback.to_owned());

    let re = Regex::new(r#"(?x)
        (?P<start>[[:xdigit:]]{4,6})
        (?:
            \.\.
            (?P<end>[[:xdigit:]]{4,6})
        )?
        \s*
        ;
        \s*
        (?P<value>\w+)
        (?:
            \s*
            \#
            \s*
            (?P<comment>.+)
        )?
    "#).unwrap();
    for x in property_definition.lines() {
        if x == "" || x.starts_with("#") { continue }
        let captures = re.captures(x).unwrap_or_else(|| panic!("{}", x));
        let start = parse_hex(&captures["start"]);
        let end = captures.name("end")
            .map(|x| parse_hex(x.as_str()))
            .unwrap_or(start);
        let value = captures["value"].to_owned();
        let comment = captures.name("comment").map(|x| x.as_str().to_owned());
        properties.set(start..(end + 1), value, comment);
    }
    let mut properties = properties.iter().peekable();

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
    if properties.peek().unwrap().0.start > &0 {
        let p = properties.peek().unwrap();
        w!("    // So every possible input is always found in the table");
        w!("    ({:#04X}, {:#04X}, {}),", 0, u32::min(p.0.start - 1, 0xFF), fallback);
    }
    while properties.peek().unwrap().0.end <= &0x100 {
        let p = properties.next().unwrap();
        for comment in p.2 {
            w!("    // {}", comment);
        }
        w!("    ({:#04X}, {:#04X}, {}),", p.0.start, p.0.end-1, p.1);
    }
    w!("];");
    let row0_limit = u32::min(0x100, *properties.peek().unwrap().0.start);
    w!("const ROW0_LIMIT: char = '\\u{{{:x}}}';", row0_limit);
    w!("const PLANE0_TABLE: LookupTable<u16, {}> = lookup_table![", type_name);
    if properties.peek().unwrap().0.start > &0x100 {
        let p = properties.peek().unwrap();
        w!("    // So every possible input is always found in the table");
        w!("    ({:#06X}, {:#06X}, {}),", 0x100, u32::min(p.0.start - 1, 0xFFFF), fallback);
    }
    while properties.peek().unwrap().0.end <= &0x10000 {
        let p = properties.next().unwrap();
        for comment in p.2 {
            w!("    // {}", comment);
        }
        w!("    ({:#06X}, {:#06X}, {}),", p.0.start, p.0.end-1, p.1);
    }
    w!("];");
    let plane0_limit = u32::min(0x10000, *properties.peek().unwrap().0.start);
    w!("const PLANE0_LIMIT: char = '\\u{{{:x}}}';", plane0_limit);
    w!("const SUPPLEMENTARY_TABLE: LookupTable<u32, {}> = lookup_table![", type_name);
    if properties.peek().unwrap().0.start > &0x10000 {
        let p = properties.peek().unwrap();
        w!("    // So every possible input is always found in the table");
        w!("    ({:#08X}, {:#08X}, {}),", 0x10000, p.0.start - 1, fallback);
    }
    for p in properties {
        for comment in p.2 {
            w!("    // {}", comment);
        }
        w!("    ({:#08X}, {:#08X}, {}),", p.0.start, p.0.end-1, p.1);
    }
    w!("];");
}

fn prev_char(c: char) -> char {
    std::char::from_u32(c as u32 - 1).unwrap()
}

fn parse_hex(hex: &str) -> u32 {
    u32::from_str_radix(hex, 16).unwrap()
}
