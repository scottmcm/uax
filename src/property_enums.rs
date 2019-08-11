#![allow(non_camel_case_types)] // We match unicode exactly

macro_rules! define_enum {
    ($name:ident => $($variant:ident,)+) => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        pub enum $name {
            $($variant,)+
        }
        impl $name {
            pub const ALL: &'static [$name] = &[
                $($name::$variant,)+
            ];
        }
    };
}

define_enum! { Word_Break =>
    Other,
    Double_Quote,
    Single_Quote,
    Hebrew_Letter,
    CR,
    LF,
    Newline,
    Extend,
    Regional_Indicator,
    Format,
    Katakana,
    ALetter,
    MidLetter,
    MidNum,
    MidNumLet,
    Numeric,
    ExtendNumLet,
    ZWJ,
    WSegSpace,
}
