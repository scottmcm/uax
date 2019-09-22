#![allow(non_camel_case_types)] // Whatever unicode says, we use

use core::{fmt, str::FromStr};

#[derive(Debug, Clone)]
pub struct ParseError(());

/// AKA `JSN`
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Jamo_Short_Name {
    A,
    AE,
    B,
    BB,
    BS,
    C,
    D,
    DD,
    E,
    EO,
    EU,
    G,
    GG,
    GS,
    H,
    I,
    J,
    JJ,
    K,
    L,
    LB,
    LG,
    LH,
    LM,
    LP,
    LS,
    LT,
    M,
    N,
    NG,
    NH,
    NJ,
    O,
    OE,
    P,
    R,
    S,
    SS,
    T,
    U,
    WA,
    WAE,
    WE,
    WEO,
    WI,
    YA,
    YAE,
    YE,
    YEO,
    YI,
    YO,
    YU,
}
impl fmt::Display for Jamo_Short_Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <str as fmt::Display>::fmt(self.display_str(), f)
    }
}
impl Jamo_Short_Name {
    fn display_str(&self) -> &'static str {
        match self {
            Jamo_Short_Name::A => "A",
            Jamo_Short_Name::AE => "AE",
            Jamo_Short_Name::B => "B",
            Jamo_Short_Name::BB => "BB",
            Jamo_Short_Name::BS => "BS",
            Jamo_Short_Name::C => "C",
            Jamo_Short_Name::D => "D",
            Jamo_Short_Name::DD => "DD",
            Jamo_Short_Name::E => "E",
            Jamo_Short_Name::EO => "EO",
            Jamo_Short_Name::EU => "EU",
            Jamo_Short_Name::G => "G",
            Jamo_Short_Name::GG => "GG",
            Jamo_Short_Name::GS => "GS",
            Jamo_Short_Name::H => "H",
            Jamo_Short_Name::I => "I",
            Jamo_Short_Name::J => "J",
            Jamo_Short_Name::JJ => "JJ",
            Jamo_Short_Name::K => "K",
            Jamo_Short_Name::L => "L",
            Jamo_Short_Name::LB => "LB",
            Jamo_Short_Name::LG => "LG",
            Jamo_Short_Name::LH => "LH",
            Jamo_Short_Name::LM => "LM",
            Jamo_Short_Name::LP => "LP",
            Jamo_Short_Name::LS => "LS",
            Jamo_Short_Name::LT => "LT",
            Jamo_Short_Name::M => "M",
            Jamo_Short_Name::N => "N",
            Jamo_Short_Name::NG => "NG",
            Jamo_Short_Name::NH => "NH",
            Jamo_Short_Name::NJ => "NJ",
            Jamo_Short_Name::O => "O",
            Jamo_Short_Name::OE => "OE",
            Jamo_Short_Name::P => "P",
            Jamo_Short_Name::R => "R",
            Jamo_Short_Name::S => "S",
            Jamo_Short_Name::SS => "SS",
            Jamo_Short_Name::T => "T",
            Jamo_Short_Name::U => "U",
            Jamo_Short_Name::WA => "WA",
            Jamo_Short_Name::WAE => "WAE",
            Jamo_Short_Name::WE => "WE",
            Jamo_Short_Name::WEO => "WEO",
            Jamo_Short_Name::WI => "WI",
            Jamo_Short_Name::YA => "YA",
            Jamo_Short_Name::YAE => "YAE",
            Jamo_Short_Name::YE => "YE",
            Jamo_Short_Name::YEO => "YEO",
            Jamo_Short_Name::YI => "YI",
            Jamo_Short_Name::YO => "YO",
            Jamo_Short_Name::YU => "YU",
        }
    }
}
impl FromStr for Jamo_Short_Name {
    type Err = ParseError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "A" => Ok(Jamo_Short_Name::A),
            "AE" => Ok(Jamo_Short_Name::AE),
            "B" => Ok(Jamo_Short_Name::B),
            "BB" => Ok(Jamo_Short_Name::BB),
            "BS" => Ok(Jamo_Short_Name::BS),
            "C" => Ok(Jamo_Short_Name::C),
            "D" => Ok(Jamo_Short_Name::D),
            "DD" => Ok(Jamo_Short_Name::DD),
            "E" => Ok(Jamo_Short_Name::E),
            "EO" => Ok(Jamo_Short_Name::EO),
            "EU" => Ok(Jamo_Short_Name::EU),
            "G" => Ok(Jamo_Short_Name::G),
            "GG" => Ok(Jamo_Short_Name::GG),
            "GS" => Ok(Jamo_Short_Name::GS),
            "H" => Ok(Jamo_Short_Name::H),
            "I" => Ok(Jamo_Short_Name::I),
            "J" => Ok(Jamo_Short_Name::J),
            "JJ" => Ok(Jamo_Short_Name::JJ),
            "K" => Ok(Jamo_Short_Name::K),
            "L" => Ok(Jamo_Short_Name::L),
            "LB" => Ok(Jamo_Short_Name::LB),
            "LG" => Ok(Jamo_Short_Name::LG),
            "LH" => Ok(Jamo_Short_Name::LH),
            "LM" => Ok(Jamo_Short_Name::LM),
            "LP" => Ok(Jamo_Short_Name::LP),
            "LS" => Ok(Jamo_Short_Name::LS),
            "LT" => Ok(Jamo_Short_Name::LT),
            "M" => Ok(Jamo_Short_Name::M),
            "N" => Ok(Jamo_Short_Name::N),
            "NG" => Ok(Jamo_Short_Name::NG),
            "NH" => Ok(Jamo_Short_Name::NH),
            "NJ" => Ok(Jamo_Short_Name::NJ),
            "O" => Ok(Jamo_Short_Name::O),
            "OE" => Ok(Jamo_Short_Name::OE),
            "P" => Ok(Jamo_Short_Name::P),
            "R" => Ok(Jamo_Short_Name::R),
            "S" => Ok(Jamo_Short_Name::S),
            "SS" => Ok(Jamo_Short_Name::SS),
            "T" => Ok(Jamo_Short_Name::T),
            "U" => Ok(Jamo_Short_Name::U),
            "WA" => Ok(Jamo_Short_Name::WA),
            "WAE" => Ok(Jamo_Short_Name::WAE),
            "WE" => Ok(Jamo_Short_Name::WE),
            "WEO" => Ok(Jamo_Short_Name::WEO),
            "WI" => Ok(Jamo_Short_Name::WI),
            "YA" => Ok(Jamo_Short_Name::YA),
            "YAE" => Ok(Jamo_Short_Name::YAE),
            "YE" => Ok(Jamo_Short_Name::YE),
            "YEO" => Ok(Jamo_Short_Name::YEO),
            "YI" => Ok(Jamo_Short_Name::YI),
            "YO" => Ok(Jamo_Short_Name::YO),
            "YU" => Ok(Jamo_Short_Name::YU),
            _ => Err(ParseError(())),
        }
    }
}

/// AKA `age`
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Age {
    /// AKA `1.1`
    V1_1,
    /// AKA `2.0`
    V2_0,
    /// AKA `2.1`
    V2_1,
    /// AKA `3.0`
    V3_0,
    /// AKA `3.1`
    V3_1,
    /// AKA `3.2`
    V3_2,
    /// AKA `4.0`
    V4_0,
    /// AKA `4.1`
    V4_1,
    /// AKA `5.0`
    V5_0,
    /// AKA `5.1`
    V5_1,
    /// AKA `5.2`
    V5_2,
    /// AKA `6.0`
    V6_0,
    /// AKA `6.1`
    V6_1,
    /// AKA `6.2`
    V6_2,
    /// AKA `6.3`
    V6_3,
    /// AKA `7.0`
    V7_0,
    /// AKA `8.0`
    V8_0,
    /// AKA `9.0`
    V9_0,
    /// AKA `10.0`
    V10_0,
    /// AKA `11.0`
    V11_0,
    /// AKA `12.0`
    V12_0,
    /// AKA `12.1`
    V12_1,
    /// AKA `NA`
    Unassigned,
}
impl fmt::Display for Age {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <str as fmt::Display>::fmt(self.display_str(), f)
    }
}
impl Age {
    fn display_str(&self) -> &'static str {
        match self {
            Age::V1_1 => "1.1",
            Age::V2_0 => "2.0",
            Age::V2_1 => "2.1",
            Age::V3_0 => "3.0",
            Age::V3_1 => "3.1",
            Age::V3_2 => "3.2",
            Age::V4_0 => "4.0",
            Age::V4_1 => "4.1",
            Age::V5_0 => "5.0",
            Age::V5_1 => "5.1",
            Age::V5_2 => "5.2",
            Age::V6_0 => "6.0",
            Age::V6_1 => "6.1",
            Age::V6_2 => "6.2",
            Age::V6_3 => "6.3",
            Age::V7_0 => "7.0",
            Age::V8_0 => "8.0",
            Age::V9_0 => "9.0",
            Age::V10_0 => "10.0",
            Age::V11_0 => "11.0",
            Age::V12_0 => "12.0",
            Age::V12_1 => "12.1",
            Age::Unassigned => "NA",
        }
    }
}
impl FromStr for Age {
    type Err = ParseError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "1.1" => Ok(Age::V1_1),
            "2.0" => Ok(Age::V2_0),
            "2.1" => Ok(Age::V2_1),
            "3.0" => Ok(Age::V3_0),
            "3.1" => Ok(Age::V3_1),
            "3.2" => Ok(Age::V3_2),
            "4.0" => Ok(Age::V4_0),
            "4.1" => Ok(Age::V4_1),
            "5.0" => Ok(Age::V5_0),
            "5.1" => Ok(Age::V5_1),
            "5.2" => Ok(Age::V5_2),
            "6.0" => Ok(Age::V6_0),
            "6.1" => Ok(Age::V6_1),
            "6.2" => Ok(Age::V6_2),
            "6.3" => Ok(Age::V6_3),
            "7.0" => Ok(Age::V7_0),
            "8.0" => Ok(Age::V8_0),
            "9.0" => Ok(Age::V9_0),
            "10.0" => Ok(Age::V10_0),
            "11.0" => Ok(Age::V11_0),
            "12.0" => Ok(Age::V12_0),
            "12.1" => Ok(Age::V12_1),
            "NA" => Ok(Age::Unassigned),
            _ => Err(ParseError(())),
        }
    }
}

/// AKA `blk`
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Block {
    Adlam,
    Aegean_Numbers,
    Ahom,
    /// AKA `Alchemical`
    Alchemical_Symbols,
    /// AKA `Alphabetic_PF`
    Alphabetic_Presentation_Forms,
    Anatolian_Hieroglyphs,
    /// AKA `Ancient_Greek_Music`
    Ancient_Greek_Musical_Notation,
    Ancient_Greek_Numbers,
    Ancient_Symbols,
    Arabic,
    /// AKA `Arabic_Ext_A`
    Arabic_Extended_A,
    /// AKA `Arabic_Math`
    Arabic_Mathematical_Alphabetic_Symbols,
    /// AKA `Arabic_PF_A`
    Arabic_Presentation_Forms_A,
    /// AKA `Arabic_PF_B`
    Arabic_Presentation_Forms_B,
    /// AKA `Arabic_Sup`
    Arabic_Supplement,
    Armenian,
    Arrows,
    /// AKA `ASCII`
    Basic_Latin,
    Avestan,
    Balinese,
    Bamum,
    /// AKA `Bamum_Sup`
    Bamum_Supplement,
    Bassa_Vah,
    Batak,
    Bengali,
    Bhaiksuki,
    Block_Elements,
    Bopomofo,
    /// AKA `Bopomofo_Ext`
    Bopomofo_Extended,
    Box_Drawing,
    Brahmi,
    /// AKA `Braille`
    Braille_Patterns,
    Buginese,
    Buhid,
    /// AKA `Byzantine_Music`
    Byzantine_Musical_Symbols,
    Carian,
    Caucasian_Albanian,
    Chakma,
    Cham,
    Cherokee,
    /// AKA `Cherokee_Sup`
    Cherokee_Supplement,
    Chess_Symbols,
    /// AKA `CJK`
    CJK_Unified_Ideographs,
    /// AKA `CJK_Compat`
    CJK_Compatibility,
    /// AKA `CJK_Compat_Forms`
    CJK_Compatibility_Forms,
    /// AKA `CJK_Compat_Ideographs`
    CJK_Compatibility_Ideographs,
    /// AKA `CJK_Compat_Ideographs_Sup`
    CJK_Compatibility_Ideographs_Supplement,
    /// AKA `CJK_Ext_A`
    CJK_Unified_Ideographs_Extension_A,
    /// AKA `CJK_Ext_B`
    CJK_Unified_Ideographs_Extension_B,
    /// AKA `CJK_Ext_C`
    CJK_Unified_Ideographs_Extension_C,
    /// AKA `CJK_Ext_D`
    CJK_Unified_Ideographs_Extension_D,
    /// AKA `CJK_Ext_E`
    CJK_Unified_Ideographs_Extension_E,
    /// AKA `CJK_Ext_F`
    CJK_Unified_Ideographs_Extension_F,
    /// AKA `CJK_Radicals_Sup`
    CJK_Radicals_Supplement,
    CJK_Strokes,
    /// AKA `CJK_Symbols`
    CJK_Symbols_And_Punctuation,
    /// AKA `Compat_Jamo`
    Hangul_Compatibility_Jamo,
    Control_Pictures,
    Coptic,
    Coptic_Epact_Numbers,
    /// AKA `Counting_Rod`
    Counting_Rod_Numerals,
    Cuneiform,
    /// AKA `Cuneiform_Numbers`
    Cuneiform_Numbers_And_Punctuation,
    Currency_Symbols,
    Cypriot_Syllabary,
    Cyrillic,
    /// AKA `Cyrillic_Ext_A`
    Cyrillic_Extended_A,
    /// AKA `Cyrillic_Ext_B`
    Cyrillic_Extended_B,
    /// AKA `Cyrillic_Ext_C`
    Cyrillic_Extended_C,
    /// AKA `Cyrillic_Sup`
    Cyrillic_Supplement,
    Deseret,
    Devanagari,
    /// AKA `Devanagari_Ext`
    Devanagari_Extended,
    /// AKA `Diacriticals`
    Combining_Diacritical_Marks,
    /// AKA `Diacriticals_Ext`
    Combining_Diacritical_Marks_Extended,
    /// AKA `Diacriticals_For_Symbols`
    Combining_Diacritical_Marks_For_Symbols,
    /// AKA `Diacriticals_Sup`
    Combining_Diacritical_Marks_Supplement,
    Dingbats,
    Dogra,
    /// AKA `Domino`
    Domino_Tiles,
    Duployan,
    Early_Dynastic_Cuneiform,
    Egyptian_Hieroglyph_Format_Controls,
    Egyptian_Hieroglyphs,
    Elbasan,
    Elymaic,
    Emoticons,
    /// AKA `Enclosed_Alphanum`
    Enclosed_Alphanumerics,
    /// AKA `Enclosed_Alphanum_Sup`
    Enclosed_Alphanumeric_Supplement,
    /// AKA `Enclosed_CJK`
    Enclosed_CJK_Letters_And_Months,
    /// AKA `Enclosed_Ideographic_Sup`
    Enclosed_Ideographic_Supplement,
    Ethiopic,
    /// AKA `Ethiopic_Ext`
    Ethiopic_Extended,
    /// AKA `Ethiopic_Ext_A`
    Ethiopic_Extended_A,
    /// AKA `Ethiopic_Sup`
    Ethiopic_Supplement,
    Geometric_Shapes,
    /// AKA `Geometric_Shapes_Ext`
    Geometric_Shapes_Extended,
    Georgian,
    /// AKA `Georgian_Ext`
    Georgian_Extended,
    /// AKA `Georgian_Sup`
    Georgian_Supplement,
    Glagolitic,
    /// AKA `Glagolitic_Sup`
    Glagolitic_Supplement,
    Gothic,
    Grantha,
    /// AKA `Greek`
    Greek_And_Coptic,
    /// AKA `Greek_Ext`
    Greek_Extended,
    Gujarati,
    Gunjala_Gondi,
    Gurmukhi,
    /// AKA `Half_And_Full_Forms`
    Halfwidth_And_Fullwidth_Forms,
    /// AKA `Half_Marks`
    Combining_Half_Marks,
    /// AKA `Hangul`
    Hangul_Syllables,
    Hanifi_Rohingya,
    Hanunoo,
    Hatran,
    Hebrew,
    /// AKA `High_PU_Surrogates`
    High_Private_Use_Surrogates,
    High_Surrogates,
    Hiragana,
    /// AKA `IDC`
    Ideographic_Description_Characters,
    /// AKA `Ideographic_Symbols`
    Ideographic_Symbols_And_Punctuation,
    Imperial_Aramaic,
    /// AKA `Indic_Number_Forms`
    Common_Indic_Number_Forms,
    Indic_Siyaq_Numbers,
    Inscriptional_Pahlavi,
    Inscriptional_Parthian,
    /// AKA `IPA_Ext`
    IPA_Extensions,
    /// AKA `Jamo`
    Hangul_Jamo,
    /// AKA `Jamo_Ext_A`
    Hangul_Jamo_Extended_A,
    /// AKA `Jamo_Ext_B`
    Hangul_Jamo_Extended_B,
    Javanese,
    Kaithi,
    /// AKA `Kana_Ext_A`
    Kana_Extended_A,
    /// AKA `Kana_Sup`
    Kana_Supplement,
    Kanbun,
    /// AKA `Kangxi`
    Kangxi_Radicals,
    Kannada,
    Katakana,
    /// AKA `Katakana_Ext`
    Katakana_Phonetic_Extensions,
    Kayah_Li,
    Kharoshthi,
    Khmer,
    Khmer_Symbols,
    Khojki,
    Khudawadi,
    Lao,
    /// AKA `Latin_1_Sup`
    Latin_1_Supplement,
    /// AKA `Latin_Ext_A`
    Latin_Extended_A,
    /// AKA `Latin_Ext_Additional`
    Latin_Extended_Additional,
    /// AKA `Latin_Ext_B`
    Latin_Extended_B,
    /// AKA `Latin_Ext_C`
    Latin_Extended_C,
    /// AKA `Latin_Ext_D`
    Latin_Extended_D,
    /// AKA `Latin_Ext_E`
    Latin_Extended_E,
    Lepcha,
    Letterlike_Symbols,
    Limbu,
    Linear_A,
    Linear_B_Ideograms,
    Linear_B_Syllabary,
    Lisu,
    Low_Surrogates,
    Lycian,
    Lydian,
    Mahajani,
    /// AKA `Mahjong`
    Mahjong_Tiles,
    Makasar,
    Malayalam,
    Mandaic,
    Manichaean,
    Marchen,
    Masaram_Gondi,
    /// AKA `Math_Alphanum`
    Mathematical_Alphanumeric_Symbols,
    /// AKA `Math_Operators`
    Mathematical_Operators,
    Mayan_Numerals,
    Medefaidrin,
    Meetei_Mayek,
    /// AKA `Meetei_Mayek_Ext`
    Meetei_Mayek_Extensions,
    Mende_Kikakui,
    Meroitic_Cursive,
    Meroitic_Hieroglyphs,
    Miao,
    /// AKA `Misc_Arrows`
    Miscellaneous_Symbols_And_Arrows,
    /// AKA `Misc_Math_Symbols_A`
    Miscellaneous_Mathematical_Symbols_A,
    /// AKA `Misc_Math_Symbols_B`
    Miscellaneous_Mathematical_Symbols_B,
    /// AKA `Misc_Pictographs`
    Miscellaneous_Symbols_And_Pictographs,
    /// AKA `Misc_Symbols`
    Miscellaneous_Symbols,
    /// AKA `Misc_Technical`
    Miscellaneous_Technical,
    Modi,
    /// AKA `Modifier_Letters`
    Spacing_Modifier_Letters,
    Modifier_Tone_Letters,
    Mongolian,
    /// AKA `Mongolian_Sup`
    Mongolian_Supplement,
    Mro,
    Multani,
    /// AKA `Music`
    Musical_Symbols,
    Myanmar,
    /// AKA `Myanmar_Ext_A`
    Myanmar_Extended_A,
    /// AKA `Myanmar_Ext_B`
    Myanmar_Extended_B,
    Nabataean,
    Nandinagari,
    /// AKA `NB`
    No_Block,
    New_Tai_Lue,
    Newa,
    NKo,
    Number_Forms,
    Nushu,
    Nyiakeng_Puachue_Hmong,
    /// AKA `OCR`
    Optical_Character_Recognition,
    Ogham,
    Ol_Chiki,
    Old_Hungarian,
    Old_Italic,
    Old_North_Arabian,
    Old_Permic,
    Old_Persian,
    Old_Sogdian,
    Old_South_Arabian,
    Old_Turkic,
    Oriya,
    Ornamental_Dingbats,
    Osage,
    Osmanya,
    Ottoman_Siyaq_Numbers,
    Pahawh_Hmong,
    Palmyrene,
    Pau_Cin_Hau,
    Phags_Pa,
    /// AKA `Phaistos`
    Phaistos_Disc,
    Phoenician,
    /// AKA `Phonetic_Ext`
    Phonetic_Extensions,
    /// AKA `Phonetic_Ext_Sup`
    Phonetic_Extensions_Supplement,
    Playing_Cards,
    Psalter_Pahlavi,
    /// AKA `PUA`
    Private_Use_Area,
    /// AKA `Punctuation`
    General_Punctuation,
    Rejang,
    /// AKA `Rumi`
    Rumi_Numeral_Symbols,
    Runic,
    Samaritan,
    Saurashtra,
    Sharada,
    Shavian,
    Shorthand_Format_Controls,
    Siddham,
    Sinhala,
    Sinhala_Archaic_Numbers,
    /// AKA `Small_Forms`
    Small_Form_Variants,
    /// AKA `Small_Kana_Ext`
    Small_Kana_Extension,
    Sogdian,
    Sora_Sompeng,
    Soyombo,
    Specials,
    Sundanese,
    /// AKA `Sundanese_Sup`
    Sundanese_Supplement,
    /// AKA `Sup_Arrows_A`
    Supplemental_Arrows_A,
    /// AKA `Sup_Arrows_B`
    Supplemental_Arrows_B,
    /// AKA `Sup_Arrows_C`
    Supplemental_Arrows_C,
    /// AKA `Sup_Math_Operators`
    Supplemental_Mathematical_Operators,
    /// AKA `Sup_PUA_A`
    Supplementary_Private_Use_Area_A,
    /// AKA `Sup_PUA_B`
    Supplementary_Private_Use_Area_B,
    /// AKA `Sup_Punctuation`
    Supplemental_Punctuation,
    /// AKA `Sup_Symbols_And_Pictographs`
    Supplemental_Symbols_And_Pictographs,
    /// AKA `Super_And_Sub`
    Superscripts_And_Subscripts,
    Sutton_SignWriting,
    Syloti_Nagri,
    /// AKA `Symbols_And_Pictographs_Ext_A`
    Symbols_And_Pictographs_Extended_A,
    Syriac,
    /// AKA `Syriac_Sup`
    Syriac_Supplement,
    Tagalog,
    Tagbanwa,
    Tags,
    Tai_Le,
    Tai_Tham,
    Tai_Viet,
    /// AKA `Tai_Xuan_Jing`
    Tai_Xuan_Jing_Symbols,
    Takri,
    Tamil,
    /// AKA `Tamil_Sup`
    Tamil_Supplement,
    Tangut,
    Tangut_Components,
    Telugu,
    Thaana,
    Thai,
    Tibetan,
    Tifinagh,
    Tirhuta,
    /// AKA `Transport_And_Map`
    Transport_And_Map_Symbols,
    /// AKA `UCAS`
    Unified_Canadian_Aboriginal_Syllabics,
    /// AKA `UCAS_Ext`
    Unified_Canadian_Aboriginal_Syllabics_Extended,
    Ugaritic,
    Vai,
    /// AKA `Vedic_Ext`
    Vedic_Extensions,
    Vertical_Forms,
    /// AKA `VS`
    Variation_Selectors,
    /// AKA `VS_Sup`
    Variation_Selectors_Supplement,
    Wancho,
    Warang_Citi,
    Yi_Radicals,
    Yi_Syllables,
    /// AKA `Yijing`
    Yijing_Hexagram_Symbols,
    Zanabazar_Square,
}
impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <str as fmt::Display>::fmt(self.display_str(), f)
    }
}
impl Block {
    fn display_str(&self) -> &'static str {
        match self {
            Block::Adlam => "Adlam",
            Block::Aegean_Numbers => "Aegean_Numbers",
            Block::Ahom => "Ahom",
            Block::Alchemical_Symbols => "Alchemical",
            Block::Alphabetic_Presentation_Forms => "Alphabetic_PF",
            Block::Anatolian_Hieroglyphs => "Anatolian_Hieroglyphs",
            Block::Ancient_Greek_Musical_Notation => "Ancient_Greek_Music",
            Block::Ancient_Greek_Numbers => "Ancient_Greek_Numbers",
            Block::Ancient_Symbols => "Ancient_Symbols",
            Block::Arabic => "Arabic",
            Block::Arabic_Extended_A => "Arabic_Ext_A",
            Block::Arabic_Mathematical_Alphabetic_Symbols => "Arabic_Math",
            Block::Arabic_Presentation_Forms_A => "Arabic_PF_A",
            Block::Arabic_Presentation_Forms_B => "Arabic_PF_B",
            Block::Arabic_Supplement => "Arabic_Sup",
            Block::Armenian => "Armenian",
            Block::Arrows => "Arrows",
            Block::Basic_Latin => "ASCII",
            Block::Avestan => "Avestan",
            Block::Balinese => "Balinese",
            Block::Bamum => "Bamum",
            Block::Bamum_Supplement => "Bamum_Sup",
            Block::Bassa_Vah => "Bassa_Vah",
            Block::Batak => "Batak",
            Block::Bengali => "Bengali",
            Block::Bhaiksuki => "Bhaiksuki",
            Block::Block_Elements => "Block_Elements",
            Block::Bopomofo => "Bopomofo",
            Block::Bopomofo_Extended => "Bopomofo_Ext",
            Block::Box_Drawing => "Box_Drawing",
            Block::Brahmi => "Brahmi",
            Block::Braille_Patterns => "Braille",
            Block::Buginese => "Buginese",
            Block::Buhid => "Buhid",
            Block::Byzantine_Musical_Symbols => "Byzantine_Music",
            Block::Carian => "Carian",
            Block::Caucasian_Albanian => "Caucasian_Albanian",
            Block::Chakma => "Chakma",
            Block::Cham => "Cham",
            Block::Cherokee => "Cherokee",
            Block::Cherokee_Supplement => "Cherokee_Sup",
            Block::Chess_Symbols => "Chess_Symbols",
            Block::CJK_Unified_Ideographs => "CJK",
            Block::CJK_Compatibility => "CJK_Compat",
            Block::CJK_Compatibility_Forms => "CJK_Compat_Forms",
            Block::CJK_Compatibility_Ideographs => "CJK_Compat_Ideographs",
            Block::CJK_Compatibility_Ideographs_Supplement => "CJK_Compat_Ideographs_Sup",
            Block::CJK_Unified_Ideographs_Extension_A => "CJK_Ext_A",
            Block::CJK_Unified_Ideographs_Extension_B => "CJK_Ext_B",
            Block::CJK_Unified_Ideographs_Extension_C => "CJK_Ext_C",
            Block::CJK_Unified_Ideographs_Extension_D => "CJK_Ext_D",
            Block::CJK_Unified_Ideographs_Extension_E => "CJK_Ext_E",
            Block::CJK_Unified_Ideographs_Extension_F => "CJK_Ext_F",
            Block::CJK_Radicals_Supplement => "CJK_Radicals_Sup",
            Block::CJK_Strokes => "CJK_Strokes",
            Block::CJK_Symbols_And_Punctuation => "CJK_Symbols",
            Block::Hangul_Compatibility_Jamo => "Compat_Jamo",
            Block::Control_Pictures => "Control_Pictures",
            Block::Coptic => "Coptic",
            Block::Coptic_Epact_Numbers => "Coptic_Epact_Numbers",
            Block::Counting_Rod_Numerals => "Counting_Rod",
            Block::Cuneiform => "Cuneiform",
            Block::Cuneiform_Numbers_And_Punctuation => "Cuneiform_Numbers",
            Block::Currency_Symbols => "Currency_Symbols",
            Block::Cypriot_Syllabary => "Cypriot_Syllabary",
            Block::Cyrillic => "Cyrillic",
            Block::Cyrillic_Extended_A => "Cyrillic_Ext_A",
            Block::Cyrillic_Extended_B => "Cyrillic_Ext_B",
            Block::Cyrillic_Extended_C => "Cyrillic_Ext_C",
            Block::Cyrillic_Supplement => "Cyrillic_Sup",
            Block::Deseret => "Deseret",
            Block::Devanagari => "Devanagari",
            Block::Devanagari_Extended => "Devanagari_Ext",
            Block::Combining_Diacritical_Marks => "Diacriticals",
            Block::Combining_Diacritical_Marks_Extended => "Diacriticals_Ext",
            Block::Combining_Diacritical_Marks_For_Symbols => "Diacriticals_For_Symbols",
            Block::Combining_Diacritical_Marks_Supplement => "Diacriticals_Sup",
            Block::Dingbats => "Dingbats",
            Block::Dogra => "Dogra",
            Block::Domino_Tiles => "Domino",
            Block::Duployan => "Duployan",
            Block::Early_Dynastic_Cuneiform => "Early_Dynastic_Cuneiform",
            Block::Egyptian_Hieroglyph_Format_Controls => "Egyptian_Hieroglyph_Format_Controls",
            Block::Egyptian_Hieroglyphs => "Egyptian_Hieroglyphs",
            Block::Elbasan => "Elbasan",
            Block::Elymaic => "Elymaic",
            Block::Emoticons => "Emoticons",
            Block::Enclosed_Alphanumerics => "Enclosed_Alphanum",
            Block::Enclosed_Alphanumeric_Supplement => "Enclosed_Alphanum_Sup",
            Block::Enclosed_CJK_Letters_And_Months => "Enclosed_CJK",
            Block::Enclosed_Ideographic_Supplement => "Enclosed_Ideographic_Sup",
            Block::Ethiopic => "Ethiopic",
            Block::Ethiopic_Extended => "Ethiopic_Ext",
            Block::Ethiopic_Extended_A => "Ethiopic_Ext_A",
            Block::Ethiopic_Supplement => "Ethiopic_Sup",
            Block::Geometric_Shapes => "Geometric_Shapes",
            Block::Geometric_Shapes_Extended => "Geometric_Shapes_Ext",
            Block::Georgian => "Georgian",
            Block::Georgian_Extended => "Georgian_Ext",
            Block::Georgian_Supplement => "Georgian_Sup",
            Block::Glagolitic => "Glagolitic",
            Block::Glagolitic_Supplement => "Glagolitic_Sup",
            Block::Gothic => "Gothic",
            Block::Grantha => "Grantha",
            Block::Greek_And_Coptic => "Greek",
            Block::Greek_Extended => "Greek_Ext",
            Block::Gujarati => "Gujarati",
            Block::Gunjala_Gondi => "Gunjala_Gondi",
            Block::Gurmukhi => "Gurmukhi",
            Block::Halfwidth_And_Fullwidth_Forms => "Half_And_Full_Forms",
            Block::Combining_Half_Marks => "Half_Marks",
            Block::Hangul_Syllables => "Hangul",
            Block::Hanifi_Rohingya => "Hanifi_Rohingya",
            Block::Hanunoo => "Hanunoo",
            Block::Hatran => "Hatran",
            Block::Hebrew => "Hebrew",
            Block::High_Private_Use_Surrogates => "High_PU_Surrogates",
            Block::High_Surrogates => "High_Surrogates",
            Block::Hiragana => "Hiragana",
            Block::Ideographic_Description_Characters => "IDC",
            Block::Ideographic_Symbols_And_Punctuation => "Ideographic_Symbols",
            Block::Imperial_Aramaic => "Imperial_Aramaic",
            Block::Common_Indic_Number_Forms => "Indic_Number_Forms",
            Block::Indic_Siyaq_Numbers => "Indic_Siyaq_Numbers",
            Block::Inscriptional_Pahlavi => "Inscriptional_Pahlavi",
            Block::Inscriptional_Parthian => "Inscriptional_Parthian",
            Block::IPA_Extensions => "IPA_Ext",
            Block::Hangul_Jamo => "Jamo",
            Block::Hangul_Jamo_Extended_A => "Jamo_Ext_A",
            Block::Hangul_Jamo_Extended_B => "Jamo_Ext_B",
            Block::Javanese => "Javanese",
            Block::Kaithi => "Kaithi",
            Block::Kana_Extended_A => "Kana_Ext_A",
            Block::Kana_Supplement => "Kana_Sup",
            Block::Kanbun => "Kanbun",
            Block::Kangxi_Radicals => "Kangxi",
            Block::Kannada => "Kannada",
            Block::Katakana => "Katakana",
            Block::Katakana_Phonetic_Extensions => "Katakana_Ext",
            Block::Kayah_Li => "Kayah_Li",
            Block::Kharoshthi => "Kharoshthi",
            Block::Khmer => "Khmer",
            Block::Khmer_Symbols => "Khmer_Symbols",
            Block::Khojki => "Khojki",
            Block::Khudawadi => "Khudawadi",
            Block::Lao => "Lao",
            Block::Latin_1_Supplement => "Latin_1_Sup",
            Block::Latin_Extended_A => "Latin_Ext_A",
            Block::Latin_Extended_Additional => "Latin_Ext_Additional",
            Block::Latin_Extended_B => "Latin_Ext_B",
            Block::Latin_Extended_C => "Latin_Ext_C",
            Block::Latin_Extended_D => "Latin_Ext_D",
            Block::Latin_Extended_E => "Latin_Ext_E",
            Block::Lepcha => "Lepcha",
            Block::Letterlike_Symbols => "Letterlike_Symbols",
            Block::Limbu => "Limbu",
            Block::Linear_A => "Linear_A",
            Block::Linear_B_Ideograms => "Linear_B_Ideograms",
            Block::Linear_B_Syllabary => "Linear_B_Syllabary",
            Block::Lisu => "Lisu",
            Block::Low_Surrogates => "Low_Surrogates",
            Block::Lycian => "Lycian",
            Block::Lydian => "Lydian",
            Block::Mahajani => "Mahajani",
            Block::Mahjong_Tiles => "Mahjong",
            Block::Makasar => "Makasar",
            Block::Malayalam => "Malayalam",
            Block::Mandaic => "Mandaic",
            Block::Manichaean => "Manichaean",
            Block::Marchen => "Marchen",
            Block::Masaram_Gondi => "Masaram_Gondi",
            Block::Mathematical_Alphanumeric_Symbols => "Math_Alphanum",
            Block::Mathematical_Operators => "Math_Operators",
            Block::Mayan_Numerals => "Mayan_Numerals",
            Block::Medefaidrin => "Medefaidrin",
            Block::Meetei_Mayek => "Meetei_Mayek",
            Block::Meetei_Mayek_Extensions => "Meetei_Mayek_Ext",
            Block::Mende_Kikakui => "Mende_Kikakui",
            Block::Meroitic_Cursive => "Meroitic_Cursive",
            Block::Meroitic_Hieroglyphs => "Meroitic_Hieroglyphs",
            Block::Miao => "Miao",
            Block::Miscellaneous_Symbols_And_Arrows => "Misc_Arrows",
            Block::Miscellaneous_Mathematical_Symbols_A => "Misc_Math_Symbols_A",
            Block::Miscellaneous_Mathematical_Symbols_B => "Misc_Math_Symbols_B",
            Block::Miscellaneous_Symbols_And_Pictographs => "Misc_Pictographs",
            Block::Miscellaneous_Symbols => "Misc_Symbols",
            Block::Miscellaneous_Technical => "Misc_Technical",
            Block::Modi => "Modi",
            Block::Spacing_Modifier_Letters => "Modifier_Letters",
            Block::Modifier_Tone_Letters => "Modifier_Tone_Letters",
            Block::Mongolian => "Mongolian",
            Block::Mongolian_Supplement => "Mongolian_Sup",
            Block::Mro => "Mro",
            Block::Multani => "Multani",
            Block::Musical_Symbols => "Music",
            Block::Myanmar => "Myanmar",
            Block::Myanmar_Extended_A => "Myanmar_Ext_A",
            Block::Myanmar_Extended_B => "Myanmar_Ext_B",
            Block::Nabataean => "Nabataean",
            Block::Nandinagari => "Nandinagari",
            Block::No_Block => "NB",
            Block::New_Tai_Lue => "New_Tai_Lue",
            Block::Newa => "Newa",
            Block::NKo => "NKo",
            Block::Number_Forms => "Number_Forms",
            Block::Nushu => "Nushu",
            Block::Nyiakeng_Puachue_Hmong => "Nyiakeng_Puachue_Hmong",
            Block::Optical_Character_Recognition => "OCR",
            Block::Ogham => "Ogham",
            Block::Ol_Chiki => "Ol_Chiki",
            Block::Old_Hungarian => "Old_Hungarian",
            Block::Old_Italic => "Old_Italic",
            Block::Old_North_Arabian => "Old_North_Arabian",
            Block::Old_Permic => "Old_Permic",
            Block::Old_Persian => "Old_Persian",
            Block::Old_Sogdian => "Old_Sogdian",
            Block::Old_South_Arabian => "Old_South_Arabian",
            Block::Old_Turkic => "Old_Turkic",
            Block::Oriya => "Oriya",
            Block::Ornamental_Dingbats => "Ornamental_Dingbats",
            Block::Osage => "Osage",
            Block::Osmanya => "Osmanya",
            Block::Ottoman_Siyaq_Numbers => "Ottoman_Siyaq_Numbers",
            Block::Pahawh_Hmong => "Pahawh_Hmong",
            Block::Palmyrene => "Palmyrene",
            Block::Pau_Cin_Hau => "Pau_Cin_Hau",
            Block::Phags_Pa => "Phags_Pa",
            Block::Phaistos_Disc => "Phaistos",
            Block::Phoenician => "Phoenician",
            Block::Phonetic_Extensions => "Phonetic_Ext",
            Block::Phonetic_Extensions_Supplement => "Phonetic_Ext_Sup",
            Block::Playing_Cards => "Playing_Cards",
            Block::Psalter_Pahlavi => "Psalter_Pahlavi",
            Block::Private_Use_Area => "PUA",
            Block::General_Punctuation => "Punctuation",
            Block::Rejang => "Rejang",
            Block::Rumi_Numeral_Symbols => "Rumi",
            Block::Runic => "Runic",
            Block::Samaritan => "Samaritan",
            Block::Saurashtra => "Saurashtra",
            Block::Sharada => "Sharada",
            Block::Shavian => "Shavian",
            Block::Shorthand_Format_Controls => "Shorthand_Format_Controls",
            Block::Siddham => "Siddham",
            Block::Sinhala => "Sinhala",
            Block::Sinhala_Archaic_Numbers => "Sinhala_Archaic_Numbers",
            Block::Small_Form_Variants => "Small_Forms",
            Block::Small_Kana_Extension => "Small_Kana_Ext",
            Block::Sogdian => "Sogdian",
            Block::Sora_Sompeng => "Sora_Sompeng",
            Block::Soyombo => "Soyombo",
            Block::Specials => "Specials",
            Block::Sundanese => "Sundanese",
            Block::Sundanese_Supplement => "Sundanese_Sup",
            Block::Supplemental_Arrows_A => "Sup_Arrows_A",
            Block::Supplemental_Arrows_B => "Sup_Arrows_B",
            Block::Supplemental_Arrows_C => "Sup_Arrows_C",
            Block::Supplemental_Mathematical_Operators => "Sup_Math_Operators",
            Block::Supplementary_Private_Use_Area_A => "Sup_PUA_A",
            Block::Supplementary_Private_Use_Area_B => "Sup_PUA_B",
            Block::Supplemental_Punctuation => "Sup_Punctuation",
            Block::Supplemental_Symbols_And_Pictographs => "Sup_Symbols_And_Pictographs",
            Block::Superscripts_And_Subscripts => "Super_And_Sub",
            Block::Sutton_SignWriting => "Sutton_SignWriting",
            Block::Syloti_Nagri => "Syloti_Nagri",
            Block::Symbols_And_Pictographs_Extended_A => "Symbols_And_Pictographs_Ext_A",
            Block::Syriac => "Syriac",
            Block::Syriac_Supplement => "Syriac_Sup",
            Block::Tagalog => "Tagalog",
            Block::Tagbanwa => "Tagbanwa",
            Block::Tags => "Tags",
            Block::Tai_Le => "Tai_Le",
            Block::Tai_Tham => "Tai_Tham",
            Block::Tai_Viet => "Tai_Viet",
            Block::Tai_Xuan_Jing_Symbols => "Tai_Xuan_Jing",
            Block::Takri => "Takri",
            Block::Tamil => "Tamil",
            Block::Tamil_Supplement => "Tamil_Sup",
            Block::Tangut => "Tangut",
            Block::Tangut_Components => "Tangut_Components",
            Block::Telugu => "Telugu",
            Block::Thaana => "Thaana",
            Block::Thai => "Thai",
            Block::Tibetan => "Tibetan",
            Block::Tifinagh => "Tifinagh",
            Block::Tirhuta => "Tirhuta",
            Block::Transport_And_Map_Symbols => "Transport_And_Map",
            Block::Unified_Canadian_Aboriginal_Syllabics => "UCAS",
            Block::Unified_Canadian_Aboriginal_Syllabics_Extended => "UCAS_Ext",
            Block::Ugaritic => "Ugaritic",
            Block::Vai => "Vai",
            Block::Vedic_Extensions => "Vedic_Ext",
            Block::Vertical_Forms => "Vertical_Forms",
            Block::Variation_Selectors => "VS",
            Block::Variation_Selectors_Supplement => "VS_Sup",
            Block::Wancho => "Wancho",
            Block::Warang_Citi => "Warang_Citi",
            Block::Yi_Radicals => "Yi_Radicals",
            Block::Yi_Syllables => "Yi_Syllables",
            Block::Yijing_Hexagram_Symbols => "Yijing",
            Block::Zanabazar_Square => "Zanabazar_Square",
        }
    }
}
impl FromStr for Block {
    type Err = ParseError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "Adlam" => Ok(Block::Adlam),
            "Aegean_Numbers" => Ok(Block::Aegean_Numbers),
            "Ahom" => Ok(Block::Ahom),
            "Alchemical" => Ok(Block::Alchemical_Symbols),
            "Alphabetic_PF" => Ok(Block::Alphabetic_Presentation_Forms),
            "Anatolian_Hieroglyphs" => Ok(Block::Anatolian_Hieroglyphs),
            "Ancient_Greek_Music" => Ok(Block::Ancient_Greek_Musical_Notation),
            "Ancient_Greek_Numbers" => Ok(Block::Ancient_Greek_Numbers),
            "Ancient_Symbols" => Ok(Block::Ancient_Symbols),
            "Arabic" => Ok(Block::Arabic),
            "Arabic_Ext_A" => Ok(Block::Arabic_Extended_A),
            "Arabic_Math" => Ok(Block::Arabic_Mathematical_Alphabetic_Symbols),
            "Arabic_PF_A" => Ok(Block::Arabic_Presentation_Forms_A),
            "Arabic_PF_B" => Ok(Block::Arabic_Presentation_Forms_B),
            "Arabic_Sup" => Ok(Block::Arabic_Supplement),
            "Armenian" => Ok(Block::Armenian),
            "Arrows" => Ok(Block::Arrows),
            "ASCII" => Ok(Block::Basic_Latin),
            "Avestan" => Ok(Block::Avestan),
            "Balinese" => Ok(Block::Balinese),
            "Bamum" => Ok(Block::Bamum),
            "Bamum_Sup" => Ok(Block::Bamum_Supplement),
            "Bassa_Vah" => Ok(Block::Bassa_Vah),
            "Batak" => Ok(Block::Batak),
            "Bengali" => Ok(Block::Bengali),
            "Bhaiksuki" => Ok(Block::Bhaiksuki),
            "Block_Elements" => Ok(Block::Block_Elements),
            "Bopomofo" => Ok(Block::Bopomofo),
            "Bopomofo_Ext" => Ok(Block::Bopomofo_Extended),
            "Box_Drawing" => Ok(Block::Box_Drawing),
            "Brahmi" => Ok(Block::Brahmi),
            "Braille" => Ok(Block::Braille_Patterns),
            "Buginese" => Ok(Block::Buginese),
            "Buhid" => Ok(Block::Buhid),
            "Byzantine_Music" => Ok(Block::Byzantine_Musical_Symbols),
            "Carian" => Ok(Block::Carian),
            "Caucasian_Albanian" => Ok(Block::Caucasian_Albanian),
            "Chakma" => Ok(Block::Chakma),
            "Cham" => Ok(Block::Cham),
            "Cherokee" => Ok(Block::Cherokee),
            "Cherokee_Sup" => Ok(Block::Cherokee_Supplement),
            "Chess_Symbols" => Ok(Block::Chess_Symbols),
            "CJK" => Ok(Block::CJK_Unified_Ideographs),
            "CJK_Compat" => Ok(Block::CJK_Compatibility),
            "CJK_Compat_Forms" => Ok(Block::CJK_Compatibility_Forms),
            "CJK_Compat_Ideographs" => Ok(Block::CJK_Compatibility_Ideographs),
            "CJK_Compat_Ideographs_Sup" => Ok(Block::CJK_Compatibility_Ideographs_Supplement),
            "CJK_Ext_A" => Ok(Block::CJK_Unified_Ideographs_Extension_A),
            "CJK_Ext_B" => Ok(Block::CJK_Unified_Ideographs_Extension_B),
            "CJK_Ext_C" => Ok(Block::CJK_Unified_Ideographs_Extension_C),
            "CJK_Ext_D" => Ok(Block::CJK_Unified_Ideographs_Extension_D),
            "CJK_Ext_E" => Ok(Block::CJK_Unified_Ideographs_Extension_E),
            "CJK_Ext_F" => Ok(Block::CJK_Unified_Ideographs_Extension_F),
            "CJK_Radicals_Sup" => Ok(Block::CJK_Radicals_Supplement),
            "CJK_Strokes" => Ok(Block::CJK_Strokes),
            "CJK_Symbols" => Ok(Block::CJK_Symbols_And_Punctuation),
            "Compat_Jamo" => Ok(Block::Hangul_Compatibility_Jamo),
            "Control_Pictures" => Ok(Block::Control_Pictures),
            "Coptic" => Ok(Block::Coptic),
            "Coptic_Epact_Numbers" => Ok(Block::Coptic_Epact_Numbers),
            "Counting_Rod" => Ok(Block::Counting_Rod_Numerals),
            "Cuneiform" => Ok(Block::Cuneiform),
            "Cuneiform_Numbers" => Ok(Block::Cuneiform_Numbers_And_Punctuation),
            "Currency_Symbols" => Ok(Block::Currency_Symbols),
            "Cypriot_Syllabary" => Ok(Block::Cypriot_Syllabary),
            "Cyrillic" => Ok(Block::Cyrillic),
            "Cyrillic_Ext_A" => Ok(Block::Cyrillic_Extended_A),
            "Cyrillic_Ext_B" => Ok(Block::Cyrillic_Extended_B),
            "Cyrillic_Ext_C" => Ok(Block::Cyrillic_Extended_C),
            "Cyrillic_Sup" => Ok(Block::Cyrillic_Supplement),
            "Deseret" => Ok(Block::Deseret),
            "Devanagari" => Ok(Block::Devanagari),
            "Devanagari_Ext" => Ok(Block::Devanagari_Extended),
            "Diacriticals" => Ok(Block::Combining_Diacritical_Marks),
            "Diacriticals_Ext" => Ok(Block::Combining_Diacritical_Marks_Extended),
            "Diacriticals_For_Symbols" => Ok(Block::Combining_Diacritical_Marks_For_Symbols),
            "Diacriticals_Sup" => Ok(Block::Combining_Diacritical_Marks_Supplement),
            "Dingbats" => Ok(Block::Dingbats),
            "Dogra" => Ok(Block::Dogra),
            "Domino" => Ok(Block::Domino_Tiles),
            "Duployan" => Ok(Block::Duployan),
            "Early_Dynastic_Cuneiform" => Ok(Block::Early_Dynastic_Cuneiform),
            "Egyptian_Hieroglyph_Format_Controls" => Ok(Block::Egyptian_Hieroglyph_Format_Controls),
            "Egyptian_Hieroglyphs" => Ok(Block::Egyptian_Hieroglyphs),
            "Elbasan" => Ok(Block::Elbasan),
            "Elymaic" => Ok(Block::Elymaic),
            "Emoticons" => Ok(Block::Emoticons),
            "Enclosed_Alphanum" => Ok(Block::Enclosed_Alphanumerics),
            "Enclosed_Alphanum_Sup" => Ok(Block::Enclosed_Alphanumeric_Supplement),
            "Enclosed_CJK" => Ok(Block::Enclosed_CJK_Letters_And_Months),
            "Enclosed_Ideographic_Sup" => Ok(Block::Enclosed_Ideographic_Supplement),
            "Ethiopic" => Ok(Block::Ethiopic),
            "Ethiopic_Ext" => Ok(Block::Ethiopic_Extended),
            "Ethiopic_Ext_A" => Ok(Block::Ethiopic_Extended_A),
            "Ethiopic_Sup" => Ok(Block::Ethiopic_Supplement),
            "Geometric_Shapes" => Ok(Block::Geometric_Shapes),
            "Geometric_Shapes_Ext" => Ok(Block::Geometric_Shapes_Extended),
            "Georgian" => Ok(Block::Georgian),
            "Georgian_Ext" => Ok(Block::Georgian_Extended),
            "Georgian_Sup" => Ok(Block::Georgian_Supplement),
            "Glagolitic" => Ok(Block::Glagolitic),
            "Glagolitic_Sup" => Ok(Block::Glagolitic_Supplement),
            "Gothic" => Ok(Block::Gothic),
            "Grantha" => Ok(Block::Grantha),
            "Greek" => Ok(Block::Greek_And_Coptic),
            "Greek_Ext" => Ok(Block::Greek_Extended),
            "Gujarati" => Ok(Block::Gujarati),
            "Gunjala_Gondi" => Ok(Block::Gunjala_Gondi),
            "Gurmukhi" => Ok(Block::Gurmukhi),
            "Half_And_Full_Forms" => Ok(Block::Halfwidth_And_Fullwidth_Forms),
            "Half_Marks" => Ok(Block::Combining_Half_Marks),
            "Hangul" => Ok(Block::Hangul_Syllables),
            "Hanifi_Rohingya" => Ok(Block::Hanifi_Rohingya),
            "Hanunoo" => Ok(Block::Hanunoo),
            "Hatran" => Ok(Block::Hatran),
            "Hebrew" => Ok(Block::Hebrew),
            "High_PU_Surrogates" => Ok(Block::High_Private_Use_Surrogates),
            "High_Surrogates" => Ok(Block::High_Surrogates),
            "Hiragana" => Ok(Block::Hiragana),
            "IDC" => Ok(Block::Ideographic_Description_Characters),
            "Ideographic_Symbols" => Ok(Block::Ideographic_Symbols_And_Punctuation),
            "Imperial_Aramaic" => Ok(Block::Imperial_Aramaic),
            "Indic_Number_Forms" => Ok(Block::Common_Indic_Number_Forms),
            "Indic_Siyaq_Numbers" => Ok(Block::Indic_Siyaq_Numbers),
            "Inscriptional_Pahlavi" => Ok(Block::Inscriptional_Pahlavi),
            "Inscriptional_Parthian" => Ok(Block::Inscriptional_Parthian),
            "IPA_Ext" => Ok(Block::IPA_Extensions),
            "Jamo" => Ok(Block::Hangul_Jamo),
            "Jamo_Ext_A" => Ok(Block::Hangul_Jamo_Extended_A),
            "Jamo_Ext_B" => Ok(Block::Hangul_Jamo_Extended_B),
            "Javanese" => Ok(Block::Javanese),
            "Kaithi" => Ok(Block::Kaithi),
            "Kana_Ext_A" => Ok(Block::Kana_Extended_A),
            "Kana_Sup" => Ok(Block::Kana_Supplement),
            "Kanbun" => Ok(Block::Kanbun),
            "Kangxi" => Ok(Block::Kangxi_Radicals),
            "Kannada" => Ok(Block::Kannada),
            "Katakana" => Ok(Block::Katakana),
            "Katakana_Ext" => Ok(Block::Katakana_Phonetic_Extensions),
            "Kayah_Li" => Ok(Block::Kayah_Li),
            "Kharoshthi" => Ok(Block::Kharoshthi),
            "Khmer" => Ok(Block::Khmer),
            "Khmer_Symbols" => Ok(Block::Khmer_Symbols),
            "Khojki" => Ok(Block::Khojki),
            "Khudawadi" => Ok(Block::Khudawadi),
            "Lao" => Ok(Block::Lao),
            "Latin_1_Sup" => Ok(Block::Latin_1_Supplement),
            "Latin_Ext_A" => Ok(Block::Latin_Extended_A),
            "Latin_Ext_Additional" => Ok(Block::Latin_Extended_Additional),
            "Latin_Ext_B" => Ok(Block::Latin_Extended_B),
            "Latin_Ext_C" => Ok(Block::Latin_Extended_C),
            "Latin_Ext_D" => Ok(Block::Latin_Extended_D),
            "Latin_Ext_E" => Ok(Block::Latin_Extended_E),
            "Lepcha" => Ok(Block::Lepcha),
            "Letterlike_Symbols" => Ok(Block::Letterlike_Symbols),
            "Limbu" => Ok(Block::Limbu),
            "Linear_A" => Ok(Block::Linear_A),
            "Linear_B_Ideograms" => Ok(Block::Linear_B_Ideograms),
            "Linear_B_Syllabary" => Ok(Block::Linear_B_Syllabary),
            "Lisu" => Ok(Block::Lisu),
            "Low_Surrogates" => Ok(Block::Low_Surrogates),
            "Lycian" => Ok(Block::Lycian),
            "Lydian" => Ok(Block::Lydian),
            "Mahajani" => Ok(Block::Mahajani),
            "Mahjong" => Ok(Block::Mahjong_Tiles),
            "Makasar" => Ok(Block::Makasar),
            "Malayalam" => Ok(Block::Malayalam),
            "Mandaic" => Ok(Block::Mandaic),
            "Manichaean" => Ok(Block::Manichaean),
            "Marchen" => Ok(Block::Marchen),
            "Masaram_Gondi" => Ok(Block::Masaram_Gondi),
            "Math_Alphanum" => Ok(Block::Mathematical_Alphanumeric_Symbols),
            "Math_Operators" => Ok(Block::Mathematical_Operators),
            "Mayan_Numerals" => Ok(Block::Mayan_Numerals),
            "Medefaidrin" => Ok(Block::Medefaidrin),
            "Meetei_Mayek" => Ok(Block::Meetei_Mayek),
            "Meetei_Mayek_Ext" => Ok(Block::Meetei_Mayek_Extensions),
            "Mende_Kikakui" => Ok(Block::Mende_Kikakui),
            "Meroitic_Cursive" => Ok(Block::Meroitic_Cursive),
            "Meroitic_Hieroglyphs" => Ok(Block::Meroitic_Hieroglyphs),
            "Miao" => Ok(Block::Miao),
            "Misc_Arrows" => Ok(Block::Miscellaneous_Symbols_And_Arrows),
            "Misc_Math_Symbols_A" => Ok(Block::Miscellaneous_Mathematical_Symbols_A),
            "Misc_Math_Symbols_B" => Ok(Block::Miscellaneous_Mathematical_Symbols_B),
            "Misc_Pictographs" => Ok(Block::Miscellaneous_Symbols_And_Pictographs),
            "Misc_Symbols" => Ok(Block::Miscellaneous_Symbols),
            "Misc_Technical" => Ok(Block::Miscellaneous_Technical),
            "Modi" => Ok(Block::Modi),
            "Modifier_Letters" => Ok(Block::Spacing_Modifier_Letters),
            "Modifier_Tone_Letters" => Ok(Block::Modifier_Tone_Letters),
            "Mongolian" => Ok(Block::Mongolian),
            "Mongolian_Sup" => Ok(Block::Mongolian_Supplement),
            "Mro" => Ok(Block::Mro),
            "Multani" => Ok(Block::Multani),
            "Music" => Ok(Block::Musical_Symbols),
            "Myanmar" => Ok(Block::Myanmar),
            "Myanmar_Ext_A" => Ok(Block::Myanmar_Extended_A),
            "Myanmar_Ext_B" => Ok(Block::Myanmar_Extended_B),
            "Nabataean" => Ok(Block::Nabataean),
            "Nandinagari" => Ok(Block::Nandinagari),
            "NB" => Ok(Block::No_Block),
            "New_Tai_Lue" => Ok(Block::New_Tai_Lue),
            "Newa" => Ok(Block::Newa),
            "NKo" => Ok(Block::NKo),
            "Number_Forms" => Ok(Block::Number_Forms),
            "Nushu" => Ok(Block::Nushu),
            "Nyiakeng_Puachue_Hmong" => Ok(Block::Nyiakeng_Puachue_Hmong),
            "OCR" => Ok(Block::Optical_Character_Recognition),
            "Ogham" => Ok(Block::Ogham),
            "Ol_Chiki" => Ok(Block::Ol_Chiki),
            "Old_Hungarian" => Ok(Block::Old_Hungarian),
            "Old_Italic" => Ok(Block::Old_Italic),
            "Old_North_Arabian" => Ok(Block::Old_North_Arabian),
            "Old_Permic" => Ok(Block::Old_Permic),
            "Old_Persian" => Ok(Block::Old_Persian),
            "Old_Sogdian" => Ok(Block::Old_Sogdian),
            "Old_South_Arabian" => Ok(Block::Old_South_Arabian),
            "Old_Turkic" => Ok(Block::Old_Turkic),
            "Oriya" => Ok(Block::Oriya),
            "Ornamental_Dingbats" => Ok(Block::Ornamental_Dingbats),
            "Osage" => Ok(Block::Osage),
            "Osmanya" => Ok(Block::Osmanya),
            "Ottoman_Siyaq_Numbers" => Ok(Block::Ottoman_Siyaq_Numbers),
            "Pahawh_Hmong" => Ok(Block::Pahawh_Hmong),
            "Palmyrene" => Ok(Block::Palmyrene),
            "Pau_Cin_Hau" => Ok(Block::Pau_Cin_Hau),
            "Phags_Pa" => Ok(Block::Phags_Pa),
            "Phaistos" => Ok(Block::Phaistos_Disc),
            "Phoenician" => Ok(Block::Phoenician),
            "Phonetic_Ext" => Ok(Block::Phonetic_Extensions),
            "Phonetic_Ext_Sup" => Ok(Block::Phonetic_Extensions_Supplement),
            "Playing_Cards" => Ok(Block::Playing_Cards),
            "Psalter_Pahlavi" => Ok(Block::Psalter_Pahlavi),
            "PUA" => Ok(Block::Private_Use_Area),
            "Punctuation" => Ok(Block::General_Punctuation),
            "Rejang" => Ok(Block::Rejang),
            "Rumi" => Ok(Block::Rumi_Numeral_Symbols),
            "Runic" => Ok(Block::Runic),
            "Samaritan" => Ok(Block::Samaritan),
            "Saurashtra" => Ok(Block::Saurashtra),
            "Sharada" => Ok(Block::Sharada),
            "Shavian" => Ok(Block::Shavian),
            "Shorthand_Format_Controls" => Ok(Block::Shorthand_Format_Controls),
            "Siddham" => Ok(Block::Siddham),
            "Sinhala" => Ok(Block::Sinhala),
            "Sinhala_Archaic_Numbers" => Ok(Block::Sinhala_Archaic_Numbers),
            "Small_Forms" => Ok(Block::Small_Form_Variants),
            "Small_Kana_Ext" => Ok(Block::Small_Kana_Extension),
            "Sogdian" => Ok(Block::Sogdian),
            "Sora_Sompeng" => Ok(Block::Sora_Sompeng),
            "Soyombo" => Ok(Block::Soyombo),
            "Specials" => Ok(Block::Specials),
            "Sundanese" => Ok(Block::Sundanese),
            "Sundanese_Sup" => Ok(Block::Sundanese_Supplement),
            "Sup_Arrows_A" => Ok(Block::Supplemental_Arrows_A),
            "Sup_Arrows_B" => Ok(Block::Supplemental_Arrows_B),
            "Sup_Arrows_C" => Ok(Block::Supplemental_Arrows_C),
            "Sup_Math_Operators" => Ok(Block::Supplemental_Mathematical_Operators),
            "Sup_PUA_A" => Ok(Block::Supplementary_Private_Use_Area_A),
            "Sup_PUA_B" => Ok(Block::Supplementary_Private_Use_Area_B),
            "Sup_Punctuation" => Ok(Block::Supplemental_Punctuation),
            "Sup_Symbols_And_Pictographs" => Ok(Block::Supplemental_Symbols_And_Pictographs),
            "Super_And_Sub" => Ok(Block::Superscripts_And_Subscripts),
            "Sutton_SignWriting" => Ok(Block::Sutton_SignWriting),
            "Syloti_Nagri" => Ok(Block::Syloti_Nagri),
            "Symbols_And_Pictographs_Ext_A" => Ok(Block::Symbols_And_Pictographs_Extended_A),
            "Syriac" => Ok(Block::Syriac),
            "Syriac_Sup" => Ok(Block::Syriac_Supplement),
            "Tagalog" => Ok(Block::Tagalog),
            "Tagbanwa" => Ok(Block::Tagbanwa),
            "Tags" => Ok(Block::Tags),
            "Tai_Le" => Ok(Block::Tai_Le),
            "Tai_Tham" => Ok(Block::Tai_Tham),
            "Tai_Viet" => Ok(Block::Tai_Viet),
            "Tai_Xuan_Jing" => Ok(Block::Tai_Xuan_Jing_Symbols),
            "Takri" => Ok(Block::Takri),
            "Tamil" => Ok(Block::Tamil),
            "Tamil_Sup" => Ok(Block::Tamil_Supplement),
            "Tangut" => Ok(Block::Tangut),
            "Tangut_Components" => Ok(Block::Tangut_Components),
            "Telugu" => Ok(Block::Telugu),
            "Thaana" => Ok(Block::Thaana),
            "Thai" => Ok(Block::Thai),
            "Tibetan" => Ok(Block::Tibetan),
            "Tifinagh" => Ok(Block::Tifinagh),
            "Tirhuta" => Ok(Block::Tirhuta),
            "Transport_And_Map" => Ok(Block::Transport_And_Map_Symbols),
            "UCAS" => Ok(Block::Unified_Canadian_Aboriginal_Syllabics),
            "UCAS_Ext" => Ok(Block::Unified_Canadian_Aboriginal_Syllabics_Extended),
            "Ugaritic" => Ok(Block::Ugaritic),
            "Vai" => Ok(Block::Vai),
            "Vedic_Ext" => Ok(Block::Vedic_Extensions),
            "Vertical_Forms" => Ok(Block::Vertical_Forms),
            "VS" => Ok(Block::Variation_Selectors),
            "VS_Sup" => Ok(Block::Variation_Selectors_Supplement),
            "Wancho" => Ok(Block::Wancho),
            "Warang_Citi" => Ok(Block::Warang_Citi),
            "Yi_Radicals" => Ok(Block::Yi_Radicals),
            "Yi_Syllables" => Ok(Block::Yi_Syllables),
            "Yijing" => Ok(Block::Yijing_Hexagram_Symbols),
            "Zanabazar_Square" => Ok(Block::Zanabazar_Square),
            _ => Err(ParseError(())),
        }
    }
}

/// AKA `sc`
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Script {
    /// AKA `Adlm`
    Adlam,
    /// AKA `Aghb`
    Caucasian_Albanian,
    Ahom,
    /// AKA `Arab`
    Arabic,
    /// AKA `Armi`
    Imperial_Aramaic,
    /// AKA `Armn`
    Armenian,
    /// AKA `Avst`
    Avestan,
    /// AKA `Bali`
    Balinese,
    /// AKA `Bamu`
    Bamum,
    /// AKA `Bass`
    Bassa_Vah,
    /// AKA `Batk`
    Batak,
    /// AKA `Beng`
    Bengali,
    /// AKA `Bhks`
    Bhaiksuki,
    /// AKA `Bopo`
    Bopomofo,
    /// AKA `Brah`
    Brahmi,
    /// AKA `Brai`
    Braille,
    /// AKA `Bugi`
    Buginese,
    /// AKA `Buhd`
    Buhid,
    /// AKA `Cakm`
    Chakma,
    /// AKA `Cans`
    Canadian_Aboriginal,
    /// AKA `Cari`
    Carian,
    Cham,
    /// AKA `Cher`
    Cherokee,
    /// AKA `Copt`
    Coptic,
    /// AKA `Cprt`
    Cypriot,
    /// AKA `Cyrl`
    Cyrillic,
    /// AKA `Deva`
    Devanagari,
    /// AKA `Dogr`
    Dogra,
    /// AKA `Dsrt`
    Deseret,
    /// AKA `Dupl`
    Duployan,
    /// AKA `Egyp`
    Egyptian_Hieroglyphs,
    /// AKA `Elba`
    Elbasan,
    /// AKA `Elym`
    Elymaic,
    /// AKA `Ethi`
    Ethiopic,
    /// AKA `Geor`
    Georgian,
    /// AKA `Glag`
    Glagolitic,
    /// AKA `Gong`
    Gunjala_Gondi,
    /// AKA `Gonm`
    Masaram_Gondi,
    /// AKA `Goth`
    Gothic,
    /// AKA `Gran`
    Grantha,
    /// AKA `Grek`
    Greek,
    /// AKA `Gujr`
    Gujarati,
    /// AKA `Guru`
    Gurmukhi,
    /// AKA `Hang`
    Hangul,
    /// AKA `Hani`
    Han,
    /// AKA `Hano`
    Hanunoo,
    /// AKA `Hatr`
    Hatran,
    /// AKA `Hebr`
    Hebrew,
    /// AKA `Hira`
    Hiragana,
    /// AKA `Hluw`
    Anatolian_Hieroglyphs,
    /// AKA `Hmng`
    Pahawh_Hmong,
    /// AKA `Hmnp`
    Nyiakeng_Puachue_Hmong,
    /// AKA `Hrkt`
    Katakana_Or_Hiragana,
    /// AKA `Hung`
    Old_Hungarian,
    /// AKA `Ital`
    Old_Italic,
    /// AKA `Java`
    Javanese,
    /// AKA `Kali`
    Kayah_Li,
    /// AKA `Kana`
    Katakana,
    /// AKA `Khar`
    Kharoshthi,
    /// AKA `Khmr`
    Khmer,
    /// AKA `Khoj`
    Khojki,
    /// AKA `Knda`
    Kannada,
    /// AKA `Kthi`
    Kaithi,
    /// AKA `Lana`
    Tai_Tham,
    /// AKA `Laoo`
    Lao,
    /// AKA `Latn`
    Latin,
    /// AKA `Lepc`
    Lepcha,
    /// AKA `Limb`
    Limbu,
    /// AKA `Lina`
    Linear_A,
    /// AKA `Linb`
    Linear_B,
    Lisu,
    /// AKA `Lyci`
    Lycian,
    /// AKA `Lydi`
    Lydian,
    /// AKA `Mahj`
    Mahajani,
    /// AKA `Maka`
    Makasar,
    /// AKA `Mand`
    Mandaic,
    /// AKA `Mani`
    Manichaean,
    /// AKA `Marc`
    Marchen,
    /// AKA `Medf`
    Medefaidrin,
    /// AKA `Mend`
    Mende_Kikakui,
    /// AKA `Merc`
    Meroitic_Cursive,
    /// AKA `Mero`
    Meroitic_Hieroglyphs,
    /// AKA `Mlym`
    Malayalam,
    Modi,
    /// AKA `Mong`
    Mongolian,
    /// AKA `Mroo`
    Mro,
    /// AKA `Mtei`
    Meetei_Mayek,
    /// AKA `Mult`
    Multani,
    /// AKA `Mymr`
    Myanmar,
    /// AKA `Nand`
    Nandinagari,
    /// AKA `Narb`
    Old_North_Arabian,
    /// AKA `Nbat`
    Nabataean,
    Newa,
    /// AKA `Nkoo`
    Nko,
    /// AKA `Nshu`
    Nushu,
    /// AKA `Ogam`
    Ogham,
    /// AKA `Olck`
    Ol_Chiki,
    /// AKA `Orkh`
    Old_Turkic,
    /// AKA `Orya`
    Oriya,
    /// AKA `Osge`
    Osage,
    /// AKA `Osma`
    Osmanya,
    /// AKA `Palm`
    Palmyrene,
    /// AKA `Pauc`
    Pau_Cin_Hau,
    /// AKA `Perm`
    Old_Permic,
    /// AKA `Phag`
    Phags_Pa,
    /// AKA `Phli`
    Inscriptional_Pahlavi,
    /// AKA `Phlp`
    Psalter_Pahlavi,
    /// AKA `Phnx`
    Phoenician,
    /// AKA `Plrd`
    Miao,
    /// AKA `Prti`
    Inscriptional_Parthian,
    /// AKA `Rjng`
    Rejang,
    /// AKA `Rohg`
    Hanifi_Rohingya,
    /// AKA `Runr`
    Runic,
    /// AKA `Samr`
    Samaritan,
    /// AKA `Sarb`
    Old_South_Arabian,
    /// AKA `Saur`
    Saurashtra,
    /// AKA `Sgnw`
    SignWriting,
    /// AKA `Shaw`
    Shavian,
    /// AKA `Shrd`
    Sharada,
    /// AKA `Sidd`
    Siddham,
    /// AKA `Sind`
    Khudawadi,
    /// AKA `Sinh`
    Sinhala,
    /// AKA `Sogd`
    Sogdian,
    /// AKA `Sogo`
    Old_Sogdian,
    /// AKA `Sora`
    Sora_Sompeng,
    /// AKA `Soyo`
    Soyombo,
    /// AKA `Sund`
    Sundanese,
    /// AKA `Sylo`
    Syloti_Nagri,
    /// AKA `Syrc`
    Syriac,
    /// AKA `Tagb`
    Tagbanwa,
    /// AKA `Takr`
    Takri,
    /// AKA `Tale`
    Tai_Le,
    /// AKA `Talu`
    New_Tai_Lue,
    /// AKA `Taml`
    Tamil,
    /// AKA `Tang`
    Tangut,
    /// AKA `Tavt`
    Tai_Viet,
    /// AKA `Telu`
    Telugu,
    /// AKA `Tfng`
    Tifinagh,
    /// AKA `Tglg`
    Tagalog,
    /// AKA `Thaa`
    Thaana,
    Thai,
    /// AKA `Tibt`
    Tibetan,
    /// AKA `Tirh`
    Tirhuta,
    /// AKA `Ugar`
    Ugaritic,
    /// AKA `Vaii`
    Vai,
    /// AKA `Wara`
    Warang_Citi,
    /// AKA `Wcho`
    Wancho,
    /// AKA `Xpeo`
    Old_Persian,
    /// AKA `Xsux`
    Cuneiform,
    /// AKA `Yiii`
    Yi,
    /// AKA `Zanb`
    Zanabazar_Square,
    /// AKA `Zinh`
    Inherited,
    /// AKA `Zyyy`
    Common,
    /// AKA `Zzzz`
    Unknown,
}
impl fmt::Display for Script {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <str as fmt::Display>::fmt(self.display_str(), f)
    }
}
impl Script {
    fn display_str(&self) -> &'static str {
        match self {
            Script::Adlam => "Adlm",
            Script::Caucasian_Albanian => "Aghb",
            Script::Ahom => "Ahom",
            Script::Arabic => "Arab",
            Script::Imperial_Aramaic => "Armi",
            Script::Armenian => "Armn",
            Script::Avestan => "Avst",
            Script::Balinese => "Bali",
            Script::Bamum => "Bamu",
            Script::Bassa_Vah => "Bass",
            Script::Batak => "Batk",
            Script::Bengali => "Beng",
            Script::Bhaiksuki => "Bhks",
            Script::Bopomofo => "Bopo",
            Script::Brahmi => "Brah",
            Script::Braille => "Brai",
            Script::Buginese => "Bugi",
            Script::Buhid => "Buhd",
            Script::Chakma => "Cakm",
            Script::Canadian_Aboriginal => "Cans",
            Script::Carian => "Cari",
            Script::Cham => "Cham",
            Script::Cherokee => "Cher",
            Script::Coptic => "Copt",
            Script::Cypriot => "Cprt",
            Script::Cyrillic => "Cyrl",
            Script::Devanagari => "Deva",
            Script::Dogra => "Dogr",
            Script::Deseret => "Dsrt",
            Script::Duployan => "Dupl",
            Script::Egyptian_Hieroglyphs => "Egyp",
            Script::Elbasan => "Elba",
            Script::Elymaic => "Elym",
            Script::Ethiopic => "Ethi",
            Script::Georgian => "Geor",
            Script::Glagolitic => "Glag",
            Script::Gunjala_Gondi => "Gong",
            Script::Masaram_Gondi => "Gonm",
            Script::Gothic => "Goth",
            Script::Grantha => "Gran",
            Script::Greek => "Grek",
            Script::Gujarati => "Gujr",
            Script::Gurmukhi => "Guru",
            Script::Hangul => "Hang",
            Script::Han => "Hani",
            Script::Hanunoo => "Hano",
            Script::Hatran => "Hatr",
            Script::Hebrew => "Hebr",
            Script::Hiragana => "Hira",
            Script::Anatolian_Hieroglyphs => "Hluw",
            Script::Pahawh_Hmong => "Hmng",
            Script::Nyiakeng_Puachue_Hmong => "Hmnp",
            Script::Katakana_Or_Hiragana => "Hrkt",
            Script::Old_Hungarian => "Hung",
            Script::Old_Italic => "Ital",
            Script::Javanese => "Java",
            Script::Kayah_Li => "Kali",
            Script::Katakana => "Kana",
            Script::Kharoshthi => "Khar",
            Script::Khmer => "Khmr",
            Script::Khojki => "Khoj",
            Script::Kannada => "Knda",
            Script::Kaithi => "Kthi",
            Script::Tai_Tham => "Lana",
            Script::Lao => "Laoo",
            Script::Latin => "Latn",
            Script::Lepcha => "Lepc",
            Script::Limbu => "Limb",
            Script::Linear_A => "Lina",
            Script::Linear_B => "Linb",
            Script::Lisu => "Lisu",
            Script::Lycian => "Lyci",
            Script::Lydian => "Lydi",
            Script::Mahajani => "Mahj",
            Script::Makasar => "Maka",
            Script::Mandaic => "Mand",
            Script::Manichaean => "Mani",
            Script::Marchen => "Marc",
            Script::Medefaidrin => "Medf",
            Script::Mende_Kikakui => "Mend",
            Script::Meroitic_Cursive => "Merc",
            Script::Meroitic_Hieroglyphs => "Mero",
            Script::Malayalam => "Mlym",
            Script::Modi => "Modi",
            Script::Mongolian => "Mong",
            Script::Mro => "Mroo",
            Script::Meetei_Mayek => "Mtei",
            Script::Multani => "Mult",
            Script::Myanmar => "Mymr",
            Script::Nandinagari => "Nand",
            Script::Old_North_Arabian => "Narb",
            Script::Nabataean => "Nbat",
            Script::Newa => "Newa",
            Script::Nko => "Nkoo",
            Script::Nushu => "Nshu",
            Script::Ogham => "Ogam",
            Script::Ol_Chiki => "Olck",
            Script::Old_Turkic => "Orkh",
            Script::Oriya => "Orya",
            Script::Osage => "Osge",
            Script::Osmanya => "Osma",
            Script::Palmyrene => "Palm",
            Script::Pau_Cin_Hau => "Pauc",
            Script::Old_Permic => "Perm",
            Script::Phags_Pa => "Phag",
            Script::Inscriptional_Pahlavi => "Phli",
            Script::Psalter_Pahlavi => "Phlp",
            Script::Phoenician => "Phnx",
            Script::Miao => "Plrd",
            Script::Inscriptional_Parthian => "Prti",
            Script::Rejang => "Rjng",
            Script::Hanifi_Rohingya => "Rohg",
            Script::Runic => "Runr",
            Script::Samaritan => "Samr",
            Script::Old_South_Arabian => "Sarb",
            Script::Saurashtra => "Saur",
            Script::SignWriting => "Sgnw",
            Script::Shavian => "Shaw",
            Script::Sharada => "Shrd",
            Script::Siddham => "Sidd",
            Script::Khudawadi => "Sind",
            Script::Sinhala => "Sinh",
            Script::Sogdian => "Sogd",
            Script::Old_Sogdian => "Sogo",
            Script::Sora_Sompeng => "Sora",
            Script::Soyombo => "Soyo",
            Script::Sundanese => "Sund",
            Script::Syloti_Nagri => "Sylo",
            Script::Syriac => "Syrc",
            Script::Tagbanwa => "Tagb",
            Script::Takri => "Takr",
            Script::Tai_Le => "Tale",
            Script::New_Tai_Lue => "Talu",
            Script::Tamil => "Taml",
            Script::Tangut => "Tang",
            Script::Tai_Viet => "Tavt",
            Script::Telugu => "Telu",
            Script::Tifinagh => "Tfng",
            Script::Tagalog => "Tglg",
            Script::Thaana => "Thaa",
            Script::Thai => "Thai",
            Script::Tibetan => "Tibt",
            Script::Tirhuta => "Tirh",
            Script::Ugaritic => "Ugar",
            Script::Vai => "Vaii",
            Script::Warang_Citi => "Wara",
            Script::Wancho => "Wcho",
            Script::Old_Persian => "Xpeo",
            Script::Cuneiform => "Xsux",
            Script::Yi => "Yiii",
            Script::Zanabazar_Square => "Zanb",
            Script::Inherited => "Zinh",
            Script::Common => "Zyyy",
            Script::Unknown => "Zzzz",
        }
    }
}
impl FromStr for Script {
    type Err = ParseError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "Adlm" => Ok(Script::Adlam),
            "Aghb" => Ok(Script::Caucasian_Albanian),
            "Ahom" => Ok(Script::Ahom),
            "Arab" => Ok(Script::Arabic),
            "Armi" => Ok(Script::Imperial_Aramaic),
            "Armn" => Ok(Script::Armenian),
            "Avst" => Ok(Script::Avestan),
            "Bali" => Ok(Script::Balinese),
            "Bamu" => Ok(Script::Bamum),
            "Bass" => Ok(Script::Bassa_Vah),
            "Batk" => Ok(Script::Batak),
            "Beng" => Ok(Script::Bengali),
            "Bhks" => Ok(Script::Bhaiksuki),
            "Bopo" => Ok(Script::Bopomofo),
            "Brah" => Ok(Script::Brahmi),
            "Brai" => Ok(Script::Braille),
            "Bugi" => Ok(Script::Buginese),
            "Buhd" => Ok(Script::Buhid),
            "Cakm" => Ok(Script::Chakma),
            "Cans" => Ok(Script::Canadian_Aboriginal),
            "Cari" => Ok(Script::Carian),
            "Cham" => Ok(Script::Cham),
            "Cher" => Ok(Script::Cherokee),
            "Copt" => Ok(Script::Coptic),
            "Cprt" => Ok(Script::Cypriot),
            "Cyrl" => Ok(Script::Cyrillic),
            "Deva" => Ok(Script::Devanagari),
            "Dogr" => Ok(Script::Dogra),
            "Dsrt" => Ok(Script::Deseret),
            "Dupl" => Ok(Script::Duployan),
            "Egyp" => Ok(Script::Egyptian_Hieroglyphs),
            "Elba" => Ok(Script::Elbasan),
            "Elym" => Ok(Script::Elymaic),
            "Ethi" => Ok(Script::Ethiopic),
            "Geor" => Ok(Script::Georgian),
            "Glag" => Ok(Script::Glagolitic),
            "Gong" => Ok(Script::Gunjala_Gondi),
            "Gonm" => Ok(Script::Masaram_Gondi),
            "Goth" => Ok(Script::Gothic),
            "Gran" => Ok(Script::Grantha),
            "Grek" => Ok(Script::Greek),
            "Gujr" => Ok(Script::Gujarati),
            "Guru" => Ok(Script::Gurmukhi),
            "Hang" => Ok(Script::Hangul),
            "Hani" => Ok(Script::Han),
            "Hano" => Ok(Script::Hanunoo),
            "Hatr" => Ok(Script::Hatran),
            "Hebr" => Ok(Script::Hebrew),
            "Hira" => Ok(Script::Hiragana),
            "Hluw" => Ok(Script::Anatolian_Hieroglyphs),
            "Hmng" => Ok(Script::Pahawh_Hmong),
            "Hmnp" => Ok(Script::Nyiakeng_Puachue_Hmong),
            "Hrkt" => Ok(Script::Katakana_Or_Hiragana),
            "Hung" => Ok(Script::Old_Hungarian),
            "Ital" => Ok(Script::Old_Italic),
            "Java" => Ok(Script::Javanese),
            "Kali" => Ok(Script::Kayah_Li),
            "Kana" => Ok(Script::Katakana),
            "Khar" => Ok(Script::Kharoshthi),
            "Khmr" => Ok(Script::Khmer),
            "Khoj" => Ok(Script::Khojki),
            "Knda" => Ok(Script::Kannada),
            "Kthi" => Ok(Script::Kaithi),
            "Lana" => Ok(Script::Tai_Tham),
            "Laoo" => Ok(Script::Lao),
            "Latn" => Ok(Script::Latin),
            "Lepc" => Ok(Script::Lepcha),
            "Limb" => Ok(Script::Limbu),
            "Lina" => Ok(Script::Linear_A),
            "Linb" => Ok(Script::Linear_B),
            "Lisu" => Ok(Script::Lisu),
            "Lyci" => Ok(Script::Lycian),
            "Lydi" => Ok(Script::Lydian),
            "Mahj" => Ok(Script::Mahajani),
            "Maka" => Ok(Script::Makasar),
            "Mand" => Ok(Script::Mandaic),
            "Mani" => Ok(Script::Manichaean),
            "Marc" => Ok(Script::Marchen),
            "Medf" => Ok(Script::Medefaidrin),
            "Mend" => Ok(Script::Mende_Kikakui),
            "Merc" => Ok(Script::Meroitic_Cursive),
            "Mero" => Ok(Script::Meroitic_Hieroglyphs),
            "Mlym" => Ok(Script::Malayalam),
            "Modi" => Ok(Script::Modi),
            "Mong" => Ok(Script::Mongolian),
            "Mroo" => Ok(Script::Mro),
            "Mtei" => Ok(Script::Meetei_Mayek),
            "Mult" => Ok(Script::Multani),
            "Mymr" => Ok(Script::Myanmar),
            "Nand" => Ok(Script::Nandinagari),
            "Narb" => Ok(Script::Old_North_Arabian),
            "Nbat" => Ok(Script::Nabataean),
            "Newa" => Ok(Script::Newa),
            "Nkoo" => Ok(Script::Nko),
            "Nshu" => Ok(Script::Nushu),
            "Ogam" => Ok(Script::Ogham),
            "Olck" => Ok(Script::Ol_Chiki),
            "Orkh" => Ok(Script::Old_Turkic),
            "Orya" => Ok(Script::Oriya),
            "Osge" => Ok(Script::Osage),
            "Osma" => Ok(Script::Osmanya),
            "Palm" => Ok(Script::Palmyrene),
            "Pauc" => Ok(Script::Pau_Cin_Hau),
            "Perm" => Ok(Script::Old_Permic),
            "Phag" => Ok(Script::Phags_Pa),
            "Phli" => Ok(Script::Inscriptional_Pahlavi),
            "Phlp" => Ok(Script::Psalter_Pahlavi),
            "Phnx" => Ok(Script::Phoenician),
            "Plrd" => Ok(Script::Miao),
            "Prti" => Ok(Script::Inscriptional_Parthian),
            "Rjng" => Ok(Script::Rejang),
            "Rohg" => Ok(Script::Hanifi_Rohingya),
            "Runr" => Ok(Script::Runic),
            "Samr" => Ok(Script::Samaritan),
            "Sarb" => Ok(Script::Old_South_Arabian),
            "Saur" => Ok(Script::Saurashtra),
            "Sgnw" => Ok(Script::SignWriting),
            "Shaw" => Ok(Script::Shavian),
            "Shrd" => Ok(Script::Sharada),
            "Sidd" => Ok(Script::Siddham),
            "Sind" => Ok(Script::Khudawadi),
            "Sinh" => Ok(Script::Sinhala),
            "Sogd" => Ok(Script::Sogdian),
            "Sogo" => Ok(Script::Old_Sogdian),
            "Sora" => Ok(Script::Sora_Sompeng),
            "Soyo" => Ok(Script::Soyombo),
            "Sund" => Ok(Script::Sundanese),
            "Sylo" => Ok(Script::Syloti_Nagri),
            "Syrc" => Ok(Script::Syriac),
            "Tagb" => Ok(Script::Tagbanwa),
            "Takr" => Ok(Script::Takri),
            "Tale" => Ok(Script::Tai_Le),
            "Talu" => Ok(Script::New_Tai_Lue),
            "Taml" => Ok(Script::Tamil),
            "Tang" => Ok(Script::Tangut),
            "Tavt" => Ok(Script::Tai_Viet),
            "Telu" => Ok(Script::Telugu),
            "Tfng" => Ok(Script::Tifinagh),
            "Tglg" => Ok(Script::Tagalog),
            "Thaa" => Ok(Script::Thaana),
            "Thai" => Ok(Script::Thai),
            "Tibt" => Ok(Script::Tibetan),
            "Tirh" => Ok(Script::Tirhuta),
            "Ugar" => Ok(Script::Ugaritic),
            "Vaii" => Ok(Script::Vai),
            "Wara" => Ok(Script::Warang_Citi),
            "Wcho" => Ok(Script::Wancho),
            "Xpeo" => Ok(Script::Old_Persian),
            "Xsux" => Ok(Script::Cuneiform),
            "Yiii" => Ok(Script::Yi),
            "Zanb" => Ok(Script::Zanabazar_Square),
            "Zinh" => Ok(Script::Inherited),
            "Zyyy" => Ok(Script::Common),
            "Zzzz" => Ok(Script::Unknown),
            _ => Err(ParseError(())),
        }
    }
}

/// AKA `bc`
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Bidi_Class {
    /// AKA `AL`
    Arabic_Letter,
    /// AKA `AN`
    Arabic_Number,
    /// AKA `B`
    Paragraph_Separator,
    /// AKA `BN`
    Boundary_Neutral,
    /// AKA `CS`
    Common_Separator,
    /// AKA `EN`
    European_Number,
    /// AKA `ES`
    European_Separator,
    /// AKA `ET`
    European_Terminator,
    /// AKA `FSI`
    First_Strong_Isolate,
    /// AKA `L`
    Left_To_Right,
    /// AKA `LRE`
    Left_To_Right_Embedding,
    /// AKA `LRI`
    Left_To_Right_Isolate,
    /// AKA `LRO`
    Left_To_Right_Override,
    /// AKA `NSM`
    Nonspacing_Mark,
    /// AKA `ON`
    Other_Neutral,
    /// AKA `PDF`
    Pop_Directional_Format,
    /// AKA `PDI`
    Pop_Directional_Isolate,
    /// AKA `R`
    Right_To_Left,
    /// AKA `RLE`
    Right_To_Left_Embedding,
    /// AKA `RLI`
    Right_To_Left_Isolate,
    /// AKA `RLO`
    Right_To_Left_Override,
    /// AKA `S`
    Segment_Separator,
    /// AKA `WS`
    White_Space,
}
impl fmt::Display for Bidi_Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <str as fmt::Display>::fmt(self.display_str(), f)
    }
}
impl Bidi_Class {
    fn display_str(&self) -> &'static str {
        match self {
            Bidi_Class::Arabic_Letter => "AL",
            Bidi_Class::Arabic_Number => "AN",
            Bidi_Class::Paragraph_Separator => "B",
            Bidi_Class::Boundary_Neutral => "BN",
            Bidi_Class::Common_Separator => "CS",
            Bidi_Class::European_Number => "EN",
            Bidi_Class::European_Separator => "ES",
            Bidi_Class::European_Terminator => "ET",
            Bidi_Class::First_Strong_Isolate => "FSI",
            Bidi_Class::Left_To_Right => "L",
            Bidi_Class::Left_To_Right_Embedding => "LRE",
            Bidi_Class::Left_To_Right_Isolate => "LRI",
            Bidi_Class::Left_To_Right_Override => "LRO",
            Bidi_Class::Nonspacing_Mark => "NSM",
            Bidi_Class::Other_Neutral => "ON",
            Bidi_Class::Pop_Directional_Format => "PDF",
            Bidi_Class::Pop_Directional_Isolate => "PDI",
            Bidi_Class::Right_To_Left => "R",
            Bidi_Class::Right_To_Left_Embedding => "RLE",
            Bidi_Class::Right_To_Left_Isolate => "RLI",
            Bidi_Class::Right_To_Left_Override => "RLO",
            Bidi_Class::Segment_Separator => "S",
            Bidi_Class::White_Space => "WS",
        }
    }
}
impl FromStr for Bidi_Class {
    type Err = ParseError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "AL" => Ok(Bidi_Class::Arabic_Letter),
            "AN" => Ok(Bidi_Class::Arabic_Number),
            "B" => Ok(Bidi_Class::Paragraph_Separator),
            "BN" => Ok(Bidi_Class::Boundary_Neutral),
            "CS" => Ok(Bidi_Class::Common_Separator),
            "EN" => Ok(Bidi_Class::European_Number),
            "ES" => Ok(Bidi_Class::European_Separator),
            "ET" => Ok(Bidi_Class::European_Terminator),
            "FSI" => Ok(Bidi_Class::First_Strong_Isolate),
            "L" => Ok(Bidi_Class::Left_To_Right),
            "LRE" => Ok(Bidi_Class::Left_To_Right_Embedding),
            "LRI" => Ok(Bidi_Class::Left_To_Right_Isolate),
            "LRO" => Ok(Bidi_Class::Left_To_Right_Override),
            "NSM" => Ok(Bidi_Class::Nonspacing_Mark),
            "ON" => Ok(Bidi_Class::Other_Neutral),
            "PDF" => Ok(Bidi_Class::Pop_Directional_Format),
            "PDI" => Ok(Bidi_Class::Pop_Directional_Isolate),
            "R" => Ok(Bidi_Class::Right_To_Left),
            "RLE" => Ok(Bidi_Class::Right_To_Left_Embedding),
            "RLI" => Ok(Bidi_Class::Right_To_Left_Isolate),
            "RLO" => Ok(Bidi_Class::Right_To_Left_Override),
            "S" => Ok(Bidi_Class::Segment_Separator),
            "WS" => Ok(Bidi_Class::White_Space),
            _ => Err(ParseError(())),
        }
    }
}

/// AKA `bpt`
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Bidi_Paired_Bracket_Type {
    /// AKA `c`
    Close,
    /// AKA `n`
    None,
    /// AKA `o`
    Open,
}
impl fmt::Display for Bidi_Paired_Bracket_Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <str as fmt::Display>::fmt(self.display_str(), f)
    }
}
impl Bidi_Paired_Bracket_Type {
    fn display_str(&self) -> &'static str {
        match self {
            Bidi_Paired_Bracket_Type::Close => "c",
            Bidi_Paired_Bracket_Type::None => "n",
            Bidi_Paired_Bracket_Type::Open => "o",
        }
    }
}
impl FromStr for Bidi_Paired_Bracket_Type {
    type Err = ParseError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "c" => Ok(Bidi_Paired_Bracket_Type::Close),
            "n" => Ok(Bidi_Paired_Bracket_Type::None),
            "o" => Ok(Bidi_Paired_Bracket_Type::Open),
            _ => Err(ParseError(())),
        }
    }
}

/// AKA `ccc`
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Canonical_Combining_Class {
    /// AKA `0`
    NR,
    /// AKA `1`
    OV,
    /// AKA `7`
    NK,
    /// AKA `8`
    KV,
    /// AKA `9`
    VR,
    /// AKA `10`
    CCC10,
    /// AKA `11`
    CCC11,
    /// AKA `12`
    CCC12,
    /// AKA `13`
    CCC13,
    /// AKA `14`
    CCC14,
    /// AKA `15`
    CCC15,
    /// AKA `16`
    CCC16,
    /// AKA `17`
    CCC17,
    /// AKA `18`
    CCC18,
    /// AKA `19`
    CCC19,
    /// AKA `20`
    CCC20,
    /// AKA `21`
    CCC21,
    /// AKA `22`
    CCC22,
    /// AKA `23`
    CCC23,
    /// AKA `24`
    CCC24,
    /// AKA `25`
    CCC25,
    /// AKA `26`
    CCC26,
    /// AKA `27`
    CCC27,
    /// AKA `28`
    CCC28,
    /// AKA `29`
    CCC29,
    /// AKA `30`
    CCC30,
    /// AKA `31`
    CCC31,
    /// AKA `32`
    CCC32,
    /// AKA `33`
    CCC33,
    /// AKA `34`
    CCC34,
    /// AKA `35`
    CCC35,
    /// AKA `36`
    CCC36,
    /// AKA `84`
    CCC84,
    /// AKA `91`
    CCC91,
    /// AKA `103`
    CCC103,
    /// AKA `107`
    CCC107,
    /// AKA `118`
    CCC118,
    /// AKA `122`
    CCC122,
    /// AKA `129`
    CCC129,
    /// AKA `130`
    CCC130,
    /// AKA `132`
    CCC132,
    /// AKA `133`
    CCC133,
    /// AKA `200`
    ATBL,
    /// AKA `202`
    ATB,
    /// AKA `214`
    ATA,
    /// AKA `216`
    ATAR,
    /// AKA `218`
    BL,
    /// AKA `220`
    B,
    /// AKA `222`
    BR,
    /// AKA `224`
    L,
    /// AKA `226`
    R,
    /// AKA `228`
    AL,
    /// AKA `230`
    A,
    /// AKA `232`
    AR,
    /// AKA `233`
    DB,
    /// AKA `234`
    DA,
    /// AKA `240`
    IS,
}
impl fmt::Display for Canonical_Combining_Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <str as fmt::Display>::fmt(self.display_str(), f)
    }
}
impl Canonical_Combining_Class {
    fn display_str(&self) -> &'static str {
        match self {
            Canonical_Combining_Class::NR => "0",
            Canonical_Combining_Class::OV => "1",
            Canonical_Combining_Class::NK => "7",
            Canonical_Combining_Class::KV => "8",
            Canonical_Combining_Class::VR => "9",
            Canonical_Combining_Class::CCC10 => "10",
            Canonical_Combining_Class::CCC11 => "11",
            Canonical_Combining_Class::CCC12 => "12",
            Canonical_Combining_Class::CCC13 => "13",
            Canonical_Combining_Class::CCC14 => "14",
            Canonical_Combining_Class::CCC15 => "15",
            Canonical_Combining_Class::CCC16 => "16",
            Canonical_Combining_Class::CCC17 => "17",
            Canonical_Combining_Class::CCC18 => "18",
            Canonical_Combining_Class::CCC19 => "19",
            Canonical_Combining_Class::CCC20 => "20",
            Canonical_Combining_Class::CCC21 => "21",
            Canonical_Combining_Class::CCC22 => "22",
            Canonical_Combining_Class::CCC23 => "23",
            Canonical_Combining_Class::CCC24 => "24",
            Canonical_Combining_Class::CCC25 => "25",
            Canonical_Combining_Class::CCC26 => "26",
            Canonical_Combining_Class::CCC27 => "27",
            Canonical_Combining_Class::CCC28 => "28",
            Canonical_Combining_Class::CCC29 => "29",
            Canonical_Combining_Class::CCC30 => "30",
            Canonical_Combining_Class::CCC31 => "31",
            Canonical_Combining_Class::CCC32 => "32",
            Canonical_Combining_Class::CCC33 => "33",
            Canonical_Combining_Class::CCC34 => "34",
            Canonical_Combining_Class::CCC35 => "35",
            Canonical_Combining_Class::CCC36 => "36",
            Canonical_Combining_Class::CCC84 => "84",
            Canonical_Combining_Class::CCC91 => "91",
            Canonical_Combining_Class::CCC103 => "103",
            Canonical_Combining_Class::CCC107 => "107",
            Canonical_Combining_Class::CCC118 => "118",
            Canonical_Combining_Class::CCC122 => "122",
            Canonical_Combining_Class::CCC129 => "129",
            Canonical_Combining_Class::CCC130 => "130",
            Canonical_Combining_Class::CCC132 => "132",
            Canonical_Combining_Class::CCC133 => "133",
            Canonical_Combining_Class::ATBL => "200",
            Canonical_Combining_Class::ATB => "202",
            Canonical_Combining_Class::ATA => "214",
            Canonical_Combining_Class::ATAR => "216",
            Canonical_Combining_Class::BL => "218",
            Canonical_Combining_Class::B => "220",
            Canonical_Combining_Class::BR => "222",
            Canonical_Combining_Class::L => "224",
            Canonical_Combining_Class::R => "226",
            Canonical_Combining_Class::AL => "228",
            Canonical_Combining_Class::A => "230",
            Canonical_Combining_Class::AR => "232",
            Canonical_Combining_Class::DB => "233",
            Canonical_Combining_Class::DA => "234",
            Canonical_Combining_Class::IS => "240",
        }
    }
}
impl FromStr for Canonical_Combining_Class {
    type Err = ParseError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "0" => Ok(Canonical_Combining_Class::NR),
            "1" => Ok(Canonical_Combining_Class::OV),
            "7" => Ok(Canonical_Combining_Class::NK),
            "8" => Ok(Canonical_Combining_Class::KV),
            "9" => Ok(Canonical_Combining_Class::VR),
            "10" => Ok(Canonical_Combining_Class::CCC10),
            "11" => Ok(Canonical_Combining_Class::CCC11),
            "12" => Ok(Canonical_Combining_Class::CCC12),
            "13" => Ok(Canonical_Combining_Class::CCC13),
            "14" => Ok(Canonical_Combining_Class::CCC14),
            "15" => Ok(Canonical_Combining_Class::CCC15),
            "16" => Ok(Canonical_Combining_Class::CCC16),
            "17" => Ok(Canonical_Combining_Class::CCC17),
            "18" => Ok(Canonical_Combining_Class::CCC18),
            "19" => Ok(Canonical_Combining_Class::CCC19),
            "20" => Ok(Canonical_Combining_Class::CCC20),
            "21" => Ok(Canonical_Combining_Class::CCC21),
            "22" => Ok(Canonical_Combining_Class::CCC22),
            "23" => Ok(Canonical_Combining_Class::CCC23),
            "24" => Ok(Canonical_Combining_Class::CCC24),
            "25" => Ok(Canonical_Combining_Class::CCC25),
            "26" => Ok(Canonical_Combining_Class::CCC26),
            "27" => Ok(Canonical_Combining_Class::CCC27),
            "28" => Ok(Canonical_Combining_Class::CCC28),
            "29" => Ok(Canonical_Combining_Class::CCC29),
            "30" => Ok(Canonical_Combining_Class::CCC30),
            "31" => Ok(Canonical_Combining_Class::CCC31),
            "32" => Ok(Canonical_Combining_Class::CCC32),
            "33" => Ok(Canonical_Combining_Class::CCC33),
            "34" => Ok(Canonical_Combining_Class::CCC34),
            "35" => Ok(Canonical_Combining_Class::CCC35),
            "36" => Ok(Canonical_Combining_Class::CCC36),
            "84" => Ok(Canonical_Combining_Class::CCC84),
            "91" => Ok(Canonical_Combining_Class::CCC91),
            "103" => Ok(Canonical_Combining_Class::CCC103),
            "107" => Ok(Canonical_Combining_Class::CCC107),
            "118" => Ok(Canonical_Combining_Class::CCC118),
            "122" => Ok(Canonical_Combining_Class::CCC122),
            "129" => Ok(Canonical_Combining_Class::CCC129),
            "130" => Ok(Canonical_Combining_Class::CCC130),
            "132" => Ok(Canonical_Combining_Class::CCC132),
            "133" => Ok(Canonical_Combining_Class::CCC133),
            "200" => Ok(Canonical_Combining_Class::ATBL),
            "202" => Ok(Canonical_Combining_Class::ATB),
            "214" => Ok(Canonical_Combining_Class::ATA),
            "216" => Ok(Canonical_Combining_Class::ATAR),
            "218" => Ok(Canonical_Combining_Class::BL),
            "220" => Ok(Canonical_Combining_Class::B),
            "222" => Ok(Canonical_Combining_Class::BR),
            "224" => Ok(Canonical_Combining_Class::L),
            "226" => Ok(Canonical_Combining_Class::R),
            "228" => Ok(Canonical_Combining_Class::AL),
            "230" => Ok(Canonical_Combining_Class::A),
            "232" => Ok(Canonical_Combining_Class::AR),
            "233" => Ok(Canonical_Combining_Class::DB),
            "234" => Ok(Canonical_Combining_Class::DA),
            "240" => Ok(Canonical_Combining_Class::IS),
            _ => Err(ParseError(())),
        }
    }
}

/// AKA `dt`
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Decomposition_Type {
    /// AKA `Can`
    Canonical,
    /// AKA `Com`
    Compat,
    /// AKA `Enc`
    Circle,
    /// AKA `Fin`
    Final,
    Font,
    /// AKA `Fra`
    Fraction,
    /// AKA `Init`
    Initial,
    /// AKA `Iso`
    Isolated,
    /// AKA `Med`
    Medial,
    /// AKA `Nar`
    Narrow,
    /// AKA `Nb`
    Nobreak,
    None,
    /// AKA `Sml`
    Small,
    /// AKA `Sqr`
    Square,
    Sub,
    /// AKA `Sup`
    Super,
    /// AKA `Vert`
    Vertical,
    Wide,
}
impl fmt::Display for Decomposition_Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <str as fmt::Display>::fmt(self.display_str(), f)
    }
}
impl Decomposition_Type {
    fn display_str(&self) -> &'static str {
        match self {
            Decomposition_Type::Canonical => "Can",
            Decomposition_Type::Compat => "Com",
            Decomposition_Type::Circle => "Enc",
            Decomposition_Type::Final => "Fin",
            Decomposition_Type::Font => "Font",
            Decomposition_Type::Fraction => "Fra",
            Decomposition_Type::Initial => "Init",
            Decomposition_Type::Isolated => "Iso",
            Decomposition_Type::Medial => "Med",
            Decomposition_Type::Narrow => "Nar",
            Decomposition_Type::Nobreak => "Nb",
            Decomposition_Type::None => "None",
            Decomposition_Type::Small => "Sml",
            Decomposition_Type::Square => "Sqr",
            Decomposition_Type::Sub => "Sub",
            Decomposition_Type::Super => "Sup",
            Decomposition_Type::Vertical => "Vert",
            Decomposition_Type::Wide => "Wide",
        }
    }
}
impl FromStr for Decomposition_Type {
    type Err = ParseError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "Can" => Ok(Decomposition_Type::Canonical),
            "Com" => Ok(Decomposition_Type::Compat),
            "Enc" => Ok(Decomposition_Type::Circle),
            "Fin" => Ok(Decomposition_Type::Final),
            "Font" => Ok(Decomposition_Type::Font),
            "Fra" => Ok(Decomposition_Type::Fraction),
            "Init" => Ok(Decomposition_Type::Initial),
            "Iso" => Ok(Decomposition_Type::Isolated),
            "Med" => Ok(Decomposition_Type::Medial),
            "Nar" => Ok(Decomposition_Type::Narrow),
            "Nb" => Ok(Decomposition_Type::Nobreak),
            "None" => Ok(Decomposition_Type::None),
            "Sml" => Ok(Decomposition_Type::Small),
            "Sqr" => Ok(Decomposition_Type::Square),
            "Sub" => Ok(Decomposition_Type::Sub),
            "Sup" => Ok(Decomposition_Type::Super),
            "Vert" => Ok(Decomposition_Type::Vertical),
            "Wide" => Ok(Decomposition_Type::Wide),
            _ => Err(ParseError(())),
        }
    }
}

/// AKA `ea`
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum East_Asian_Width {
    /// AKA `A`
    Ambiguous,
    /// AKA `F`
    Fullwidth,
    /// AKA `H`
    Halfwidth,
    /// AKA `N`
    Neutral,
    /// AKA `Na`
    Narrow,
    /// AKA `W`
    Wide,
}
impl fmt::Display for East_Asian_Width {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <str as fmt::Display>::fmt(self.display_str(), f)
    }
}
impl East_Asian_Width {
    fn display_str(&self) -> &'static str {
        match self {
            East_Asian_Width::Ambiguous => "A",
            East_Asian_Width::Fullwidth => "F",
            East_Asian_Width::Halfwidth => "H",
            East_Asian_Width::Neutral => "N",
            East_Asian_Width::Narrow => "Na",
            East_Asian_Width::Wide => "W",
        }
    }
}
impl FromStr for East_Asian_Width {
    type Err = ParseError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "A" => Ok(East_Asian_Width::Ambiguous),
            "F" => Ok(East_Asian_Width::Fullwidth),
            "H" => Ok(East_Asian_Width::Halfwidth),
            "N" => Ok(East_Asian_Width::Neutral),
            "Na" => Ok(East_Asian_Width::Narrow),
            "W" => Ok(East_Asian_Width::Wide),
            _ => Err(ParseError(())),
        }
    }
}

/// AKA `gc`
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum General_Category {
    /// AKA `C`
    Other,
    /// AKA `Cc`
    Control,
    /// AKA `Cf`
    Format,
    /// AKA `Cn`
    Unassigned,
    /// AKA `Co`
    Private_Use,
    /// AKA `Cs`
    Surrogate,
    /// AKA `L`
    Letter,
    /// AKA `LC`
    Cased_Letter,
    /// AKA `Ll`
    Lowercase_Letter,
    /// AKA `Lm`
    Modifier_Letter,
    /// AKA `Lo`
    Other_Letter,
    /// AKA `Lt`
    Titlecase_Letter,
    /// AKA `Lu`
    Uppercase_Letter,
    /// AKA `M`
    Mark,
    /// AKA `Mc`
    Spacing_Mark,
    /// AKA `Me`
    Enclosing_Mark,
    /// AKA `Mn`
    Nonspacing_Mark,
    /// AKA `N`
    Number,
    /// AKA `Nd`
    Decimal_Number,
    /// AKA `Nl`
    Letter_Number,
    /// AKA `No`
    Other_Number,
    /// AKA `P`
    Punctuation,
    /// AKA `Pc`
    Connector_Punctuation,
    /// AKA `Pd`
    Dash_Punctuation,
    /// AKA `Pe`
    Close_Punctuation,
    /// AKA `Pf`
    Final_Punctuation,
    /// AKA `Pi`
    Initial_Punctuation,
    /// AKA `Po`
    Other_Punctuation,
    /// AKA `Ps`
    Open_Punctuation,
    /// AKA `S`
    Symbol,
    /// AKA `Sc`
    Currency_Symbol,
    /// AKA `Sk`
    Modifier_Symbol,
    /// AKA `Sm`
    Math_Symbol,
    /// AKA `So`
    Other_Symbol,
    /// AKA `Z`
    Separator,
    /// AKA `Zl`
    Line_Separator,
    /// AKA `Zp`
    Paragraph_Separator,
    /// AKA `Zs`
    Space_Separator,
}
impl fmt::Display for General_Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <str as fmt::Display>::fmt(self.display_str(), f)
    }
}
impl General_Category {
    fn display_str(&self) -> &'static str {
        match self {
            General_Category::Other => "C",
            General_Category::Control => "Cc",
            General_Category::Format => "Cf",
            General_Category::Unassigned => "Cn",
            General_Category::Private_Use => "Co",
            General_Category::Surrogate => "Cs",
            General_Category::Letter => "L",
            General_Category::Cased_Letter => "LC",
            General_Category::Lowercase_Letter => "Ll",
            General_Category::Modifier_Letter => "Lm",
            General_Category::Other_Letter => "Lo",
            General_Category::Titlecase_Letter => "Lt",
            General_Category::Uppercase_Letter => "Lu",
            General_Category::Mark => "M",
            General_Category::Spacing_Mark => "Mc",
            General_Category::Enclosing_Mark => "Me",
            General_Category::Nonspacing_Mark => "Mn",
            General_Category::Number => "N",
            General_Category::Decimal_Number => "Nd",
            General_Category::Letter_Number => "Nl",
            General_Category::Other_Number => "No",
            General_Category::Punctuation => "P",
            General_Category::Connector_Punctuation => "Pc",
            General_Category::Dash_Punctuation => "Pd",
            General_Category::Close_Punctuation => "Pe",
            General_Category::Final_Punctuation => "Pf",
            General_Category::Initial_Punctuation => "Pi",
            General_Category::Other_Punctuation => "Po",
            General_Category::Open_Punctuation => "Ps",
            General_Category::Symbol => "S",
            General_Category::Currency_Symbol => "Sc",
            General_Category::Modifier_Symbol => "Sk",
            General_Category::Math_Symbol => "Sm",
            General_Category::Other_Symbol => "So",
            General_Category::Separator => "Z",
            General_Category::Line_Separator => "Zl",
            General_Category::Paragraph_Separator => "Zp",
            General_Category::Space_Separator => "Zs",
        }
    }
}
impl FromStr for General_Category {
    type Err = ParseError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "C" => Ok(General_Category::Other),
            "Cc" => Ok(General_Category::Control),
            "Cf" => Ok(General_Category::Format),
            "Cn" => Ok(General_Category::Unassigned),
            "Co" => Ok(General_Category::Private_Use),
            "Cs" => Ok(General_Category::Surrogate),
            "L" => Ok(General_Category::Letter),
            "LC" => Ok(General_Category::Cased_Letter),
            "Ll" => Ok(General_Category::Lowercase_Letter),
            "Lm" => Ok(General_Category::Modifier_Letter),
            "Lo" => Ok(General_Category::Other_Letter),
            "Lt" => Ok(General_Category::Titlecase_Letter),
            "Lu" => Ok(General_Category::Uppercase_Letter),
            "M" => Ok(General_Category::Mark),
            "Mc" => Ok(General_Category::Spacing_Mark),
            "Me" => Ok(General_Category::Enclosing_Mark),
            "Mn" => Ok(General_Category::Nonspacing_Mark),
            "N" => Ok(General_Category::Number),
            "Nd" => Ok(General_Category::Decimal_Number),
            "Nl" => Ok(General_Category::Letter_Number),
            "No" => Ok(General_Category::Other_Number),
            "P" => Ok(General_Category::Punctuation),
            "Pc" => Ok(General_Category::Connector_Punctuation),
            "Pd" => Ok(General_Category::Dash_Punctuation),
            "Pe" => Ok(General_Category::Close_Punctuation),
            "Pf" => Ok(General_Category::Final_Punctuation),
            "Pi" => Ok(General_Category::Initial_Punctuation),
            "Po" => Ok(General_Category::Other_Punctuation),
            "Ps" => Ok(General_Category::Open_Punctuation),
            "S" => Ok(General_Category::Symbol),
            "Sc" => Ok(General_Category::Currency_Symbol),
            "Sk" => Ok(General_Category::Modifier_Symbol),
            "Sm" => Ok(General_Category::Math_Symbol),
            "So" => Ok(General_Category::Other_Symbol),
            "Z" => Ok(General_Category::Separator),
            "Zl" => Ok(General_Category::Line_Separator),
            "Zp" => Ok(General_Category::Paragraph_Separator),
            "Zs" => Ok(General_Category::Space_Separator),
            _ => Err(ParseError(())),
        }
    }
}

/// AKA `GCB`
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Grapheme_Cluster_Break {
    /// AKA `CN`
    Control,
    CR,
    /// AKA `EB`
    E_Base,
    /// AKA `EBG`
    E_Base_GAZ,
    /// AKA `EM`
    E_Modifier,
    /// AKA `EX`
    Extend,
    /// AKA `GAZ`
    Glue_After_Zwj,
    L,
    LF,
    LV,
    LVT,
    /// AKA `PP`
    Prepend,
    /// AKA `RI`
    Regional_Indicator,
    /// AKA `SM`
    SpacingMark,
    T,
    V,
    /// AKA `XX`
    Other,
    ZWJ,
}
impl fmt::Display for Grapheme_Cluster_Break {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <str as fmt::Display>::fmt(self.display_str(), f)
    }
}
impl Grapheme_Cluster_Break {
    fn display_str(&self) -> &'static str {
        match self {
            Grapheme_Cluster_Break::Control => "CN",
            Grapheme_Cluster_Break::CR => "CR",
            Grapheme_Cluster_Break::E_Base => "EB",
            Grapheme_Cluster_Break::E_Base_GAZ => "EBG",
            Grapheme_Cluster_Break::E_Modifier => "EM",
            Grapheme_Cluster_Break::Extend => "EX",
            Grapheme_Cluster_Break::Glue_After_Zwj => "GAZ",
            Grapheme_Cluster_Break::L => "L",
            Grapheme_Cluster_Break::LF => "LF",
            Grapheme_Cluster_Break::LV => "LV",
            Grapheme_Cluster_Break::LVT => "LVT",
            Grapheme_Cluster_Break::Prepend => "PP",
            Grapheme_Cluster_Break::Regional_Indicator => "RI",
            Grapheme_Cluster_Break::SpacingMark => "SM",
            Grapheme_Cluster_Break::T => "T",
            Grapheme_Cluster_Break::V => "V",
            Grapheme_Cluster_Break::Other => "XX",
            Grapheme_Cluster_Break::ZWJ => "ZWJ",
        }
    }
}
impl FromStr for Grapheme_Cluster_Break {
    type Err = ParseError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "CN" => Ok(Grapheme_Cluster_Break::Control),
            "CR" => Ok(Grapheme_Cluster_Break::CR),
            "EB" => Ok(Grapheme_Cluster_Break::E_Base),
            "EBG" => Ok(Grapheme_Cluster_Break::E_Base_GAZ),
            "EM" => Ok(Grapheme_Cluster_Break::E_Modifier),
            "EX" => Ok(Grapheme_Cluster_Break::Extend),
            "GAZ" => Ok(Grapheme_Cluster_Break::Glue_After_Zwj),
            "L" => Ok(Grapheme_Cluster_Break::L),
            "LF" => Ok(Grapheme_Cluster_Break::LF),
            "LV" => Ok(Grapheme_Cluster_Break::LV),
            "LVT" => Ok(Grapheme_Cluster_Break::LVT),
            "PP" => Ok(Grapheme_Cluster_Break::Prepend),
            "RI" => Ok(Grapheme_Cluster_Break::Regional_Indicator),
            "SM" => Ok(Grapheme_Cluster_Break::SpacingMark),
            "T" => Ok(Grapheme_Cluster_Break::T),
            "V" => Ok(Grapheme_Cluster_Break::V),
            "XX" => Ok(Grapheme_Cluster_Break::Other),
            "ZWJ" => Ok(Grapheme_Cluster_Break::ZWJ),
            _ => Err(ParseError(())),
        }
    }
}

/// AKA `hst`
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Hangul_Syllable_Type {
    /// AKA `L`
    Leading_Jamo,
    /// AKA `LV`
    LV_Syllable,
    /// AKA `LVT`
    LVT_Syllable,
    /// AKA `NA`
    Not_Applicable,
    /// AKA `T`
    Trailing_Jamo,
    /// AKA `V`
    Vowel_Jamo,
}
impl fmt::Display for Hangul_Syllable_Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <str as fmt::Display>::fmt(self.display_str(), f)
    }
}
impl Hangul_Syllable_Type {
    fn display_str(&self) -> &'static str {
        match self {
            Hangul_Syllable_Type::Leading_Jamo => "L",
            Hangul_Syllable_Type::LV_Syllable => "LV",
            Hangul_Syllable_Type::LVT_Syllable => "LVT",
            Hangul_Syllable_Type::Not_Applicable => "NA",
            Hangul_Syllable_Type::Trailing_Jamo => "T",
            Hangul_Syllable_Type::Vowel_Jamo => "V",
        }
    }
}
impl FromStr for Hangul_Syllable_Type {
    type Err = ParseError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "L" => Ok(Hangul_Syllable_Type::Leading_Jamo),
            "LV" => Ok(Hangul_Syllable_Type::LV_Syllable),
            "LVT" => Ok(Hangul_Syllable_Type::LVT_Syllable),
            "NA" => Ok(Hangul_Syllable_Type::Not_Applicable),
            "T" => Ok(Hangul_Syllable_Type::Trailing_Jamo),
            "V" => Ok(Hangul_Syllable_Type::Vowel_Jamo),
            _ => Err(ParseError(())),
        }
    }
}

/// AKA `InPC`
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Indic_Positional_Category {
    Bottom,
    Bottom_And_Left,
    Bottom_And_Right,
    Left,
    Left_And_Right,
    NA,
    Overstruck,
    Right,
    Top,
    Top_And_Bottom,
    Top_And_Bottom_And_Right,
    Top_And_Left,
    Top_And_Left_And_Right,
    Top_And_Right,
    Visual_Order_Left,
}
impl fmt::Display for Indic_Positional_Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <str as fmt::Display>::fmt(self.display_str(), f)
    }
}
impl Indic_Positional_Category {
    fn display_str(&self) -> &'static str {
        match self {
            Indic_Positional_Category::Bottom => "Bottom",
            Indic_Positional_Category::Bottom_And_Left => "Bottom_And_Left",
            Indic_Positional_Category::Bottom_And_Right => "Bottom_And_Right",
            Indic_Positional_Category::Left => "Left",
            Indic_Positional_Category::Left_And_Right => "Left_And_Right",
            Indic_Positional_Category::NA => "NA",
            Indic_Positional_Category::Overstruck => "Overstruck",
            Indic_Positional_Category::Right => "Right",
            Indic_Positional_Category::Top => "Top",
            Indic_Positional_Category::Top_And_Bottom => "Top_And_Bottom",
            Indic_Positional_Category::Top_And_Bottom_And_Right => "Top_And_Bottom_And_Right",
            Indic_Positional_Category::Top_And_Left => "Top_And_Left",
            Indic_Positional_Category::Top_And_Left_And_Right => "Top_And_Left_And_Right",
            Indic_Positional_Category::Top_And_Right => "Top_And_Right",
            Indic_Positional_Category::Visual_Order_Left => "Visual_Order_Left",
        }
    }
}
impl FromStr for Indic_Positional_Category {
    type Err = ParseError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "Bottom" => Ok(Indic_Positional_Category::Bottom),
            "Bottom_And_Left" => Ok(Indic_Positional_Category::Bottom_And_Left),
            "Bottom_And_Right" => Ok(Indic_Positional_Category::Bottom_And_Right),
            "Left" => Ok(Indic_Positional_Category::Left),
            "Left_And_Right" => Ok(Indic_Positional_Category::Left_And_Right),
            "NA" => Ok(Indic_Positional_Category::NA),
            "Overstruck" => Ok(Indic_Positional_Category::Overstruck),
            "Right" => Ok(Indic_Positional_Category::Right),
            "Top" => Ok(Indic_Positional_Category::Top),
            "Top_And_Bottom" => Ok(Indic_Positional_Category::Top_And_Bottom),
            "Top_And_Bottom_And_Right" => Ok(Indic_Positional_Category::Top_And_Bottom_And_Right),
            "Top_And_Left" => Ok(Indic_Positional_Category::Top_And_Left),
            "Top_And_Left_And_Right" => Ok(Indic_Positional_Category::Top_And_Left_And_Right),
            "Top_And_Right" => Ok(Indic_Positional_Category::Top_And_Right),
            "Visual_Order_Left" => Ok(Indic_Positional_Category::Visual_Order_Left),
            _ => Err(ParseError(())),
        }
    }
}

/// AKA `InSC`
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Indic_Syllabic_Category {
    Avagraha,
    Bindu,
    Brahmi_Joining_Number,
    Cantillation_Mark,
    Consonant,
    Consonant_Dead,
    Consonant_Final,
    Consonant_Head_Letter,
    Consonant_Initial_Postfixed,
    Consonant_Killer,
    Consonant_Medial,
    Consonant_Placeholder,
    Consonant_Preceding_Repha,
    Consonant_Prefixed,
    Consonant_Subjoined,
    Consonant_Succeeding_Repha,
    Consonant_With_Stacker,
    Gemination_Mark,
    Invisible_Stacker,
    Joiner,
    Modifying_Letter,
    Non_Joiner,
    Nukta,
    Number,
    Number_Joiner,
    Other,
    Pure_Killer,
    Register_Shifter,
    Syllable_Modifier,
    Tone_Letter,
    Tone_Mark,
    Virama,
    Visarga,
    Vowel,
    Vowel_Dependent,
    Vowel_Independent,
}
impl fmt::Display for Indic_Syllabic_Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <str as fmt::Display>::fmt(self.display_str(), f)
    }
}
impl Indic_Syllabic_Category {
    fn display_str(&self) -> &'static str {
        match self {
            Indic_Syllabic_Category::Avagraha => "Avagraha",
            Indic_Syllabic_Category::Bindu => "Bindu",
            Indic_Syllabic_Category::Brahmi_Joining_Number => "Brahmi_Joining_Number",
            Indic_Syllabic_Category::Cantillation_Mark => "Cantillation_Mark",
            Indic_Syllabic_Category::Consonant => "Consonant",
            Indic_Syllabic_Category::Consonant_Dead => "Consonant_Dead",
            Indic_Syllabic_Category::Consonant_Final => "Consonant_Final",
            Indic_Syllabic_Category::Consonant_Head_Letter => "Consonant_Head_Letter",
            Indic_Syllabic_Category::Consonant_Initial_Postfixed => "Consonant_Initial_Postfixed",
            Indic_Syllabic_Category::Consonant_Killer => "Consonant_Killer",
            Indic_Syllabic_Category::Consonant_Medial => "Consonant_Medial",
            Indic_Syllabic_Category::Consonant_Placeholder => "Consonant_Placeholder",
            Indic_Syllabic_Category::Consonant_Preceding_Repha => "Consonant_Preceding_Repha",
            Indic_Syllabic_Category::Consonant_Prefixed => "Consonant_Prefixed",
            Indic_Syllabic_Category::Consonant_Subjoined => "Consonant_Subjoined",
            Indic_Syllabic_Category::Consonant_Succeeding_Repha => "Consonant_Succeeding_Repha",
            Indic_Syllabic_Category::Consonant_With_Stacker => "Consonant_With_Stacker",
            Indic_Syllabic_Category::Gemination_Mark => "Gemination_Mark",
            Indic_Syllabic_Category::Invisible_Stacker => "Invisible_Stacker",
            Indic_Syllabic_Category::Joiner => "Joiner",
            Indic_Syllabic_Category::Modifying_Letter => "Modifying_Letter",
            Indic_Syllabic_Category::Non_Joiner => "Non_Joiner",
            Indic_Syllabic_Category::Nukta => "Nukta",
            Indic_Syllabic_Category::Number => "Number",
            Indic_Syllabic_Category::Number_Joiner => "Number_Joiner",
            Indic_Syllabic_Category::Other => "Other",
            Indic_Syllabic_Category::Pure_Killer => "Pure_Killer",
            Indic_Syllabic_Category::Register_Shifter => "Register_Shifter",
            Indic_Syllabic_Category::Syllable_Modifier => "Syllable_Modifier",
            Indic_Syllabic_Category::Tone_Letter => "Tone_Letter",
            Indic_Syllabic_Category::Tone_Mark => "Tone_Mark",
            Indic_Syllabic_Category::Virama => "Virama",
            Indic_Syllabic_Category::Visarga => "Visarga",
            Indic_Syllabic_Category::Vowel => "Vowel",
            Indic_Syllabic_Category::Vowel_Dependent => "Vowel_Dependent",
            Indic_Syllabic_Category::Vowel_Independent => "Vowel_Independent",
        }
    }
}
impl FromStr for Indic_Syllabic_Category {
    type Err = ParseError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "Avagraha" => Ok(Indic_Syllabic_Category::Avagraha),
            "Bindu" => Ok(Indic_Syllabic_Category::Bindu),
            "Brahmi_Joining_Number" => Ok(Indic_Syllabic_Category::Brahmi_Joining_Number),
            "Cantillation_Mark" => Ok(Indic_Syllabic_Category::Cantillation_Mark),
            "Consonant" => Ok(Indic_Syllabic_Category::Consonant),
            "Consonant_Dead" => Ok(Indic_Syllabic_Category::Consonant_Dead),
            "Consonant_Final" => Ok(Indic_Syllabic_Category::Consonant_Final),
            "Consonant_Head_Letter" => Ok(Indic_Syllabic_Category::Consonant_Head_Letter),
            "Consonant_Initial_Postfixed" => Ok(Indic_Syllabic_Category::Consonant_Initial_Postfixed),
            "Consonant_Killer" => Ok(Indic_Syllabic_Category::Consonant_Killer),
            "Consonant_Medial" => Ok(Indic_Syllabic_Category::Consonant_Medial),
            "Consonant_Placeholder" => Ok(Indic_Syllabic_Category::Consonant_Placeholder),
            "Consonant_Preceding_Repha" => Ok(Indic_Syllabic_Category::Consonant_Preceding_Repha),
            "Consonant_Prefixed" => Ok(Indic_Syllabic_Category::Consonant_Prefixed),
            "Consonant_Subjoined" => Ok(Indic_Syllabic_Category::Consonant_Subjoined),
            "Consonant_Succeeding_Repha" => Ok(Indic_Syllabic_Category::Consonant_Succeeding_Repha),
            "Consonant_With_Stacker" => Ok(Indic_Syllabic_Category::Consonant_With_Stacker),
            "Gemination_Mark" => Ok(Indic_Syllabic_Category::Gemination_Mark),
            "Invisible_Stacker" => Ok(Indic_Syllabic_Category::Invisible_Stacker),
            "Joiner" => Ok(Indic_Syllabic_Category::Joiner),
            "Modifying_Letter" => Ok(Indic_Syllabic_Category::Modifying_Letter),
            "Non_Joiner" => Ok(Indic_Syllabic_Category::Non_Joiner),
            "Nukta" => Ok(Indic_Syllabic_Category::Nukta),
            "Number" => Ok(Indic_Syllabic_Category::Number),
            "Number_Joiner" => Ok(Indic_Syllabic_Category::Number_Joiner),
            "Other" => Ok(Indic_Syllabic_Category::Other),
            "Pure_Killer" => Ok(Indic_Syllabic_Category::Pure_Killer),
            "Register_Shifter" => Ok(Indic_Syllabic_Category::Register_Shifter),
            "Syllable_Modifier" => Ok(Indic_Syllabic_Category::Syllable_Modifier),
            "Tone_Letter" => Ok(Indic_Syllabic_Category::Tone_Letter),
            "Tone_Mark" => Ok(Indic_Syllabic_Category::Tone_Mark),
            "Virama" => Ok(Indic_Syllabic_Category::Virama),
            "Visarga" => Ok(Indic_Syllabic_Category::Visarga),
            "Vowel" => Ok(Indic_Syllabic_Category::Vowel),
            "Vowel_Dependent" => Ok(Indic_Syllabic_Category::Vowel_Dependent),
            "Vowel_Independent" => Ok(Indic_Syllabic_Category::Vowel_Independent),
            _ => Err(ParseError(())),
        }
    }
}

/// AKA `jg`
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Joining_Group {
    African_Feh,
    African_Noon,
    African_Qaf,
    Ain,
    Alaph,
    Alef,
    Beh,
    Beth,
    Burushaski_Yeh_Barree,
    Dal,
    Dalath_Rish,
    E,
    Farsi_Yeh,
    Fe,
    Feh,
    Final_Semkath,
    Gaf,
    Gamal,
    Hah,
    Hanifi_Rohingya_Kinna_Ya,
    Hanifi_Rohingya_Pa,
    He,
    Heh,
    Heh_Goal,
    Heth,
    Kaf,
    Kaph,
    Khaph,
    Knotted_Heh,
    Lam,
    Lamadh,
    Malayalam_Bha,
    Malayalam_Ja,
    Malayalam_Lla,
    Malayalam_Llla,
    Malayalam_Nga,
    Malayalam_Nna,
    Malayalam_Nnna,
    Malayalam_Nya,
    Malayalam_Ra,
    Malayalam_Ssa,
    Malayalam_Tta,
    Manichaean_Aleph,
    Manichaean_Ayin,
    Manichaean_Beth,
    Manichaean_Daleth,
    Manichaean_Dhamedh,
    Manichaean_Five,
    Manichaean_Gimel,
    Manichaean_Heth,
    Manichaean_Hundred,
    Manichaean_Kaph,
    Manichaean_Lamedh,
    Manichaean_Mem,
    Manichaean_Nun,
    Manichaean_One,
    Manichaean_Pe,
    Manichaean_Qoph,
    Manichaean_Resh,
    Manichaean_Sadhe,
    Manichaean_Samekh,
    Manichaean_Taw,
    Manichaean_Ten,
    Manichaean_Teth,
    Manichaean_Thamedh,
    Manichaean_Twenty,
    Manichaean_Waw,
    Manichaean_Yodh,
    Manichaean_Zayin,
    Meem,
    Mim,
    No_Joining_Group,
    Noon,
    Nun,
    Nya,
    Pe,
    Qaf,
    Qaph,
    Reh,
    Reversed_Pe,
    Rohingya_Yeh,
    Sad,
    Sadhe,
    Seen,
    Semkath,
    Shin,
    Straight_Waw,
    Swash_Kaf,
    Syriac_Waw,
    Tah,
    Taw,
    Teh_Marbuta,
    /// AKA `Teh_Marbuta_Goal`
    Hamza_On_Heh_Goal,
    Teth,
    Waw,
    Yeh,
    Yeh_Barree,
    Yeh_With_Tail,
    Yudh,
    Yudh_He,
    Zain,
    Zhain,
}
impl fmt::Display for Joining_Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <str as fmt::Display>::fmt(self.display_str(), f)
    }
}
impl Joining_Group {
    fn display_str(&self) -> &'static str {
        match self {
            Joining_Group::African_Feh => "African_Feh",
            Joining_Group::African_Noon => "African_Noon",
            Joining_Group::African_Qaf => "African_Qaf",
            Joining_Group::Ain => "Ain",
            Joining_Group::Alaph => "Alaph",
            Joining_Group::Alef => "Alef",
            Joining_Group::Beh => "Beh",
            Joining_Group::Beth => "Beth",
            Joining_Group::Burushaski_Yeh_Barree => "Burushaski_Yeh_Barree",
            Joining_Group::Dal => "Dal",
            Joining_Group::Dalath_Rish => "Dalath_Rish",
            Joining_Group::E => "E",
            Joining_Group::Farsi_Yeh => "Farsi_Yeh",
            Joining_Group::Fe => "Fe",
            Joining_Group::Feh => "Feh",
            Joining_Group::Final_Semkath => "Final_Semkath",
            Joining_Group::Gaf => "Gaf",
            Joining_Group::Gamal => "Gamal",
            Joining_Group::Hah => "Hah",
            Joining_Group::Hanifi_Rohingya_Kinna_Ya => "Hanifi_Rohingya_Kinna_Ya",
            Joining_Group::Hanifi_Rohingya_Pa => "Hanifi_Rohingya_Pa",
            Joining_Group::He => "He",
            Joining_Group::Heh => "Heh",
            Joining_Group::Heh_Goal => "Heh_Goal",
            Joining_Group::Heth => "Heth",
            Joining_Group::Kaf => "Kaf",
            Joining_Group::Kaph => "Kaph",
            Joining_Group::Khaph => "Khaph",
            Joining_Group::Knotted_Heh => "Knotted_Heh",
            Joining_Group::Lam => "Lam",
            Joining_Group::Lamadh => "Lamadh",
            Joining_Group::Malayalam_Bha => "Malayalam_Bha",
            Joining_Group::Malayalam_Ja => "Malayalam_Ja",
            Joining_Group::Malayalam_Lla => "Malayalam_Lla",
            Joining_Group::Malayalam_Llla => "Malayalam_Llla",
            Joining_Group::Malayalam_Nga => "Malayalam_Nga",
            Joining_Group::Malayalam_Nna => "Malayalam_Nna",
            Joining_Group::Malayalam_Nnna => "Malayalam_Nnna",
            Joining_Group::Malayalam_Nya => "Malayalam_Nya",
            Joining_Group::Malayalam_Ra => "Malayalam_Ra",
            Joining_Group::Malayalam_Ssa => "Malayalam_Ssa",
            Joining_Group::Malayalam_Tta => "Malayalam_Tta",
            Joining_Group::Manichaean_Aleph => "Manichaean_Aleph",
            Joining_Group::Manichaean_Ayin => "Manichaean_Ayin",
            Joining_Group::Manichaean_Beth => "Manichaean_Beth",
            Joining_Group::Manichaean_Daleth => "Manichaean_Daleth",
            Joining_Group::Manichaean_Dhamedh => "Manichaean_Dhamedh",
            Joining_Group::Manichaean_Five => "Manichaean_Five",
            Joining_Group::Manichaean_Gimel => "Manichaean_Gimel",
            Joining_Group::Manichaean_Heth => "Manichaean_Heth",
            Joining_Group::Manichaean_Hundred => "Manichaean_Hundred",
            Joining_Group::Manichaean_Kaph => "Manichaean_Kaph",
            Joining_Group::Manichaean_Lamedh => "Manichaean_Lamedh",
            Joining_Group::Manichaean_Mem => "Manichaean_Mem",
            Joining_Group::Manichaean_Nun => "Manichaean_Nun",
            Joining_Group::Manichaean_One => "Manichaean_One",
            Joining_Group::Manichaean_Pe => "Manichaean_Pe",
            Joining_Group::Manichaean_Qoph => "Manichaean_Qoph",
            Joining_Group::Manichaean_Resh => "Manichaean_Resh",
            Joining_Group::Manichaean_Sadhe => "Manichaean_Sadhe",
            Joining_Group::Manichaean_Samekh => "Manichaean_Samekh",
            Joining_Group::Manichaean_Taw => "Manichaean_Taw",
            Joining_Group::Manichaean_Ten => "Manichaean_Ten",
            Joining_Group::Manichaean_Teth => "Manichaean_Teth",
            Joining_Group::Manichaean_Thamedh => "Manichaean_Thamedh",
            Joining_Group::Manichaean_Twenty => "Manichaean_Twenty",
            Joining_Group::Manichaean_Waw => "Manichaean_Waw",
            Joining_Group::Manichaean_Yodh => "Manichaean_Yodh",
            Joining_Group::Manichaean_Zayin => "Manichaean_Zayin",
            Joining_Group::Meem => "Meem",
            Joining_Group::Mim => "Mim",
            Joining_Group::No_Joining_Group => "No_Joining_Group",
            Joining_Group::Noon => "Noon",
            Joining_Group::Nun => "Nun",
            Joining_Group::Nya => "Nya",
            Joining_Group::Pe => "Pe",
            Joining_Group::Qaf => "Qaf",
            Joining_Group::Qaph => "Qaph",
            Joining_Group::Reh => "Reh",
            Joining_Group::Reversed_Pe => "Reversed_Pe",
            Joining_Group::Rohingya_Yeh => "Rohingya_Yeh",
            Joining_Group::Sad => "Sad",
            Joining_Group::Sadhe => "Sadhe",
            Joining_Group::Seen => "Seen",
            Joining_Group::Semkath => "Semkath",
            Joining_Group::Shin => "Shin",
            Joining_Group::Straight_Waw => "Straight_Waw",
            Joining_Group::Swash_Kaf => "Swash_Kaf",
            Joining_Group::Syriac_Waw => "Syriac_Waw",
            Joining_Group::Tah => "Tah",
            Joining_Group::Taw => "Taw",
            Joining_Group::Teh_Marbuta => "Teh_Marbuta",
            Joining_Group::Hamza_On_Heh_Goal => "Teh_Marbuta_Goal",
            Joining_Group::Teth => "Teth",
            Joining_Group::Waw => "Waw",
            Joining_Group::Yeh => "Yeh",
            Joining_Group::Yeh_Barree => "Yeh_Barree",
            Joining_Group::Yeh_With_Tail => "Yeh_With_Tail",
            Joining_Group::Yudh => "Yudh",
            Joining_Group::Yudh_He => "Yudh_He",
            Joining_Group::Zain => "Zain",
            Joining_Group::Zhain => "Zhain",
        }
    }
}
impl FromStr for Joining_Group {
    type Err = ParseError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "African_Feh" => Ok(Joining_Group::African_Feh),
            "African_Noon" => Ok(Joining_Group::African_Noon),
            "African_Qaf" => Ok(Joining_Group::African_Qaf),
            "Ain" => Ok(Joining_Group::Ain),
            "Alaph" => Ok(Joining_Group::Alaph),
            "Alef" => Ok(Joining_Group::Alef),
            "Beh" => Ok(Joining_Group::Beh),
            "Beth" => Ok(Joining_Group::Beth),
            "Burushaski_Yeh_Barree" => Ok(Joining_Group::Burushaski_Yeh_Barree),
            "Dal" => Ok(Joining_Group::Dal),
            "Dalath_Rish" => Ok(Joining_Group::Dalath_Rish),
            "E" => Ok(Joining_Group::E),
            "Farsi_Yeh" => Ok(Joining_Group::Farsi_Yeh),
            "Fe" => Ok(Joining_Group::Fe),
            "Feh" => Ok(Joining_Group::Feh),
            "Final_Semkath" => Ok(Joining_Group::Final_Semkath),
            "Gaf" => Ok(Joining_Group::Gaf),
            "Gamal" => Ok(Joining_Group::Gamal),
            "Hah" => Ok(Joining_Group::Hah),
            "Hanifi_Rohingya_Kinna_Ya" => Ok(Joining_Group::Hanifi_Rohingya_Kinna_Ya),
            "Hanifi_Rohingya_Pa" => Ok(Joining_Group::Hanifi_Rohingya_Pa),
            "He" => Ok(Joining_Group::He),
            "Heh" => Ok(Joining_Group::Heh),
            "Heh_Goal" => Ok(Joining_Group::Heh_Goal),
            "Heth" => Ok(Joining_Group::Heth),
            "Kaf" => Ok(Joining_Group::Kaf),
            "Kaph" => Ok(Joining_Group::Kaph),
            "Khaph" => Ok(Joining_Group::Khaph),
            "Knotted_Heh" => Ok(Joining_Group::Knotted_Heh),
            "Lam" => Ok(Joining_Group::Lam),
            "Lamadh" => Ok(Joining_Group::Lamadh),
            "Malayalam_Bha" => Ok(Joining_Group::Malayalam_Bha),
            "Malayalam_Ja" => Ok(Joining_Group::Malayalam_Ja),
            "Malayalam_Lla" => Ok(Joining_Group::Malayalam_Lla),
            "Malayalam_Llla" => Ok(Joining_Group::Malayalam_Llla),
            "Malayalam_Nga" => Ok(Joining_Group::Malayalam_Nga),
            "Malayalam_Nna" => Ok(Joining_Group::Malayalam_Nna),
            "Malayalam_Nnna" => Ok(Joining_Group::Malayalam_Nnna),
            "Malayalam_Nya" => Ok(Joining_Group::Malayalam_Nya),
            "Malayalam_Ra" => Ok(Joining_Group::Malayalam_Ra),
            "Malayalam_Ssa" => Ok(Joining_Group::Malayalam_Ssa),
            "Malayalam_Tta" => Ok(Joining_Group::Malayalam_Tta),
            "Manichaean_Aleph" => Ok(Joining_Group::Manichaean_Aleph),
            "Manichaean_Ayin" => Ok(Joining_Group::Manichaean_Ayin),
            "Manichaean_Beth" => Ok(Joining_Group::Manichaean_Beth),
            "Manichaean_Daleth" => Ok(Joining_Group::Manichaean_Daleth),
            "Manichaean_Dhamedh" => Ok(Joining_Group::Manichaean_Dhamedh),
            "Manichaean_Five" => Ok(Joining_Group::Manichaean_Five),
            "Manichaean_Gimel" => Ok(Joining_Group::Manichaean_Gimel),
            "Manichaean_Heth" => Ok(Joining_Group::Manichaean_Heth),
            "Manichaean_Hundred" => Ok(Joining_Group::Manichaean_Hundred),
            "Manichaean_Kaph" => Ok(Joining_Group::Manichaean_Kaph),
            "Manichaean_Lamedh" => Ok(Joining_Group::Manichaean_Lamedh),
            "Manichaean_Mem" => Ok(Joining_Group::Manichaean_Mem),
            "Manichaean_Nun" => Ok(Joining_Group::Manichaean_Nun),
            "Manichaean_One" => Ok(Joining_Group::Manichaean_One),
            "Manichaean_Pe" => Ok(Joining_Group::Manichaean_Pe),
            "Manichaean_Qoph" => Ok(Joining_Group::Manichaean_Qoph),
            "Manichaean_Resh" => Ok(Joining_Group::Manichaean_Resh),
            "Manichaean_Sadhe" => Ok(Joining_Group::Manichaean_Sadhe),
            "Manichaean_Samekh" => Ok(Joining_Group::Manichaean_Samekh),
            "Manichaean_Taw" => Ok(Joining_Group::Manichaean_Taw),
            "Manichaean_Ten" => Ok(Joining_Group::Manichaean_Ten),
            "Manichaean_Teth" => Ok(Joining_Group::Manichaean_Teth),
            "Manichaean_Thamedh" => Ok(Joining_Group::Manichaean_Thamedh),
            "Manichaean_Twenty" => Ok(Joining_Group::Manichaean_Twenty),
            "Manichaean_Waw" => Ok(Joining_Group::Manichaean_Waw),
            "Manichaean_Yodh" => Ok(Joining_Group::Manichaean_Yodh),
            "Manichaean_Zayin" => Ok(Joining_Group::Manichaean_Zayin),
            "Meem" => Ok(Joining_Group::Meem),
            "Mim" => Ok(Joining_Group::Mim),
            "No_Joining_Group" => Ok(Joining_Group::No_Joining_Group),
            "Noon" => Ok(Joining_Group::Noon),
            "Nun" => Ok(Joining_Group::Nun),
            "Nya" => Ok(Joining_Group::Nya),
            "Pe" => Ok(Joining_Group::Pe),
            "Qaf" => Ok(Joining_Group::Qaf),
            "Qaph" => Ok(Joining_Group::Qaph),
            "Reh" => Ok(Joining_Group::Reh),
            "Reversed_Pe" => Ok(Joining_Group::Reversed_Pe),
            "Rohingya_Yeh" => Ok(Joining_Group::Rohingya_Yeh),
            "Sad" => Ok(Joining_Group::Sad),
            "Sadhe" => Ok(Joining_Group::Sadhe),
            "Seen" => Ok(Joining_Group::Seen),
            "Semkath" => Ok(Joining_Group::Semkath),
            "Shin" => Ok(Joining_Group::Shin),
            "Straight_Waw" => Ok(Joining_Group::Straight_Waw),
            "Swash_Kaf" => Ok(Joining_Group::Swash_Kaf),
            "Syriac_Waw" => Ok(Joining_Group::Syriac_Waw),
            "Tah" => Ok(Joining_Group::Tah),
            "Taw" => Ok(Joining_Group::Taw),
            "Teh_Marbuta" => Ok(Joining_Group::Teh_Marbuta),
            "Teh_Marbuta_Goal" => Ok(Joining_Group::Hamza_On_Heh_Goal),
            "Teth" => Ok(Joining_Group::Teth),
            "Waw" => Ok(Joining_Group::Waw),
            "Yeh" => Ok(Joining_Group::Yeh),
            "Yeh_Barree" => Ok(Joining_Group::Yeh_Barree),
            "Yeh_With_Tail" => Ok(Joining_Group::Yeh_With_Tail),
            "Yudh" => Ok(Joining_Group::Yudh),
            "Yudh_He" => Ok(Joining_Group::Yudh_He),
            "Zain" => Ok(Joining_Group::Zain),
            "Zhain" => Ok(Joining_Group::Zhain),
            _ => Err(ParseError(())),
        }
    }
}

/// AKA `jt`
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Joining_Type {
    /// AKA `C`
    Join_Causing,
    /// AKA `D`
    Dual_Joining,
    /// AKA `L`
    Left_Joining,
    /// AKA `R`
    Right_Joining,
    /// AKA `T`
    Transparent,
    /// AKA `U`
    Non_Joining,
}
impl fmt::Display for Joining_Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <str as fmt::Display>::fmt(self.display_str(), f)
    }
}
impl Joining_Type {
    fn display_str(&self) -> &'static str {
        match self {
            Joining_Type::Join_Causing => "C",
            Joining_Type::Dual_Joining => "D",
            Joining_Type::Left_Joining => "L",
            Joining_Type::Right_Joining => "R",
            Joining_Type::Transparent => "T",
            Joining_Type::Non_Joining => "U",
        }
    }
}
impl FromStr for Joining_Type {
    type Err = ParseError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "C" => Ok(Joining_Type::Join_Causing),
            "D" => Ok(Joining_Type::Dual_Joining),
            "L" => Ok(Joining_Type::Left_Joining),
            "R" => Ok(Joining_Type::Right_Joining),
            "T" => Ok(Joining_Type::Transparent),
            "U" => Ok(Joining_Type::Non_Joining),
            _ => Err(ParseError(())),
        }
    }
}

/// AKA `lb`
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Line_Break {
    /// AKA `AI`
    Ambiguous,
    /// AKA `AL`
    Alphabetic,
    /// AKA `B2`
    Break_Both,
    /// AKA `BA`
    Break_After,
    /// AKA `BB`
    Break_Before,
    /// AKA `BK`
    Mandatory_Break,
    /// AKA `CB`
    Contingent_Break,
    /// AKA `CJ`
    Conditional_Japanese_Starter,
    /// AKA `CL`
    Close_Punctuation,
    /// AKA `CM`
    Combining_Mark,
    /// AKA `CP`
    Close_Parenthesis,
    /// AKA `CR`
    Carriage_Return,
    /// AKA `EB`
    E_Base,
    /// AKA `EM`
    E_Modifier,
    /// AKA `EX`
    Exclamation,
    /// AKA `GL`
    Glue,
    H2,
    H3,
    /// AKA `HL`
    Hebrew_Letter,
    /// AKA `HY`
    Hyphen,
    /// AKA `ID`
    Ideographic,
    /// AKA `IN`
    Inseparable,
    /// AKA `IS`
    Infix_Numeric,
    JL,
    JT,
    JV,
    /// AKA `LF`
    Line_Feed,
    /// AKA `NL`
    Next_Line,
    /// AKA `NS`
    Nonstarter,
    /// AKA `NU`
    Numeric,
    /// AKA `OP`
    Open_Punctuation,
    /// AKA `PO`
    Postfix_Numeric,
    /// AKA `PR`
    Prefix_Numeric,
    /// AKA `QU`
    Quotation,
    /// AKA `RI`
    Regional_Indicator,
    /// AKA `SA`
    Complex_Context,
    /// AKA `SG`
    Surrogate,
    /// AKA `SP`
    Space,
    /// AKA `SY`
    Break_Symbols,
    /// AKA `WJ`
    Word_Joiner,
    /// AKA `XX`
    Unknown,
    /// AKA `ZW`
    ZWSpace,
    ZWJ,
}
impl fmt::Display for Line_Break {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <str as fmt::Display>::fmt(self.display_str(), f)
    }
}
impl Line_Break {
    fn display_str(&self) -> &'static str {
        match self {
            Line_Break::Ambiguous => "AI",
            Line_Break::Alphabetic => "AL",
            Line_Break::Break_Both => "B2",
            Line_Break::Break_After => "BA",
            Line_Break::Break_Before => "BB",
            Line_Break::Mandatory_Break => "BK",
            Line_Break::Contingent_Break => "CB",
            Line_Break::Conditional_Japanese_Starter => "CJ",
            Line_Break::Close_Punctuation => "CL",
            Line_Break::Combining_Mark => "CM",
            Line_Break::Close_Parenthesis => "CP",
            Line_Break::Carriage_Return => "CR",
            Line_Break::E_Base => "EB",
            Line_Break::E_Modifier => "EM",
            Line_Break::Exclamation => "EX",
            Line_Break::Glue => "GL",
            Line_Break::H2 => "H2",
            Line_Break::H3 => "H3",
            Line_Break::Hebrew_Letter => "HL",
            Line_Break::Hyphen => "HY",
            Line_Break::Ideographic => "ID",
            Line_Break::Inseparable => "IN",
            Line_Break::Infix_Numeric => "IS",
            Line_Break::JL => "JL",
            Line_Break::JT => "JT",
            Line_Break::JV => "JV",
            Line_Break::Line_Feed => "LF",
            Line_Break::Next_Line => "NL",
            Line_Break::Nonstarter => "NS",
            Line_Break::Numeric => "NU",
            Line_Break::Open_Punctuation => "OP",
            Line_Break::Postfix_Numeric => "PO",
            Line_Break::Prefix_Numeric => "PR",
            Line_Break::Quotation => "QU",
            Line_Break::Regional_Indicator => "RI",
            Line_Break::Complex_Context => "SA",
            Line_Break::Surrogate => "SG",
            Line_Break::Space => "SP",
            Line_Break::Break_Symbols => "SY",
            Line_Break::Word_Joiner => "WJ",
            Line_Break::Unknown => "XX",
            Line_Break::ZWSpace => "ZW",
            Line_Break::ZWJ => "ZWJ",
        }
    }
}
impl FromStr for Line_Break {
    type Err = ParseError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "AI" => Ok(Line_Break::Ambiguous),
            "AL" => Ok(Line_Break::Alphabetic),
            "B2" => Ok(Line_Break::Break_Both),
            "BA" => Ok(Line_Break::Break_After),
            "BB" => Ok(Line_Break::Break_Before),
            "BK" => Ok(Line_Break::Mandatory_Break),
            "CB" => Ok(Line_Break::Contingent_Break),
            "CJ" => Ok(Line_Break::Conditional_Japanese_Starter),
            "CL" => Ok(Line_Break::Close_Punctuation),
            "CM" => Ok(Line_Break::Combining_Mark),
            "CP" => Ok(Line_Break::Close_Parenthesis),
            "CR" => Ok(Line_Break::Carriage_Return),
            "EB" => Ok(Line_Break::E_Base),
            "EM" => Ok(Line_Break::E_Modifier),
            "EX" => Ok(Line_Break::Exclamation),
            "GL" => Ok(Line_Break::Glue),
            "H2" => Ok(Line_Break::H2),
            "H3" => Ok(Line_Break::H3),
            "HL" => Ok(Line_Break::Hebrew_Letter),
            "HY" => Ok(Line_Break::Hyphen),
            "ID" => Ok(Line_Break::Ideographic),
            "IN" => Ok(Line_Break::Inseparable),
            "IS" => Ok(Line_Break::Infix_Numeric),
            "JL" => Ok(Line_Break::JL),
            "JT" => Ok(Line_Break::JT),
            "JV" => Ok(Line_Break::JV),
            "LF" => Ok(Line_Break::Line_Feed),
            "NL" => Ok(Line_Break::Next_Line),
            "NS" => Ok(Line_Break::Nonstarter),
            "NU" => Ok(Line_Break::Numeric),
            "OP" => Ok(Line_Break::Open_Punctuation),
            "PO" => Ok(Line_Break::Postfix_Numeric),
            "PR" => Ok(Line_Break::Prefix_Numeric),
            "QU" => Ok(Line_Break::Quotation),
            "RI" => Ok(Line_Break::Regional_Indicator),
            "SA" => Ok(Line_Break::Complex_Context),
            "SG" => Ok(Line_Break::Surrogate),
            "SP" => Ok(Line_Break::Space),
            "SY" => Ok(Line_Break::Break_Symbols),
            "WJ" => Ok(Line_Break::Word_Joiner),
            "XX" => Ok(Line_Break::Unknown),
            "ZW" => Ok(Line_Break::ZWSpace),
            "ZWJ" => Ok(Line_Break::ZWJ),
            _ => Err(ParseError(())),
        }
    }
}

/// AKA `NFC_QC`
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum NFC_Quick_Check {
    /// AKA `M`
    Maybe,
    /// AKA `N`
    No,
    /// AKA `Y`
    Yes,
}
impl fmt::Display for NFC_Quick_Check {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <str as fmt::Display>::fmt(self.display_str(), f)
    }
}
impl NFC_Quick_Check {
    fn display_str(&self) -> &'static str {
        match self {
            NFC_Quick_Check::Maybe => "M",
            NFC_Quick_Check::No => "N",
            NFC_Quick_Check::Yes => "Y",
        }
    }
}
impl FromStr for NFC_Quick_Check {
    type Err = ParseError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "M" => Ok(NFC_Quick_Check::Maybe),
            "N" => Ok(NFC_Quick_Check::No),
            "Y" => Ok(NFC_Quick_Check::Yes),
            _ => Err(ParseError(())),
        }
    }
}

/// AKA `NFKC_QC`
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum NFKC_Quick_Check {
    /// AKA `M`
    Maybe,
    /// AKA `N`
    No,
    /// AKA `Y`
    Yes,
}
impl fmt::Display for NFKC_Quick_Check {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <str as fmt::Display>::fmt(self.display_str(), f)
    }
}
impl NFKC_Quick_Check {
    fn display_str(&self) -> &'static str {
        match self {
            NFKC_Quick_Check::Maybe => "M",
            NFKC_Quick_Check::No => "N",
            NFKC_Quick_Check::Yes => "Y",
        }
    }
}
impl FromStr for NFKC_Quick_Check {
    type Err = ParseError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "M" => Ok(NFKC_Quick_Check::Maybe),
            "N" => Ok(NFKC_Quick_Check::No),
            "Y" => Ok(NFKC_Quick_Check::Yes),
            _ => Err(ParseError(())),
        }
    }
}

/// AKA `nt`
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Numeric_Type {
    /// AKA `De`
    Decimal,
    /// AKA `Di`
    Digit,
    None,
    /// AKA `Nu`
    Numeric,
}
impl fmt::Display for Numeric_Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <str as fmt::Display>::fmt(self.display_str(), f)
    }
}
impl Numeric_Type {
    fn display_str(&self) -> &'static str {
        match self {
            Numeric_Type::Decimal => "De",
            Numeric_Type::Digit => "Di",
            Numeric_Type::None => "None",
            Numeric_Type::Numeric => "Nu",
        }
    }
}
impl FromStr for Numeric_Type {
    type Err = ParseError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "De" => Ok(Numeric_Type::Decimal),
            "Di" => Ok(Numeric_Type::Digit),
            "None" => Ok(Numeric_Type::None),
            "Nu" => Ok(Numeric_Type::Numeric),
            _ => Err(ParseError(())),
        }
    }
}

/// AKA `SB`
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Sentence_Break {
    /// AKA `AT`
    ATerm,
    /// AKA `CL`
    Close,
    CR,
    /// AKA `EX`
    Extend,
    /// AKA `FO`
    Format,
    /// AKA `LE`
    OLetter,
    LF,
    /// AKA `LO`
    Lower,
    /// AKA `NU`
    Numeric,
    /// AKA `SC`
    SContinue,
    /// AKA `SE`
    Sep,
    /// AKA `SP`
    Sp,
    /// AKA `ST`
    STerm,
    /// AKA `UP`
    Upper,
    /// AKA `XX`
    Other,
}
impl fmt::Display for Sentence_Break {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <str as fmt::Display>::fmt(self.display_str(), f)
    }
}
impl Sentence_Break {
    fn display_str(&self) -> &'static str {
        match self {
            Sentence_Break::ATerm => "AT",
            Sentence_Break::Close => "CL",
            Sentence_Break::CR => "CR",
            Sentence_Break::Extend => "EX",
            Sentence_Break::Format => "FO",
            Sentence_Break::OLetter => "LE",
            Sentence_Break::LF => "LF",
            Sentence_Break::Lower => "LO",
            Sentence_Break::Numeric => "NU",
            Sentence_Break::SContinue => "SC",
            Sentence_Break::Sep => "SE",
            Sentence_Break::Sp => "SP",
            Sentence_Break::STerm => "ST",
            Sentence_Break::Upper => "UP",
            Sentence_Break::Other => "XX",
        }
    }
}
impl FromStr for Sentence_Break {
    type Err = ParseError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "AT" => Ok(Sentence_Break::ATerm),
            "CL" => Ok(Sentence_Break::Close),
            "CR" => Ok(Sentence_Break::CR),
            "EX" => Ok(Sentence_Break::Extend),
            "FO" => Ok(Sentence_Break::Format),
            "LE" => Ok(Sentence_Break::OLetter),
            "LF" => Ok(Sentence_Break::LF),
            "LO" => Ok(Sentence_Break::Lower),
            "NU" => Ok(Sentence_Break::Numeric),
            "SC" => Ok(Sentence_Break::SContinue),
            "SE" => Ok(Sentence_Break::Sep),
            "SP" => Ok(Sentence_Break::Sp),
            "ST" => Ok(Sentence_Break::STerm),
            "UP" => Ok(Sentence_Break::Upper),
            "XX" => Ok(Sentence_Break::Other),
            _ => Err(ParseError(())),
        }
    }
}

/// AKA `vo`
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Vertical_Orientation {
    /// AKA `R`
    Rotated,
    /// AKA `Tr`
    Transformed_Rotated,
    /// AKA `Tu`
    Transformed_Upright,
    /// AKA `U`
    Upright,
}
impl fmt::Display for Vertical_Orientation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <str as fmt::Display>::fmt(self.display_str(), f)
    }
}
impl Vertical_Orientation {
    fn display_str(&self) -> &'static str {
        match self {
            Vertical_Orientation::Rotated => "R",
            Vertical_Orientation::Transformed_Rotated => "Tr",
            Vertical_Orientation::Transformed_Upright => "Tu",
            Vertical_Orientation::Upright => "U",
        }
    }
}
impl FromStr for Vertical_Orientation {
    type Err = ParseError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "R" => Ok(Vertical_Orientation::Rotated),
            "Tr" => Ok(Vertical_Orientation::Transformed_Rotated),
            "Tu" => Ok(Vertical_Orientation::Transformed_Upright),
            "U" => Ok(Vertical_Orientation::Upright),
            _ => Err(ParseError(())),
        }
    }
}

/// AKA `WB`
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Word_Break {
    CR,
    /// AKA `DQ`
    Double_Quote,
    /// AKA `EB`
    E_Base,
    /// AKA `EBG`
    E_Base_GAZ,
    /// AKA `EM`
    E_Modifier,
    /// AKA `EX`
    ExtendNumLet,
    Extend,
    /// AKA `FO`
    Format,
    /// AKA `GAZ`
    Glue_After_Zwj,
    /// AKA `HL`
    Hebrew_Letter,
    /// AKA `KA`
    Katakana,
    /// AKA `LE`
    ALetter,
    LF,
    /// AKA `MB`
    MidNumLet,
    /// AKA `ML`
    MidLetter,
    /// AKA `MN`
    MidNum,
    /// AKA `NL`
    Newline,
    /// AKA `NU`
    Numeric,
    /// AKA `RI`
    Regional_Indicator,
    /// AKA `SQ`
    Single_Quote,
    WSegSpace,
    /// AKA `XX`
    Other,
    ZWJ,
}
impl fmt::Display for Word_Break {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <str as fmt::Display>::fmt(self.display_str(), f)
    }
}
impl Word_Break {
    fn display_str(&self) -> &'static str {
        match self {
            Word_Break::CR => "CR",
            Word_Break::Double_Quote => "DQ",
            Word_Break::E_Base => "EB",
            Word_Break::E_Base_GAZ => "EBG",
            Word_Break::E_Modifier => "EM",
            Word_Break::ExtendNumLet => "EX",
            Word_Break::Extend => "Extend",
            Word_Break::Format => "FO",
            Word_Break::Glue_After_Zwj => "GAZ",
            Word_Break::Hebrew_Letter => "HL",
            Word_Break::Katakana => "KA",
            Word_Break::ALetter => "LE",
            Word_Break::LF => "LF",
            Word_Break::MidNumLet => "MB",
            Word_Break::MidLetter => "ML",
            Word_Break::MidNum => "MN",
            Word_Break::Newline => "NL",
            Word_Break::Numeric => "NU",
            Word_Break::Regional_Indicator => "RI",
            Word_Break::Single_Quote => "SQ",
            Word_Break::WSegSpace => "WSegSpace",
            Word_Break::Other => "XX",
            Word_Break::ZWJ => "ZWJ",
        }
    }
}
impl FromStr for Word_Break {
    type Err = ParseError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "CR" => Ok(Word_Break::CR),
            "DQ" => Ok(Word_Break::Double_Quote),
            "EB" => Ok(Word_Break::E_Base),
            "EBG" => Ok(Word_Break::E_Base_GAZ),
            "EM" => Ok(Word_Break::E_Modifier),
            "EX" => Ok(Word_Break::ExtendNumLet),
            "Extend" => Ok(Word_Break::Extend),
            "FO" => Ok(Word_Break::Format),
            "GAZ" => Ok(Word_Break::Glue_After_Zwj),
            "HL" => Ok(Word_Break::Hebrew_Letter),
            "KA" => Ok(Word_Break::Katakana),
            "LE" => Ok(Word_Break::ALetter),
            "LF" => Ok(Word_Break::LF),
            "MB" => Ok(Word_Break::MidNumLet),
            "ML" => Ok(Word_Break::MidLetter),
            "MN" => Ok(Word_Break::MidNum),
            "NL" => Ok(Word_Break::Newline),
            "NU" => Ok(Word_Break::Numeric),
            "RI" => Ok(Word_Break::Regional_Indicator),
            "SQ" => Ok(Word_Break::Single_Quote),
            "WSegSpace" => Ok(Word_Break::WSegSpace),
            "XX" => Ok(Word_Break::Other),
            "ZWJ" => Ok(Word_Break::ZWJ),
            _ => Err(ParseError(())),
        }
    }
}
