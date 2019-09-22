use std::fs::File;
use std::io::Write;

use regex::Regex;

const PROPERTY_ALIASES: &str = include_str!("../data/PropertyAliases.txt");
const PROPERTY_VALUE_ALIASES: &str = include_str!("../data/PropertyValueAliases.txt");

macro_rules! coalesce {
    ($e:expr, $($f:expr),+) => {
        if let Some(v) = $e {
            v
        } else {
            coalesce!($($f),+)
        }
    };
    ($e:expr) => { $e };
}

#[derive(Debug, Copy, Clone)]
struct PropertyAlias<'a> {
    short: &'a str,
    long: &'a str,
}

fn property_aliases() -> Vec<PropertyAlias<'static>> {
    let mut v = vec![];
    let re = Regex::new(r"^(\w+)\s*;\s*(\w+)").unwrap();
    for line in PROPERTY_ALIASES.lines() {
        let captures = coalesce!(re.captures(line), continue);
        let short = captures.get(1).unwrap().as_str();
        let long = captures.get(2).unwrap().as_str();
        v.push(PropertyAlias { short, long });
    }
    v
}

#[derive(Debug, Copy, Clone)]
struct PropertyValueAlias<'a> {
    property_short: &'a str,
    short: &'a str,
    long: &'a str,
    comment: &'a str,
}

fn property_value_aliases() -> Vec<PropertyValueAlias<'static>> {
    let mut v = vec![];
    let re = Regex::new(r"^(\w+)\s*;\s*(.+?)\s*;\s*(\w+)(?:.*#\s*(.+)$)?").unwrap();
    for line in PROPERTY_VALUE_ALIASES.lines() {
        let captures = coalesce!(re.captures(line), continue);
        let property_short = captures.get(1).unwrap().as_str();
        let short = captures.get(2).unwrap().as_str();
        let long = captures.get(3).unwrap().as_str();
        let comment = coalesce!(captures.get(4).map(|m| m.as_str()), "");
        v.push(PropertyValueAlias { property_short, short, long, comment });
    }
    v
}


fn main() {
    let mut f = File::create("src/properties.rs").unwrap();
    macro_rules! w {
        () => { w!("") };
        ($($x:tt)+) => {
            writeln!(f, $($x)+).unwrap()
        };
    }
    w!("#![allow(non_camel_case_types)] // Whatever unicode says, we use");
    w!();
    w!("use core::{{fmt, str::FromStr}};");
    w!();
    w!("#[derive(Debug, Clone)]");
    w!("pub struct ParseError(());");

    let pas = property_aliases();
    let pvas = property_value_aliases();
    for pa in pas {
        let pvas = pvas.iter().copied().filter(|x| x.property_short == pa.short).collect::<Vec<_>>();
        if pvas.len() <= 2 { continue }

        w!();
        w!("/// AKA `{}`", pa.short);
        w!("#[derive(Debug, Copy, Clone, Eq, PartialEq)]");
        w!("pub enum {} {{", pa.long);
        for pva in &pvas {
            if pva.short != pva.long {
                w!("    /// AKA `{}`", pva.short);
            }
            w!("    {},", pva.long);
        }
        w!("}}");
        w!("impl fmt::Display for {} {{", pa.long);
        w!("    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{");
        w!("        <str as fmt::Display>::fmt(self.display_str(), f)");
        w!("    }}");
        w!("}}");
        w!("impl {} {{", pa.long);
        w!("    fn display_str(&self) -> &'static str {{");
        w!("        match self {{");
        for pva in &pvas {
            w!("            {}::{} => \"{}\",", pa.long, pva.long, pva.short);
        }
        w!("        }}");
        w!("    }}");
        w!("}}");
        w!("impl FromStr for {} {{", pa.long);
        w!("    type Err = ParseError;");
        w!("    fn from_str(text: &str) -> Result<Self, Self::Err> {{");
        w!("        match text {{");
        for pva in &pvas {
            w!("            \"{}\" => Ok({}::{}),", pva.short, pa.long, pva.long);
        }
        w!("            _ => Err(ParseError(())),");
        w!("        }}");
        w!("    }}");
        w!("}}");
    }
}
