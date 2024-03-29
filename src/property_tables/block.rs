use crate::lookup_table::LookupTable;

use crate::properties::Block;
use Block::*;

impl From<char> for Block {
    #[inline]
    fn from(c: char) -> Self {
        if c < ROW0_LIMIT {
            return ROW0_TABLE.get_or(&(c as u8), No_Block);
        }
        if c < PLANE0_LIMIT {
            return PLANE0_TABLE.get_or(&(c as u16), No_Block);
        }
        return SUPPLEMENTARY_TABLE.get_or(&(c as u32), No_Block);
    }
}

#[test]
fn validate_tables() {
    use std::convert::TryInto;
    ROW0_TABLE.validate();
    if let Ok(x) = (ROW0_LIMIT as u32).try_into() { assert!(!ROW0_TABLE.contains(&x)); }
    PLANE0_TABLE.validate();
    if let Ok(x) = (PLANE0_LIMIT as u32).try_into() { assert!(!PLANE0_TABLE.contains(&x)); }
    SUPPLEMENTARY_TABLE.validate();
}

const ROW0_TABLE: LookupTable<u8, Block> = lookup_table![
    (0x00, 0x7F, Basic),
];
const ROW0_LIMIT: char = '\u{80}';
const PLANE0_TABLE: LookupTable<u16, Block> = lookup_table![
    (0x0080, 0x024F, Latin),
    (0x0250, 0x02AF, IPA),
    (0x02B0, 0x02FF, Spacing),
    (0x0300, 0x036F, Combining),
    (0x0370, 0x03FF, Greek),
    (0x0400, 0x052F, Cyrillic),
    (0x0530, 0x058F, Armenian),
    (0x0590, 0x05FF, Hebrew),
    (0x0600, 0x06FF, Arabic),
    (0x0700, 0x074F, Syriac),
    (0x0750, 0x077F, Arabic),
    (0x0780, 0x07BF, Thaana),
    (0x07C0, 0x07FF, NKo),
    (0x0800, 0x083F, Samaritan),
    (0x0840, 0x085F, Mandaic),
    (0x0860, 0x086F, Syriac),
    (0x08A0, 0x08FF, Arabic),
    (0x0900, 0x097F, Devanagari),
    (0x0980, 0x09FF, Bengali),
    (0x0A00, 0x0A7F, Gurmukhi),
    (0x0A80, 0x0AFF, Gujarati),
    (0x0B00, 0x0B7F, Oriya),
    (0x0B80, 0x0BFF, Tamil),
    (0x0C00, 0x0C7F, Telugu),
    (0x0C80, 0x0CFF, Kannada),
    (0x0D00, 0x0D7F, Malayalam),
    (0x0D80, 0x0DFF, Sinhala),
    (0x0E00, 0x0E7F, Thai),
    (0x0E80, 0x0EFF, Lao),
    (0x0F00, 0x0FFF, Tibetan),
    (0x1000, 0x109F, Myanmar),
    (0x10A0, 0x10FF, Georgian),
    (0x1100, 0x11FF, Hangul),
    (0x1200, 0x139F, Ethiopic),
    (0x13A0, 0x13FF, Cherokee),
    (0x1400, 0x167F, Unified),
    (0x1680, 0x169F, Ogham),
    (0x16A0, 0x16FF, Runic),
    (0x1700, 0x171F, Tagalog),
    (0x1720, 0x173F, Hanunoo),
    (0x1740, 0x175F, Buhid),
    (0x1760, 0x177F, Tagbanwa),
    (0x1780, 0x17FF, Khmer),
    (0x1800, 0x18AF, Mongolian),
    (0x18B0, 0x18FF, Unified),
    (0x1900, 0x194F, Limbu),
    (0x1950, 0x197F, Tai),
    (0x1980, 0x19DF, New),
    (0x19E0, 0x19FF, Khmer),
    (0x1A00, 0x1A1F, Buginese),
    (0x1A20, 0x1AAF, Tai),
    (0x1AB0, 0x1AFF, Combining),
    (0x1B00, 0x1B7F, Balinese),
    (0x1B80, 0x1BBF, Sundanese),
    (0x1BC0, 0x1BFF, Batak),
    (0x1C00, 0x1C4F, Lepcha),
    (0x1C50, 0x1C7F, Ol),
    (0x1C80, 0x1C8F, Cyrillic),
    (0x1C90, 0x1CBF, Georgian),
    (0x1CC0, 0x1CCF, Sundanese),
    (0x1CD0, 0x1CFF, Vedic),
    (0x1D00, 0x1DBF, Phonetic),
    (0x1DC0, 0x1DFF, Combining),
    (0x1E00, 0x1EFF, Latin),
    (0x1F00, 0x1FFF, Greek),
    (0x2000, 0x206F, General),
    (0x2070, 0x209F, Superscripts),
    (0x20A0, 0x20CF, Currency),
    (0x20D0, 0x20FF, Combining),
    (0x2100, 0x214F, Letterlike),
    (0x2150, 0x218F, Number),
    (0x2190, 0x21FF, Arrows),
    (0x2200, 0x22FF, Mathematical),
    (0x2300, 0x23FF, Miscellaneous),
    (0x2400, 0x243F, Control),
    (0x2440, 0x245F, Optical),
    (0x2460, 0x24FF, Enclosed),
    (0x2500, 0x257F, Box),
    (0x2580, 0x259F, Block),
    (0x25A0, 0x25FF, Geometric),
    (0x2600, 0x26FF, Miscellaneous),
    (0x2700, 0x27BF, Dingbats),
    (0x27C0, 0x27EF, Miscellaneous),
    (0x27F0, 0x27FF, Supplemental),
    (0x2800, 0x28FF, Braille),
    (0x2900, 0x297F, Supplemental),
    (0x2980, 0x29FF, Miscellaneous),
    (0x2A00, 0x2AFF, Supplemental),
    (0x2B00, 0x2BFF, Miscellaneous),
    (0x2C00, 0x2C5F, Glagolitic),
    (0x2C60, 0x2C7F, Latin),
    (0x2C80, 0x2CFF, Coptic),
    (0x2D00, 0x2D2F, Georgian),
    (0x2D30, 0x2D7F, Tifinagh),
    (0x2D80, 0x2DDF, Ethiopic),
    (0x2DE0, 0x2DFF, Cyrillic),
    (0x2E00, 0x2E7F, Supplemental),
    (0x2E80, 0x2EFF, CJK),
    (0x2F00, 0x2FDF, Kangxi),
    (0x2FF0, 0x2FFF, Ideographic),
    (0x3000, 0x303F, CJK),
    (0x3040, 0x309F, Hiragana),
    (0x30A0, 0x30FF, Katakana),
    (0x3100, 0x312F, Bopomofo),
    (0x3130, 0x318F, Hangul),
    (0x3190, 0x319F, Kanbun),
    (0x31A0, 0x31BF, Bopomofo),
    (0x31C0, 0x31EF, CJK),
    (0x31F0, 0x31FF, Katakana),
    (0x3200, 0x32FF, Enclosed),
    (0x3300, 0x4DBF, CJK),
    (0x4DC0, 0x4DFF, Yijing),
    (0x4E00, 0x9FFF, CJK),
    (0xA000, 0xA4CF, Yi),
    (0xA4D0, 0xA4FF, Lisu),
    (0xA500, 0xA63F, Vai),
    (0xA640, 0xA69F, Cyrillic),
    (0xA6A0, 0xA6FF, Bamum),
    (0xA700, 0xA71F, Modifier),
    (0xA720, 0xA7FF, Latin),
    (0xA800, 0xA82F, Syloti),
    (0xA830, 0xA83F, Common),
    (0xA840, 0xA87F, Phags),
    (0xA880, 0xA8DF, Saurashtra),
    (0xA8E0, 0xA8FF, Devanagari),
    (0xA900, 0xA92F, Kayah),
    (0xA930, 0xA95F, Rejang),
    (0xA960, 0xA97F, Hangul),
    (0xA980, 0xA9DF, Javanese),
    (0xA9E0, 0xA9FF, Myanmar),
    (0xAA00, 0xAA5F, Cham),
    (0xAA60, 0xAA7F, Myanmar),
    (0xAA80, 0xAADF, Tai),
    (0xAAE0, 0xAAFF, Meetei),
    (0xAB00, 0xAB2F, Ethiopic),
    (0xAB30, 0xAB6F, Latin),
    (0xAB70, 0xABBF, Cherokee),
    (0xABC0, 0xABFF, Meetei),
    (0xAC00, 0xD7FF, Hangul),
    (0xD800, 0xDBFF, High),
    (0xDC00, 0xDFFF, Low),
    (0xE000, 0xF8FF, Private),
    (0xF900, 0xFAFF, CJK),
    (0xFB00, 0xFB4F, Alphabetic),
    (0xFB50, 0xFDFF, Arabic),
    (0xFE00, 0xFE0F, Variation),
    (0xFE10, 0xFE1F, Vertical),
    (0xFE20, 0xFE2F, Combining),
    (0xFE30, 0xFE4F, CJK),
    (0xFE50, 0xFE6F, Small),
    (0xFE70, 0xFEFF, Arabic),
    (0xFF00, 0xFFEF, Halfwidth),
    (0xFFF0, 0xFFFF, Specials),
];
const PLANE0_LIMIT: char = '\u{10000}';
const SUPPLEMENTARY_TABLE: LookupTable<u32, Block> = lookup_table![
    (0x010000, 0x0100FF, Linear),
    (0x010100, 0x01013F, Aegean),
    (0x010140, 0x0101CF, Ancient),
    (0x0101D0, 0x0101FF, Phaistos),
    (0x010280, 0x01029F, Lycian),
    (0x0102A0, 0x0102DF, Carian),
    (0x0102E0, 0x0102FF, Coptic),
    (0x010300, 0x01032F, Old),
    (0x010330, 0x01034F, Gothic),
    (0x010350, 0x01037F, Old),
    (0x010380, 0x01039F, Ugaritic),
    (0x0103A0, 0x0103DF, Old),
    (0x010400, 0x01044F, Deseret),
    (0x010450, 0x01047F, Shavian),
    (0x010480, 0x0104AF, Osmanya),
    (0x0104B0, 0x0104FF, Osage),
    (0x010500, 0x01052F, Elbasan),
    (0x010530, 0x01056F, Caucasian),
    (0x010600, 0x01077F, Linear),
    (0x010800, 0x01083F, Cypriot),
    (0x010840, 0x01085F, Imperial),
    (0x010860, 0x01087F, Palmyrene),
    (0x010880, 0x0108AF, Nabataean),
    (0x0108E0, 0x0108FF, Hatran),
    (0x010900, 0x01091F, Phoenician),
    (0x010920, 0x01093F, Lydian),
    (0x010980, 0x0109FF, Meroitic),
    (0x010A00, 0x010A5F, Kharoshthi),
    (0x010A60, 0x010A9F, Old),
    (0x010AC0, 0x010AFF, Manichaean),
    (0x010B00, 0x010B3F, Avestan),
    (0x010B40, 0x010B7F, Inscriptional),
    (0x010B80, 0x010BAF, Psalter),
    (0x010C00, 0x010C4F, Old),
    (0x010C80, 0x010CFF, Old),
    (0x010D00, 0x010D3F, Hanifi),
    (0x010E60, 0x010E7F, Rumi),
    (0x010F00, 0x010F2F, Old),
    (0x010F30, 0x010F6F, Sogdian),
    (0x010FE0, 0x010FFF, Elymaic),
    (0x011000, 0x01107F, Brahmi),
    (0x011080, 0x0110CF, Kaithi),
    (0x0110D0, 0x0110FF, Sora),
    (0x011100, 0x01114F, Chakma),
    (0x011150, 0x01117F, Mahajani),
    (0x011180, 0x0111DF, Sharada),
    (0x0111E0, 0x0111FF, Sinhala),
    (0x011200, 0x01124F, Khojki),
    (0x011280, 0x0112AF, Multani),
    (0x0112B0, 0x0112FF, Khudawadi),
    (0x011300, 0x01137F, Grantha),
    (0x011400, 0x01147F, Newa),
    (0x011480, 0x0114DF, Tirhuta),
    (0x011580, 0x0115FF, Siddham),
    (0x011600, 0x01165F, Modi),
    (0x011660, 0x01167F, Mongolian),
    (0x011680, 0x0116CF, Takri),
    (0x011700, 0x01173F, Ahom),
    (0x011800, 0x01184F, Dogra),
    (0x0118A0, 0x0118FF, Warang),
    (0x0119A0, 0x0119FF, Nandinagari),
    (0x011A00, 0x011A4F, Zanabazar),
    (0x011A50, 0x011AAF, Soyombo),
    (0x011AC0, 0x011AFF, Pau),
    (0x011C00, 0x011C6F, Bhaiksuki),
    (0x011C70, 0x011CBF, Marchen),
    (0x011D00, 0x011D5F, Masaram),
    (0x011D60, 0x011DAF, Gunjala),
    (0x011EE0, 0x011EFF, Makasar),
    (0x011FC0, 0x011FFF, Tamil),
    (0x012000, 0x01247F, Cuneiform),
    (0x012480, 0x01254F, Early),
    (0x013000, 0x01343F, Egyptian),
    (0x014400, 0x01467F, Anatolian),
    (0x016800, 0x016A3F, Bamum),
    (0x016A40, 0x016A6F, Mro),
    (0x016AD0, 0x016AFF, Bassa),
    (0x016B00, 0x016B8F, Pahawh),
    (0x016E40, 0x016E9F, Medefaidrin),
    (0x016F00, 0x016F9F, Miao),
    (0x016FE0, 0x016FFF, Ideographic),
    (0x017000, 0x018AFF, Tangut),
    (0x01B000, 0x01B12F, Kana),
    (0x01B130, 0x01B16F, Small),
    (0x01B170, 0x01B2FF, Nushu),
    (0x01BC00, 0x01BC9F, Duployan),
    (0x01BCA0, 0x01BCAF, Shorthand),
    (0x01D000, 0x01D0FF, Byzantine),
    (0x01D100, 0x01D1FF, Musical),
    (0x01D200, 0x01D24F, Ancient),
    (0x01D2E0, 0x01D2FF, Mayan),
    (0x01D300, 0x01D35F, Tai),
    (0x01D360, 0x01D37F, Counting),
    (0x01D400, 0x01D7FF, Mathematical),
    (0x01D800, 0x01DAAF, Sutton),
    (0x01E000, 0x01E02F, Glagolitic),
    (0x01E100, 0x01E14F, Nyiakeng),
    (0x01E2C0, 0x01E2FF, Wancho),
    (0x01E800, 0x01E8DF, Mende),
    (0x01E900, 0x01E95F, Adlam),
    (0x01EC70, 0x01ECBF, Indic),
    (0x01ED00, 0x01ED4F, Ottoman),
    (0x01EE00, 0x01EEFF, Arabic),
    (0x01F000, 0x01F02F, Mahjong),
    (0x01F030, 0x01F09F, Domino),
    (0x01F0A0, 0x01F0FF, Playing),
    (0x01F100, 0x01F2FF, Enclosed),
    (0x01F300, 0x01F5FF, Miscellaneous),
    (0x01F600, 0x01F64F, Emoticons),
    (0x01F650, 0x01F67F, Ornamental),
    (0x01F680, 0x01F6FF, Transport),
    (0x01F700, 0x01F77F, Alchemical),
    (0x01F780, 0x01F7FF, Geometric),
    (0x01F800, 0x01F9FF, Supplemental),
    (0x01FA00, 0x01FA6F, Chess),
    (0x01FA70, 0x01FAFF, Symbols),
    (0x020000, 0x02A6DF, CJK),
    (0x02A700, 0x02EBEF, CJK),
    (0x02F800, 0x02FA1F, CJK),
    (0x0E0000, 0x0E007F, Tags),
    (0x0E0100, 0x0E01EF, Variation),
    (0x0F0000, 0x10FFFF, Supplementary),
];
