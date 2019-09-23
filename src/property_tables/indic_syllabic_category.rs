use crate::lookup_table::LookupTable;

use crate::properties::Indic_Syllabic_Category;
use Indic_Syllabic_Category::*;

impl From<char> for Indic_Syllabic_Category {
    #[inline]
    fn from(c: char) -> Self {
        if c < ROW0_LIMIT {
            return ROW0_TABLE.get_or(&(c as u8), Other);
        }
        if c < PLANE0_LIMIT {
            return PLANE0_TABLE.get_or(&(c as u16), Other);
        }
        return SUPPLEMENTARY_TABLE.get_or(&(c as u32), Other);
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

const ROW0_TABLE: LookupTable<u8, Indic_Syllabic_Category> = lookup_table![
    // So every possible input is always found in the table
    (0x00, 0x2C, Other),
    // Pd       HYPHEN-MINUS
    (0x2D, 0x2D, Consonant_Placeholder),
    // Nd  [10] DIGIT ZERO..DIGIT NINE
    (0x30, 0x39, Number),
    // Zs       NO-BREAK SPACE
    (0xA0, 0xA0, Consonant_Placeholder),
    // No   [2] SUPERSCRIPT TWO..SUPERSCRIPT THREE
    (0xB2, 0xB3, Syllable_Modifier),
    // Sm       MULTIPLICATION SIGN
    (0xD7, 0xD7, Consonant_Placeholder),
];
const ROW0_LIMIT: char = '\u{100}';
const PLANE0_TABLE: LookupTable<u16, Indic_Syllabic_Category> = lookup_table![
    // So every possible input is always found in the table
    (0x0100, 0x08FF, Other),
    // Mn   [3] DEVANAGARI SIGN INVERTED CANDRABINDU..DEVANAGARI SIGN ANUSVARA
    (0x0900, 0x0902, Bindu),
    // Mc       DEVANAGARI SIGN VISARGA
    (0x0903, 0x0903, Visarga),
    // Lo  [17] DEVANAGARI LETTER SHORT A..DEVANAGARI LETTER AU
    (0x0904, 0x0914, Vowel_Independent),
    // Lo  [37] DEVANAGARI LETTER KA..DEVANAGARI LETTER HA
    (0x0915, 0x0939, Consonant),
    // Mn       DEVANAGARI VOWEL SIGN OE
    // Mc       DEVANAGARI VOWEL SIGN OOE
    (0x093A, 0x093B, Vowel_Dependent),
    // Mn       DEVANAGARI SIGN NUKTA
    (0x093C, 0x093C, Nukta),
    // Lo       DEVANAGARI SIGN AVAGRAHA
    (0x093D, 0x093D, Avagraha),
    // Mc   [3] DEVANAGARI VOWEL SIGN AA..DEVANAGARI VOWEL SIGN II
    // Mn   [8] DEVANAGARI VOWEL SIGN U..DEVANAGARI VOWEL SIGN AI
    // Mc   [4] DEVANAGARI VOWEL SIGN CANDRA O..DEVANAGARI VOWEL SIGN AU
    (0x093E, 0x094C, Vowel_Dependent),
    // Mn       DEVANAGARI SIGN VIRAMA
    (0x094D, 0x094D, Virama),
    // Mc   [2] DEVANAGARI VOWEL SIGN PRISHTHAMATRA E..DEVANAGARI VOWEL SIGN AW
    (0x094E, 0x094F, Vowel_Dependent),
    // Mn   [2] DEVANAGARI STRESS SIGN UDATTA..DEVANAGARI STRESS SIGN ANUDATTA
    (0x0951, 0x0952, Cantillation_Mark),
    // Mn   [3] DEVANAGARI VOWEL SIGN CANDRA LONG E..DEVANAGARI VOWEL SIGN UUE
    (0x0955, 0x0957, Vowel_Dependent),
    // Lo   [8] DEVANAGARI LETTER QA..DEVANAGARI LETTER YYA
    (0x0958, 0x095F, Consonant),
    // Lo   [2] DEVANAGARI LETTER VOCALIC RR..DEVANAGARI LETTER VOCALIC LL
    (0x0960, 0x0961, Vowel_Independent),
    // Mn   [2] DEVANAGARI VOWEL SIGN VOCALIC L..DEVANAGARI VOWEL SIGN VOCALIC LL
    (0x0962, 0x0963, Vowel_Dependent),
    // Nd  [10] DEVANAGARI DIGIT ZERO..DEVANAGARI DIGIT NINE
    (0x0966, 0x096F, Number),
    // Lo   [6] DEVANAGARI LETTER CANDRA A..DEVANAGARI LETTER UUE
    (0x0972, 0x0977, Vowel_Independent),
    // Lo   [8] DEVANAGARI LETTER MARWARI DDA..DEVANAGARI LETTER BBA
    (0x0978, 0x097F, Consonant),
    // Lo       BENGALI ANJI
    (0x0980, 0x0980, Consonant_Placeholder),
    // Mn       BENGALI SIGN CANDRABINDU
    // Mc       BENGALI SIGN ANUSVARA
    (0x0981, 0x0982, Bindu),
    // Mc       BENGALI SIGN VISARGA
    (0x0983, 0x0983, Visarga),
    // Lo   [8] BENGALI LETTER A..BENGALI LETTER VOCALIC L
    (0x0985, 0x098C, Vowel_Independent),
    // Lo   [2] BENGALI LETTER E..BENGALI LETTER AI
    (0x098F, 0x0990, Vowel_Independent),
    // Lo   [2] BENGALI LETTER O..BENGALI LETTER AU
    (0x0993, 0x0994, Vowel_Independent),
    // Lo  [20] BENGALI LETTER KA..BENGALI LETTER NA
    (0x0995, 0x09A8, Consonant),
    // Lo   [7] BENGALI LETTER PA..BENGALI LETTER RA
    (0x09AA, 0x09B0, Consonant),
    // Lo       BENGALI LETTER LA
    (0x09B2, 0x09B2, Consonant),
    // Lo   [4] BENGALI LETTER SHA..BENGALI LETTER HA
    (0x09B6, 0x09B9, Consonant),
    // Mn       BENGALI SIGN NUKTA
    (0x09BC, 0x09BC, Nukta),
    // Lo       BENGALI SIGN AVAGRAHA
    (0x09BD, 0x09BD, Avagraha),
    // Mc   [3] BENGALI VOWEL SIGN AA..BENGALI VOWEL SIGN II
    // Mn   [4] BENGALI VOWEL SIGN U..BENGALI VOWEL SIGN VOCALIC RR
    (0x09BE, 0x09C4, Vowel_Dependent),
    // Mc   [2] BENGALI VOWEL SIGN E..BENGALI VOWEL SIGN AI
    (0x09C7, 0x09C8, Vowel_Dependent),
    // Mc   [2] BENGALI VOWEL SIGN O..BENGALI VOWEL SIGN AU
    (0x09CB, 0x09CC, Vowel_Dependent),
    // Mn       BENGALI SIGN VIRAMA
    (0x09CD, 0x09CD, Virama),
    // Lo       BENGALI LETTER KHANDA TA
    (0x09CE, 0x09CE, Consonant_Dead),
    // Mc       BENGALI AU LENGTH MARK
    (0x09D7, 0x09D7, Vowel_Dependent),
    // Lo   [2] BENGALI LETTER RRA..BENGALI LETTER RHA
    (0x09DC, 0x09DD, Consonant),
    // Lo       BENGALI LETTER YYA
    (0x09DF, 0x09DF, Consonant),
    // Lo   [2] BENGALI LETTER VOCALIC RR..BENGALI LETTER VOCALIC LL
    (0x09E0, 0x09E1, Vowel_Independent),
    // Mn   [2] BENGALI VOWEL SIGN VOCALIC L..BENGALI VOWEL SIGN VOCALIC LL
    (0x09E2, 0x09E3, Vowel_Dependent),
    // Nd  [10] BENGALI DIGIT ZERO..BENGALI DIGIT NINE
    (0x09E6, 0x09EF, Number),
    // Lo   [2] BENGALI LETTER RA WITH MIDDLE DIAGONAL..BENGALI LETTER RA WITH LOWER DIAGONAL
    (0x09F0, 0x09F1, Consonant),
    // Lo       BENGALI LETTER VEDIC ANUSVARA
    (0x09FC, 0x09FC, Bindu),
    // Mn       BENGALI SANDHI MARK
    (0x09FE, 0x09FE, Syllable_Modifier),
    // Mn   [2] GURMUKHI SIGN ADAK BINDI..GURMUKHI SIGN BINDI
    (0x0A01, 0x0A02, Bindu),
    // Mc       GURMUKHI SIGN VISARGA
    (0x0A03, 0x0A03, Visarga),
    // Lo   [6] GURMUKHI LETTER A..GURMUKHI LETTER UU
    (0x0A05, 0x0A0A, Vowel_Independent),
    // Lo   [2] GURMUKHI LETTER EE..GURMUKHI LETTER AI
    (0x0A0F, 0x0A10, Vowel_Independent),
    // Lo   [2] GURMUKHI LETTER OO..GURMUKHI LETTER AU
    (0x0A13, 0x0A14, Vowel_Independent),
    // Lo  [20] GURMUKHI LETTER KA..GURMUKHI LETTER NA
    (0x0A15, 0x0A28, Consonant),
    // Lo   [7] GURMUKHI LETTER PA..GURMUKHI LETTER RA
    (0x0A2A, 0x0A30, Consonant),
    // Lo   [2] GURMUKHI LETTER LA..GURMUKHI LETTER LLA
    (0x0A32, 0x0A33, Consonant),
    // Lo   [2] GURMUKHI LETTER VA..GURMUKHI LETTER SHA
    (0x0A35, 0x0A36, Consonant),
    // Lo   [2] GURMUKHI LETTER SA..GURMUKHI LETTER HA
    (0x0A38, 0x0A39, Consonant),
    // Mn       GURMUKHI SIGN NUKTA
    (0x0A3C, 0x0A3C, Nukta),
    // Mc   [3] GURMUKHI VOWEL SIGN AA..GURMUKHI VOWEL SIGN II
    // Mn   [2] GURMUKHI VOWEL SIGN U..GURMUKHI VOWEL SIGN UU
    (0x0A3E, 0x0A42, Vowel_Dependent),
    // Mn   [2] GURMUKHI VOWEL SIGN EE..GURMUKHI VOWEL SIGN AI
    (0x0A47, 0x0A48, Vowel_Dependent),
    // Mn   [2] GURMUKHI VOWEL SIGN OO..GURMUKHI VOWEL SIGN AU
    (0x0A4B, 0x0A4C, Vowel_Dependent),
    // Mn       GURMUKHI SIGN VIRAMA
    (0x0A4D, 0x0A4D, Virama),
    // Mn       GURMUKHI SIGN UDAAT
    (0x0A51, 0x0A51, Cantillation_Mark),
    // Lo   [4] GURMUKHI LETTER KHHA..GURMUKHI LETTER RRA
    (0x0A59, 0x0A5C, Consonant),
    // Lo       GURMUKHI LETTER FA
    (0x0A5E, 0x0A5E, Consonant),
    // Nd  [10] GURMUKHI DIGIT ZERO..GURMUKHI DIGIT NINE
    (0x0A66, 0x0A6F, Number),
    // Mn       GURMUKHI TIPPI
    (0x0A70, 0x0A70, Bindu),
    // Mn       GURMUKHI ADDAK
    (0x0A71, 0x0A71, Gemination_Mark),
    // Lo   [2] GURMUKHI IRI..GURMUKHI URA
    (0x0A72, 0x0A73, Consonant_Placeholder),
    // Mn       GURMUKHI SIGN YAKASH
    (0x0A75, 0x0A75, Consonant_Medial),
    // Mn   [2] GUJARATI SIGN CANDRABINDU..GUJARATI SIGN ANUSVARA
    (0x0A81, 0x0A82, Bindu),
    // Mc       GUJARATI SIGN VISARGA
    (0x0A83, 0x0A83, Visarga),
    // Lo   [9] GUJARATI LETTER A..GUJARATI VOWEL CANDRA E
    (0x0A85, 0x0A8D, Vowel_Independent),
    // Lo   [3] GUJARATI LETTER E..GUJARATI VOWEL CANDRA O
    (0x0A8F, 0x0A91, Vowel_Independent),
    // Lo   [2] GUJARATI LETTER O..GUJARATI LETTER AU
    (0x0A93, 0x0A94, Vowel_Independent),
    // Lo  [20] GUJARATI LETTER KA..GUJARATI LETTER NA
    (0x0A95, 0x0AA8, Consonant),
    // Lo   [7] GUJARATI LETTER PA..GUJARATI LETTER RA
    (0x0AAA, 0x0AB0, Consonant),
    // Lo   [2] GUJARATI LETTER LA..GUJARATI LETTER LLA
    (0x0AB2, 0x0AB3, Consonant),
    // Lo   [5] GUJARATI LETTER VA..GUJARATI LETTER HA
    (0x0AB5, 0x0AB9, Consonant),
    // Mn       GUJARATI SIGN NUKTA
    (0x0ABC, 0x0ABC, Nukta),
    // Lo       GUJARATI SIGN AVAGRAHA
    (0x0ABD, 0x0ABD, Avagraha),
    // Mc   [3] GUJARATI VOWEL SIGN AA..GUJARATI VOWEL SIGN II
    // Mn   [5] GUJARATI VOWEL SIGN U..GUJARATI VOWEL SIGN CANDRA E
    (0x0ABE, 0x0AC5, Vowel_Dependent),
    // Mn   [2] GUJARATI VOWEL SIGN E..GUJARATI VOWEL SIGN AI
    // Mc       GUJARATI VOWEL SIGN CANDRA O
    (0x0AC7, 0x0AC9, Vowel_Dependent),
    // Mc   [2] GUJARATI VOWEL SIGN O..GUJARATI VOWEL SIGN AU
    (0x0ACB, 0x0ACC, Vowel_Dependent),
    // Mn       GUJARATI SIGN VIRAMA
    (0x0ACD, 0x0ACD, Virama),
    // Lo   [2] GUJARATI LETTER VOCALIC RR..GUJARATI LETTER VOCALIC LL
    (0x0AE0, 0x0AE1, Vowel_Independent),
    // Mn   [2] GUJARATI VOWEL SIGN VOCALIC L..GUJARATI VOWEL SIGN VOCALIC LL
    (0x0AE2, 0x0AE3, Vowel_Dependent),
    // Nd  [10] GUJARATI DIGIT ZERO..GUJARATI DIGIT NINE
    (0x0AE6, 0x0AEF, Number),
    // Lo       GUJARATI LETTER ZHA
    (0x0AF9, 0x0AF9, Consonant),
    // Mn   [3] GUJARATI SIGN SUKUN..GUJARATI SIGN MADDAH
    (0x0AFA, 0x0AFC, Cantillation_Mark),
    // Mn   [3] GUJARATI SIGN THREE-DOT NUKTA ABOVE..GUJARATI SIGN TWO-CIRCLE NUKTA ABOVE
    (0x0AFD, 0x0AFF, Nukta),
    // Mn       ORIYA SIGN CANDRABINDU
    // Mc       ORIYA SIGN ANUSVARA
    (0x0B01, 0x0B02, Bindu),
    // Mc       ORIYA SIGN VISARGA
    (0x0B03, 0x0B03, Visarga),
    // Lo   [8] ORIYA LETTER A..ORIYA LETTER VOCALIC L
    (0x0B05, 0x0B0C, Vowel_Independent),
    // Lo   [2] ORIYA LETTER E..ORIYA LETTER AI
    (0x0B0F, 0x0B10, Vowel_Independent),
    // Lo   [2] ORIYA LETTER O..ORIYA LETTER AU
    (0x0B13, 0x0B14, Vowel_Independent),
    // Lo  [20] ORIYA LETTER KA..ORIYA LETTER NA
    (0x0B15, 0x0B28, Consonant),
    // Lo   [7] ORIYA LETTER PA..ORIYA LETTER RA
    (0x0B2A, 0x0B30, Consonant),
    // Lo   [2] ORIYA LETTER LA..ORIYA LETTER LLA
    (0x0B32, 0x0B33, Consonant),
    // Lo   [5] ORIYA LETTER VA..ORIYA LETTER HA
    (0x0B35, 0x0B39, Consonant),
    // Mn       ORIYA SIGN NUKTA
    (0x0B3C, 0x0B3C, Nukta),
    // Lo       ORIYA SIGN AVAGRAHA
    (0x0B3D, 0x0B3D, Avagraha),
    // Mc       ORIYA VOWEL SIGN AA
    // Mn       ORIYA VOWEL SIGN I
    // Mc       ORIYA VOWEL SIGN II
    // Mn   [4] ORIYA VOWEL SIGN U..ORIYA VOWEL SIGN VOCALIC RR
    (0x0B3E, 0x0B44, Vowel_Dependent),
    // Mc   [2] ORIYA VOWEL SIGN E..ORIYA VOWEL SIGN AI
    (0x0B47, 0x0B48, Vowel_Dependent),
    // Mc   [2] ORIYA VOWEL SIGN O..ORIYA VOWEL SIGN AU
    (0x0B4B, 0x0B4C, Vowel_Dependent),
    // Mn       ORIYA SIGN VIRAMA
    (0x0B4D, 0x0B4D, Virama),
    // Mn       ORIYA AI LENGTH MARK
    // Mc       ORIYA AU LENGTH MARK
    (0x0B56, 0x0B57, Vowel_Dependent),
    // Lo   [2] ORIYA LETTER RRA..ORIYA LETTER RHA
    (0x0B5C, 0x0B5D, Consonant),
    // Lo       ORIYA LETTER YYA
    (0x0B5F, 0x0B5F, Consonant),
    // Lo   [2] ORIYA LETTER VOCALIC RR..ORIYA LETTER VOCALIC LL
    (0x0B60, 0x0B61, Vowel_Independent),
    // Mn   [2] ORIYA VOWEL SIGN VOCALIC L..ORIYA VOWEL SIGN VOCALIC LL
    (0x0B62, 0x0B63, Vowel_Dependent),
    // Nd  [10] ORIYA DIGIT ZERO..ORIYA DIGIT NINE
    (0x0B66, 0x0B6F, Number),
    // Lo       ORIYA LETTER WA
    (0x0B71, 0x0B71, Consonant),
    // Mn       TAMIL SIGN ANUSVARA
    (0x0B82, 0x0B82, Bindu),
    // Lo       TAMIL SIGN VISARGA
    (0x0B83, 0x0B83, Modifying_Letter),
    // Lo   [6] TAMIL LETTER A..TAMIL LETTER UU
    (0x0B85, 0x0B8A, Vowel_Independent),
    // Lo   [3] TAMIL LETTER E..TAMIL LETTER AI
    (0x0B8E, 0x0B90, Vowel_Independent),
    // Lo   [3] TAMIL LETTER O..TAMIL LETTER AU
    (0x0B92, 0x0B94, Vowel_Independent),
    // Lo       TAMIL LETTER KA
    (0x0B95, 0x0B95, Consonant),
    // Lo   [2] TAMIL LETTER NGA..TAMIL LETTER CA
    (0x0B99, 0x0B9A, Consonant),
    // Lo       TAMIL LETTER JA
    (0x0B9C, 0x0B9C, Consonant),
    // Lo   [2] TAMIL LETTER NYA..TAMIL LETTER TTA
    (0x0B9E, 0x0B9F, Consonant),
    // Lo   [2] TAMIL LETTER NNA..TAMIL LETTER TA
    (0x0BA3, 0x0BA4, Consonant),
    // Lo   [3] TAMIL LETTER NA..TAMIL LETTER PA
    (0x0BA8, 0x0BAA, Consonant),
    // Lo  [12] TAMIL LETTER MA..TAMIL LETTER HA
    (0x0BAE, 0x0BB9, Consonant),
    // Mc   [2] TAMIL VOWEL SIGN AA..TAMIL VOWEL SIGN I
    // Mn       TAMIL VOWEL SIGN II
    // Mc   [2] TAMIL VOWEL SIGN U..TAMIL VOWEL SIGN UU
    (0x0BBE, 0x0BC2, Vowel_Dependent),
    // Mc   [3] TAMIL VOWEL SIGN E..TAMIL VOWEL SIGN AI
    (0x0BC6, 0x0BC8, Vowel_Dependent),
    // Mc   [3] TAMIL VOWEL SIGN O..TAMIL VOWEL SIGN AU
    (0x0BCA, 0x0BCC, Vowel_Dependent),
    // Mn       TAMIL SIGN VIRAMA
    (0x0BCD, 0x0BCD, Virama),
    // Mc       TAMIL AU LENGTH MARK
    (0x0BD7, 0x0BD7, Vowel_Dependent),
    // Nd  [10] TAMIL DIGIT ZERO..TAMIL DIGIT NINE
    (0x0BE6, 0x0BEF, Number),
    // Mn       TELUGU SIGN COMBINING CANDRABINDU ABOVE
    // Mc   [2] TELUGU SIGN CANDRABINDU..TELUGU SIGN ANUSVARA
    (0x0C00, 0x0C02, Bindu),
    // Mc       TELUGU SIGN VISARGA
    (0x0C03, 0x0C03, Visarga),
    // Mn       TELUGU SIGN COMBINING ANUSVARA ABOVE
    (0x0C04, 0x0C04, Bindu),
    // Lo   [8] TELUGU LETTER A..TELUGU LETTER VOCALIC L
    (0x0C05, 0x0C0C, Vowel_Independent),
    // Lo   [3] TELUGU LETTER E..TELUGU LETTER AI
    (0x0C0E, 0x0C10, Vowel_Independent),
    // Lo   [3] TELUGU LETTER O..TELUGU LETTER AU
    (0x0C12, 0x0C14, Vowel_Independent),
    // Lo  [20] TELUGU LETTER KA..TELUGU LETTER NA
    (0x0C15, 0x0C28, Consonant),
    // Lo  [16] TELUGU LETTER PA..TELUGU LETTER HA
    (0x0C2A, 0x0C39, Consonant),
    // Lo       TELUGU SIGN AVAGRAHA
    (0x0C3D, 0x0C3D, Avagraha),
    // Mn   [3] TELUGU VOWEL SIGN AA..TELUGU VOWEL SIGN II
    // Mc   [4] TELUGU VOWEL SIGN U..TELUGU VOWEL SIGN VOCALIC RR
    (0x0C3E, 0x0C44, Vowel_Dependent),
    // Mn   [3] TELUGU VOWEL SIGN E..TELUGU VOWEL SIGN AI
    (0x0C46, 0x0C48, Vowel_Dependent),
    // Mn   [3] TELUGU VOWEL SIGN O..TELUGU VOWEL SIGN AU
    (0x0C4A, 0x0C4C, Vowel_Dependent),
    // Mn       TELUGU SIGN VIRAMA
    (0x0C4D, 0x0C4D, Virama),
    // Mn   [2] TELUGU LENGTH MARK..TELUGU AI LENGTH MARK
    (0x0C55, 0x0C56, Vowel_Dependent),
    // Lo   [3] TELUGU LETTER TSA..TELUGU LETTER RRRA
    (0x0C58, 0x0C5A, Consonant),
    // Lo   [2] TELUGU LETTER VOCALIC RR..TELUGU LETTER VOCALIC LL
    (0x0C60, 0x0C61, Vowel_Independent),
    // Mn   [2] TELUGU VOWEL SIGN VOCALIC L..TELUGU VOWEL SIGN VOCALIC LL
    (0x0C62, 0x0C63, Vowel_Dependent),
    // Nd  [10] TELUGU DIGIT ZERO..TELUGU DIGIT NINE
    (0x0C66, 0x0C6F, Number),
    // Lo       KANNADA SIGN SPACING CANDRABINDU
    // Mn       KANNADA SIGN CANDRABINDU
    // Mc       KANNADA SIGN ANUSVARA
    (0x0C80, 0x0C82, Bindu),
    // Mc       KANNADA SIGN VISARGA
    (0x0C83, 0x0C83, Visarga),
    // Lo   [8] KANNADA LETTER A..KANNADA LETTER VOCALIC L
    (0x0C85, 0x0C8C, Vowel_Independent),
    // Lo   [3] KANNADA LETTER E..KANNADA LETTER AI
    (0x0C8E, 0x0C90, Vowel_Independent),
    // Lo   [3] KANNADA LETTER O..KANNADA LETTER AU
    (0x0C92, 0x0C94, Vowel_Independent),
    // Lo  [20] KANNADA LETTER KA..KANNADA LETTER NA
    (0x0C95, 0x0CA8, Consonant),
    // Lo  [10] KANNADA LETTER PA..KANNADA LETTER LLA
    (0x0CAA, 0x0CB3, Consonant),
    // Lo   [5] KANNADA LETTER VA..KANNADA LETTER HA
    (0x0CB5, 0x0CB9, Consonant),
    // Mn       KANNADA SIGN NUKTA
    (0x0CBC, 0x0CBC, Nukta),
    // Lo       KANNADA SIGN AVAGRAHA
    (0x0CBD, 0x0CBD, Avagraha),
    // Mc       KANNADA VOWEL SIGN AA
    // Mn       KANNADA VOWEL SIGN I
    // Mc   [5] KANNADA VOWEL SIGN II..KANNADA VOWEL SIGN VOCALIC RR
    (0x0CBE, 0x0CC4, Vowel_Dependent),
    // Mn       KANNADA VOWEL SIGN E
    // Mc   [2] KANNADA VOWEL SIGN EE..KANNADA VOWEL SIGN AI
    (0x0CC6, 0x0CC8, Vowel_Dependent),
    // Mc   [2] KANNADA VOWEL SIGN O..KANNADA VOWEL SIGN OO
    // Mn       KANNADA VOWEL SIGN AU
    (0x0CCA, 0x0CCC, Vowel_Dependent),
    // Mn       KANNADA SIGN VIRAMA
    (0x0CCD, 0x0CCD, Virama),
    // Mc   [2] KANNADA LENGTH MARK..KANNADA AI LENGTH MARK
    (0x0CD5, 0x0CD6, Vowel_Dependent),
    // Lo       KANNADA LETTER FA
    (0x0CDE, 0x0CDE, Consonant),
    // Lo   [2] KANNADA LETTER VOCALIC RR..KANNADA LETTER VOCALIC LL
    (0x0CE0, 0x0CE1, Vowel_Independent),
    // Mn   [2] KANNADA VOWEL SIGN VOCALIC L..KANNADA VOWEL SIGN VOCALIC LL
    (0x0CE2, 0x0CE3, Vowel_Dependent),
    // Nd  [10] KANNADA DIGIT ZERO..KANNADA DIGIT NINE
    (0x0CE6, 0x0CEF, Number),
    // Lo   [2] KANNADA SIGN JIHVAMULIYA..KANNADA SIGN UPADHMANIYA
    (0x0CF1, 0x0CF2, Consonant_With_Stacker),
    // Mn   [2] MALAYALAM SIGN COMBINING ANUSVARA ABOVE..MALAYALAM SIGN CANDRABINDU
    // Mc       MALAYALAM SIGN ANUSVARA
    (0x0D00, 0x0D02, Bindu),
    // Mc       MALAYALAM SIGN VISARGA
    (0x0D03, 0x0D03, Visarga),
    // Lo   [8] MALAYALAM LETTER A..MALAYALAM LETTER VOCALIC L
    (0x0D05, 0x0D0C, Vowel_Independent),
    // Lo   [3] MALAYALAM LETTER E..MALAYALAM LETTER AI
    (0x0D0E, 0x0D10, Vowel_Independent),
    // Lo   [3] MALAYALAM LETTER O..MALAYALAM LETTER AU
    (0x0D12, 0x0D14, Vowel_Independent),
    // Lo  [38] MALAYALAM LETTER KA..MALAYALAM LETTER TTTA
    (0x0D15, 0x0D3A, Consonant),
    // Mn   [2] MALAYALAM SIGN VERTICAL BAR VIRAMA..MALAYALAM SIGN CIRCULAR VIRAMA
    (0x0D3B, 0x0D3C, Pure_Killer),
    // Lo       MALAYALAM SIGN AVAGRAHA
    (0x0D3D, 0x0D3D, Avagraha),
    // Mc   [3] MALAYALAM VOWEL SIGN AA..MALAYALAM VOWEL SIGN II
    // Mn   [4] MALAYALAM VOWEL SIGN U..MALAYALAM VOWEL SIGN VOCALIC RR
    (0x0D3E, 0x0D44, Vowel_Dependent),
    // Mc   [3] MALAYALAM VOWEL SIGN E..MALAYALAM VOWEL SIGN AI
    (0x0D46, 0x0D48, Vowel_Dependent),
    // Mc   [3] MALAYALAM VOWEL SIGN O..MALAYALAM VOWEL SIGN AU
    (0x0D4A, 0x0D4C, Vowel_Dependent),
    // Mn       MALAYALAM SIGN VIRAMA
    (0x0D4D, 0x0D4D, Virama),
    // Lo       MALAYALAM LETTER DOT REPH
    (0x0D4E, 0x0D4E, Consonant_Preceding_Repha),
    // Lo   [3] MALAYALAM LETTER CHILLU M..MALAYALAM LETTER CHILLU LLL
    (0x0D54, 0x0D56, Consonant_Dead),
    // Mc       MALAYALAM AU LENGTH MARK
    (0x0D57, 0x0D57, Vowel_Dependent),
    // Lo   [3] MALAYALAM LETTER ARCHAIC II..MALAYALAM LETTER VOCALIC LL
    (0x0D5F, 0x0D61, Vowel_Independent),
    // Mn   [2] MALAYALAM VOWEL SIGN VOCALIC L..MALAYALAM VOWEL SIGN VOCALIC LL
    (0x0D62, 0x0D63, Vowel_Dependent),
    // Nd  [10] MALAYALAM DIGIT ZERO..MALAYALAM DIGIT NINE
    (0x0D66, 0x0D6F, Number),
    // Lo   [6] MALAYALAM LETTER CHILLU NN..MALAYALAM LETTER CHILLU K
    (0x0D7A, 0x0D7F, Consonant_Dead),
    // Mc       SINHALA SIGN ANUSVARAYA
    (0x0D82, 0x0D82, Bindu),
    // Mc       SINHALA SIGN VISARGAYA
    (0x0D83, 0x0D83, Visarga),
    // Lo  [18] SINHALA LETTER AYANNA..SINHALA LETTER AUYANNA
    (0x0D85, 0x0D96, Vowel_Independent),
    // Lo  [24] SINHALA LETTER ALPAPRAANA KAYANNA..SINHALA LETTER DANTAJA NAYANNA
    (0x0D9A, 0x0DB1, Consonant),
    // Lo   [9] SINHALA LETTER SANYAKA DAYANNA..SINHALA LETTER RAYANNA
    (0x0DB3, 0x0DBB, Consonant),
    // Lo       SINHALA LETTER DANTAJA LAYANNA
    (0x0DBD, 0x0DBD, Consonant),
    // Lo   [7] SINHALA LETTER VAYANNA..SINHALA LETTER FAYANNA
    (0x0DC0, 0x0DC6, Consonant),
    // Mn       SINHALA SIGN AL-LAKUNA
    (0x0DCA, 0x0DCA, Virama),
    // Mc   [3] SINHALA VOWEL SIGN AELA-PILLA..SINHALA VOWEL SIGN DIGA AEDA-PILLA
    // Mn   [3] SINHALA VOWEL SIGN KETTI IS-PILLA..SINHALA VOWEL SIGN KETTI PAA-PILLA
    (0x0DCF, 0x0DD4, Vowel_Dependent),
    // Mn       SINHALA VOWEL SIGN DIGA PAA-PILLA
    (0x0DD6, 0x0DD6, Vowel_Dependent),
    // Mc   [8] SINHALA VOWEL SIGN GAETTA-PILLA..SINHALA VOWEL SIGN GAYANUKITTA
    (0x0DD8, 0x0DDF, Vowel_Dependent),
    // Nd  [10] SINHALA LITH DIGIT ZERO..SINHALA LITH DIGIT NINE
    (0x0DE6, 0x0DEF, Number),
    // Mc   [2] SINHALA VOWEL SIGN DIGA GAETTA-PILLA..SINHALA VOWEL SIGN DIGA GAYANUKITTA
    (0x0DF2, 0x0DF3, Vowel_Dependent),
    // Lo  [46] THAI CHARACTER KO KAI..THAI CHARACTER HO NOKHUK
    (0x0E01, 0x0E2E, Consonant),
    // Lo       THAI CHARACTER SARA A
    // Mn       THAI CHARACTER MAI HAN-AKAT
    // Lo   [2] THAI CHARACTER SARA AA..THAI CHARACTER SARA AM
    // Mn   [6] THAI CHARACTER SARA I..THAI CHARACTER SARA UU
    (0x0E30, 0x0E39, Vowel_Dependent),
    // Mn       THAI CHARACTER PHINTHU
    (0x0E3A, 0x0E3A, Pure_Killer),
    // Lo   [6] THAI CHARACTER SARA E..THAI CHARACTER LAKKHANGYAO
    (0x0E40, 0x0E45, Vowel_Dependent),
    // Mn       THAI CHARACTER MAITAIKHU
    (0x0E47, 0x0E47, Vowel_Dependent),
    // Mn   [4] THAI CHARACTER MAI EK..THAI CHARACTER MAI CHATTAWA
    (0x0E48, 0x0E4B, Tone_Mark),
    // Mn       THAI CHARACTER THANTHAKHAT
    (0x0E4C, 0x0E4C, Consonant_Killer),
    // Mn       THAI CHARACTER NIKHAHIT
    (0x0E4D, 0x0E4D, Bindu),
    // Mn       THAI CHARACTER YAMAKKAN
    (0x0E4E, 0x0E4E, Pure_Killer),
    // Nd  [10] THAI DIGIT ZERO..THAI DIGIT NINE
    (0x0E50, 0x0E59, Number),
    // Lo   [2] LAO LETTER KO..LAO LETTER KHO SUNG
    (0x0E81, 0x0E82, Consonant),
    // Lo       LAO LETTER KHO TAM
    (0x0E84, 0x0E84, Consonant),
    // Lo   [5] LAO LETTER PALI GHA..LAO LETTER SO TAM
    (0x0E86, 0x0E8A, Consonant),
    // Lo  [24] LAO LETTER PALI JHA..LAO LETTER LO LING
    (0x0E8C, 0x0EA3, Consonant),
    // Lo       LAO LETTER LO LOOT
    (0x0EA5, 0x0EA5, Consonant),
    // Lo   [8] LAO LETTER WO..LAO LETTER HO TAM
    (0x0EA7, 0x0EAE, Consonant),
    // Lo       LAO VOWEL SIGN A
    // Mn       LAO VOWEL SIGN MAI KAN
    // Lo   [2] LAO VOWEL SIGN AA..LAO VOWEL SIGN AM
    // Mn   [6] LAO VOWEL SIGN I..LAO VOWEL SIGN UU
    (0x0EB0, 0x0EB9, Vowel_Dependent),
    // Mn       LAO SIGN PALI VIRAMA
    (0x0EBA, 0x0EBA, Pure_Killer),
    // Mn       LAO VOWEL SIGN MAI KON
    (0x0EBB, 0x0EBB, Vowel_Dependent),
    // Mn       LAO SEMIVOWEL SIGN LO
    // Lo       LAO SEMIVOWEL SIGN NYO
    (0x0EBC, 0x0EBD, Consonant_Medial),
    // Lo   [5] LAO VOWEL SIGN E..LAO VOWEL SIGN AI
    (0x0EC0, 0x0EC4, Vowel_Dependent),
    // Mn   [4] LAO TONE MAI EK..LAO TONE MAI CATAWA
    (0x0EC8, 0x0ECB, Tone_Mark),
    // Mn       LAO NIGGAHITA
    (0x0ECD, 0x0ECD, Bindu),
    // Nd  [10] LAO DIGIT ZERO..LAO DIGIT NINE
    (0x0ED0, 0x0ED9, Number),
    // Lo   [4] LAO HO NO..LAO LETTER KHMU NYO
    (0x0EDC, 0x0EDF, Consonant),
    // Nd  [10] TIBETAN DIGIT ZERO..TIBETAN DIGIT NINE
    // No  [10] TIBETAN DIGIT HALF ONE..TIBETAN DIGIT HALF ZERO
    (0x0F20, 0x0F33, Number),
    // Mn       TIBETAN MARK NGAS BZUNG NYI ZLA
    (0x0F35, 0x0F35, Syllable_Modifier),
    // Mn       TIBETAN MARK NGAS BZUNG SGOR RTAGS
    (0x0F37, 0x0F37, Syllable_Modifier),
    // Mn       TIBETAN MARK TSA -PHRU
    (0x0F39, 0x0F39, Nukta),
    // Lo   [8] TIBETAN LETTER KA..TIBETAN LETTER JA
    (0x0F40, 0x0F47, Consonant),
    // Lo  [36] TIBETAN LETTER NYA..TIBETAN LETTER RRA
    (0x0F49, 0x0F6C, Consonant),
    // Mn  [13] TIBETAN VOWEL SIGN AA..TIBETAN VOWEL SIGN OO
    (0x0F71, 0x0F7D, Vowel_Dependent),
    // Mn       TIBETAN SIGN RJES SU NGA RO
    (0x0F7E, 0x0F7E, Bindu),
    // Mc       TIBETAN SIGN RNAM BCAD
    (0x0F7F, 0x0F7F, Visarga),
    // Mn   [2] TIBETAN VOWEL SIGN REVERSED I..TIBETAN VOWEL SIGN REVERSED II
    (0x0F80, 0x0F81, Vowel_Dependent),
    // Mn   [2] TIBETAN SIGN NYI ZLA NAA DA..TIBETAN SIGN SNA LDAN
    (0x0F82, 0x0F83, Bindu),
    // Mn       TIBETAN MARK HALANTA
    (0x0F84, 0x0F84, Pure_Killer),
    // Po       TIBETAN MARK PALUTA
    (0x0F85, 0x0F85, Avagraha),
    // Lo   [5] TIBETAN SIGN LCE TSA CAN..TIBETAN SIGN INVERTED MCHU CAN
    (0x0F88, 0x0F8C, Consonant_Head_Letter),
    // Mn  [11] TIBETAN SUBJOINED SIGN LCE TSA CAN..TIBETAN SUBJOINED LETTER JA
    (0x0F8D, 0x0F97, Consonant_Subjoined),
    // Mn  [36] TIBETAN SUBJOINED LETTER NYA..TIBETAN SUBJOINED LETTER FIXED-FORM RA
    (0x0F99, 0x0FBC, Consonant_Subjoined),
    // Mn       TIBETAN SYMBOL PADMA GDAN
    (0x0FC6, 0x0FC6, Syllable_Modifier),
    // Lo  [33] MYANMAR LETTER KA..MYANMAR LETTER LLA
    (0x1000, 0x1020, Consonant),
    // Lo  [10] MYANMAR LETTER A..MYANMAR LETTER AU
    (0x1021, 0x102A, Vowel_Independent),
    // Mc   [2] MYANMAR VOWEL SIGN TALL AA..MYANMAR VOWEL SIGN AA
    // Mn   [4] MYANMAR VOWEL SIGN I..MYANMAR VOWEL SIGN UU
    // Mc       MYANMAR VOWEL SIGN E
    // Mn   [4] MYANMAR VOWEL SIGN AI..MYANMAR VOWEL SIGN E ABOVE
    (0x102B, 0x1035, Vowel_Dependent),
    // Mn       MYANMAR SIGN ANUSVARA
    (0x1036, 0x1036, Bindu),
    // Mn       MYANMAR SIGN DOT BELOW
    (0x1037, 0x1037, Tone_Mark),
    // Mc       MYANMAR SIGN VISARGA
    (0x1038, 0x1038, Visarga),
    // Mn       MYANMAR SIGN VIRAMA
    (0x1039, 0x1039, Invisible_Stacker),
    // Mn       MYANMAR SIGN ASAT
    (0x103A, 0x103A, Pure_Killer),
    // Mc   [2] MYANMAR CONSONANT SIGN MEDIAL YA..MYANMAR CONSONANT SIGN MEDIAL RA
    // Mn   [2] MYANMAR CONSONANT SIGN MEDIAL WA..MYANMAR CONSONANT SIGN MEDIAL HA
    (0x103B, 0x103E, Consonant_Medial),
    // Lo       MYANMAR LETTER GREAT SA
    (0x103F, 0x103F, Consonant),
    // Nd  [10] MYANMAR DIGIT ZERO..MYANMAR DIGIT NINE
    (0x1040, 0x1049, Number),
    // Po       MYANMAR SIGN SECTION
    (0x104B, 0x104B, Consonant_Placeholder),
    // Po       MYANMAR SYMBOL AFOREMENTIONED
    (0x104E, 0x104E, Consonant_Placeholder),
    // Lo   [2] MYANMAR LETTER SHA..MYANMAR LETTER SSA
    (0x1050, 0x1051, Consonant),
    // Lo   [4] MYANMAR LETTER VOCALIC R..MYANMAR LETTER VOCALIC LL
    (0x1052, 0x1055, Vowel_Independent),
    // Mc   [2] MYANMAR VOWEL SIGN VOCALIC R..MYANMAR VOWEL SIGN VOCALIC RR
    // Mn   [2] MYANMAR VOWEL SIGN VOCALIC L..MYANMAR VOWEL SIGN VOCALIC LL
    (0x1056, 0x1059, Vowel_Dependent),
    // Lo   [4] MYANMAR LETTER MON NGA..MYANMAR LETTER MON BBE
    (0x105A, 0x105D, Consonant),
    // Mn   [3] MYANMAR CONSONANT SIGN MON MEDIAL NA..MYANMAR CONSONANT SIGN MON MEDIAL LA
    (0x105E, 0x1060, Consonant_Medial),
    // Lo       MYANMAR LETTER SGAW KAREN SHA
    (0x1061, 0x1061, Consonant),
    // Mc       MYANMAR VOWEL SIGN SGAW KAREN EU
    (0x1062, 0x1062, Vowel_Dependent),
    // Mc   [2] MYANMAR TONE MARK SGAW KAREN HATHI..MYANMAR TONE MARK SGAW KAREN KE PHO
    (0x1063, 0x1064, Tone_Mark),
    // Lo   [2] MYANMAR LETTER WESTERN PWO KAREN THA..MYANMAR LETTER WESTERN PWO KAREN PWA
    (0x1065, 0x1066, Consonant),
    // Mc   [2] MYANMAR VOWEL SIGN WESTERN PWO KAREN EU..MYANMAR VOWEL SIGN WESTERN PWO KAREN UE
    (0x1067, 0x1068, Vowel_Dependent),
    // Mc   [5] MYANMAR SIGN WESTERN PWO KAREN TONE-1..MYANMAR SIGN WESTERN PWO KAREN TONE-5
    (0x1069, 0x106D, Tone_Mark),
    // Lo   [3] MYANMAR LETTER EASTERN PWO KAREN NNA..MYANMAR LETTER EASTERN PWO KAREN GHWA
    (0x106E, 0x1070, Consonant),
    // Mn   [4] MYANMAR VOWEL SIGN GEBA KAREN I..MYANMAR VOWEL SIGN KAYAH EE
    (0x1071, 0x1074, Vowel_Dependent),
    // Lo  [13] MYANMAR LETTER SHAN KA..MYANMAR LETTER SHAN HA
    (0x1075, 0x1081, Consonant),
    // Mn       MYANMAR CONSONANT SIGN SHAN MEDIAL WA
    (0x1082, 0x1082, Consonant_Medial),
    // Mc   [2] MYANMAR VOWEL SIGN SHAN AA..MYANMAR VOWEL SIGN SHAN E
    // Mn   [2] MYANMAR VOWEL SIGN SHAN E ABOVE..MYANMAR VOWEL SIGN SHAN FINAL Y
    (0x1083, 0x1086, Vowel_Dependent),
    // Mc   [6] MYANMAR SIGN SHAN TONE-2..MYANMAR SIGN SHAN COUNCIL TONE-3
    // Mn       MYANMAR SIGN SHAN COUNCIL EMPHATIC TONE
    (0x1087, 0x108D, Tone_Mark),
    // Lo       MYANMAR LETTER RUMAI PALAUNG FA
    (0x108E, 0x108E, Consonant),
    // Mc       MYANMAR SIGN RUMAI PALAUNG TONE-5
    (0x108F, 0x108F, Tone_Mark),
    // Nd  [10] MYANMAR SHAN DIGIT ZERO..MYANMAR SHAN DIGIT NINE
    (0x1090, 0x1099, Number),
    // Mc   [2] MYANMAR SIGN KHAMTI TONE-1..MYANMAR SIGN KHAMTI TONE-3
    (0x109A, 0x109B, Tone_Mark),
    // Mc       MYANMAR VOWEL SIGN AITON A
    // Mn       MYANMAR VOWEL SIGN AITON AI
    (0x109C, 0x109D, Vowel_Dependent),
    // Lo   [3] TAGALOG LETTER A..TAGALOG LETTER U
    (0x1700, 0x1702, Vowel_Independent),
    // Lo  [10] TAGALOG LETTER KA..TAGALOG LETTER YA
    (0x1703, 0x170C, Consonant),
    // Lo   [4] TAGALOG LETTER LA..TAGALOG LETTER HA
    (0x170E, 0x1711, Consonant),
    // Mn   [2] TAGALOG VOWEL SIGN I..TAGALOG VOWEL SIGN U
    (0x1712, 0x1713, Vowel_Dependent),
    // Mn       TAGALOG SIGN VIRAMA
    (0x1714, 0x1714, Pure_Killer),
    // Lo   [3] HANUNOO LETTER A..HANUNOO LETTER U
    (0x1720, 0x1722, Vowel_Independent),
    // Lo  [15] HANUNOO LETTER KA..HANUNOO LETTER HA
    (0x1723, 0x1731, Consonant),
    // Mn   [2] HANUNOO VOWEL SIGN I..HANUNOO VOWEL SIGN U
    (0x1732, 0x1733, Vowel_Dependent),
    // Mn       HANUNOO SIGN PAMUDPOD
    (0x1734, 0x1734, Pure_Killer),
    // Lo   [3] BUHID LETTER A..BUHID LETTER U
    (0x1740, 0x1742, Vowel_Independent),
    // Lo  [15] BUHID LETTER KA..BUHID LETTER HA
    (0x1743, 0x1751, Consonant),
    // Mn   [2] BUHID VOWEL SIGN I..BUHID VOWEL SIGN U
    (0x1752, 0x1753, Vowel_Dependent),
    // Lo   [3] TAGBANWA LETTER A..TAGBANWA LETTER U
    (0x1760, 0x1762, Vowel_Independent),
    // Lo  [10] TAGBANWA LETTER KA..TAGBANWA LETTER YA
    (0x1763, 0x176C, Consonant),
    // Lo   [3] TAGBANWA LETTER LA..TAGBANWA LETTER SA
    (0x176E, 0x1770, Consonant),
    // Mn   [2] TAGBANWA VOWEL SIGN I..TAGBANWA VOWEL SIGN U
    (0x1772, 0x1773, Vowel_Dependent),
    // Lo  [35] KHMER LETTER KA..KHMER LETTER QA
    (0x1780, 0x17A2, Consonant),
    // Lo  [17] KHMER INDEPENDENT VOWEL QAQ..KHMER INDEPENDENT VOWEL QAU
    (0x17A3, 0x17B3, Vowel_Independent),
    // Mc       KHMER VOWEL SIGN AA
    // Mn   [7] KHMER VOWEL SIGN I..KHMER VOWEL SIGN UA
    // Mc   [8] KHMER VOWEL SIGN OE..KHMER VOWEL SIGN AU
    (0x17B6, 0x17C5, Vowel_Dependent),
    // Mn       KHMER SIGN NIKAHIT
    (0x17C6, 0x17C6, Bindu),
    // Mc       KHMER SIGN REAHMUK
    (0x17C7, 0x17C7, Visarga),
    // Mc       KHMER SIGN YUUKALEAPINTU
    (0x17C8, 0x17C8, Vowel_Dependent),
    // Mn   [2] KHMER SIGN MUUSIKATOAN..KHMER SIGN TRIISAP
    (0x17C9, 0x17CA, Register_Shifter),
    // Mn       KHMER SIGN BANTOC
    (0x17CB, 0x17CB, Syllable_Modifier),
    // Mn       KHMER SIGN ROBAT
    (0x17CC, 0x17CC, Consonant_Succeeding_Repha),
    // Mn       KHMER SIGN TOANDAKHIAT
    (0x17CD, 0x17CD, Consonant_Killer),
    // Mn   [3] KHMER SIGN KAKABAT..KHMER SIGN SAMYOK SANNYA
    (0x17CE, 0x17D0, Syllable_Modifier),
    // Mn       KHMER SIGN VIRIAM
    (0x17D1, 0x17D1, Pure_Killer),
    // Mn       KHMER SIGN COENG
    (0x17D2, 0x17D2, Invisible_Stacker),
    // Mn       KHMER SIGN BATHAMASAT
    (0x17D3, 0x17D3, Syllable_Modifier),
    // Lo       KHMER SIGN AVAKRAHASANYA
    (0x17DC, 0x17DC, Avagraha),
    // Mn       KHMER SIGN ATTHACAN
    (0x17DD, 0x17DD, Syllable_Modifier),
    // Nd  [10] KHMER DIGIT ZERO..KHMER DIGIT NINE
    (0x17E0, 0x17E9, Number),
    // Lo       LIMBU VOWEL-CARRIER LETTER
    (0x1900, 0x1900, Consonant_Placeholder),
    // Lo  [30] LIMBU LETTER KA..LIMBU LETTER TRA
    (0x1901, 0x191E, Consonant),
    // Mn   [3] LIMBU VOWEL SIGN A..LIMBU VOWEL SIGN U
    // Mc   [4] LIMBU VOWEL SIGN EE..LIMBU VOWEL SIGN AU
    // Mn   [2] LIMBU VOWEL SIGN E..LIMBU VOWEL SIGN O
    (0x1920, 0x1928, Vowel_Dependent),
    // Mc   [3] LIMBU SUBJOINED LETTER YA..LIMBU SUBJOINED LETTER WA
    (0x1929, 0x192B, Consonant_Subjoined),
    // Mc   [2] LIMBU SMALL LETTER KA..LIMBU SMALL LETTER NGA
    (0x1930, 0x1931, Consonant_Final),
    // Mn       LIMBU SMALL LETTER ANUSVARA
    (0x1932, 0x1932, Bindu),
    // Mc   [6] LIMBU SMALL LETTER TA..LIMBU SMALL LETTER LA
    // Mn       LIMBU SIGN MUKPHRENG
    (0x1933, 0x1939, Consonant_Final),
    // Mn       LIMBU SIGN KEMPHRENG
    (0x193A, 0x193A, Vowel_Dependent),
    // Mn       LIMBU SIGN SA-I
    (0x193B, 0x193B, Syllable_Modifier),
    // Nd  [10] LIMBU DIGIT ZERO..LIMBU DIGIT NINE
    (0x1946, 0x194F, Number),
    // Lo  [19] TAI LE LETTER KA..TAI LE LETTER NA
    (0x1950, 0x1962, Consonant),
    // Lo  [11] TAI LE LETTER A..TAI LE LETTER AI
    (0x1963, 0x196D, Vowel),
    // Lo   [5] TAI LE LETTER TONE-2..TAI LE LETTER TONE-6
    (0x1970, 0x1974, Tone_Letter),
    // Lo  [44] NEW TAI LUE LETTER HIGH QA..NEW TAI LUE LETTER LOW SUA
    (0x1980, 0x19AB, Consonant),
    // Lo  [17] NEW TAI LUE VOWEL SIGN VOWEL SHORTENER..NEW TAI LUE VOWEL SIGN IY
    (0x19B0, 0x19C0, Vowel_Dependent),
    // Lo   [7] NEW TAI LUE LETTER FINAL V..NEW TAI LUE LETTER FINAL B
    (0x19C1, 0x19C7, Consonant_Final),
    // Lo   [2] NEW TAI LUE TONE MARK-1..NEW TAI LUE TONE MARK-2
    (0x19C8, 0x19C9, Tone_Mark),
    // Nd  [10] NEW TAI LUE DIGIT ZERO..NEW TAI LUE DIGIT NINE
    // No       NEW TAI LUE THAM DIGIT ONE
    (0x19D0, 0x19DA, Number),
    // Lo  [23] BUGINESE LETTER KA..BUGINESE LETTER HA
    (0x1A00, 0x1A16, Consonant),
    // Mn   [2] BUGINESE VOWEL SIGN I..BUGINESE VOWEL SIGN U
    // Mc   [2] BUGINESE VOWEL SIGN E..BUGINESE VOWEL SIGN O
    // Mn       BUGINESE VOWEL SIGN AE
    (0x1A17, 0x1A1B, Vowel_Dependent),
    // Lo  [45] TAI THAM LETTER HIGH KA..TAI THAM LETTER LOW HA
    (0x1A20, 0x1A4C, Consonant),
    // Lo   [6] TAI THAM LETTER I..TAI THAM LETTER OO
    (0x1A4D, 0x1A52, Vowel_Independent),
    // Lo   [2] TAI THAM LETTER LAE..TAI THAM LETTER GREAT SA
    (0x1A53, 0x1A54, Consonant),
    // Mc       TAI THAM CONSONANT SIGN MEDIAL RA
    // Mn       TAI THAM CONSONANT SIGN MEDIAL LA
    (0x1A55, 0x1A56, Consonant_Medial),
    // Mc       TAI THAM CONSONANT SIGN LA TANG LAI
    (0x1A57, 0x1A57, Consonant_Subjoined),
    // Mn   [2] TAI THAM SIGN MAI KANG LAI..TAI THAM CONSONANT SIGN FINAL NGA
    (0x1A58, 0x1A59, Consonant_Final),
    // Mn       TAI THAM CONSONANT SIGN LOW PA
    (0x1A5A, 0x1A5A, Consonant_Initial_Postfixed),
    // Mn   [4] TAI THAM CONSONANT SIGN HIGH RATHA OR LOW PA..TAI THAM CONSONANT SIGN SA
    (0x1A5B, 0x1A5E, Consonant_Subjoined),
    // Mn       TAI THAM SIGN SAKOT
    (0x1A60, 0x1A60, Invisible_Stacker),
    // Mc       TAI THAM VOWEL SIGN A
    // Mn       TAI THAM VOWEL SIGN MAI SAT
    // Mc   [2] TAI THAM VOWEL SIGN AA..TAI THAM VOWEL SIGN TALL AA
    // Mn   [8] TAI THAM VOWEL SIGN I..TAI THAM VOWEL SIGN OA BELOW
    // Mc   [6] TAI THAM VOWEL SIGN OY..TAI THAM VOWEL SIGN THAM AI
    // Mn       TAI THAM VOWEL SIGN OA ABOVE
    (0x1A61, 0x1A73, Vowel_Dependent),
    // Mn       TAI THAM SIGN MAI KANG
    (0x1A74, 0x1A74, Bindu),
    // Mn   [5] TAI THAM SIGN TONE-1..TAI THAM SIGN KHUEN TONE-5
    (0x1A75, 0x1A79, Tone_Mark),
    // Mn       TAI THAM SIGN RA HAAM
    (0x1A7A, 0x1A7A, Pure_Killer),
    // Mn   [2] TAI THAM SIGN MAI SAM..TAI THAM SIGN KHUEN-LUE KARAN
    (0x1A7B, 0x1A7C, Syllable_Modifier),
    // Mn       TAI THAM COMBINING CRYPTOGRAMMIC DOT
    (0x1A7F, 0x1A7F, Syllable_Modifier),
    // Nd  [10] TAI THAM HORA DIGIT ZERO..TAI THAM HORA DIGIT NINE
    (0x1A80, 0x1A89, Number),
    // Nd  [10] TAI THAM THAM DIGIT ZERO..TAI THAM THAM DIGIT NINE
    (0x1A90, 0x1A99, Number),
    // Mn   [3] BALINESE SIGN ULU RICEM..BALINESE SIGN CECEK
    (0x1B00, 0x1B02, Bindu),
    // Mn       BALINESE SIGN SURANG
    (0x1B03, 0x1B03, Consonant_Succeeding_Repha),
    // Mc       BALINESE SIGN BISAH
    (0x1B04, 0x1B04, Visarga),
    // Lo  [14] BALINESE LETTER AKARA..BALINESE LETTER OKARA TEDUNG
    (0x1B05, 0x1B12, Vowel_Independent),
    // Lo  [33] BALINESE LETTER KA..BALINESE LETTER HA
    (0x1B13, 0x1B33, Consonant),
    // Mn       BALINESE SIGN REREKAN
    (0x1B34, 0x1B34, Nukta),
    // Mc       BALINESE VOWEL SIGN TEDUNG
    // Mn   [5] BALINESE VOWEL SIGN ULU..BALINESE VOWEL SIGN RA REPA
    // Mc       BALINESE VOWEL SIGN RA REPA TEDUNG
    // Mn       BALINESE VOWEL SIGN LA LENGA
    // Mc   [5] BALINESE VOWEL SIGN LA LENGA TEDUNG..BALINESE VOWEL SIGN TALING REPA TEDUNG
    // Mn       BALINESE VOWEL SIGN PEPET
    // Mc       BALINESE VOWEL SIGN PEPET TEDUNG
    (0x1B35, 0x1B43, Vowel_Dependent),
    // Mc       BALINESE ADEG ADEG
    (0x1B44, 0x1B44, Virama),
    // Lo   [7] BALINESE LETTER KAF SASAK..BALINESE LETTER ASYURA SASAK
    (0x1B45, 0x1B4B, Consonant),
    // Nd  [10] BALINESE DIGIT ZERO..BALINESE DIGIT NINE
    (0x1B50, 0x1B59, Number),
    // Mn       SUNDANESE SIGN PANYECEK
    (0x1B80, 0x1B80, Bindu),
    // Mn       SUNDANESE SIGN PANGLAYAR
    (0x1B81, 0x1B81, Consonant_Succeeding_Repha),
    // Mc       SUNDANESE SIGN PANGWISAD
    (0x1B82, 0x1B82, Visarga),
    // Lo   [7] SUNDANESE LETTER A..SUNDANESE LETTER EU
    (0x1B83, 0x1B89, Vowel_Independent),
    // Lo  [23] SUNDANESE LETTER KA..SUNDANESE LETTER HA
    (0x1B8A, 0x1BA0, Consonant),
    // Mc       SUNDANESE CONSONANT SIGN PAMINGKAL
    // Mn   [2] SUNDANESE CONSONANT SIGN PANYAKRA..SUNDANESE CONSONANT SIGN PANYIKU
    (0x1BA1, 0x1BA3, Consonant_Subjoined),
    // Mn   [2] SUNDANESE VOWEL SIGN PANGHULU..SUNDANESE VOWEL SIGN PANYUKU
    // Mc   [2] SUNDANESE VOWEL SIGN PANAELAENG..SUNDANESE VOWEL SIGN PANOLONG
    // Mn   [2] SUNDANESE VOWEL SIGN PAMEPET..SUNDANESE VOWEL SIGN PANEULEUNG
    (0x1BA4, 0x1BA9, Vowel_Dependent),
    // Mc       SUNDANESE SIGN PAMAAEH
    (0x1BAA, 0x1BAA, Pure_Killer),
    // Mn       SUNDANESE SIGN VIRAMA
    (0x1BAB, 0x1BAB, Invisible_Stacker),
    // Mn   [2] SUNDANESE CONSONANT SIGN PASANGAN MA..SUNDANESE CONSONANT SIGN PASANGAN WA
    (0x1BAC, 0x1BAD, Consonant_Subjoined),
    // Lo   [2] SUNDANESE LETTER KHA..SUNDANESE LETTER SYA
    (0x1BAE, 0x1BAF, Consonant),
    // Nd  [10] SUNDANESE DIGIT ZERO..SUNDANESE DIGIT NINE
    (0x1BB0, 0x1BB9, Number),
    // Lo       SUNDANESE AVAGRAHA
    (0x1BBA, 0x1BBA, Avagraha),
    // Lo   [3] SUNDANESE LETTER REU..SUNDANESE LETTER BHA
    (0x1BBB, 0x1BBD, Consonant),
    // Lo   [2] SUNDANESE LETTER FINAL K..SUNDANESE LETTER FINAL M
    (0x1BBE, 0x1BBF, Consonant_Final),
    // Lo  [36] BATAK LETTER A..BATAK LETTER MBA
    (0x1BC0, 0x1BE3, Consonant),
    // Lo   [2] BATAK LETTER I..BATAK LETTER U
    (0x1BE4, 0x1BE5, Vowel_Independent),
    // Mn       BATAK SIGN TOMPI
    (0x1BE6, 0x1BE6, Nukta),
    // Mc       BATAK VOWEL SIGN E
    // Mn   [2] BATAK VOWEL SIGN PAKPAK E..BATAK VOWEL SIGN EE
    // Mc   [3] BATAK VOWEL SIGN I..BATAK VOWEL SIGN O
    // Mn       BATAK VOWEL SIGN KARO O
    // Mc       BATAK VOWEL SIGN U
    // Mn       BATAK VOWEL SIGN U FOR SIMALUNGUN SA
    (0x1BE7, 0x1BEF, Vowel_Dependent),
    // Mn   [2] BATAK CONSONANT SIGN NG..BATAK CONSONANT SIGN H
    (0x1BF0, 0x1BF1, Consonant_Final),
    // Mc   [2] BATAK PANGOLAT..BATAK PANONGONAN
    (0x1BF2, 0x1BF3, Pure_Killer),
    // Lo  [36] LEPCHA LETTER KA..LEPCHA LETTER A
    (0x1C00, 0x1C23, Consonant),
    // Mc   [2] LEPCHA SUBJOINED LETTER YA..LEPCHA SUBJOINED LETTER RA
    (0x1C24, 0x1C25, Consonant_Subjoined),
    // Mc   [6] LEPCHA VOWEL SIGN AA..LEPCHA VOWEL SIGN UU
    // Mn       LEPCHA VOWEL SIGN E
    (0x1C26, 0x1C2C, Vowel_Dependent),
    // Mn   [7] LEPCHA CONSONANT SIGN K..LEPCHA CONSONANT SIGN T
    (0x1C2D, 0x1C33, Consonant_Final),
    // Mc   [2] LEPCHA CONSONANT SIGN NYIN-DO..LEPCHA CONSONANT SIGN KANG
    (0x1C34, 0x1C35, Bindu),
    // Mn       LEPCHA SIGN RAN
    (0x1C36, 0x1C36, Syllable_Modifier),
    // Mn       LEPCHA SIGN NUKTA
    (0x1C37, 0x1C37, Nukta),
    // Nd  [10] LEPCHA DIGIT ZERO..LEPCHA DIGIT NINE
    (0x1C40, 0x1C49, Number),
    // Lo   [3] LEPCHA LETTER TTA..LEPCHA LETTER DDA
    (0x1C4D, 0x1C4F, Consonant),
    // Mn   [3] VEDIC TONE KARSHANA..VEDIC TONE PRENKHA
    (0x1CD0, 0x1CD2, Cantillation_Mark),
    // Mn  [13] VEDIC SIGN YAJURVEDIC MIDLINE SVARITA..VEDIC TONE RIGVEDIC KASHMIRI INDEPENDENT SVARITA
    // Mc       VEDIC TONE ATHARVAVEDIC INDEPENDENT SVARITA
    (0x1CD4, 0x1CE1, Cantillation_Mark),
    // Lo   [2] VEDIC SIGN ARDHAVISARGA..VEDIC SIGN ROTATED ARDHAVISARGA
    (0x1CF2, 0x1CF3, Consonant_Dead),
    // Mn       VEDIC TONE CANDRA ABOVE
    (0x1CF4, 0x1CF4, Cantillation_Mark),
    // Lo   [2] VEDIC SIGN JIHVAMULIYA..VEDIC SIGN UPADHMANIYA
    (0x1CF5, 0x1CF6, Consonant_With_Stacker),
    // Mc       VEDIC SIGN ATIKRAMA
    // Mn   [2] VEDIC TONE RING ABOVE..VEDIC TONE DOUBLE RING ABOVE
    (0x1CF7, 0x1CF9, Cantillation_Mark),
    // Lo       VEDIC SIGN DOUBLE ANUSVARA ANTARGOMUKHA
    (0x1CFA, 0x1CFA, Consonant_Placeholder),
    // Mn       COMBINING DELETION MARK
    (0x1DFB, 0x1DFB, Syllable_Modifier),
    // Cf       ZERO WIDTH NON-JOINER
    (0x200C, 0x200C, Non_Joiner),
    // Cf       ZERO WIDTH JOINER
    (0x200D, 0x200D, Joiner),
    // Pd   [5] HYPHEN..EM DASH
    (0x2010, 0x2014, Consonant_Placeholder),
    // No       SUPERSCRIPT FOUR
    (0x2074, 0x2074, Syllable_Modifier),
    // No   [3] SUBSCRIPT TWO..SUBSCRIPT FOUR
    (0x2082, 0x2084, Syllable_Modifier),
    // Mn       COMBINING ASTERISK ABOVE
    (0x20F0, 0x20F0, Cantillation_Mark),
    // So       DOTTED CIRCLE
    (0x25CC, 0x25CC, Consonant_Placeholder),
    // Lo   [2] SYLOTI NAGRI LETTER A..SYLOTI NAGRI LETTER I
    (0xA800, 0xA801, Vowel_Independent),
    // Mn       SYLOTI NAGRI SIGN DVISVARA
    (0xA802, 0xA802, Vowel_Dependent),
    // Lo   [3] SYLOTI NAGRI LETTER U..SYLOTI NAGRI LETTER O
    (0xA803, 0xA805, Vowel_Independent),
    // Mn       SYLOTI NAGRI SIGN HASANTA
    (0xA806, 0xA806, Virama),
    // Lo   [4] SYLOTI NAGRI LETTER KO..SYLOTI NAGRI LETTER GHO
    (0xA807, 0xA80A, Consonant),
    // Mn       SYLOTI NAGRI SIGN ANUSVARA
    (0xA80B, 0xA80B, Bindu),
    // Lo  [23] SYLOTI NAGRI LETTER CO..SYLOTI NAGRI LETTER HO
    (0xA80C, 0xA822, Consonant),
    // Mc   [2] SYLOTI NAGRI VOWEL SIGN A..SYLOTI NAGRI VOWEL SIGN I
    // Mn   [2] SYLOTI NAGRI VOWEL SIGN U..SYLOTI NAGRI VOWEL SIGN E
    // Mc       SYLOTI NAGRI VOWEL SIGN OO
    (0xA823, 0xA827, Vowel_Dependent),
    // Lo  [30] PHAGS-PA LETTER KA..PHAGS-PA LETTER A
    (0xA840, 0xA85D, Consonant),
    // Lo   [4] PHAGS-PA LETTER I..PHAGS-PA LETTER O
    (0xA85E, 0xA861, Vowel),
    // Lo   [4] PHAGS-PA LETTER QA..PHAGS-PA LETTER GGA
    (0xA862, 0xA865, Consonant),
    // Lo       PHAGS-PA LETTER EE
    (0xA866, 0xA866, Vowel),
    // Lo   [2] PHAGS-PA SUBJOINED LETTER WA..PHAGS-PA SUBJOINED LETTER YA
    (0xA867, 0xA868, Consonant_Subjoined),
    // Lo   [8] PHAGS-PA LETTER TTA..PHAGS-PA LETTER ASPIRATED FA
    (0xA869, 0xA870, Consonant),
    // Lo       PHAGS-PA SUBJOINED LETTER RA
    (0xA871, 0xA871, Consonant_Subjoined),
    // Lo       PHAGS-PA SUPERFIXED LETTER RA
    (0xA872, 0xA872, Consonant),
    // Lo       PHAGS-PA LETTER CANDRABINDU
    (0xA873, 0xA873, Bindu),
    // Mc       SAURASHTRA SIGN ANUSVARA
    (0xA880, 0xA880, Bindu),
    // Mc       SAURASHTRA SIGN VISARGA
    (0xA881, 0xA881, Visarga),
    // Lo  [16] SAURASHTRA LETTER A..SAURASHTRA LETTER AU
    (0xA882, 0xA891, Vowel_Independent),
    // Lo  [34] SAURASHTRA LETTER KA..SAURASHTRA LETTER LLA
    (0xA892, 0xA8B3, Consonant),
    // Mc       SAURASHTRA CONSONANT SIGN HAARU
    (0xA8B4, 0xA8B4, Consonant_Medial),
    // Mc  [15] SAURASHTRA VOWEL SIGN AA..SAURASHTRA VOWEL SIGN AU
    (0xA8B5, 0xA8C3, Vowel_Dependent),
    // Mn       SAURASHTRA SIGN VIRAMA
    (0xA8C4, 0xA8C4, Virama),
    // Mn       SAURASHTRA SIGN CANDRABINDU
    (0xA8C5, 0xA8C5, Bindu),
    // Nd  [10] SAURASHTRA DIGIT ZERO..SAURASHTRA DIGIT NINE
    (0xA8D0, 0xA8D9, Number),
    // Mn  [18] COMBINING DEVANAGARI DIGIT ZERO..COMBINING DEVANAGARI SIGN AVAGRAHA
    (0xA8E0, 0xA8F1, Cantillation_Mark),
    // Lo   [2] DEVANAGARI SIGN SPACING CANDRABINDU..DEVANAGARI SIGN CANDRABINDU VIRAMA
    (0xA8F2, 0xA8F3, Bindu),
    // Lo       DEVANAGARI LETTER AY
    (0xA8FE, 0xA8FE, Vowel_Independent),
    // Mn       DEVANAGARI VOWEL SIGN AY
    (0xA8FF, 0xA8FF, Vowel_Dependent),
    // Nd  [10] KAYAH LI DIGIT ZERO..KAYAH LI DIGIT NINE
    (0xA900, 0xA909, Number),
    // Lo  [24] KAYAH LI LETTER KA..KAYAH LI LETTER CA
    (0xA90A, 0xA921, Consonant),
    // Lo   [4] KAYAH LI LETTER A..KAYAH LI LETTER OO
    // Mn   [5] KAYAH LI VOWEL UE..KAYAH LI VOWEL O
    (0xA922, 0xA92A, Vowel),
    // Mn   [3] KAYAH LI TONE PLOPHU..KAYAH LI TONE CALYA PLOPHU
    (0xA92B, 0xA92D, Tone_Mark),
    // Lo  [23] REJANG LETTER KA..REJANG LETTER A
    (0xA930, 0xA946, Consonant),
    // Mn   [8] REJANG VOWEL SIGN I..REJANG VOWEL SIGN EA
    (0xA947, 0xA94E, Vowel_Dependent),
    // Mn   [3] REJANG CONSONANT SIGN NG..REJANG CONSONANT SIGN R
    // Mc       REJANG CONSONANT SIGN H
    (0xA94F, 0xA952, Consonant_Final),
    // Mc       REJANG VIRAMA
    (0xA953, 0xA953, Pure_Killer),
    // Mn   [2] JAVANESE SIGN PANYANGGA..JAVANESE SIGN CECAK
    (0xA980, 0xA981, Bindu),
    // Mn       JAVANESE SIGN LAYAR
    (0xA982, 0xA982, Consonant_Succeeding_Repha),
    // Mc       JAVANESE SIGN WIGNYAN
    (0xA983, 0xA983, Visarga),
    // Lo   [5] JAVANESE LETTER A..JAVANESE LETTER U
    (0xA984, 0xA988, Vowel_Independent),
    // Lo   [3] JAVANESE LETTER PA CEREK..JAVANESE LETTER NGA LELET RASWADI
    (0xA989, 0xA98B, Consonant),
    // Lo   [3] JAVANESE LETTER E..JAVANESE LETTER O
    (0xA98C, 0xA98E, Vowel_Independent),
    // Lo  [36] JAVANESE LETTER KA..JAVANESE LETTER HA
    (0xA98F, 0xA9B2, Consonant),
    // Mn       JAVANESE SIGN CECAK TELU
    (0xA9B3, 0xA9B3, Nukta),
    // Mc   [2] JAVANESE VOWEL SIGN TARUNG..JAVANESE VOWEL SIGN TOLONG
    // Mn   [4] JAVANESE VOWEL SIGN WULU..JAVANESE VOWEL SIGN SUKU MENDUT
    // Mc   [2] JAVANESE VOWEL SIGN TALING..JAVANESE VOWEL SIGN DIRGA MURE
    // Mn       JAVANESE VOWEL SIGN PEPET
    (0xA9B4, 0xA9BC, Vowel_Dependent),
    // Mn       JAVANESE CONSONANT SIGN KERET
    // Mc   [2] JAVANESE CONSONANT SIGN PENGKAL..JAVANESE CONSONANT SIGN CAKRA
    (0xA9BD, 0xA9BF, Consonant_Medial),
    // Mc       JAVANESE PANGKON
    (0xA9C0, 0xA9C0, Virama),
    // Nd  [10] JAVANESE DIGIT ZERO..JAVANESE DIGIT NINE
    (0xA9D0, 0xA9D9, Number),
    // Lo   [5] MYANMAR LETTER SHAN GHA..MYANMAR LETTER SHAN BHA
    (0xA9E0, 0xA9E4, Consonant),
    // Mn       MYANMAR SIGN SHAN SAW
    (0xA9E5, 0xA9E5, Vowel_Dependent),
    // Lo   [9] MYANMAR LETTER TAI LAING NYA..MYANMAR LETTER TAI LAING NNA
    (0xA9E7, 0xA9EF, Consonant),
    // Nd  [10] MYANMAR TAI LAING DIGIT ZERO..MYANMAR TAI LAING DIGIT NINE
    (0xA9F0, 0xA9F9, Number),
    // Lo   [5] MYANMAR LETTER TAI LAING LLA..MYANMAR LETTER TAI LAING BHA
    (0xA9FA, 0xA9FE, Consonant),
    // Lo   [6] CHAM LETTER A..CHAM LETTER O
    (0xAA00, 0xAA05, Vowel_Independent),
    // Lo  [35] CHAM LETTER KA..CHAM LETTER HA
    (0xAA06, 0xAA28, Consonant),
    // Mn   [6] CHAM VOWEL SIGN AA..CHAM VOWEL SIGN OE
    // Mc   [2] CHAM VOWEL SIGN O..CHAM VOWEL SIGN AI
    // Mn   [2] CHAM VOWEL SIGN AU..CHAM VOWEL SIGN UE
    (0xAA29, 0xAA32, Vowel_Dependent),
    // Mc   [2] CHAM CONSONANT SIGN YA..CHAM CONSONANT SIGN RA
    // Mn   [2] CHAM CONSONANT SIGN LA..CHAM CONSONANT SIGN WA
    (0xAA33, 0xAA36, Consonant_Medial),
    // Lo   [3] CHAM LETTER FINAL K..CHAM LETTER FINAL NG
    // Mn       CHAM CONSONANT SIGN FINAL NG
    // Lo   [8] CHAM LETTER FINAL CH..CHAM LETTER FINAL SS
    // Mn       CHAM CONSONANT SIGN FINAL M
    // Mc       CHAM CONSONANT SIGN FINAL H
    (0xAA40, 0xAA4D, Consonant_Final),
    // Nd  [10] CHAM DIGIT ZERO..CHAM DIGIT NINE
    (0xAA50, 0xAA59, Number),
    // Lo  [16] MYANMAR LETTER KHAMTI GA..MYANMAR LETTER KHAMTI FA
    (0xAA60, 0xAA6F, Consonant),
    // Lo   [3] MYANMAR LETTER KHAMTI XA..MYANMAR LETTER KHAMTI RA
    (0xAA71, 0xAA73, Consonant),
    // Lo   [3] MYANMAR LOGOGRAM KHAMTI OAY..MYANMAR LOGOGRAM KHAMTI HM
    (0xAA74, 0xAA76, Consonant_Placeholder),
    // Lo       MYANMAR LETTER AITON RA
    (0xAA7A, 0xAA7A, Consonant),
    // Mc       MYANMAR SIGN PAO KAREN TONE
    // Mn       MYANMAR SIGN TAI LAING TONE-2
    // Mc       MYANMAR SIGN TAI LAING TONE-5
    (0xAA7B, 0xAA7D, Tone_Mark),
    // Lo   [2] MYANMAR LETTER SHWE PALAUNG CHA..MYANMAR LETTER SHWE PALAUNG SHA
    // Lo  [48] TAI VIET LETTER LOW KO..TAI VIET LETTER HIGH O
    (0xAA7E, 0xAAAF, Consonant),
    // Mn       TAI VIET MAI KANG
    // Lo       TAI VIET VOWEL AA
    // Mn   [3] TAI VIET VOWEL I..TAI VIET VOWEL U
    // Lo   [2] TAI VIET VOWEL E..TAI VIET VOWEL O
    // Mn   [2] TAI VIET MAI KHIT..TAI VIET VOWEL IA
    // Lo   [5] TAI VIET VOWEL UEA..TAI VIET VOWEL AN
    // Mn       TAI VIET VOWEL AM
    (0xAAB0, 0xAABE, Vowel_Dependent),
    // Mn       TAI VIET TONE MAI EK
    (0xAABF, 0xAABF, Tone_Mark),
    // Lo       TAI VIET TONE MAI NUENG
    (0xAAC0, 0xAAC0, Tone_Letter),
    // Mn       TAI VIET TONE MAI THO
    (0xAAC1, 0xAAC1, Tone_Mark),
    // Lo       TAI VIET TONE MAI SONG
    (0xAAC2, 0xAAC2, Tone_Letter),
    // Lo   [2] MEETEI MAYEK LETTER E..MEETEI MAYEK LETTER O
    (0xAAE0, 0xAAE1, Vowel_Independent),
    // Lo   [9] MEETEI MAYEK LETTER CHA..MEETEI MAYEK LETTER SSA
    (0xAAE2, 0xAAEA, Consonant),
    // Mc       MEETEI MAYEK VOWEL SIGN II
    // Mn   [2] MEETEI MAYEK VOWEL SIGN UU..MEETEI MAYEK VOWEL SIGN AAI
    // Mc   [2] MEETEI MAYEK VOWEL SIGN AU..MEETEI MAYEK VOWEL SIGN AAU
    (0xAAEB, 0xAAEF, Vowel_Dependent),
    // Mc       MEETEI MAYEK VOWEL SIGN VISARGA
    (0xAAF5, 0xAAF5, Visarga),
    // Mn       MEETEI MAYEK VIRAMA
    (0xAAF6, 0xAAF6, Invisible_Stacker),
    // Lo  [14] MEETEI MAYEK LETTER KOK..MEETEI MAYEK LETTER HUK
    (0xABC0, 0xABCD, Consonant),
    // Lo   [2] MEETEI MAYEK LETTER UN..MEETEI MAYEK LETTER I
    (0xABCE, 0xABCF, Vowel_Independent),
    // Lo       MEETEI MAYEK LETTER PHAM
    (0xABD0, 0xABD0, Consonant),
    // Lo       MEETEI MAYEK LETTER ATIYA
    (0xABD1, 0xABD1, Vowel_Independent),
    // Lo   [9] MEETEI MAYEK LETTER GOK..MEETEI MAYEK LETTER BHAM
    (0xABD2, 0xABDA, Consonant),
    // Lo   [8] MEETEI MAYEK LETTER KOK LONSUM..MEETEI MAYEK LETTER I LONSUM
    (0xABDB, 0xABE2, Consonant_Final),
    // Mc   [2] MEETEI MAYEK VOWEL SIGN ONAP..MEETEI MAYEK VOWEL SIGN INAP
    // Mn       MEETEI MAYEK VOWEL SIGN ANAP
    // Mc   [2] MEETEI MAYEK VOWEL SIGN YENAP..MEETEI MAYEK VOWEL SIGN SOUNAP
    // Mn       MEETEI MAYEK VOWEL SIGN UNAP
    // Mc   [2] MEETEI MAYEK VOWEL SIGN CHEINAP..MEETEI MAYEK VOWEL SIGN NUNG
    (0xABE3, 0xABEA, Vowel_Dependent),
    // Mc       MEETEI MAYEK LUM IYEK
    (0xABEC, 0xABEC, Tone_Mark),
    // Mn       MEETEI MAYEK APUN IYEK
    (0xABED, 0xABED, Pure_Killer),
    // Nd  [10] MEETEI MAYEK DIGIT ZERO..MEETEI MAYEK DIGIT NINE
    (0xABF0, 0xABF9, Number),
];
const PLANE0_LIMIT: char = '\u{10000}';
const SUPPLEMENTARY_TABLE: LookupTable<u32, Indic_Syllabic_Category> = lookup_table![
    // So every possible input is always found in the table
    (0x010000, 0x0109FF, Other),
    // Lo       KHAROSHTHI LETTER A
    (0x010A00, 0x010A00, Consonant),
    // Mn   [3] KHAROSHTHI VOWEL SIGN I..KHAROSHTHI VOWEL SIGN VOCALIC R
    (0x010A01, 0x010A03, Vowel_Dependent),
    // Mn   [2] KHAROSHTHI VOWEL SIGN E..KHAROSHTHI VOWEL SIGN O
    (0x010A05, 0x010A06, Vowel_Dependent),
    // Mn   [2] KHAROSHTHI VOWEL LENGTH MARK..KHAROSHTHI SIGN DOUBLE RING BELOW
    (0x010A0C, 0x010A0D, Vowel_Dependent),
    // Mn       KHAROSHTHI SIGN ANUSVARA
    (0x010A0E, 0x010A0E, Bindu),
    // Mn       KHAROSHTHI SIGN VISARGA
    (0x010A0F, 0x010A0F, Visarga),
    // Lo   [4] KHAROSHTHI LETTER KA..KHAROSHTHI LETTER GHA
    (0x010A10, 0x010A13, Consonant),
    // Lo   [3] KHAROSHTHI LETTER CA..KHAROSHTHI LETTER JA
    (0x010A15, 0x010A17, Consonant),
    // Lo  [29] KHAROSHTHI LETTER NYA..KHAROSHTHI LETTER VHA
    (0x010A19, 0x010A35, Consonant),
    // Mn   [3] KHAROSHTHI SIGN BAR ABOVE..KHAROSHTHI SIGN DOT BELOW
    (0x010A38, 0x010A3A, Nukta),
    // Mn       KHAROSHTHI VIRAMA
    (0x010A3F, 0x010A3F, Invisible_Stacker),
    // No   [9] KHAROSHTHI DIGIT ONE..KHAROSHTHI FRACTION ONE HALF
    (0x010A40, 0x010A48, Number),
    // Mc       BRAHMI SIGN CANDRABINDU
    // Mn       BRAHMI SIGN ANUSVARA
    (0x011000, 0x011001, Bindu),
    // Mc       BRAHMI SIGN VISARGA
    (0x011002, 0x011002, Visarga),
    // Lo   [2] BRAHMI SIGN JIHVAMULIYA..BRAHMI SIGN UPADHMANIYA
    (0x011003, 0x011004, Consonant_With_Stacker),
    // Lo  [14] BRAHMI LETTER A..BRAHMI LETTER AU
    (0x011005, 0x011012, Vowel_Independent),
    // Lo  [37] BRAHMI LETTER KA..BRAHMI LETTER OLD TAMIL NNNA
    (0x011013, 0x011037, Consonant),
    // Mn  [14] BRAHMI VOWEL SIGN AA..BRAHMI VOWEL SIGN AU
    (0x011038, 0x011045, Vowel_Dependent),
    // Mn       BRAHMI VIRAMA
    (0x011046, 0x011046, Virama),
    // No  [20] BRAHMI NUMBER ONE..BRAHMI NUMBER ONE THOUSAND
    (0x011052, 0x011065, Brahmi_Joining_Number),
    // Nd  [10] BRAHMI DIGIT ZERO..BRAHMI DIGIT NINE
    (0x011066, 0x01106F, Number),
    // Mn       BRAHMI NUMBER JOINER
    (0x01107F, 0x01107F, Number_Joiner),
    // Mn   [2] KAITHI SIGN CANDRABINDU..KAITHI SIGN ANUSVARA
    (0x011080, 0x011081, Bindu),
    // Mc       KAITHI SIGN VISARGA
    (0x011082, 0x011082, Visarga),
    // Lo  [10] KAITHI LETTER A..KAITHI LETTER AU
    (0x011083, 0x01108C, Vowel_Independent),
    // Lo  [35] KAITHI LETTER KA..KAITHI LETTER HA
    (0x01108D, 0x0110AF, Consonant),
    // Mc   [3] KAITHI VOWEL SIGN AA..KAITHI VOWEL SIGN II
    // Mn   [4] KAITHI VOWEL SIGN U..KAITHI VOWEL SIGN AI
    // Mc   [2] KAITHI VOWEL SIGN O..KAITHI VOWEL SIGN AU
    (0x0110B0, 0x0110B8, Vowel_Dependent),
    // Mn       KAITHI SIGN VIRAMA
    (0x0110B9, 0x0110B9, Virama),
    // Mn       KAITHI SIGN NUKTA
    (0x0110BA, 0x0110BA, Nukta),
    // Mn   [2] CHAKMA SIGN CANDRABINDU..CHAKMA SIGN ANUSVARA
    (0x011100, 0x011101, Bindu),
    // Mn       CHAKMA SIGN VISARGA
    (0x011102, 0x011102, Visarga),
    // Lo   [4] CHAKMA LETTER AA..CHAKMA LETTER E
    (0x011103, 0x011106, Vowel_Independent),
    // Lo  [32] CHAKMA LETTER KAA..CHAKMA LETTER HAA
    (0x011107, 0x011126, Consonant),
    // Mn   [5] CHAKMA VOWEL SIGN A..CHAKMA VOWEL SIGN UU
    // Mc       CHAKMA VOWEL SIGN E
    // Mn   [6] CHAKMA VOWEL SIGN AI..CHAKMA AU MARK
    (0x011127, 0x011132, Vowel_Dependent),
    // Mn       CHAKMA VIRAMA
    (0x011133, 0x011133, Invisible_Stacker),
    // Mn       CHAKMA MAAYYAA
    (0x011134, 0x011134, Pure_Killer),
    // Nd  [10] CHAKMA DIGIT ZERO..CHAKMA DIGIT NINE
    (0x011136, 0x01113F, Number),
    // Lo       CHAKMA LETTER LHAA
    (0x011144, 0x011144, Consonant),
    // Mc   [2] CHAKMA VOWEL SIGN AA..CHAKMA VOWEL SIGN EI
    (0x011145, 0x011146, Vowel_Dependent),
    // Lo   [5] MAHAJANI LETTER A..MAHAJANI LETTER O
    (0x011150, 0x011154, Vowel),
    // Lo  [30] MAHAJANI LETTER KA..MAHAJANI LETTER RRA
    (0x011155, 0x011172, Consonant),
    // Mn       MAHAJANI SIGN NUKTA
    (0x011173, 0x011173, Nukta),
    // Mn   [2] SHARADA SIGN CANDRABINDU..SHARADA SIGN ANUSVARA
    (0x011180, 0x011181, Bindu),
    // Mc       SHARADA SIGN VISARGA
    (0x011182, 0x011182, Visarga),
    // Lo  [14] SHARADA LETTER A..SHARADA LETTER AU
    (0x011183, 0x011190, Vowel_Independent),
    // Lo  [34] SHARADA LETTER KA..SHARADA LETTER HA
    (0x011191, 0x0111B2, Consonant),
    // Mc   [3] SHARADA VOWEL SIGN AA..SHARADA VOWEL SIGN II
    // Mn   [9] SHARADA VOWEL SIGN U..SHARADA VOWEL SIGN O
    // Mc       SHARADA VOWEL SIGN AU
    (0x0111B3, 0x0111BF, Vowel_Dependent),
    // Mc       SHARADA SIGN VIRAMA
    (0x0111C0, 0x0111C0, Virama),
    // Lo       SHARADA SIGN AVAGRAHA
    (0x0111C1, 0x0111C1, Avagraha),
    // Lo   [2] SHARADA SIGN JIHVAMULIYA..SHARADA SIGN UPADHMANIYA
    (0x0111C2, 0x0111C3, Consonant_Prefixed),
    // Mn       SHARADA SANDHI MARK
    (0x0111C9, 0x0111C9, Syllable_Modifier),
    // Mn       SHARADA SIGN NUKTA
    (0x0111CA, 0x0111CA, Nukta),
    // Mn   [2] SHARADA VOWEL MODIFIER MARK..SHARADA EXTRA SHORT VOWEL MARK
    (0x0111CB, 0x0111CC, Vowel_Dependent),
    // Nd  [10] SHARADA DIGIT ZERO..SHARADA DIGIT NINE
    (0x0111D0, 0x0111D9, Number),
    // No  [20] SINHALA ARCHAIC DIGIT ONE..SINHALA ARCHAIC NUMBER ONE THOUSAND
    (0x0111E1, 0x0111F4, Number),
    // Lo   [8] KHOJKI LETTER A..KHOJKI LETTER AU
    (0x011200, 0x011207, Vowel_Independent),
    // Lo  [10] KHOJKI LETTER KA..KHOJKI LETTER JJA
    (0x011208, 0x011211, Consonant),
    // Lo  [25] KHOJKI LETTER NYA..KHOJKI LETTER LLA
    (0x011213, 0x01122B, Consonant),
    // Mc   [3] KHOJKI VOWEL SIGN AA..KHOJKI VOWEL SIGN II
    // Mn   [3] KHOJKI VOWEL SIGN U..KHOJKI VOWEL SIGN AI
    // Mc   [2] KHOJKI VOWEL SIGN O..KHOJKI VOWEL SIGN AU
    (0x01122C, 0x011233, Vowel_Dependent),
    // Mn       KHOJKI SIGN ANUSVARA
    (0x011234, 0x011234, Bindu),
    // Mc       KHOJKI SIGN VIRAMA
    (0x011235, 0x011235, Virama),
    // Mn       KHOJKI SIGN NUKTA
    (0x011236, 0x011236, Nukta),
    // Mn       KHOJKI SIGN SHADDA
    (0x011237, 0x011237, Gemination_Mark),
    // Mn       KHOJKI SIGN SUKUN
    (0x01123E, 0x01123E, Cantillation_Mark),
    // Lo   [4] MULTANI LETTER A..MULTANI LETTER E
    (0x011280, 0x011283, Vowel_Independent),
    // Lo   [3] MULTANI LETTER KA..MULTANI LETTER GA
    (0x011284, 0x011286, Consonant),
    // Lo       MULTANI LETTER GHA
    (0x011288, 0x011288, Consonant),
    // Lo   [4] MULTANI LETTER CA..MULTANI LETTER JJA
    (0x01128A, 0x01128D, Consonant),
    // Lo  [15] MULTANI LETTER NYA..MULTANI LETTER BA
    (0x01128F, 0x01129D, Consonant),
    // Lo  [10] MULTANI LETTER BHA..MULTANI LETTER RHA
    (0x01129F, 0x0112A8, Consonant),
    // Lo  [10] KHUDAWADI LETTER A..KHUDAWADI LETTER AU
    (0x0112B0, 0x0112B9, Vowel_Independent),
    // Lo  [37] KHUDAWADI LETTER KA..KHUDAWADI LETTER HA
    (0x0112BA, 0x0112DE, Consonant),
    // Mn       KHUDAWADI SIGN ANUSVARA
    (0x0112DF, 0x0112DF, Bindu),
    // Mc   [3] KHUDAWADI VOWEL SIGN AA..KHUDAWADI VOWEL SIGN II
    // Mn   [6] KHUDAWADI VOWEL SIGN U..KHUDAWADI VOWEL SIGN AU
    (0x0112E0, 0x0112E8, Vowel_Dependent),
    // Mn       KHUDAWADI SIGN NUKTA
    (0x0112E9, 0x0112E9, Nukta),
    // Mn       KHUDAWADI SIGN VIRAMA
    (0x0112EA, 0x0112EA, Pure_Killer),
    // Nd  [10] KHUDAWADI DIGIT ZERO..KHUDAWADI DIGIT NINE
    (0x0112F0, 0x0112F9, Number),
    // Mn   [2] GRANTHA SIGN COMBINING ANUSVARA ABOVE..GRANTHA SIGN CANDRABINDU
    // Mc       GRANTHA SIGN ANUSVARA
    (0x011300, 0x011302, Bindu),
    // Mc       GRANTHA SIGN VISARGA
    (0x011303, 0x011303, Visarga),
    // Lo   [8] GRANTHA LETTER A..GRANTHA LETTER VOCALIC L
    (0x011305, 0x01130C, Vowel_Independent),
    // Lo   [2] GRANTHA LETTER EE..GRANTHA LETTER AI
    (0x01130F, 0x011310, Vowel_Independent),
    // Lo   [2] GRANTHA LETTER OO..GRANTHA LETTER AU
    (0x011313, 0x011314, Vowel_Independent),
    // Lo  [20] GRANTHA LETTER KA..GRANTHA LETTER NA
    (0x011315, 0x011328, Consonant),
    // Lo   [7] GRANTHA LETTER PA..GRANTHA LETTER RA
    (0x01132A, 0x011330, Consonant),
    // Lo   [2] GRANTHA LETTER LA..GRANTHA LETTER LLA
    (0x011332, 0x011333, Consonant),
    // Lo   [5] GRANTHA LETTER VA..GRANTHA LETTER HA
    (0x011335, 0x011339, Consonant),
    // Mn   [2] COMBINING BINDU BELOW..GRANTHA SIGN NUKTA
    (0x01133B, 0x01133C, Nukta),
    // Lo       GRANTHA SIGN AVAGRAHA
    (0x01133D, 0x01133D, Avagraha),
    // Mc   [2] GRANTHA VOWEL SIGN AA..GRANTHA VOWEL SIGN I
    // Mn       GRANTHA VOWEL SIGN II
    // Mc   [4] GRANTHA VOWEL SIGN U..GRANTHA VOWEL SIGN VOCALIC RR
    (0x01133E, 0x011344, Vowel_Dependent),
    // Mc   [2] GRANTHA VOWEL SIGN EE..GRANTHA VOWEL SIGN AI
    (0x011347, 0x011348, Vowel_Dependent),
    // Mc   [2] GRANTHA VOWEL SIGN OO..GRANTHA VOWEL SIGN AU
    (0x01134B, 0x01134C, Vowel_Dependent),
    // Mc       GRANTHA SIGN VIRAMA
    (0x01134D, 0x01134D, Virama),
    // Mc       GRANTHA AU LENGTH MARK
    (0x011357, 0x011357, Vowel_Dependent),
    // Lo   [2] GRANTHA LETTER VEDIC ANUSVARA..GRANTHA LETTER VEDIC DOUBLE ANUSVARA
    (0x01135E, 0x01135F, Bindu),
    // Lo   [2] GRANTHA LETTER VOCALIC RR..GRANTHA LETTER VOCALIC LL
    (0x011360, 0x011361, Vowel_Independent),
    // Mc   [2] GRANTHA VOWEL SIGN VOCALIC L..GRANTHA VOWEL SIGN VOCALIC LL
    (0x011362, 0x011363, Vowel_Dependent),
    // Mn   [7] COMBINING GRANTHA DIGIT ZERO..COMBINING GRANTHA DIGIT SIX
    (0x011366, 0x01136C, Cantillation_Mark),
    // Mn   [5] COMBINING GRANTHA LETTER A..COMBINING GRANTHA LETTER PA
    (0x011370, 0x011374, Cantillation_Mark),
    // Lo  [14] NEWA LETTER A..NEWA LETTER AU
    (0x011400, 0x01140D, Vowel_Independent),
    // Lo  [39] NEWA LETTER KA..NEWA LETTER HA
    (0x01140E, 0x011434, Consonant),
    // Mc   [3] NEWA VOWEL SIGN AA..NEWA VOWEL SIGN II
    // Mn   [8] NEWA VOWEL SIGN U..NEWA VOWEL SIGN AI
    // Mc   [2] NEWA VOWEL SIGN O..NEWA VOWEL SIGN AU
    (0x011435, 0x011441, Vowel_Dependent),
    // Mn       NEWA SIGN VIRAMA
    (0x011442, 0x011442, Virama),
    // Mn   [2] NEWA SIGN CANDRABINDU..NEWA SIGN ANUSVARA
    (0x011443, 0x011444, Bindu),
    // Mc       NEWA SIGN VISARGA
    (0x011445, 0x011445, Visarga),
    // Mn       NEWA SIGN NUKTA
    (0x011446, 0x011446, Nukta),
    // Lo       NEWA SIGN AVAGRAHA
    (0x011447, 0x011447, Avagraha),
    // Nd  [10] NEWA DIGIT ZERO..NEWA DIGIT NINE
    (0x011450, 0x011459, Number),
    // Mn       NEWA SANDHI MARK
    (0x01145E, 0x01145E, Syllable_Modifier),
    // Lo       NEWA LETTER VEDIC ANUSVARA
    (0x01145F, 0x01145F, Bindu),
    // Lo  [14] TIRHUTA LETTER A..TIRHUTA LETTER AU
    (0x011481, 0x01148E, Vowel_Independent),
    // Lo  [33] TIRHUTA LETTER KA..TIRHUTA LETTER HA
    (0x01148F, 0x0114AF, Consonant),
    // Mc   [3] TIRHUTA VOWEL SIGN AA..TIRHUTA VOWEL SIGN II
    // Mn   [6] TIRHUTA VOWEL SIGN U..TIRHUTA VOWEL SIGN VOCALIC LL
    // Mc       TIRHUTA VOWEL SIGN E
    // Mn       TIRHUTA VOWEL SIGN SHORT E
    // Mc   [4] TIRHUTA VOWEL SIGN AI..TIRHUTA VOWEL SIGN AU
    (0x0114B0, 0x0114BE, Vowel_Dependent),
    // Mn   [2] TIRHUTA SIGN CANDRABINDU..TIRHUTA SIGN ANUSVARA
    (0x0114BF, 0x0114C0, Bindu),
    // Mc       TIRHUTA SIGN VISARGA
    (0x0114C1, 0x0114C1, Visarga),
    // Mn       TIRHUTA SIGN VIRAMA
    (0x0114C2, 0x0114C2, Virama),
    // Mn       TIRHUTA SIGN NUKTA
    (0x0114C3, 0x0114C3, Nukta),
    // Lo       TIRHUTA SIGN AVAGRAHA
    (0x0114C4, 0x0114C4, Avagraha),
    // Nd  [10] TIRHUTA DIGIT ZERO..TIRHUTA DIGIT NINE
    (0x0114D0, 0x0114D9, Number),
    // Lo  [14] SIDDHAM LETTER A..SIDDHAM LETTER AU
    (0x011580, 0x01158D, Vowel_Independent),
    // Lo  [33] SIDDHAM LETTER KA..SIDDHAM LETTER HA
    (0x01158E, 0x0115AE, Consonant),
    // Mc   [3] SIDDHAM VOWEL SIGN AA..SIDDHAM VOWEL SIGN II
    // Mn   [4] SIDDHAM VOWEL SIGN U..SIDDHAM VOWEL SIGN VOCALIC RR
    (0x0115AF, 0x0115B5, Vowel_Dependent),
    // Mc   [4] SIDDHAM VOWEL SIGN E..SIDDHAM VOWEL SIGN AU
    (0x0115B8, 0x0115BB, Vowel_Dependent),
    // Mn   [2] SIDDHAM SIGN CANDRABINDU..SIDDHAM SIGN ANUSVARA
    (0x0115BC, 0x0115BD, Bindu),
    // Mc       SIDDHAM SIGN VISARGA
    (0x0115BE, 0x0115BE, Visarga),
    // Mn       SIDDHAM SIGN VIRAMA
    (0x0115BF, 0x0115BF, Virama),
    // Mn       SIDDHAM SIGN NUKTA
    (0x0115C0, 0x0115C0, Nukta),
    // Lo   [4] SIDDHAM LETTER THREE-CIRCLE ALTERNATE I..SIDDHAM LETTER ALTERNATE U
    (0x0115D8, 0x0115DB, Vowel_Independent),
    // Mn   [2] SIDDHAM VOWEL SIGN ALTERNATE U..SIDDHAM VOWEL SIGN ALTERNATE UU
    (0x0115DC, 0x0115DD, Vowel_Dependent),
    // Lo  [14] MODI LETTER A..MODI LETTER AU
    (0x011600, 0x01160D, Vowel_Independent),
    // Lo  [34] MODI LETTER KA..MODI LETTER LLA
    (0x01160E, 0x01162F, Consonant),
    // Mc   [3] MODI VOWEL SIGN AA..MODI VOWEL SIGN II
    // Mn   [8] MODI VOWEL SIGN U..MODI VOWEL SIGN AI
    // Mc   [2] MODI VOWEL SIGN O..MODI VOWEL SIGN AU
    (0x011630, 0x01163C, Vowel_Dependent),
    // Mn       MODI SIGN ANUSVARA
    (0x01163D, 0x01163D, Bindu),
    // Mc       MODI SIGN VISARGA
    (0x01163E, 0x01163E, Visarga),
    // Mn       MODI SIGN VIRAMA
    (0x01163F, 0x01163F, Virama),
    // Mn       MODI SIGN ARDHACANDRA
    (0x011640, 0x011640, Vowel_Dependent),
    // Nd  [10] MODI DIGIT ZERO..MODI DIGIT NINE
    (0x011650, 0x011659, Number),
    // Lo  [10] TAKRI LETTER A..TAKRI LETTER AU
    (0x011680, 0x011689, Vowel_Independent),
    // Lo  [33] TAKRI LETTER KA..TAKRI LETTER RRA
    (0x01168A, 0x0116AA, Consonant),
    // Mn       TAKRI SIGN ANUSVARA
    (0x0116AB, 0x0116AB, Bindu),
    // Mc       TAKRI SIGN VISARGA
    (0x0116AC, 0x0116AC, Visarga),
    // Mn       TAKRI VOWEL SIGN AA
    // Mc   [2] TAKRI VOWEL SIGN I..TAKRI VOWEL SIGN II
    // Mn   [6] TAKRI VOWEL SIGN U..TAKRI VOWEL SIGN AU
    (0x0116AD, 0x0116B5, Vowel_Dependent),
    // Mc       TAKRI SIGN VIRAMA
    (0x0116B6, 0x0116B6, Virama),
    // Mn       TAKRI SIGN NUKTA
    (0x0116B7, 0x0116B7, Nukta),
    // Lo       TAKRI LETTER ARCHAIC KHA
    (0x0116B8, 0x0116B8, Consonant),
    // Nd  [10] TAKRI DIGIT ZERO..TAKRI DIGIT NINE
    (0x0116C0, 0x0116C9, Number),
    // Lo  [27] AHOM LETTER KA..AHOM LETTER ALTERNATE BA
    (0x011700, 0x01171A, Consonant),
    // Mn   [3] AHOM CONSONANT SIGN MEDIAL LA..AHOM CONSONANT SIGN MEDIAL LIGATING RA
    (0x01171D, 0x01171F, Consonant_Medial),
    // Mc   [2] AHOM VOWEL SIGN A..AHOM VOWEL SIGN AA
    // Mn   [4] AHOM VOWEL SIGN I..AHOM VOWEL SIGN UU
    // Mc       AHOM VOWEL SIGN E
    // Mn   [4] AHOM VOWEL SIGN AW..AHOM VOWEL SIGN AM
    (0x011720, 0x01172A, Vowel_Dependent),
    // Mn       AHOM SIGN KILLER
    (0x01172B, 0x01172B, Pure_Killer),
    // Nd  [10] AHOM DIGIT ZERO..AHOM DIGIT NINE
    // No   [2] AHOM NUMBER TEN..AHOM NUMBER TWENTY
    (0x011730, 0x01173B, Number),
    // Lo  [10] DOGRA LETTER A..DOGRA LETTER AU
    (0x011800, 0x011809, Vowel_Independent),
    // Lo  [34] DOGRA LETTER KA..DOGRA LETTER RRA
    (0x01180A, 0x01182B, Consonant),
    // Mc   [3] DOGRA VOWEL SIGN AA..DOGRA VOWEL SIGN II
    // Mn   [8] DOGRA VOWEL SIGN U..DOGRA VOWEL SIGN AU
    (0x01182C, 0x011836, Vowel_Dependent),
    // Mn       DOGRA SIGN ANUSVARA
    (0x011837, 0x011837, Bindu),
    // Mc       DOGRA SIGN VISARGA
    (0x011838, 0x011838, Visarga),
    // Mn       DOGRA SIGN VIRAMA
    (0x011839, 0x011839, Virama),
    // Mn       DOGRA SIGN NUKTA
    (0x01183A, 0x01183A, Nukta),
    // Lo   [8] NANDINAGARI LETTER A..NANDINAGARI LETTER VOCALIC RR
    (0x0119A0, 0x0119A7, Vowel_Independent),
    // Lo   [4] NANDINAGARI LETTER E..NANDINAGARI LETTER AU
    (0x0119AA, 0x0119AD, Vowel_Independent),
    // Lo  [35] NANDINAGARI LETTER KA..NANDINAGARI LETTER RRA
    (0x0119AE, 0x0119D0, Consonant),
    // Mc   [3] NANDINAGARI VOWEL SIGN AA..NANDINAGARI VOWEL SIGN II
    // Mn   [4] NANDINAGARI VOWEL SIGN U..NANDINAGARI VOWEL SIGN VOCALIC RR
    (0x0119D1, 0x0119D7, Vowel_Dependent),
    // Mn   [2] NANDINAGARI VOWEL SIGN E..NANDINAGARI VOWEL SIGN AI
    // Mc   [2] NANDINAGARI VOWEL SIGN O..NANDINAGARI VOWEL SIGN AU
    (0x0119DA, 0x0119DD, Vowel_Dependent),
    // Mc       NANDINAGARI SIGN ANUSVARA
    (0x0119DE, 0x0119DE, Bindu),
    // Mc       NANDINAGARI SIGN VISARGA
    (0x0119DF, 0x0119DF, Visarga),
    // Mn       NANDINAGARI SIGN VIRAMA
    (0x0119E0, 0x0119E0, Virama),
    // Lo       NANDINAGARI SIGN AVAGRAHA
    (0x0119E1, 0x0119E1, Avagraha),
    // Mc       NANDINAGARI VOWEL SIGN PRISHTHAMATRA E
    (0x0119E4, 0x0119E4, Vowel_Dependent),
    // Lo       ZANABAZAR SQUARE LETTER A
    (0x011A00, 0x011A00, Vowel_Independent),
    // Mn  [10] ZANABAZAR SQUARE VOWEL SIGN I..ZANABAZAR SQUARE VOWEL LENGTH MARK
    (0x011A01, 0x011A0A, Vowel_Dependent),
    // Lo  [40] ZANABAZAR SQUARE LETTER KA..ZANABAZAR SQUARE LETTER KSSA
    (0x011A0B, 0x011A32, Consonant),
    // Mn       ZANABAZAR SQUARE FINAL CONSONANT MARK
    (0x011A33, 0x011A33, Syllable_Modifier),
    // Mn       ZANABAZAR SQUARE SIGN VIRAMA
    (0x011A34, 0x011A34, Pure_Killer),
    // Mn   [4] ZANABAZAR SQUARE SIGN CANDRABINDU..ZANABAZAR SQUARE SIGN ANUSVARA
    (0x011A35, 0x011A38, Bindu),
    // Mc       ZANABAZAR SQUARE SIGN VISARGA
    (0x011A39, 0x011A39, Visarga),
    // Lo       ZANABAZAR SQUARE CLUSTER-INITIAL LETTER RA
    (0x011A3A, 0x011A3A, Consonant_Prefixed),
    // Mn   [4] ZANABAZAR SQUARE CLUSTER-FINAL LETTER YA..ZANABAZAR SQUARE CLUSTER-FINAL LETTER VA
    (0x011A3B, 0x011A3E, Consonant_Medial),
    // Po       ZANABAZAR SQUARE INITIAL HEAD MARK
    (0x011A3F, 0x011A3F, Consonant_Placeholder),
    // Po       ZANABAZAR SQUARE INITIAL DOUBLE-LINED HEAD MARK
    (0x011A45, 0x011A45, Consonant_Placeholder),
    // Mn       ZANABAZAR SQUARE SUBJOINER
    (0x011A47, 0x011A47, Invisible_Stacker),
    // Lo       SOYOMBO LETTER A
    (0x011A50, 0x011A50, Vowel_Independent),
    // Mn   [6] SOYOMBO VOWEL SIGN I..SOYOMBO VOWEL SIGN OE
    // Mc   [2] SOYOMBO VOWEL SIGN AI..SOYOMBO VOWEL SIGN AU
    // Mn   [3] SOYOMBO VOWEL SIGN VOCALIC R..SOYOMBO VOWEL LENGTH MARK
    (0x011A51, 0x011A5B, Vowel_Dependent),
    // Lo  [40] SOYOMBO LETTER KA..SOYOMBO LETTER KSSA
    (0x011A5C, 0x011A83, Consonant),
    // Lo   [6] SOYOMBO SIGN JIHVAMULIYA..SOYOMBO CLUSTER-INITIAL LETTER SA
    (0x011A84, 0x011A89, Consonant_Prefixed),
    // Mn  [12] SOYOMBO FINAL CONSONANT SIGN G..SOYOMBO FINAL CONSONANT SIGN -A
    (0x011A8A, 0x011A95, Consonant_Final),
    // Mn       SOYOMBO SIGN ANUSVARA
    (0x011A96, 0x011A96, Bindu),
    // Mc       SOYOMBO SIGN VISARGA
    (0x011A97, 0x011A97, Visarga),
    // Mn       SOYOMBO GEMINATION MARK
    (0x011A98, 0x011A98, Gemination_Mark),
    // Mn       SOYOMBO SUBJOINER
    (0x011A99, 0x011A99, Invisible_Stacker),
    // Lo       SOYOMBO MARK PLUTA
    (0x011A9D, 0x011A9D, Avagraha),
    // Lo   [9] BHAIKSUKI LETTER A..BHAIKSUKI LETTER VOCALIC L
    (0x011C00, 0x011C08, Vowel_Independent),
    // Lo   [4] BHAIKSUKI LETTER E..BHAIKSUKI LETTER AU
    (0x011C0A, 0x011C0D, Vowel_Independent),
    // Lo  [33] BHAIKSUKI LETTER KA..BHAIKSUKI LETTER HA
    (0x011C0E, 0x011C2E, Consonant),
    // Mc       BHAIKSUKI VOWEL SIGN AA
    // Mn   [7] BHAIKSUKI VOWEL SIGN I..BHAIKSUKI VOWEL SIGN VOCALIC L
    (0x011C2F, 0x011C36, Vowel_Dependent),
    // Mn   [4] BHAIKSUKI VOWEL SIGN E..BHAIKSUKI VOWEL SIGN AU
    (0x011C38, 0x011C3B, Vowel_Dependent),
    // Mn   [2] BHAIKSUKI SIGN CANDRABINDU..BHAIKSUKI SIGN ANUSVARA
    (0x011C3C, 0x011C3D, Bindu),
    // Mc       BHAIKSUKI SIGN VISARGA
    (0x011C3E, 0x011C3E, Visarga),
    // Mn       BHAIKSUKI SIGN VIRAMA
    (0x011C3F, 0x011C3F, Virama),
    // Lo       BHAIKSUKI SIGN AVAGRAHA
    (0x011C40, 0x011C40, Avagraha),
    // Nd  [10] BHAIKSUKI DIGIT ZERO..BHAIKSUKI DIGIT NINE
    // No  [19] BHAIKSUKI NUMBER ONE..BHAIKSUKI HUNDREDS UNIT MARK
    (0x011C50, 0x011C6C, Number),
    // Lo  [30] MARCHEN LETTER KA..MARCHEN LETTER A
    (0x011C72, 0x011C8F, Consonant),
    // Mn  [22] MARCHEN SUBJOINED LETTER KA..MARCHEN SUBJOINED LETTER ZA
    (0x011C92, 0x011CA7, Consonant_Subjoined),
    // Mc       MARCHEN SUBJOINED LETTER YA
    // Mn   [6] MARCHEN SUBJOINED LETTER RA..MARCHEN SUBJOINED LETTER A
    (0x011CA9, 0x011CAF, Consonant_Subjoined),
    // Mn       MARCHEN VOWEL SIGN AA
    // Mc       MARCHEN VOWEL SIGN I
    // Mn   [2] MARCHEN VOWEL SIGN U..MARCHEN VOWEL SIGN E
    // Mc       MARCHEN VOWEL SIGN O
    (0x011CB0, 0x011CB4, Vowel_Dependent),
    // Mn   [2] MARCHEN SIGN ANUSVARA..MARCHEN SIGN CANDRABINDU
    (0x011CB5, 0x011CB6, Bindu),
    // Lo   [7] MASARAM GONDI LETTER A..MASARAM GONDI LETTER E
    (0x011D00, 0x011D06, Vowel_Independent),
    // Lo   [2] MASARAM GONDI LETTER AI..MASARAM GONDI LETTER O
    (0x011D08, 0x011D09, Vowel_Independent),
    // Lo       MASARAM GONDI LETTER AU
    (0x011D0B, 0x011D0B, Vowel_Independent),
    // Lo  [37] MASARAM GONDI LETTER KA..MASARAM GONDI LETTER TRA
    (0x011D0C, 0x011D30, Consonant),
    // Mn   [6] MASARAM GONDI VOWEL SIGN AA..MASARAM GONDI VOWEL SIGN VOCALIC R
    (0x011D31, 0x011D36, Vowel_Dependent),
    // Mn       MASARAM GONDI VOWEL SIGN E
    (0x011D3A, 0x011D3A, Vowel_Dependent),
    // Mn   [2] MASARAM GONDI VOWEL SIGN AI..MASARAM GONDI VOWEL SIGN O
    (0x011D3C, 0x011D3D, Vowel_Dependent),
    // Mn       MASARAM GONDI VOWEL SIGN AU
    (0x011D3F, 0x011D3F, Vowel_Dependent),
    // Mn       MASARAM GONDI SIGN ANUSVARA
    (0x011D40, 0x011D40, Bindu),
    // Mn       MASARAM GONDI SIGN VISARGA
    (0x011D41, 0x011D41, Visarga),
    // Mn       MASARAM GONDI SIGN NUKTA
    (0x011D42, 0x011D42, Nukta),
    // Mn       MASARAM GONDI SIGN CANDRA
    (0x011D43, 0x011D43, Vowel_Dependent),
    // Mn       MASARAM GONDI SIGN HALANTA
    (0x011D44, 0x011D44, Pure_Killer),
    // Mn       MASARAM GONDI VIRAMA
    (0x011D45, 0x011D45, Invisible_Stacker),
    // Lo       MASARAM GONDI REPHA
    (0x011D46, 0x011D46, Consonant_Preceding_Repha),
    // Mn       MASARAM GONDI RA-KARA
    (0x011D47, 0x011D47, Consonant_Medial),
    // Nd  [10] MASARAM GONDI DIGIT ZERO..MASARAM GONDI DIGIT NINE
    (0x011D50, 0x011D59, Number),
    // Lo   [6] GUNJALA GONDI LETTER A..GUNJALA GONDI LETTER UU
    (0x011D60, 0x011D65, Vowel_Independent),
    // Lo   [2] GUNJALA GONDI LETTER EE..GUNJALA GONDI LETTER AI
    (0x011D67, 0x011D68, Vowel_Independent),
    // Lo   [2] GUNJALA GONDI LETTER OO..GUNJALA GONDI LETTER AU
    (0x011D6A, 0x011D6B, Vowel_Independent),
    // Lo  [30] GUNJALA GONDI LETTER YA..GUNJALA GONDI LETTER SA
    (0x011D6C, 0x011D89, Consonant),
    // Mc   [5] GUNJALA GONDI VOWEL SIGN AA..GUNJALA GONDI VOWEL SIGN UU
    (0x011D8A, 0x011D8E, Vowel_Dependent),
    // Mn   [2] GUNJALA GONDI VOWEL SIGN EE..GUNJALA GONDI VOWEL SIGN AI
    (0x011D90, 0x011D91, Vowel_Dependent),
    // Mc   [2] GUNJALA GONDI VOWEL SIGN OO..GUNJALA GONDI VOWEL SIGN AU
    (0x011D93, 0x011D94, Vowel_Dependent),
    // Mn       GUNJALA GONDI SIGN ANUSVARA
    (0x011D95, 0x011D95, Bindu),
    // Mc       GUNJALA GONDI SIGN VISARGA
    (0x011D96, 0x011D96, Visarga),
    // Mn       GUNJALA GONDI VIRAMA
    (0x011D97, 0x011D97, Invisible_Stacker),
    // Nd  [10] GUNJALA GONDI DIGIT ZERO..GUNJALA GONDI DIGIT NINE
    (0x011DA0, 0x011DA9, Number),
    // Lo  [18] MAKASAR LETTER KA..MAKASAR LETTER A
    (0x011EE0, 0x011EF1, Consonant),
    // Lo       MAKASAR ANGKA
    (0x011EF2, 0x011EF2, Consonant_Placeholder),
    // Mn   [2] MAKASAR VOWEL SIGN I..MAKASAR VOWEL SIGN U
    // Mc   [2] MAKASAR VOWEL SIGN E..MAKASAR VOWEL SIGN O
    (0x011EF3, 0x011EF6, Vowel_Dependent),
];
