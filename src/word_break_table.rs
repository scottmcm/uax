use crate::property_enums::Word_Break;
use crate::lookup_table::LookupTable;
use Word_Break::*;

impl From<char> for Word_Break {
    #[inline]
    fn from(c: char) -> Self {
        return CHAR_TABLE.get_or(&c, Other);
    }
}

static CHAR_TABLE: LookupTable<char, Word_Break> = lookup_table![
    // So every possible input is always found in the table
    ('\u{0}', '\u{9}', Other),
    // Cc       <control-000A>
    ('\u{a}', '\u{a}', LF),
    // Cc   [2] <control-000B>..<control-000C>
    ('\u{b}', '\u{c}', Newline),
    // Cc       <control-000D>
    ('\u{d}', '\u{d}', CR),
    // Zs       SPACE
    ('\u{20}', '\u{20}', WSegSpace),
    // Po       QUOTATION MARK
    ('\u{22}', '\u{22}', Double_Quote),
    // Po       APOSTROPHE
    ('\u{27}', '\u{27}', Single_Quote),
    // Po       COMMA
    ('\u{2c}', '\u{2c}', MidNum),
    // Po       FULL STOP
    ('\u{2e}', '\u{2e}', MidNumLet),
    // Nd  [10] DIGIT ZERO..DIGIT NINE
    ('\u{30}', '\u{39}', Numeric),
    // Po       COLON
    ('\u{3a}', '\u{3a}', MidLetter),
    // Po       SEMICOLON
    ('\u{3b}', '\u{3b}', MidNum),
    // L&  [26] LATIN CAPITAL LETTER A..LATIN CAPITAL LETTER Z
    ('\u{41}', '\u{5a}', ALetter),
    // Pc       LOW LINE
    ('\u{5f}', '\u{5f}', ExtendNumLet),
    // L&  [26] LATIN SMALL LETTER A..LATIN SMALL LETTER Z
    ('\u{61}', '\u{7a}', ALetter),
    // Cc       <control-0085>
    ('\u{85}', '\u{85}', Newline),
    // Lo       FEMININE ORDINAL INDICATOR
    ('\u{aa}', '\u{aa}', ALetter),
    // Cf       SOFT HYPHEN
    ('\u{ad}', '\u{ad}', Format),
    // L&       MICRO SIGN
    ('\u{b5}', '\u{b5}', ALetter),
    // Po       MIDDLE DOT
    ('\u{b7}', '\u{b7}', MidLetter),
    // Lo       MASCULINE ORDINAL INDICATOR
    ('\u{ba}', '\u{ba}', ALetter),
    // L&  [23] LATIN CAPITAL LETTER A WITH GRAVE..LATIN CAPITAL LETTER O WITH DIAERESIS
    ('\u{c0}', '\u{d6}', ALetter),
    // L&  [31] LATIN CAPITAL LETTER O WITH STROKE..LATIN SMALL LETTER O WITH DIAERESIS
    ('\u{d8}', '\u{f6}', ALetter),
    // L& [195] LATIN SMALL LETTER O WITH STROKE..LATIN SMALL LETTER EZH WITH TAIL
    ('\u{f8}', '\u{1ba}', ALetter),
    // Lo       LATIN LETTER TWO WITH STROKE
    ('\u{1bb}', '\u{1bb}', ALetter),
    // L&   [4] LATIN CAPITAL LETTER TONE FIVE..LATIN LETTER WYNN
    ('\u{1bc}', '\u{1bf}', ALetter),
    // Lo   [4] LATIN LETTER DENTAL CLICK..LATIN LETTER RETROFLEX CLICK
    ('\u{1c0}', '\u{1c3}', ALetter),
    // L& [208] LATIN CAPITAL LETTER DZ WITH CARON..LATIN SMALL LETTER EZH WITH CURL
    ('\u{1c4}', '\u{293}', ALetter),
    // Lo       LATIN LETTER GLOTTAL STOP
    ('\u{294}', '\u{294}', ALetter),
    // L&  [27] LATIN LETTER PHARYNGEAL VOICED FRICATIVE..LATIN SMALL LETTER TURNED H WITH FISHHOOK AND TAIL
    ('\u{295}', '\u{2af}', ALetter),
    // Lm  [18] MODIFIER LETTER SMALL H..MODIFIER LETTER REVERSED GLOTTAL STOP
    ('\u{2b0}', '\u{2c1}', ALetter),
    // Sk   [4] MODIFIER LETTER LEFT ARROWHEAD..MODIFIER LETTER DOWN ARROWHEAD
    ('\u{2c2}', '\u{2c5}', ALetter),
    // Lm  [12] MODIFIER LETTER CIRCUMFLEX ACCENT..MODIFIER LETTER HALF TRIANGULAR COLON
    ('\u{2c6}', '\u{2d1}', ALetter),
    // Sk   [6] MODIFIER LETTER CENTRED RIGHT HALF RING..MODIFIER LETTER MINUS SIGN
    ('\u{2d2}', '\u{2d7}', ALetter),
    // Sk   [2] MODIFIER LETTER RHOTIC HOOK..MODIFIER LETTER CROSS ACCENT
    ('\u{2de}', '\u{2df}', ALetter),
    // Lm   [5] MODIFIER LETTER SMALL GAMMA..MODIFIER LETTER SMALL REVERSED GLOTTAL STOP
    ('\u{2e0}', '\u{2e4}', ALetter),
    // Lm       MODIFIER LETTER VOICING
    ('\u{2ec}', '\u{2ec}', ALetter),
    // Sk       MODIFIER LETTER UNASPIRATED
    ('\u{2ed}', '\u{2ed}', ALetter),
    // Lm       MODIFIER LETTER DOUBLE APOSTROPHE
    ('\u{2ee}', '\u{2ee}', ALetter),
    // Sk  [17] MODIFIER LETTER LOW DOWN ARROWHEAD..MODIFIER LETTER LOW LEFT ARROW
    ('\u{2ef}', '\u{2ff}', ALetter),
    // Mn [112] COMBINING GRAVE ACCENT..COMBINING LATIN SMALL LETTER X
    ('\u{300}', '\u{36f}', Extend),
    // L&   [4] GREEK CAPITAL LETTER HETA..GREEK SMALL LETTER ARCHAIC SAMPI
    ('\u{370}', '\u{373}', ALetter),
    // Lm       GREEK NUMERAL SIGN
    ('\u{374}', '\u{374}', ALetter),
    // L&   [2] GREEK CAPITAL LETTER PAMPHYLIAN DIGAMMA..GREEK SMALL LETTER PAMPHYLIAN DIGAMMA
    ('\u{376}', '\u{377}', ALetter),
    // Lm       GREEK YPOGEGRAMMENI
    ('\u{37a}', '\u{37a}', ALetter),
    // L&   [3] GREEK SMALL REVERSED LUNATE SIGMA SYMBOL..GREEK SMALL REVERSED DOTTED LUNATE SIGMA SYMBOL
    ('\u{37b}', '\u{37d}', ALetter),
    // Po       GREEK QUESTION MARK
    ('\u{37e}', '\u{37e}', MidNum),
    // L&       GREEK CAPITAL LETTER YOT
    ('\u{37f}', '\u{37f}', ALetter),
    // L&       GREEK CAPITAL LETTER ALPHA WITH TONOS
    ('\u{386}', '\u{386}', ALetter),
    // Po       GREEK ANO TELEIA
    ('\u{387}', '\u{387}', MidLetter),
    // L&   [3] GREEK CAPITAL LETTER EPSILON WITH TONOS..GREEK CAPITAL LETTER IOTA WITH TONOS
    ('\u{388}', '\u{38a}', ALetter),
    // L&       GREEK CAPITAL LETTER OMICRON WITH TONOS
    ('\u{38c}', '\u{38c}', ALetter),
    // L&  [20] GREEK CAPITAL LETTER UPSILON WITH TONOS..GREEK CAPITAL LETTER RHO
    ('\u{38e}', '\u{3a1}', ALetter),
    // L&  [83] GREEK CAPITAL LETTER SIGMA..GREEK LUNATE EPSILON SYMBOL
    ('\u{3a3}', '\u{3f5}', ALetter),
    // L& [139] GREEK CAPITAL LETTER SHO..CYRILLIC SMALL LETTER KOPPA
    ('\u{3f7}', '\u{481}', ALetter),
    // Mn   [5] COMBINING CYRILLIC TITLO..COMBINING CYRILLIC POKRYTIE
    ('\u{483}', '\u{487}', Extend),
    // Me   [2] COMBINING CYRILLIC HUNDRED THOUSANDS SIGN..COMBINING CYRILLIC MILLIONS SIGN
    ('\u{488}', '\u{489}', Extend),
    // L& [166] CYRILLIC CAPITAL LETTER SHORT I WITH TAIL..CYRILLIC SMALL LETTER EL WITH DESCENDER
    ('\u{48a}', '\u{52f}', ALetter),
    // L&  [38] ARMENIAN CAPITAL LETTER AYB..ARMENIAN CAPITAL LETTER FEH
    ('\u{531}', '\u{556}', ALetter),
    // Lm       ARMENIAN MODIFIER LETTER LEFT HALF RING
    ('\u{559}', '\u{559}', ALetter),
    // Po   [2] ARMENIAN EMPHASIS MARK..ARMENIAN EXCLAMATION MARK
    ('\u{55b}', '\u{55c}', ALetter),
    // Po       ARMENIAN QUESTION MARK
    ('\u{55e}', '\u{55e}', ALetter),
    // L&  [41] ARMENIAN SMALL LETTER TURNED AYB..ARMENIAN SMALL LETTER YI WITH STROKE
    ('\u{560}', '\u{588}', ALetter),
    // Po       ARMENIAN FULL STOP
    ('\u{589}', '\u{589}', MidNum),
    // Mn  [45] HEBREW ACCENT ETNAHTA..HEBREW POINT METEG
    ('\u{591}', '\u{5bd}', Extend),
    // Mn       HEBREW POINT RAFE
    ('\u{5bf}', '\u{5bf}', Extend),
    // Mn   [2] HEBREW POINT SHIN DOT..HEBREW POINT SIN DOT
    ('\u{5c1}', '\u{5c2}', Extend),
    // Mn   [2] HEBREW MARK UPPER DOT..HEBREW MARK LOWER DOT
    ('\u{5c4}', '\u{5c5}', Extend),
    // Mn       HEBREW POINT QAMATS QATAN
    ('\u{5c7}', '\u{5c7}', Extend),
    // Lo  [27] HEBREW LETTER ALEF..HEBREW LETTER TAV
    ('\u{5d0}', '\u{5ea}', Hebrew_Letter),
    // Lo   [4] HEBREW YOD TRIANGLE..HEBREW LIGATURE YIDDISH DOUBLE YOD
    ('\u{5ef}', '\u{5f2}', Hebrew_Letter),
    // Po       HEBREW PUNCTUATION GERESH
    ('\u{5f3}', '\u{5f3}', ALetter),
    // Po       HEBREW PUNCTUATION GERSHAYIM
    ('\u{5f4}', '\u{5f4}', MidLetter),
    // Cf   [6] ARABIC NUMBER SIGN..ARABIC NUMBER MARK ABOVE
    ('\u{600}', '\u{605}', Format),
    // Po   [2] ARABIC COMMA..ARABIC DATE SEPARATOR
    ('\u{60c}', '\u{60d}', MidNum),
    // Mn  [11] ARABIC SIGN SALLALLAHOU ALAYHE WASSALLAM..ARABIC SMALL KASRA
    ('\u{610}', '\u{61a}', Extend),
    // Cf       ARABIC LETTER MARK
    ('\u{61c}', '\u{61c}', Format),
    // Lo  [32] ARABIC LETTER KASHMIRI YEH..ARABIC LETTER FARSI YEH WITH THREE DOTS ABOVE
    ('\u{620}', '\u{63f}', ALetter),
    // Lm       ARABIC TATWEEL
    ('\u{640}', '\u{640}', ALetter),
    // Lo  [10] ARABIC LETTER FEH..ARABIC LETTER YEH
    ('\u{641}', '\u{64a}', ALetter),
    // Mn  [21] ARABIC FATHATAN..ARABIC WAVY HAMZA BELOW
    ('\u{64b}', '\u{65f}', Extend),
    // Nd  [10] ARABIC-INDIC DIGIT ZERO..ARABIC-INDIC DIGIT NINE
    ('\u{660}', '\u{669}', Numeric),
    // Po       ARABIC DECIMAL SEPARATOR
    ('\u{66b}', '\u{66b}', Numeric),
    // Po       ARABIC THOUSANDS SEPARATOR
    ('\u{66c}', '\u{66c}', MidNum),
    // Lo   [2] ARABIC LETTER DOTLESS BEH..ARABIC LETTER DOTLESS QAF
    ('\u{66e}', '\u{66f}', ALetter),
    // Mn       ARABIC LETTER SUPERSCRIPT ALEF
    ('\u{670}', '\u{670}', Extend),
    // Lo  [99] ARABIC LETTER ALEF WASLA..ARABIC LETTER YEH BARREE WITH HAMZA ABOVE
    ('\u{671}', '\u{6d3}', ALetter),
    // Lo       ARABIC LETTER AE
    ('\u{6d5}', '\u{6d5}', ALetter),
    // Mn   [7] ARABIC SMALL HIGH LIGATURE SAD WITH LAM WITH ALEF MAKSURA..ARABIC SMALL HIGH SEEN
    ('\u{6d6}', '\u{6dc}', Extend),
    // Cf       ARABIC END OF AYAH
    ('\u{6dd}', '\u{6dd}', Format),
    // Mn   [6] ARABIC SMALL HIGH ROUNDED ZERO..ARABIC SMALL HIGH MADDA
    ('\u{6df}', '\u{6e4}', Extend),
    // Lm   [2] ARABIC SMALL WAW..ARABIC SMALL YEH
    ('\u{6e5}', '\u{6e6}', ALetter),
    // Mn   [2] ARABIC SMALL HIGH YEH..ARABIC SMALL HIGH NOON
    ('\u{6e7}', '\u{6e8}', Extend),
    // Mn   [4] ARABIC EMPTY CENTRE LOW STOP..ARABIC SMALL LOW MEEM
    ('\u{6ea}', '\u{6ed}', Extend),
    // Lo   [2] ARABIC LETTER DAL WITH INVERTED V..ARABIC LETTER REH WITH INVERTED V
    ('\u{6ee}', '\u{6ef}', ALetter),
    // Nd  [10] EXTENDED ARABIC-INDIC DIGIT ZERO..EXTENDED ARABIC-INDIC DIGIT NINE
    ('\u{6f0}', '\u{6f9}', Numeric),
    // Lo   [3] ARABIC LETTER SHEEN WITH DOT BELOW..ARABIC LETTER GHAIN WITH DOT BELOW
    ('\u{6fa}', '\u{6fc}', ALetter),
    // Lo       ARABIC LETTER HEH WITH INVERTED V
    ('\u{6ff}', '\u{6ff}', ALetter),
    // Cf       SYRIAC ABBREVIATION MARK
    ('\u{70f}', '\u{70f}', Format),
    // Lo       SYRIAC LETTER ALAPH
    ('\u{710}', '\u{710}', ALetter),
    // Mn       SYRIAC LETTER SUPERSCRIPT ALAPH
    ('\u{711}', '\u{711}', Extend),
    // Lo  [30] SYRIAC LETTER BETH..SYRIAC LETTER PERSIAN DHALATH
    ('\u{712}', '\u{72f}', ALetter),
    // Mn  [27] SYRIAC PTHAHA ABOVE..SYRIAC BARREKH
    ('\u{730}', '\u{74a}', Extend),
    // Lo  [89] SYRIAC LETTER SOGDIAN ZHAIN..THAANA LETTER WAAVU
    ('\u{74d}', '\u{7a5}', ALetter),
    // Mn  [11] THAANA ABAFILI..THAANA SUKUN
    ('\u{7a6}', '\u{7b0}', Extend),
    // Lo       THAANA LETTER NAA
    ('\u{7b1}', '\u{7b1}', ALetter),
    // Nd  [10] NKO DIGIT ZERO..NKO DIGIT NINE
    ('\u{7c0}', '\u{7c9}', Numeric),
    // Lo  [33] NKO LETTER A..NKO LETTER JONA RA
    ('\u{7ca}', '\u{7ea}', ALetter),
    // Mn   [9] NKO COMBINING SHORT HIGH TONE..NKO COMBINING DOUBLE DOT ABOVE
    ('\u{7eb}', '\u{7f3}', Extend),
    // Lm   [2] NKO HIGH TONE APOSTROPHE..NKO LOW TONE APOSTROPHE
    ('\u{7f4}', '\u{7f5}', ALetter),
    // Po       NKO COMMA
    ('\u{7f8}', '\u{7f8}', MidNum),
    // Lm       NKO LAJANYALAN
    ('\u{7fa}', '\u{7fa}', ALetter),
    // Mn       NKO DANTAYALAN
    ('\u{7fd}', '\u{7fd}', Extend),
    // Lo  [22] SAMARITAN LETTER ALAF..SAMARITAN LETTER TAAF
    ('\u{800}', '\u{815}', ALetter),
    // Mn   [4] SAMARITAN MARK IN..SAMARITAN MARK DAGESH
    ('\u{816}', '\u{819}', Extend),
    // Lm       SAMARITAN MODIFIER LETTER EPENTHETIC YUT
    ('\u{81a}', '\u{81a}', ALetter),
    // Mn   [9] SAMARITAN MARK EPENTHETIC YUT..SAMARITAN VOWEL SIGN A
    ('\u{81b}', '\u{823}', Extend),
    // Lm       SAMARITAN MODIFIER LETTER SHORT A
    ('\u{824}', '\u{824}', ALetter),
    // Mn   [3] SAMARITAN VOWEL SIGN SHORT A..SAMARITAN VOWEL SIGN U
    ('\u{825}', '\u{827}', Extend),
    // Lm       SAMARITAN MODIFIER LETTER I
    ('\u{828}', '\u{828}', ALetter),
    // Mn   [5] SAMARITAN VOWEL SIGN LONG I..SAMARITAN MARK NEQUDAA
    ('\u{829}', '\u{82d}', Extend),
    // Lo  [25] MANDAIC LETTER HALQA..MANDAIC LETTER AIN
    ('\u{840}', '\u{858}', ALetter),
    // Mn   [3] MANDAIC AFFRICATION MARK..MANDAIC GEMINATION MARK
    ('\u{859}', '\u{85b}', Extend),
    // Lo  [11] SYRIAC LETTER MALAYALAM NGA..SYRIAC LETTER MALAYALAM SSA
    ('\u{860}', '\u{86a}', ALetter),
    // Lo  [21] ARABIC LETTER BEH WITH SMALL V BELOW..ARABIC LETTER KAF WITH DOT BELOW
    ('\u{8a0}', '\u{8b4}', ALetter),
    // Lo   [8] ARABIC LETTER BEH WITH SMALL MEEM ABOVE..ARABIC LETTER AFRICAN NOON
    ('\u{8b6}', '\u{8bd}', ALetter),
    // Mn  [15] ARABIC SMALL LOW WAW..ARABIC SMALL HIGH SIGN SAFHA
    ('\u{8d3}', '\u{8e1}', Extend),
    // Cf       ARABIC DISPUTED END OF AYAH
    ('\u{8e2}', '\u{8e2}', Format),
    // Mn  [32] ARABIC TURNED DAMMA BELOW..DEVANAGARI SIGN ANUSVARA
    ('\u{8e3}', '\u{902}', Extend),
    // Mc       DEVANAGARI SIGN VISARGA
    ('\u{903}', '\u{903}', Extend),
    // Lo  [54] DEVANAGARI LETTER SHORT A..DEVANAGARI LETTER HA
    ('\u{904}', '\u{939}', ALetter),
    // Mn       DEVANAGARI VOWEL SIGN OE
    ('\u{93a}', '\u{93a}', Extend),
    // Mc       DEVANAGARI VOWEL SIGN OOE
    ('\u{93b}', '\u{93b}', Extend),
    // Mn       DEVANAGARI SIGN NUKTA
    ('\u{93c}', '\u{93c}', Extend),
    // Lo       DEVANAGARI SIGN AVAGRAHA
    ('\u{93d}', '\u{93d}', ALetter),
    // Mc   [3] DEVANAGARI VOWEL SIGN AA..DEVANAGARI VOWEL SIGN II
    ('\u{93e}', '\u{940}', Extend),
    // Mn   [8] DEVANAGARI VOWEL SIGN U..DEVANAGARI VOWEL SIGN AI
    ('\u{941}', '\u{948}', Extend),
    // Mc   [4] DEVANAGARI VOWEL SIGN CANDRA O..DEVANAGARI VOWEL SIGN AU
    ('\u{949}', '\u{94c}', Extend),
    // Mn       DEVANAGARI SIGN VIRAMA
    ('\u{94d}', '\u{94d}', Extend),
    // Mc   [2] DEVANAGARI VOWEL SIGN PRISHTHAMATRA E..DEVANAGARI VOWEL SIGN AW
    ('\u{94e}', '\u{94f}', Extend),
    // Lo       DEVANAGARI OM
    ('\u{950}', '\u{950}', ALetter),
    // Mn   [7] DEVANAGARI STRESS SIGN UDATTA..DEVANAGARI VOWEL SIGN UUE
    ('\u{951}', '\u{957}', Extend),
    // Lo  [10] DEVANAGARI LETTER QA..DEVANAGARI LETTER VOCALIC LL
    ('\u{958}', '\u{961}', ALetter),
    // Mn   [2] DEVANAGARI VOWEL SIGN VOCALIC L..DEVANAGARI VOWEL SIGN VOCALIC LL
    ('\u{962}', '\u{963}', Extend),
    // Nd  [10] DEVANAGARI DIGIT ZERO..DEVANAGARI DIGIT NINE
    ('\u{966}', '\u{96f}', Numeric),
    // Lm       DEVANAGARI SIGN HIGH SPACING DOT
    ('\u{971}', '\u{971}', ALetter),
    // Lo  [15] DEVANAGARI LETTER CANDRA A..BENGALI ANJI
    ('\u{972}', '\u{980}', ALetter),
    // Mn       BENGALI SIGN CANDRABINDU
    ('\u{981}', '\u{981}', Extend),
    // Mc   [2] BENGALI SIGN ANUSVARA..BENGALI SIGN VISARGA
    ('\u{982}', '\u{983}', Extend),
    // Lo   [8] BENGALI LETTER A..BENGALI LETTER VOCALIC L
    ('\u{985}', '\u{98c}', ALetter),
    // Lo   [2] BENGALI LETTER E..BENGALI LETTER AI
    ('\u{98f}', '\u{990}', ALetter),
    // Lo  [22] BENGALI LETTER O..BENGALI LETTER NA
    ('\u{993}', '\u{9a8}', ALetter),
    // Lo   [7] BENGALI LETTER PA..BENGALI LETTER RA
    ('\u{9aa}', '\u{9b0}', ALetter),
    // Lo       BENGALI LETTER LA
    ('\u{9b2}', '\u{9b2}', ALetter),
    // Lo   [4] BENGALI LETTER SHA..BENGALI LETTER HA
    ('\u{9b6}', '\u{9b9}', ALetter),
    // Mn       BENGALI SIGN NUKTA
    ('\u{9bc}', '\u{9bc}', Extend),
    // Lo       BENGALI SIGN AVAGRAHA
    ('\u{9bd}', '\u{9bd}', ALetter),
    // Mc   [3] BENGALI VOWEL SIGN AA..BENGALI VOWEL SIGN II
    ('\u{9be}', '\u{9c0}', Extend),
    // Mn   [4] BENGALI VOWEL SIGN U..BENGALI VOWEL SIGN VOCALIC RR
    ('\u{9c1}', '\u{9c4}', Extend),
    // Mc   [2] BENGALI VOWEL SIGN E..BENGALI VOWEL SIGN AI
    ('\u{9c7}', '\u{9c8}', Extend),
    // Mc   [2] BENGALI VOWEL SIGN O..BENGALI VOWEL SIGN AU
    ('\u{9cb}', '\u{9cc}', Extend),
    // Mn       BENGALI SIGN VIRAMA
    ('\u{9cd}', '\u{9cd}', Extend),
    // Lo       BENGALI LETTER KHANDA TA
    ('\u{9ce}', '\u{9ce}', ALetter),
    // Mc       BENGALI AU LENGTH MARK
    ('\u{9d7}', '\u{9d7}', Extend),
    // Lo   [2] BENGALI LETTER RRA..BENGALI LETTER RHA
    ('\u{9dc}', '\u{9dd}', ALetter),
    // Lo   [3] BENGALI LETTER YYA..BENGALI LETTER VOCALIC LL
    ('\u{9df}', '\u{9e1}', ALetter),
    // Mn   [2] BENGALI VOWEL SIGN VOCALIC L..BENGALI VOWEL SIGN VOCALIC LL
    ('\u{9e2}', '\u{9e3}', Extend),
    // Nd  [10] BENGALI DIGIT ZERO..BENGALI DIGIT NINE
    ('\u{9e6}', '\u{9ef}', Numeric),
    // Lo   [2] BENGALI LETTER RA WITH MIDDLE DIAGONAL..BENGALI LETTER RA WITH LOWER DIAGONAL
    ('\u{9f0}', '\u{9f1}', ALetter),
    // Lo       BENGALI LETTER VEDIC ANUSVARA
    ('\u{9fc}', '\u{9fc}', ALetter),
    // Mn       BENGALI SANDHI MARK
    ('\u{9fe}', '\u{9fe}', Extend),
    // Mn   [2] GURMUKHI SIGN ADAK BINDI..GURMUKHI SIGN BINDI
    ('\u{a01}', '\u{a02}', Extend),
    // Mc       GURMUKHI SIGN VISARGA
    ('\u{a03}', '\u{a03}', Extend),
    // Lo   [6] GURMUKHI LETTER A..GURMUKHI LETTER UU
    ('\u{a05}', '\u{a0a}', ALetter),
    // Lo   [2] GURMUKHI LETTER EE..GURMUKHI LETTER AI
    ('\u{a0f}', '\u{a10}', ALetter),
    // Lo  [22] GURMUKHI LETTER OO..GURMUKHI LETTER NA
    ('\u{a13}', '\u{a28}', ALetter),
    // Lo   [7] GURMUKHI LETTER PA..GURMUKHI LETTER RA
    ('\u{a2a}', '\u{a30}', ALetter),
    // Lo   [2] GURMUKHI LETTER LA..GURMUKHI LETTER LLA
    ('\u{a32}', '\u{a33}', ALetter),
    // Lo   [2] GURMUKHI LETTER VA..GURMUKHI LETTER SHA
    ('\u{a35}', '\u{a36}', ALetter),
    // Lo   [2] GURMUKHI LETTER SA..GURMUKHI LETTER HA
    ('\u{a38}', '\u{a39}', ALetter),
    // Mn       GURMUKHI SIGN NUKTA
    ('\u{a3c}', '\u{a3c}', Extend),
    // Mc   [3] GURMUKHI VOWEL SIGN AA..GURMUKHI VOWEL SIGN II
    ('\u{a3e}', '\u{a40}', Extend),
    // Mn   [2] GURMUKHI VOWEL SIGN U..GURMUKHI VOWEL SIGN UU
    ('\u{a41}', '\u{a42}', Extend),
    // Mn   [2] GURMUKHI VOWEL SIGN EE..GURMUKHI VOWEL SIGN AI
    ('\u{a47}', '\u{a48}', Extend),
    // Mn   [3] GURMUKHI VOWEL SIGN OO..GURMUKHI SIGN VIRAMA
    ('\u{a4b}', '\u{a4d}', Extend),
    // Mn       GURMUKHI SIGN UDAAT
    ('\u{a51}', '\u{a51}', Extend),
    // Lo   [4] GURMUKHI LETTER KHHA..GURMUKHI LETTER RRA
    ('\u{a59}', '\u{a5c}', ALetter),
    // Lo       GURMUKHI LETTER FA
    ('\u{a5e}', '\u{a5e}', ALetter),
    // Nd  [10] GURMUKHI DIGIT ZERO..GURMUKHI DIGIT NINE
    ('\u{a66}', '\u{a6f}', Numeric),
    // Mn   [2] GURMUKHI TIPPI..GURMUKHI ADDAK
    ('\u{a70}', '\u{a71}', Extend),
    // Lo   [3] GURMUKHI IRI..GURMUKHI EK ONKAR
    ('\u{a72}', '\u{a74}', ALetter),
    // Mn       GURMUKHI SIGN YAKASH
    ('\u{a75}', '\u{a75}', Extend),
    // Mn   [2] GUJARATI SIGN CANDRABINDU..GUJARATI SIGN ANUSVARA
    ('\u{a81}', '\u{a82}', Extend),
    // Mc       GUJARATI SIGN VISARGA
    ('\u{a83}', '\u{a83}', Extend),
    // Lo   [9] GUJARATI LETTER A..GUJARATI VOWEL CANDRA E
    ('\u{a85}', '\u{a8d}', ALetter),
    // Lo   [3] GUJARATI LETTER E..GUJARATI VOWEL CANDRA O
    ('\u{a8f}', '\u{a91}', ALetter),
    // Lo  [22] GUJARATI LETTER O..GUJARATI LETTER NA
    ('\u{a93}', '\u{aa8}', ALetter),
    // Lo   [7] GUJARATI LETTER PA..GUJARATI LETTER RA
    ('\u{aaa}', '\u{ab0}', ALetter),
    // Lo   [2] GUJARATI LETTER LA..GUJARATI LETTER LLA
    ('\u{ab2}', '\u{ab3}', ALetter),
    // Lo   [5] GUJARATI LETTER VA..GUJARATI LETTER HA
    ('\u{ab5}', '\u{ab9}', ALetter),
    // Mn       GUJARATI SIGN NUKTA
    ('\u{abc}', '\u{abc}', Extend),
    // Lo       GUJARATI SIGN AVAGRAHA
    ('\u{abd}', '\u{abd}', ALetter),
    // Mc   [3] GUJARATI VOWEL SIGN AA..GUJARATI VOWEL SIGN II
    ('\u{abe}', '\u{ac0}', Extend),
    // Mn   [5] GUJARATI VOWEL SIGN U..GUJARATI VOWEL SIGN CANDRA E
    ('\u{ac1}', '\u{ac5}', Extend),
    // Mn   [2] GUJARATI VOWEL SIGN E..GUJARATI VOWEL SIGN AI
    ('\u{ac7}', '\u{ac8}', Extend),
    // Mc       GUJARATI VOWEL SIGN CANDRA O
    ('\u{ac9}', '\u{ac9}', Extend),
    // Mc   [2] GUJARATI VOWEL SIGN O..GUJARATI VOWEL SIGN AU
    ('\u{acb}', '\u{acc}', Extend),
    // Mn       GUJARATI SIGN VIRAMA
    ('\u{acd}', '\u{acd}', Extend),
    // Lo       GUJARATI OM
    ('\u{ad0}', '\u{ad0}', ALetter),
    // Lo   [2] GUJARATI LETTER VOCALIC RR..GUJARATI LETTER VOCALIC LL
    ('\u{ae0}', '\u{ae1}', ALetter),
    // Mn   [2] GUJARATI VOWEL SIGN VOCALIC L..GUJARATI VOWEL SIGN VOCALIC LL
    ('\u{ae2}', '\u{ae3}', Extend),
    // Nd  [10] GUJARATI DIGIT ZERO..GUJARATI DIGIT NINE
    ('\u{ae6}', '\u{aef}', Numeric),
    // Lo       GUJARATI LETTER ZHA
    ('\u{af9}', '\u{af9}', ALetter),
    // Mn   [6] GUJARATI SIGN SUKUN..GUJARATI SIGN TWO-CIRCLE NUKTA ABOVE
    ('\u{afa}', '\u{aff}', Extend),
    // Mn       ORIYA SIGN CANDRABINDU
    ('\u{b01}', '\u{b01}', Extend),
    // Mc   [2] ORIYA SIGN ANUSVARA..ORIYA SIGN VISARGA
    ('\u{b02}', '\u{b03}', Extend),
    // Lo   [8] ORIYA LETTER A..ORIYA LETTER VOCALIC L
    ('\u{b05}', '\u{b0c}', ALetter),
    // Lo   [2] ORIYA LETTER E..ORIYA LETTER AI
    ('\u{b0f}', '\u{b10}', ALetter),
    // Lo  [22] ORIYA LETTER O..ORIYA LETTER NA
    ('\u{b13}', '\u{b28}', ALetter),
    // Lo   [7] ORIYA LETTER PA..ORIYA LETTER RA
    ('\u{b2a}', '\u{b30}', ALetter),
    // Lo   [2] ORIYA LETTER LA..ORIYA LETTER LLA
    ('\u{b32}', '\u{b33}', ALetter),
    // Lo   [5] ORIYA LETTER VA..ORIYA LETTER HA
    ('\u{b35}', '\u{b39}', ALetter),
    // Mn       ORIYA SIGN NUKTA
    ('\u{b3c}', '\u{b3c}', Extend),
    // Lo       ORIYA SIGN AVAGRAHA
    ('\u{b3d}', '\u{b3d}', ALetter),
    // Mc       ORIYA VOWEL SIGN AA
    ('\u{b3e}', '\u{b3e}', Extend),
    // Mn       ORIYA VOWEL SIGN I
    ('\u{b3f}', '\u{b3f}', Extend),
    // Mc       ORIYA VOWEL SIGN II
    ('\u{b40}', '\u{b40}', Extend),
    // Mn   [4] ORIYA VOWEL SIGN U..ORIYA VOWEL SIGN VOCALIC RR
    ('\u{b41}', '\u{b44}', Extend),
    // Mc   [2] ORIYA VOWEL SIGN E..ORIYA VOWEL SIGN AI
    ('\u{b47}', '\u{b48}', Extend),
    // Mc   [2] ORIYA VOWEL SIGN O..ORIYA VOWEL SIGN AU
    ('\u{b4b}', '\u{b4c}', Extend),
    // Mn       ORIYA SIGN VIRAMA
    ('\u{b4d}', '\u{b4d}', Extend),
    // Mn       ORIYA AI LENGTH MARK
    ('\u{b56}', '\u{b56}', Extend),
    // Mc       ORIYA AU LENGTH MARK
    ('\u{b57}', '\u{b57}', Extend),
    // Lo   [2] ORIYA LETTER RRA..ORIYA LETTER RHA
    ('\u{b5c}', '\u{b5d}', ALetter),
    // Lo   [3] ORIYA LETTER YYA..ORIYA LETTER VOCALIC LL
    ('\u{b5f}', '\u{b61}', ALetter),
    // Mn   [2] ORIYA VOWEL SIGN VOCALIC L..ORIYA VOWEL SIGN VOCALIC LL
    ('\u{b62}', '\u{b63}', Extend),
    // Nd  [10] ORIYA DIGIT ZERO..ORIYA DIGIT NINE
    ('\u{b66}', '\u{b6f}', Numeric),
    // Lo       ORIYA LETTER WA
    ('\u{b71}', '\u{b71}', ALetter),
    // Mn       TAMIL SIGN ANUSVARA
    ('\u{b82}', '\u{b82}', Extend),
    // Lo       TAMIL SIGN VISARGA
    ('\u{b83}', '\u{b83}', ALetter),
    // Lo   [6] TAMIL LETTER A..TAMIL LETTER UU
    ('\u{b85}', '\u{b8a}', ALetter),
    // Lo   [3] TAMIL LETTER E..TAMIL LETTER AI
    ('\u{b8e}', '\u{b90}', ALetter),
    // Lo   [4] TAMIL LETTER O..TAMIL LETTER KA
    ('\u{b92}', '\u{b95}', ALetter),
    // Lo   [2] TAMIL LETTER NGA..TAMIL LETTER CA
    ('\u{b99}', '\u{b9a}', ALetter),
    // Lo       TAMIL LETTER JA
    ('\u{b9c}', '\u{b9c}', ALetter),
    // Lo   [2] TAMIL LETTER NYA..TAMIL LETTER TTA
    ('\u{b9e}', '\u{b9f}', ALetter),
    // Lo   [2] TAMIL LETTER NNA..TAMIL LETTER TA
    ('\u{ba3}', '\u{ba4}', ALetter),
    // Lo   [3] TAMIL LETTER NA..TAMIL LETTER PA
    ('\u{ba8}', '\u{baa}', ALetter),
    // Lo  [12] TAMIL LETTER MA..TAMIL LETTER HA
    ('\u{bae}', '\u{bb9}', ALetter),
    // Mc   [2] TAMIL VOWEL SIGN AA..TAMIL VOWEL SIGN I
    ('\u{bbe}', '\u{bbf}', Extend),
    // Mn       TAMIL VOWEL SIGN II
    ('\u{bc0}', '\u{bc0}', Extend),
    // Mc   [2] TAMIL VOWEL SIGN U..TAMIL VOWEL SIGN UU
    ('\u{bc1}', '\u{bc2}', Extend),
    // Mc   [3] TAMIL VOWEL SIGN E..TAMIL VOWEL SIGN AI
    ('\u{bc6}', '\u{bc8}', Extend),
    // Mc   [3] TAMIL VOWEL SIGN O..TAMIL VOWEL SIGN AU
    ('\u{bca}', '\u{bcc}', Extend),
    // Mn       TAMIL SIGN VIRAMA
    ('\u{bcd}', '\u{bcd}', Extend),
    // Lo       TAMIL OM
    ('\u{bd0}', '\u{bd0}', ALetter),
    // Mc       TAMIL AU LENGTH MARK
    ('\u{bd7}', '\u{bd7}', Extend),
    // Nd  [10] TAMIL DIGIT ZERO..TAMIL DIGIT NINE
    ('\u{be6}', '\u{bef}', Numeric),
    // Mn       TELUGU SIGN COMBINING CANDRABINDU ABOVE
    ('\u{c00}', '\u{c00}', Extend),
    // Mc   [3] TELUGU SIGN CANDRABINDU..TELUGU SIGN VISARGA
    ('\u{c01}', '\u{c03}', Extend),
    // Mn       TELUGU SIGN COMBINING ANUSVARA ABOVE
    ('\u{c04}', '\u{c04}', Extend),
    // Lo   [8] TELUGU LETTER A..TELUGU LETTER VOCALIC L
    ('\u{c05}', '\u{c0c}', ALetter),
    // Lo   [3] TELUGU LETTER E..TELUGU LETTER AI
    ('\u{c0e}', '\u{c10}', ALetter),
    // Lo  [23] TELUGU LETTER O..TELUGU LETTER NA
    ('\u{c12}', '\u{c28}', ALetter),
    // Lo  [16] TELUGU LETTER PA..TELUGU LETTER HA
    ('\u{c2a}', '\u{c39}', ALetter),
    // Lo       TELUGU SIGN AVAGRAHA
    ('\u{c3d}', '\u{c3d}', ALetter),
    // Mn   [3] TELUGU VOWEL SIGN AA..TELUGU VOWEL SIGN II
    ('\u{c3e}', '\u{c40}', Extend),
    // Mc   [4] TELUGU VOWEL SIGN U..TELUGU VOWEL SIGN VOCALIC RR
    ('\u{c41}', '\u{c44}', Extend),
    // Mn   [3] TELUGU VOWEL SIGN E..TELUGU VOWEL SIGN AI
    ('\u{c46}', '\u{c48}', Extend),
    // Mn   [4] TELUGU VOWEL SIGN O..TELUGU SIGN VIRAMA
    ('\u{c4a}', '\u{c4d}', Extend),
    // Mn   [2] TELUGU LENGTH MARK..TELUGU AI LENGTH MARK
    ('\u{c55}', '\u{c56}', Extend),
    // Lo   [3] TELUGU LETTER TSA..TELUGU LETTER RRRA
    ('\u{c58}', '\u{c5a}', ALetter),
    // Lo   [2] TELUGU LETTER VOCALIC RR..TELUGU LETTER VOCALIC LL
    ('\u{c60}', '\u{c61}', ALetter),
    // Mn   [2] TELUGU VOWEL SIGN VOCALIC L..TELUGU VOWEL SIGN VOCALIC LL
    ('\u{c62}', '\u{c63}', Extend),
    // Nd  [10] TELUGU DIGIT ZERO..TELUGU DIGIT NINE
    ('\u{c66}', '\u{c6f}', Numeric),
    // Lo       KANNADA SIGN SPACING CANDRABINDU
    ('\u{c80}', '\u{c80}', ALetter),
    // Mn       KANNADA SIGN CANDRABINDU
    ('\u{c81}', '\u{c81}', Extend),
    // Mc   [2] KANNADA SIGN ANUSVARA..KANNADA SIGN VISARGA
    ('\u{c82}', '\u{c83}', Extend),
    // Lo   [8] KANNADA LETTER A..KANNADA LETTER VOCALIC L
    ('\u{c85}', '\u{c8c}', ALetter),
    // Lo   [3] KANNADA LETTER E..KANNADA LETTER AI
    ('\u{c8e}', '\u{c90}', ALetter),
    // Lo  [23] KANNADA LETTER O..KANNADA LETTER NA
    ('\u{c92}', '\u{ca8}', ALetter),
    // Lo  [10] KANNADA LETTER PA..KANNADA LETTER LLA
    ('\u{caa}', '\u{cb3}', ALetter),
    // Lo   [5] KANNADA LETTER VA..KANNADA LETTER HA
    ('\u{cb5}', '\u{cb9}', ALetter),
    // Mn       KANNADA SIGN NUKTA
    ('\u{cbc}', '\u{cbc}', Extend),
    // Lo       KANNADA SIGN AVAGRAHA
    ('\u{cbd}', '\u{cbd}', ALetter),
    // Mc       KANNADA VOWEL SIGN AA
    ('\u{cbe}', '\u{cbe}', Extend),
    // Mn       KANNADA VOWEL SIGN I
    ('\u{cbf}', '\u{cbf}', Extend),
    // Mc   [5] KANNADA VOWEL SIGN II..KANNADA VOWEL SIGN VOCALIC RR
    ('\u{cc0}', '\u{cc4}', Extend),
    // Mn       KANNADA VOWEL SIGN E
    ('\u{cc6}', '\u{cc6}', Extend),
    // Mc   [2] KANNADA VOWEL SIGN EE..KANNADA VOWEL SIGN AI
    ('\u{cc7}', '\u{cc8}', Extend),
    // Mc   [2] KANNADA VOWEL SIGN O..KANNADA VOWEL SIGN OO
    ('\u{cca}', '\u{ccb}', Extend),
    // Mn   [2] KANNADA VOWEL SIGN AU..KANNADA SIGN VIRAMA
    ('\u{ccc}', '\u{ccd}', Extend),
    // Mc   [2] KANNADA LENGTH MARK..KANNADA AI LENGTH MARK
    ('\u{cd5}', '\u{cd6}', Extend),
    // Lo       KANNADA LETTER FA
    ('\u{cde}', '\u{cde}', ALetter),
    // Lo   [2] KANNADA LETTER VOCALIC RR..KANNADA LETTER VOCALIC LL
    ('\u{ce0}', '\u{ce1}', ALetter),
    // Mn   [2] KANNADA VOWEL SIGN VOCALIC L..KANNADA VOWEL SIGN VOCALIC LL
    ('\u{ce2}', '\u{ce3}', Extend),
    // Nd  [10] KANNADA DIGIT ZERO..KANNADA DIGIT NINE
    ('\u{ce6}', '\u{cef}', Numeric),
    // Lo   [2] KANNADA SIGN JIHVAMULIYA..KANNADA SIGN UPADHMANIYA
    ('\u{cf1}', '\u{cf2}', ALetter),
    // Mn   [2] MALAYALAM SIGN COMBINING ANUSVARA ABOVE..MALAYALAM SIGN CANDRABINDU
    ('\u{d00}', '\u{d01}', Extend),
    // Mc   [2] MALAYALAM SIGN ANUSVARA..MALAYALAM SIGN VISARGA
    ('\u{d02}', '\u{d03}', Extend),
    // Lo   [8] MALAYALAM LETTER A..MALAYALAM LETTER VOCALIC L
    ('\u{d05}', '\u{d0c}', ALetter),
    // Lo   [3] MALAYALAM LETTER E..MALAYALAM LETTER AI
    ('\u{d0e}', '\u{d10}', ALetter),
    // Lo  [41] MALAYALAM LETTER O..MALAYALAM LETTER TTTA
    ('\u{d12}', '\u{d3a}', ALetter),
    // Mn   [2] MALAYALAM SIGN VERTICAL BAR VIRAMA..MALAYALAM SIGN CIRCULAR VIRAMA
    ('\u{d3b}', '\u{d3c}', Extend),
    // Lo       MALAYALAM SIGN AVAGRAHA
    ('\u{d3d}', '\u{d3d}', ALetter),
    // Mc   [3] MALAYALAM VOWEL SIGN AA..MALAYALAM VOWEL SIGN II
    ('\u{d3e}', '\u{d40}', Extend),
    // Mn   [4] MALAYALAM VOWEL SIGN U..MALAYALAM VOWEL SIGN VOCALIC RR
    ('\u{d41}', '\u{d44}', Extend),
    // Mc   [3] MALAYALAM VOWEL SIGN E..MALAYALAM VOWEL SIGN AI
    ('\u{d46}', '\u{d48}', Extend),
    // Mc   [3] MALAYALAM VOWEL SIGN O..MALAYALAM VOWEL SIGN AU
    ('\u{d4a}', '\u{d4c}', Extend),
    // Mn       MALAYALAM SIGN VIRAMA
    ('\u{d4d}', '\u{d4d}', Extend),
    // Lo       MALAYALAM LETTER DOT REPH
    ('\u{d4e}', '\u{d4e}', ALetter),
    // Lo   [3] MALAYALAM LETTER CHILLU M..MALAYALAM LETTER CHILLU LLL
    ('\u{d54}', '\u{d56}', ALetter),
    // Mc       MALAYALAM AU LENGTH MARK
    ('\u{d57}', '\u{d57}', Extend),
    // Lo   [3] MALAYALAM LETTER ARCHAIC II..MALAYALAM LETTER VOCALIC LL
    ('\u{d5f}', '\u{d61}', ALetter),
    // Mn   [2] MALAYALAM VOWEL SIGN VOCALIC L..MALAYALAM VOWEL SIGN VOCALIC LL
    ('\u{d62}', '\u{d63}', Extend),
    // Nd  [10] MALAYALAM DIGIT ZERO..MALAYALAM DIGIT NINE
    ('\u{d66}', '\u{d6f}', Numeric),
    // Lo   [6] MALAYALAM LETTER CHILLU NN..MALAYALAM LETTER CHILLU K
    ('\u{d7a}', '\u{d7f}', ALetter),
    // Mc   [2] SINHALA SIGN ANUSVARAYA..SINHALA SIGN VISARGAYA
    ('\u{d82}', '\u{d83}', Extend),
    // Lo  [18] SINHALA LETTER AYANNA..SINHALA LETTER AUYANNA
    ('\u{d85}', '\u{d96}', ALetter),
    // Lo  [24] SINHALA LETTER ALPAPRAANA KAYANNA..SINHALA LETTER DANTAJA NAYANNA
    ('\u{d9a}', '\u{db1}', ALetter),
    // Lo   [9] SINHALA LETTER SANYAKA DAYANNA..SINHALA LETTER RAYANNA
    ('\u{db3}', '\u{dbb}', ALetter),
    // Lo       SINHALA LETTER DANTAJA LAYANNA
    ('\u{dbd}', '\u{dbd}', ALetter),
    // Lo   [7] SINHALA LETTER VAYANNA..SINHALA LETTER FAYANNA
    ('\u{dc0}', '\u{dc6}', ALetter),
    // Mn       SINHALA SIGN AL-LAKUNA
    ('\u{dca}', '\u{dca}', Extend),
    // Mc   [3] SINHALA VOWEL SIGN AELA-PILLA..SINHALA VOWEL SIGN DIGA AEDA-PILLA
    ('\u{dcf}', '\u{dd1}', Extend),
    // Mn   [3] SINHALA VOWEL SIGN KETTI IS-PILLA..SINHALA VOWEL SIGN KETTI PAA-PILLA
    ('\u{dd2}', '\u{dd4}', Extend),
    // Mn       SINHALA VOWEL SIGN DIGA PAA-PILLA
    ('\u{dd6}', '\u{dd6}', Extend),
    // Mc   [8] SINHALA VOWEL SIGN GAETTA-PILLA..SINHALA VOWEL SIGN GAYANUKITTA
    ('\u{dd8}', '\u{ddf}', Extend),
    // Nd  [10] SINHALA LITH DIGIT ZERO..SINHALA LITH DIGIT NINE
    ('\u{de6}', '\u{def}', Numeric),
    // Mc   [2] SINHALA VOWEL SIGN DIGA GAETTA-PILLA..SINHALA VOWEL SIGN DIGA GAYANUKITTA
    ('\u{df2}', '\u{df3}', Extend),
    // Mn       THAI CHARACTER MAI HAN-AKAT
    ('\u{e31}', '\u{e31}', Extend),
    // Mn   [7] THAI CHARACTER SARA I..THAI CHARACTER PHINTHU
    ('\u{e34}', '\u{e3a}', Extend),
    // Mn   [8] THAI CHARACTER MAITAIKHU..THAI CHARACTER YAMAKKAN
    ('\u{e47}', '\u{e4e}', Extend),
    // Nd  [10] THAI DIGIT ZERO..THAI DIGIT NINE
    ('\u{e50}', '\u{e59}', Numeric),
    // Mn       LAO VOWEL SIGN MAI KAN
    ('\u{eb1}', '\u{eb1}', Extend),
    // Mn   [9] LAO VOWEL SIGN I..LAO SEMIVOWEL SIGN LO
    ('\u{eb4}', '\u{ebc}', Extend),
    // Mn   [6] LAO TONE MAI EK..LAO NIGGAHITA
    ('\u{ec8}', '\u{ecd}', Extend),
    // Nd  [10] LAO DIGIT ZERO..LAO DIGIT NINE
    ('\u{ed0}', '\u{ed9}', Numeric),
    // Lo       TIBETAN SYLLABLE OM
    ('\u{f00}', '\u{f00}', ALetter),
    // Mn   [2] TIBETAN ASTROLOGICAL SIGN -KHYUD PA..TIBETAN ASTROLOGICAL SIGN SDONG TSHUGS
    ('\u{f18}', '\u{f19}', Extend),
    // Nd  [10] TIBETAN DIGIT ZERO..TIBETAN DIGIT NINE
    ('\u{f20}', '\u{f29}', Numeric),
    // Mn       TIBETAN MARK NGAS BZUNG NYI ZLA
    ('\u{f35}', '\u{f35}', Extend),
    // Mn       TIBETAN MARK NGAS BZUNG SGOR RTAGS
    ('\u{f37}', '\u{f37}', Extend),
    // Mn       TIBETAN MARK TSA -PHRU
    ('\u{f39}', '\u{f39}', Extend),
    // Mc   [2] TIBETAN SIGN YAR TSHES..TIBETAN SIGN MAR TSHES
    ('\u{f3e}', '\u{f3f}', Extend),
    // Lo   [8] TIBETAN LETTER KA..TIBETAN LETTER JA
    ('\u{f40}', '\u{f47}', ALetter),
    // Lo  [36] TIBETAN LETTER NYA..TIBETAN LETTER RRA
    ('\u{f49}', '\u{f6c}', ALetter),
    // Mn  [14] TIBETAN VOWEL SIGN AA..TIBETAN SIGN RJES SU NGA RO
    ('\u{f71}', '\u{f7e}', Extend),
    // Mc       TIBETAN SIGN RNAM BCAD
    ('\u{f7f}', '\u{f7f}', Extend),
    // Mn   [5] TIBETAN VOWEL SIGN REVERSED I..TIBETAN MARK HALANTA
    ('\u{f80}', '\u{f84}', Extend),
    // Mn   [2] TIBETAN SIGN LCI RTAGS..TIBETAN SIGN YANG RTAGS
    ('\u{f86}', '\u{f87}', Extend),
    // Lo   [5] TIBETAN SIGN LCE TSA CAN..TIBETAN SIGN INVERTED MCHU CAN
    ('\u{f88}', '\u{f8c}', ALetter),
    // Mn  [11] TIBETAN SUBJOINED SIGN LCE TSA CAN..TIBETAN SUBJOINED LETTER JA
    ('\u{f8d}', '\u{f97}', Extend),
    // Mn  [36] TIBETAN SUBJOINED LETTER NYA..TIBETAN SUBJOINED LETTER FIXED-FORM RA
    ('\u{f99}', '\u{fbc}', Extend),
    // Mn       TIBETAN SYMBOL PADMA GDAN
    ('\u{fc6}', '\u{fc6}', Extend),
    // Mc   [2] MYANMAR VOWEL SIGN TALL AA..MYANMAR VOWEL SIGN AA
    ('\u{102b}', '\u{102c}', Extend),
    // Mn   [4] MYANMAR VOWEL SIGN I..MYANMAR VOWEL SIGN UU
    ('\u{102d}', '\u{1030}', Extend),
    // Mc       MYANMAR VOWEL SIGN E
    ('\u{1031}', '\u{1031}', Extend),
    // Mn   [6] MYANMAR VOWEL SIGN AI..MYANMAR SIGN DOT BELOW
    ('\u{1032}', '\u{1037}', Extend),
    // Mc       MYANMAR SIGN VISARGA
    ('\u{1038}', '\u{1038}', Extend),
    // Mn   [2] MYANMAR SIGN VIRAMA..MYANMAR SIGN ASAT
    ('\u{1039}', '\u{103a}', Extend),
    // Mc   [2] MYANMAR CONSONANT SIGN MEDIAL YA..MYANMAR CONSONANT SIGN MEDIAL RA
    ('\u{103b}', '\u{103c}', Extend),
    // Mn   [2] MYANMAR CONSONANT SIGN MEDIAL WA..MYANMAR CONSONANT SIGN MEDIAL HA
    ('\u{103d}', '\u{103e}', Extend),
    // Nd  [10] MYANMAR DIGIT ZERO..MYANMAR DIGIT NINE
    ('\u{1040}', '\u{1049}', Numeric),
    // Mc   [2] MYANMAR VOWEL SIGN VOCALIC R..MYANMAR VOWEL SIGN VOCALIC RR
    ('\u{1056}', '\u{1057}', Extend),
    // Mn   [2] MYANMAR VOWEL SIGN VOCALIC L..MYANMAR VOWEL SIGN VOCALIC LL
    ('\u{1058}', '\u{1059}', Extend),
    // Mn   [3] MYANMAR CONSONANT SIGN MON MEDIAL NA..MYANMAR CONSONANT SIGN MON MEDIAL LA
    ('\u{105e}', '\u{1060}', Extend),
    // Mc   [3] MYANMAR VOWEL SIGN SGAW KAREN EU..MYANMAR TONE MARK SGAW KAREN KE PHO
    ('\u{1062}', '\u{1064}', Extend),
    // Mc   [7] MYANMAR VOWEL SIGN WESTERN PWO KAREN EU..MYANMAR SIGN WESTERN PWO KAREN TONE-5
    ('\u{1067}', '\u{106d}', Extend),
    // Mn   [4] MYANMAR VOWEL SIGN GEBA KAREN I..MYANMAR VOWEL SIGN KAYAH EE
    ('\u{1071}', '\u{1074}', Extend),
    // Mn       MYANMAR CONSONANT SIGN SHAN MEDIAL WA
    ('\u{1082}', '\u{1082}', Extend),
    // Mc   [2] MYANMAR VOWEL SIGN SHAN AA..MYANMAR VOWEL SIGN SHAN E
    ('\u{1083}', '\u{1084}', Extend),
    // Mn   [2] MYANMAR VOWEL SIGN SHAN E ABOVE..MYANMAR VOWEL SIGN SHAN FINAL Y
    ('\u{1085}', '\u{1086}', Extend),
    // Mc   [6] MYANMAR SIGN SHAN TONE-2..MYANMAR SIGN SHAN COUNCIL TONE-3
    ('\u{1087}', '\u{108c}', Extend),
    // Mn       MYANMAR SIGN SHAN COUNCIL EMPHATIC TONE
    ('\u{108d}', '\u{108d}', Extend),
    // Mc       MYANMAR SIGN RUMAI PALAUNG TONE-5
    ('\u{108f}', '\u{108f}', Extend),
    // Nd  [10] MYANMAR SHAN DIGIT ZERO..MYANMAR SHAN DIGIT NINE
    ('\u{1090}', '\u{1099}', Numeric),
    // Mc   [3] MYANMAR SIGN KHAMTI TONE-1..MYANMAR VOWEL SIGN AITON A
    ('\u{109a}', '\u{109c}', Extend),
    // Mn       MYANMAR VOWEL SIGN AITON AI
    ('\u{109d}', '\u{109d}', Extend),
    // L&  [38] GEORGIAN CAPITAL LETTER AN..GEORGIAN CAPITAL LETTER HOE
    ('\u{10a0}', '\u{10c5}', ALetter),
    // L&       GEORGIAN CAPITAL LETTER YN
    ('\u{10c7}', '\u{10c7}', ALetter),
    // L&       GEORGIAN CAPITAL LETTER AEN
    ('\u{10cd}', '\u{10cd}', ALetter),
    // L&  [43] GEORGIAN LETTER AN..GEORGIAN LETTER AIN
    ('\u{10d0}', '\u{10fa}', ALetter),
    // Lm       MODIFIER LETTER GEORGIAN NAR
    ('\u{10fc}', '\u{10fc}', ALetter),
    // L&   [3] GEORGIAN LETTER AEN..GEORGIAN LETTER LABIAL SIGN
    ('\u{10fd}', '\u{10ff}', ALetter),
    // Lo [329] HANGUL CHOSEONG KIYEOK..ETHIOPIC SYLLABLE QWA
    ('\u{1100}', '\u{1248}', ALetter),
    // Lo   [4] ETHIOPIC SYLLABLE QWI..ETHIOPIC SYLLABLE QWE
    ('\u{124a}', '\u{124d}', ALetter),
    // Lo   [7] ETHIOPIC SYLLABLE QHA..ETHIOPIC SYLLABLE QHO
    ('\u{1250}', '\u{1256}', ALetter),
    // Lo       ETHIOPIC SYLLABLE QHWA
    ('\u{1258}', '\u{1258}', ALetter),
    // Lo   [4] ETHIOPIC SYLLABLE QHWI..ETHIOPIC SYLLABLE QHWE
    ('\u{125a}', '\u{125d}', ALetter),
    // Lo  [41] ETHIOPIC SYLLABLE BA..ETHIOPIC SYLLABLE XWA
    ('\u{1260}', '\u{1288}', ALetter),
    // Lo   [4] ETHIOPIC SYLLABLE XWI..ETHIOPIC SYLLABLE XWE
    ('\u{128a}', '\u{128d}', ALetter),
    // Lo  [33] ETHIOPIC SYLLABLE NA..ETHIOPIC SYLLABLE KWA
    ('\u{1290}', '\u{12b0}', ALetter),
    // Lo   [4] ETHIOPIC SYLLABLE KWI..ETHIOPIC SYLLABLE KWE
    ('\u{12b2}', '\u{12b5}', ALetter),
    // Lo   [7] ETHIOPIC SYLLABLE KXA..ETHIOPIC SYLLABLE KXO
    ('\u{12b8}', '\u{12be}', ALetter),
    // Lo       ETHIOPIC SYLLABLE KXWA
    ('\u{12c0}', '\u{12c0}', ALetter),
    // Lo   [4] ETHIOPIC SYLLABLE KXWI..ETHIOPIC SYLLABLE KXWE
    ('\u{12c2}', '\u{12c5}', ALetter),
    // Lo  [15] ETHIOPIC SYLLABLE WA..ETHIOPIC SYLLABLE PHARYNGEAL O
    ('\u{12c8}', '\u{12d6}', ALetter),
    // Lo  [57] ETHIOPIC SYLLABLE ZA..ETHIOPIC SYLLABLE GWA
    ('\u{12d8}', '\u{1310}', ALetter),
    // Lo   [4] ETHIOPIC SYLLABLE GWI..ETHIOPIC SYLLABLE GWE
    ('\u{1312}', '\u{1315}', ALetter),
    // Lo  [67] ETHIOPIC SYLLABLE GGA..ETHIOPIC SYLLABLE FYA
    ('\u{1318}', '\u{135a}', ALetter),
    // Mn   [3] ETHIOPIC COMBINING GEMINATION AND VOWEL LENGTH MARK..ETHIOPIC COMBINING GEMINATION MARK
    ('\u{135d}', '\u{135f}', Extend),
    // Lo  [16] ETHIOPIC SYLLABLE SEBATBEIT MWA..ETHIOPIC SYLLABLE PWE
    ('\u{1380}', '\u{138f}', ALetter),
    // L&  [86] CHEROKEE LETTER A..CHEROKEE LETTER MV
    ('\u{13a0}', '\u{13f5}', ALetter),
    // L&   [6] CHEROKEE SMALL LETTER YE..CHEROKEE SMALL LETTER MV
    ('\u{13f8}', '\u{13fd}', ALetter),
    // Lo [620] CANADIAN SYLLABICS E..CANADIAN SYLLABICS CARRIER TTSA
    ('\u{1401}', '\u{166c}', ALetter),
    // Lo  [17] CANADIAN SYLLABICS QAI..CANADIAN SYLLABICS BLACKFOOT W
    ('\u{166f}', '\u{167f}', ALetter),
    // Zs       OGHAM SPACE MARK
    ('\u{1680}', '\u{1680}', WSegSpace),
    // Lo  [26] OGHAM LETTER BEITH..OGHAM LETTER PEITH
    ('\u{1681}', '\u{169a}', ALetter),
    // Lo  [75] RUNIC LETTER FEHU FEOH FE F..RUNIC LETTER X
    ('\u{16a0}', '\u{16ea}', ALetter),
    // Nl   [3] RUNIC ARLAUG SYMBOL..RUNIC BELGTHOR SYMBOL
    ('\u{16ee}', '\u{16f0}', ALetter),
    // Lo   [8] RUNIC LETTER K..RUNIC LETTER FRANKS CASKET AESC
    ('\u{16f1}', '\u{16f8}', ALetter),
    // Lo  [13] TAGALOG LETTER A..TAGALOG LETTER YA
    ('\u{1700}', '\u{170c}', ALetter),
    // Lo   [4] TAGALOG LETTER LA..TAGALOG LETTER HA
    ('\u{170e}', '\u{1711}', ALetter),
    // Mn   [3] TAGALOG VOWEL SIGN I..TAGALOG SIGN VIRAMA
    ('\u{1712}', '\u{1714}', Extend),
    // Lo  [18] HANUNOO LETTER A..HANUNOO LETTER HA
    ('\u{1720}', '\u{1731}', ALetter),
    // Mn   [3] HANUNOO VOWEL SIGN I..HANUNOO SIGN PAMUDPOD
    ('\u{1732}', '\u{1734}', Extend),
    // Lo  [18] BUHID LETTER A..BUHID LETTER HA
    ('\u{1740}', '\u{1751}', ALetter),
    // Mn   [2] BUHID VOWEL SIGN I..BUHID VOWEL SIGN U
    ('\u{1752}', '\u{1753}', Extend),
    // Lo  [13] TAGBANWA LETTER A..TAGBANWA LETTER YA
    ('\u{1760}', '\u{176c}', ALetter),
    // Lo   [3] TAGBANWA LETTER LA..TAGBANWA LETTER SA
    ('\u{176e}', '\u{1770}', ALetter),
    // Mn   [2] TAGBANWA VOWEL SIGN I..TAGBANWA VOWEL SIGN U
    ('\u{1772}', '\u{1773}', Extend),
    // Mn   [2] KHMER VOWEL INHERENT AQ..KHMER VOWEL INHERENT AA
    ('\u{17b4}', '\u{17b5}', Extend),
    // Mc       KHMER VOWEL SIGN AA
    ('\u{17b6}', '\u{17b6}', Extend),
    // Mn   [7] KHMER VOWEL SIGN I..KHMER VOWEL SIGN UA
    ('\u{17b7}', '\u{17bd}', Extend),
    // Mc   [8] KHMER VOWEL SIGN OE..KHMER VOWEL SIGN AU
    ('\u{17be}', '\u{17c5}', Extend),
    // Mn       KHMER SIGN NIKAHIT
    ('\u{17c6}', '\u{17c6}', Extend),
    // Mc   [2] KHMER SIGN REAHMUK..KHMER SIGN YUUKALEAPINTU
    ('\u{17c7}', '\u{17c8}', Extend),
    // Mn  [11] KHMER SIGN MUUSIKATOAN..KHMER SIGN BATHAMASAT
    ('\u{17c9}', '\u{17d3}', Extend),
    // Mn       KHMER SIGN ATTHACAN
    ('\u{17dd}', '\u{17dd}', Extend),
    // Nd  [10] KHMER DIGIT ZERO..KHMER DIGIT NINE
    ('\u{17e0}', '\u{17e9}', Numeric),
    // Mn   [3] MONGOLIAN FREE VARIATION SELECTOR ONE..MONGOLIAN FREE VARIATION SELECTOR THREE
    ('\u{180b}', '\u{180d}', Extend),
    // Cf       MONGOLIAN VOWEL SEPARATOR
    ('\u{180e}', '\u{180e}', Format),
    // Nd  [10] MONGOLIAN DIGIT ZERO..MONGOLIAN DIGIT NINE
    ('\u{1810}', '\u{1819}', Numeric),
    // Lo  [35] MONGOLIAN LETTER A..MONGOLIAN LETTER CHI
    ('\u{1820}', '\u{1842}', ALetter),
    // Lm       MONGOLIAN LETTER TODO LONG VOWEL SIGN
    ('\u{1843}', '\u{1843}', ALetter),
    // Lo  [53] MONGOLIAN LETTER TODO E..MONGOLIAN LETTER CHA WITH TWO DOTS
    ('\u{1844}', '\u{1878}', ALetter),
    // Lo   [5] MONGOLIAN LETTER ALI GALI ANUSVARA ONE..MONGOLIAN LETTER ALI GALI INVERTED UBADAMA
    ('\u{1880}', '\u{1884}', ALetter),
    // Mn   [2] MONGOLIAN LETTER ALI GALI BALUDA..MONGOLIAN LETTER ALI GALI THREE BALUDA
    ('\u{1885}', '\u{1886}', Extend),
    // Lo  [34] MONGOLIAN LETTER ALI GALI A..MONGOLIAN LETTER MANCHU ALI GALI BHA
    ('\u{1887}', '\u{18a8}', ALetter),
    // Mn       MONGOLIAN LETTER ALI GALI DAGALGA
    ('\u{18a9}', '\u{18a9}', Extend),
    // Lo       MONGOLIAN LETTER MANCHU ALI GALI LHA
    ('\u{18aa}', '\u{18aa}', ALetter),
    // Lo  [70] CANADIAN SYLLABICS OY..CANADIAN SYLLABICS CARRIER DENTAL S
    ('\u{18b0}', '\u{18f5}', ALetter),
    // Lo  [31] LIMBU VOWEL-CARRIER LETTER..LIMBU LETTER TRA
    ('\u{1900}', '\u{191e}', ALetter),
    // Mn   [3] LIMBU VOWEL SIGN A..LIMBU VOWEL SIGN U
    ('\u{1920}', '\u{1922}', Extend),
    // Mc   [4] LIMBU VOWEL SIGN EE..LIMBU VOWEL SIGN AU
    ('\u{1923}', '\u{1926}', Extend),
    // Mn   [2] LIMBU VOWEL SIGN E..LIMBU VOWEL SIGN O
    ('\u{1927}', '\u{1928}', Extend),
    // Mc   [3] LIMBU SUBJOINED LETTER YA..LIMBU SUBJOINED LETTER WA
    ('\u{1929}', '\u{192b}', Extend),
    // Mc   [2] LIMBU SMALL LETTER KA..LIMBU SMALL LETTER NGA
    ('\u{1930}', '\u{1931}', Extend),
    // Mn       LIMBU SMALL LETTER ANUSVARA
    ('\u{1932}', '\u{1932}', Extend),
    // Mc   [6] LIMBU SMALL LETTER TA..LIMBU SMALL LETTER LA
    ('\u{1933}', '\u{1938}', Extend),
    // Mn   [3] LIMBU SIGN MUKPHRENG..LIMBU SIGN SA-I
    ('\u{1939}', '\u{193b}', Extend),
    // Nd  [10] LIMBU DIGIT ZERO..LIMBU DIGIT NINE
    ('\u{1946}', '\u{194f}', Numeric),
    // Nd  [10] NEW TAI LUE DIGIT ZERO..NEW TAI LUE DIGIT NINE
    ('\u{19d0}', '\u{19d9}', Numeric),
    // Lo  [23] BUGINESE LETTER KA..BUGINESE LETTER HA
    ('\u{1a00}', '\u{1a16}', ALetter),
    // Mn   [2] BUGINESE VOWEL SIGN I..BUGINESE VOWEL SIGN U
    ('\u{1a17}', '\u{1a18}', Extend),
    // Mc   [2] BUGINESE VOWEL SIGN E..BUGINESE VOWEL SIGN O
    ('\u{1a19}', '\u{1a1a}', Extend),
    // Mn       BUGINESE VOWEL SIGN AE
    ('\u{1a1b}', '\u{1a1b}', Extend),
    // Mc       TAI THAM CONSONANT SIGN MEDIAL RA
    ('\u{1a55}', '\u{1a55}', Extend),
    // Mn       TAI THAM CONSONANT SIGN MEDIAL LA
    ('\u{1a56}', '\u{1a56}', Extend),
    // Mc       TAI THAM CONSONANT SIGN LA TANG LAI
    ('\u{1a57}', '\u{1a57}', Extend),
    // Mn   [7] TAI THAM SIGN MAI KANG LAI..TAI THAM CONSONANT SIGN SA
    ('\u{1a58}', '\u{1a5e}', Extend),
    // Mn       TAI THAM SIGN SAKOT
    ('\u{1a60}', '\u{1a60}', Extend),
    // Mc       TAI THAM VOWEL SIGN A
    ('\u{1a61}', '\u{1a61}', Extend),
    // Mn       TAI THAM VOWEL SIGN MAI SAT
    ('\u{1a62}', '\u{1a62}', Extend),
    // Mc   [2] TAI THAM VOWEL SIGN AA..TAI THAM VOWEL SIGN TALL AA
    ('\u{1a63}', '\u{1a64}', Extend),
    // Mn   [8] TAI THAM VOWEL SIGN I..TAI THAM VOWEL SIGN OA BELOW
    ('\u{1a65}', '\u{1a6c}', Extend),
    // Mc   [6] TAI THAM VOWEL SIGN OY..TAI THAM VOWEL SIGN THAM AI
    ('\u{1a6d}', '\u{1a72}', Extend),
    // Mn  [10] TAI THAM VOWEL SIGN OA ABOVE..TAI THAM SIGN KHUEN-LUE KARAN
    ('\u{1a73}', '\u{1a7c}', Extend),
    // Mn       TAI THAM COMBINING CRYPTOGRAMMIC DOT
    ('\u{1a7f}', '\u{1a7f}', Extend),
    // Nd  [10] TAI THAM HORA DIGIT ZERO..TAI THAM HORA DIGIT NINE
    ('\u{1a80}', '\u{1a89}', Numeric),
    // Nd  [10] TAI THAM THAM DIGIT ZERO..TAI THAM THAM DIGIT NINE
    ('\u{1a90}', '\u{1a99}', Numeric),
    // Mn  [14] COMBINING DOUBLED CIRCUMFLEX ACCENT..COMBINING PARENTHESES BELOW
    ('\u{1ab0}', '\u{1abd}', Extend),
    // Me       COMBINING PARENTHESES OVERLAY
    ('\u{1abe}', '\u{1abe}', Extend),
    // Mn   [4] BALINESE SIGN ULU RICEM..BALINESE SIGN SURANG
    ('\u{1b00}', '\u{1b03}', Extend),
    // Mc       BALINESE SIGN BISAH
    ('\u{1b04}', '\u{1b04}', Extend),
    // Lo  [47] BALINESE LETTER AKARA..BALINESE LETTER HA
    ('\u{1b05}', '\u{1b33}', ALetter),
    // Mn       BALINESE SIGN REREKAN
    ('\u{1b34}', '\u{1b34}', Extend),
    // Mc       BALINESE VOWEL SIGN TEDUNG
    ('\u{1b35}', '\u{1b35}', Extend),
    // Mn   [5] BALINESE VOWEL SIGN ULU..BALINESE VOWEL SIGN RA REPA
    ('\u{1b36}', '\u{1b3a}', Extend),
    // Mc       BALINESE VOWEL SIGN RA REPA TEDUNG
    ('\u{1b3b}', '\u{1b3b}', Extend),
    // Mn       BALINESE VOWEL SIGN LA LENGA
    ('\u{1b3c}', '\u{1b3c}', Extend),
    // Mc   [5] BALINESE VOWEL SIGN LA LENGA TEDUNG..BALINESE VOWEL SIGN TALING REPA TEDUNG
    ('\u{1b3d}', '\u{1b41}', Extend),
    // Mn       BALINESE VOWEL SIGN PEPET
    ('\u{1b42}', '\u{1b42}', Extend),
    // Mc   [2] BALINESE VOWEL SIGN PEPET TEDUNG..BALINESE ADEG ADEG
    ('\u{1b43}', '\u{1b44}', Extend),
    // Lo   [7] BALINESE LETTER KAF SASAK..BALINESE LETTER ASYURA SASAK
    ('\u{1b45}', '\u{1b4b}', ALetter),
    // Nd  [10] BALINESE DIGIT ZERO..BALINESE DIGIT NINE
    ('\u{1b50}', '\u{1b59}', Numeric),
    // Mn   [9] BALINESE MUSICAL SYMBOL COMBINING TEGEH..BALINESE MUSICAL SYMBOL COMBINING GONG
    ('\u{1b6b}', '\u{1b73}', Extend),
    // Mn   [2] SUNDANESE SIGN PANYECEK..SUNDANESE SIGN PANGLAYAR
    ('\u{1b80}', '\u{1b81}', Extend),
    // Mc       SUNDANESE SIGN PANGWISAD
    ('\u{1b82}', '\u{1b82}', Extend),
    // Lo  [30] SUNDANESE LETTER A..SUNDANESE LETTER HA
    ('\u{1b83}', '\u{1ba0}', ALetter),
    // Mc       SUNDANESE CONSONANT SIGN PAMINGKAL
    ('\u{1ba1}', '\u{1ba1}', Extend),
    // Mn   [4] SUNDANESE CONSONANT SIGN PANYAKRA..SUNDANESE VOWEL SIGN PANYUKU
    ('\u{1ba2}', '\u{1ba5}', Extend),
    // Mc   [2] SUNDANESE VOWEL SIGN PANAELAENG..SUNDANESE VOWEL SIGN PANOLONG
    ('\u{1ba6}', '\u{1ba7}', Extend),
    // Mn   [2] SUNDANESE VOWEL SIGN PAMEPET..SUNDANESE VOWEL SIGN PANEULEUNG
    ('\u{1ba8}', '\u{1ba9}', Extend),
    // Mc       SUNDANESE SIGN PAMAAEH
    ('\u{1baa}', '\u{1baa}', Extend),
    // Mn   [3] SUNDANESE SIGN VIRAMA..SUNDANESE CONSONANT SIGN PASANGAN WA
    ('\u{1bab}', '\u{1bad}', Extend),
    // Lo   [2] SUNDANESE LETTER KHA..SUNDANESE LETTER SYA
    ('\u{1bae}', '\u{1baf}', ALetter),
    // Nd  [10] SUNDANESE DIGIT ZERO..SUNDANESE DIGIT NINE
    ('\u{1bb0}', '\u{1bb9}', Numeric),
    // Lo  [44] SUNDANESE AVAGRAHA..BATAK LETTER U
    ('\u{1bba}', '\u{1be5}', ALetter),
    // Mn       BATAK SIGN TOMPI
    ('\u{1be6}', '\u{1be6}', Extend),
    // Mc       BATAK VOWEL SIGN E
    ('\u{1be7}', '\u{1be7}', Extend),
    // Mn   [2] BATAK VOWEL SIGN PAKPAK E..BATAK VOWEL SIGN EE
    ('\u{1be8}', '\u{1be9}', Extend),
    // Mc   [3] BATAK VOWEL SIGN I..BATAK VOWEL SIGN O
    ('\u{1bea}', '\u{1bec}', Extend),
    // Mn       BATAK VOWEL SIGN KARO O
    ('\u{1bed}', '\u{1bed}', Extend),
    // Mc       BATAK VOWEL SIGN U
    ('\u{1bee}', '\u{1bee}', Extend),
    // Mn   [3] BATAK VOWEL SIGN U FOR SIMALUNGUN SA..BATAK CONSONANT SIGN H
    ('\u{1bef}', '\u{1bf1}', Extend),
    // Mc   [2] BATAK PANGOLAT..BATAK PANONGONAN
    ('\u{1bf2}', '\u{1bf3}', Extend),
    // Lo  [36] LEPCHA LETTER KA..LEPCHA LETTER A
    ('\u{1c00}', '\u{1c23}', ALetter),
    // Mc   [8] LEPCHA SUBJOINED LETTER YA..LEPCHA VOWEL SIGN UU
    ('\u{1c24}', '\u{1c2b}', Extend),
    // Mn   [8] LEPCHA VOWEL SIGN E..LEPCHA CONSONANT SIGN T
    ('\u{1c2c}', '\u{1c33}', Extend),
    // Mc   [2] LEPCHA CONSONANT SIGN NYIN-DO..LEPCHA CONSONANT SIGN KANG
    ('\u{1c34}', '\u{1c35}', Extend),
    // Mn   [2] LEPCHA SIGN RAN..LEPCHA SIGN NUKTA
    ('\u{1c36}', '\u{1c37}', Extend),
    // Nd  [10] LEPCHA DIGIT ZERO..LEPCHA DIGIT NINE
    ('\u{1c40}', '\u{1c49}', Numeric),
    // Lo   [3] LEPCHA LETTER TTA..LEPCHA LETTER DDA
    ('\u{1c4d}', '\u{1c4f}', ALetter),
    // Nd  [10] OL CHIKI DIGIT ZERO..OL CHIKI DIGIT NINE
    ('\u{1c50}', '\u{1c59}', Numeric),
    // Lo  [30] OL CHIKI LETTER LA..OL CHIKI LETTER OH
    ('\u{1c5a}', '\u{1c77}', ALetter),
    // Lm   [6] OL CHIKI MU TTUDDAG..OL CHIKI AHAD
    ('\u{1c78}', '\u{1c7d}', ALetter),
    // L&   [9] CYRILLIC SMALL LETTER ROUNDED VE..CYRILLIC SMALL LETTER UNBLENDED UK
    ('\u{1c80}', '\u{1c88}', ALetter),
    // L&  [43] GEORGIAN MTAVRULI CAPITAL LETTER AN..GEORGIAN MTAVRULI CAPITAL LETTER AIN
    ('\u{1c90}', '\u{1cba}', ALetter),
    // L&   [3] GEORGIAN MTAVRULI CAPITAL LETTER AEN..GEORGIAN MTAVRULI CAPITAL LETTER LABIAL SIGN
    ('\u{1cbd}', '\u{1cbf}', ALetter),
    // Mn   [3] VEDIC TONE KARSHANA..VEDIC TONE PRENKHA
    ('\u{1cd0}', '\u{1cd2}', Extend),
    // Mn  [13] VEDIC SIGN YAJURVEDIC MIDLINE SVARITA..VEDIC TONE RIGVEDIC KASHMIRI INDEPENDENT SVARITA
    ('\u{1cd4}', '\u{1ce0}', Extend),
    // Mc       VEDIC TONE ATHARVAVEDIC INDEPENDENT SVARITA
    ('\u{1ce1}', '\u{1ce1}', Extend),
    // Mn   [7] VEDIC SIGN VISARGA SVARITA..VEDIC SIGN VISARGA ANUDATTA WITH TAIL
    ('\u{1ce2}', '\u{1ce8}', Extend),
    // Lo   [4] VEDIC SIGN ANUSVARA ANTARGOMUKHA..VEDIC SIGN ANUSVARA VAMAGOMUKHA WITH TAIL
    ('\u{1ce9}', '\u{1cec}', ALetter),
    // Mn       VEDIC SIGN TIRYAK
    ('\u{1ced}', '\u{1ced}', Extend),
    // Lo   [6] VEDIC SIGN HEXIFORM LONG ANUSVARA..VEDIC SIGN ROTATED ARDHAVISARGA
    ('\u{1cee}', '\u{1cf3}', ALetter),
    // Mn       VEDIC TONE CANDRA ABOVE
    ('\u{1cf4}', '\u{1cf4}', Extend),
    // Lo   [2] VEDIC SIGN JIHVAMULIYA..VEDIC SIGN UPADHMANIYA
    ('\u{1cf5}', '\u{1cf6}', ALetter),
    // Mc       VEDIC SIGN ATIKRAMA
    ('\u{1cf7}', '\u{1cf7}', Extend),
    // Mn   [2] VEDIC TONE RING ABOVE..VEDIC TONE DOUBLE RING ABOVE
    ('\u{1cf8}', '\u{1cf9}', Extend),
    // Lo       VEDIC SIGN DOUBLE ANUSVARA ANTARGOMUKHA
    ('\u{1cfa}', '\u{1cfa}', ALetter),
    // L&  [44] LATIN LETTER SMALL CAPITAL A..CYRILLIC LETTER SMALL CAPITAL EL
    ('\u{1d00}', '\u{1d2b}', ALetter),
    // Lm  [63] MODIFIER LETTER CAPITAL A..GREEK SUBSCRIPT SMALL LETTER CHI
    ('\u{1d2c}', '\u{1d6a}', ALetter),
    // L&  [13] LATIN SMALL LETTER UE..LATIN SMALL LETTER TURNED G
    ('\u{1d6b}', '\u{1d77}', ALetter),
    // Lm       MODIFIER LETTER CYRILLIC EN
    ('\u{1d78}', '\u{1d78}', ALetter),
    // L&  [34] LATIN SMALL LETTER INSULAR G..LATIN SMALL LETTER EZH WITH RETROFLEX HOOK
    ('\u{1d79}', '\u{1d9a}', ALetter),
    // Lm  [37] MODIFIER LETTER SMALL TURNED ALPHA..MODIFIER LETTER SMALL THETA
    ('\u{1d9b}', '\u{1dbf}', ALetter),
    // Mn  [58] COMBINING DOTTED GRAVE ACCENT..COMBINING WIDE INVERTED BRIDGE BELOW
    ('\u{1dc0}', '\u{1df9}', Extend),
    // Mn   [5] COMBINING DELETION MARK..COMBINING RIGHT ARROWHEAD AND DOWN ARROWHEAD BELOW
    ('\u{1dfb}', '\u{1dff}', Extend),
    // L& [278] LATIN CAPITAL LETTER A WITH RING BELOW..GREEK SMALL LETTER EPSILON WITH DASIA AND OXIA
    ('\u{1e00}', '\u{1f15}', ALetter),
    // L&   [6] GREEK CAPITAL LETTER EPSILON WITH PSILI..GREEK CAPITAL LETTER EPSILON WITH DASIA AND OXIA
    ('\u{1f18}', '\u{1f1d}', ALetter),
    // L&  [38] GREEK SMALL LETTER ETA WITH PSILI..GREEK SMALL LETTER OMICRON WITH DASIA AND OXIA
    ('\u{1f20}', '\u{1f45}', ALetter),
    // L&   [6] GREEK CAPITAL LETTER OMICRON WITH PSILI..GREEK CAPITAL LETTER OMICRON WITH DASIA AND OXIA
    ('\u{1f48}', '\u{1f4d}', ALetter),
    // L&   [8] GREEK SMALL LETTER UPSILON WITH PSILI..GREEK SMALL LETTER UPSILON WITH DASIA AND PERISPOMENI
    ('\u{1f50}', '\u{1f57}', ALetter),
    // L&       GREEK CAPITAL LETTER UPSILON WITH DASIA
    ('\u{1f59}', '\u{1f59}', ALetter),
    // L&       GREEK CAPITAL LETTER UPSILON WITH DASIA AND VARIA
    ('\u{1f5b}', '\u{1f5b}', ALetter),
    // L&       GREEK CAPITAL LETTER UPSILON WITH DASIA AND OXIA
    ('\u{1f5d}', '\u{1f5d}', ALetter),
    // L&  [31] GREEK CAPITAL LETTER UPSILON WITH DASIA AND PERISPOMENI..GREEK SMALL LETTER OMEGA WITH OXIA
    ('\u{1f5f}', '\u{1f7d}', ALetter),
    // L&  [53] GREEK SMALL LETTER ALPHA WITH PSILI AND YPOGEGRAMMENI..GREEK SMALL LETTER ALPHA WITH OXIA AND YPOGEGRAMMENI
    ('\u{1f80}', '\u{1fb4}', ALetter),
    // L&   [7] GREEK SMALL LETTER ALPHA WITH PERISPOMENI..GREEK CAPITAL LETTER ALPHA WITH PROSGEGRAMMENI
    ('\u{1fb6}', '\u{1fbc}', ALetter),
    // L&       GREEK PROSGEGRAMMENI
    ('\u{1fbe}', '\u{1fbe}', ALetter),
    // L&   [3] GREEK SMALL LETTER ETA WITH VARIA AND YPOGEGRAMMENI..GREEK SMALL LETTER ETA WITH OXIA AND YPOGEGRAMMENI
    ('\u{1fc2}', '\u{1fc4}', ALetter),
    // L&   [7] GREEK SMALL LETTER ETA WITH PERISPOMENI..GREEK CAPITAL LETTER ETA WITH PROSGEGRAMMENI
    ('\u{1fc6}', '\u{1fcc}', ALetter),
    // L&   [4] GREEK SMALL LETTER IOTA WITH VRACHY..GREEK SMALL LETTER IOTA WITH DIALYTIKA AND OXIA
    ('\u{1fd0}', '\u{1fd3}', ALetter),
    // L&   [6] GREEK SMALL LETTER IOTA WITH PERISPOMENI..GREEK CAPITAL LETTER IOTA WITH OXIA
    ('\u{1fd6}', '\u{1fdb}', ALetter),
    // L&  [13] GREEK SMALL LETTER UPSILON WITH VRACHY..GREEK CAPITAL LETTER RHO WITH DASIA
    ('\u{1fe0}', '\u{1fec}', ALetter),
    // L&   [3] GREEK SMALL LETTER OMEGA WITH VARIA AND YPOGEGRAMMENI..GREEK SMALL LETTER OMEGA WITH OXIA AND YPOGEGRAMMENI
    ('\u{1ff2}', '\u{1ff4}', ALetter),
    // L&   [7] GREEK SMALL LETTER OMEGA WITH PERISPOMENI..GREEK CAPITAL LETTER OMEGA WITH PROSGEGRAMMENI
    ('\u{1ff6}', '\u{1ffc}', ALetter),
    // Zs   [7] EN QUAD..SIX-PER-EM SPACE
    ('\u{2000}', '\u{2006}', WSegSpace),
    // Zs   [3] PUNCTUATION SPACE..HAIR SPACE
    ('\u{2008}', '\u{200a}', WSegSpace),
    // Cf       ZERO WIDTH NON-JOINER
    ('\u{200c}', '\u{200c}', Extend),
    // Cf       ZERO WIDTH JOINER
    ('\u{200d}', '\u{200d}', ZWJ),
    // Cf   [2] LEFT-TO-RIGHT MARK..RIGHT-TO-LEFT MARK
    ('\u{200e}', '\u{200f}', Format),
    // Pi       LEFT SINGLE QUOTATION MARK
    ('\u{2018}', '\u{2018}', MidNumLet),
    // Pf       RIGHT SINGLE QUOTATION MARK
    ('\u{2019}', '\u{2019}', MidNumLet),
    // Po       ONE DOT LEADER
    ('\u{2024}', '\u{2024}', MidNumLet),
    // Po       HYPHENATION POINT
    ('\u{2027}', '\u{2027}', MidLetter),
    // Zl       LINE SEPARATOR
    ('\u{2028}', '\u{2028}', Newline),
    // Zp       PARAGRAPH SEPARATOR
    ('\u{2029}', '\u{2029}', Newline),
    // Cf   [5] LEFT-TO-RIGHT EMBEDDING..RIGHT-TO-LEFT OVERRIDE
    ('\u{202a}', '\u{202e}', Format),
    // Zs       NARROW NO-BREAK SPACE
    ('\u{202f}', '\u{202f}', ExtendNumLet),
    // Pc   [2] UNDERTIE..CHARACTER TIE
    ('\u{203f}', '\u{2040}', ExtendNumLet),
    // Sm       FRACTION SLASH
    ('\u{2044}', '\u{2044}', MidNum),
    // Pc       INVERTED UNDERTIE
    ('\u{2054}', '\u{2054}', ExtendNumLet),
    // Zs       MEDIUM MATHEMATICAL SPACE
    ('\u{205f}', '\u{205f}', WSegSpace),
    // Cf   [5] WORD JOINER..INVISIBLE PLUS
    ('\u{2060}', '\u{2064}', Format),
    // Cf  [10] LEFT-TO-RIGHT ISOLATE..NOMINAL DIGIT SHAPES
    ('\u{2066}', '\u{206f}', Format),
    // Lm       SUPERSCRIPT LATIN SMALL LETTER I
    ('\u{2071}', '\u{2071}', ALetter),
    // Lm       SUPERSCRIPT LATIN SMALL LETTER N
    ('\u{207f}', '\u{207f}', ALetter),
    // Lm  [13] LATIN SUBSCRIPT SMALL LETTER A..LATIN SUBSCRIPT SMALL LETTER T
    ('\u{2090}', '\u{209c}', ALetter),
    // Mn  [13] COMBINING LEFT HARPOON ABOVE..COMBINING FOUR DOTS ABOVE
    ('\u{20d0}', '\u{20dc}', Extend),
    // Me   [4] COMBINING ENCLOSING CIRCLE..COMBINING ENCLOSING CIRCLE BACKSLASH
    ('\u{20dd}', '\u{20e0}', Extend),
    // Mn       COMBINING LEFT RIGHT ARROW ABOVE
    ('\u{20e1}', '\u{20e1}', Extend),
    // Me   [3] COMBINING ENCLOSING SCREEN..COMBINING ENCLOSING UPWARD POINTING TRIANGLE
    ('\u{20e2}', '\u{20e4}', Extend),
    // Mn  [12] COMBINING REVERSE SOLIDUS OVERLAY..COMBINING ASTERISK ABOVE
    ('\u{20e5}', '\u{20f0}', Extend),
    // L&       DOUBLE-STRUCK CAPITAL C
    ('\u{2102}', '\u{2102}', ALetter),
    // L&       EULER CONSTANT
    ('\u{2107}', '\u{2107}', ALetter),
    // L&  [10] SCRIPT SMALL G..SCRIPT SMALL L
    ('\u{210a}', '\u{2113}', ALetter),
    // L&       DOUBLE-STRUCK CAPITAL N
    ('\u{2115}', '\u{2115}', ALetter),
    // L&   [5] DOUBLE-STRUCK CAPITAL P..DOUBLE-STRUCK CAPITAL R
    ('\u{2119}', '\u{211d}', ALetter),
    // L&       DOUBLE-STRUCK CAPITAL Z
    ('\u{2124}', '\u{2124}', ALetter),
    // L&       OHM SIGN
    ('\u{2126}', '\u{2126}', ALetter),
    // L&       BLACK-LETTER CAPITAL Z
    ('\u{2128}', '\u{2128}', ALetter),
    // L&   [4] KELVIN SIGN..BLACK-LETTER CAPITAL C
    ('\u{212a}', '\u{212d}', ALetter),
    // L&   [6] SCRIPT SMALL E..SCRIPT SMALL O
    ('\u{212f}', '\u{2134}', ALetter),
    // Lo   [4] ALEF SYMBOL..DALET SYMBOL
    ('\u{2135}', '\u{2138}', ALetter),
    // L&       INFORMATION SOURCE
    ('\u{2139}', '\u{2139}', ALetter),
    // L&   [4] DOUBLE-STRUCK SMALL PI..DOUBLE-STRUCK CAPITAL PI
    ('\u{213c}', '\u{213f}', ALetter),
    // L&   [5] DOUBLE-STRUCK ITALIC CAPITAL D..DOUBLE-STRUCK ITALIC SMALL J
    ('\u{2145}', '\u{2149}', ALetter),
    // L&       TURNED SMALL F
    ('\u{214e}', '\u{214e}', ALetter),
    // Nl  [35] ROMAN NUMERAL ONE..ROMAN NUMERAL TEN THOUSAND
    ('\u{2160}', '\u{2182}', ALetter),
    // L&   [2] ROMAN NUMERAL REVERSED ONE HUNDRED..LATIN SMALL LETTER REVERSED C
    ('\u{2183}', '\u{2184}', ALetter),
    // Nl   [4] ROMAN NUMERAL SIX LATE FORM..ROMAN NUMERAL ONE HUNDRED THOUSAND
    ('\u{2185}', '\u{2188}', ALetter),
    // So  [52] CIRCLED LATIN CAPITAL LETTER A..CIRCLED LATIN SMALL LETTER Z
    ('\u{24b6}', '\u{24e9}', ALetter),
    // L&  [47] GLAGOLITIC CAPITAL LETTER AZU..GLAGOLITIC CAPITAL LETTER LATINATE MYSLITE
    ('\u{2c00}', '\u{2c2e}', ALetter),
    // L&  [47] GLAGOLITIC SMALL LETTER AZU..GLAGOLITIC SMALL LETTER LATINATE MYSLITE
    ('\u{2c30}', '\u{2c5e}', ALetter),
    // L&  [28] LATIN CAPITAL LETTER L WITH DOUBLE BAR..LATIN LETTER SMALL CAPITAL TURNED E
    ('\u{2c60}', '\u{2c7b}', ALetter),
    // Lm   [2] LATIN SUBSCRIPT SMALL LETTER J..MODIFIER LETTER CAPITAL V
    ('\u{2c7c}', '\u{2c7d}', ALetter),
    // L& [103] LATIN CAPITAL LETTER S WITH SWASH TAIL..COPTIC SYMBOL KAI
    ('\u{2c7e}', '\u{2ce4}', ALetter),
    // L&   [4] COPTIC CAPITAL LETTER CRYPTOGRAMMIC SHEI..COPTIC SMALL LETTER CRYPTOGRAMMIC GANGIA
    ('\u{2ceb}', '\u{2cee}', ALetter),
    // Mn   [3] COPTIC COMBINING NI ABOVE..COPTIC COMBINING SPIRITUS LENIS
    ('\u{2cef}', '\u{2cf1}', Extend),
    // L&   [2] COPTIC CAPITAL LETTER BOHAIRIC KHEI..COPTIC SMALL LETTER BOHAIRIC KHEI
    ('\u{2cf2}', '\u{2cf3}', ALetter),
    // L&  [38] GEORGIAN SMALL LETTER AN..GEORGIAN SMALL LETTER HOE
    ('\u{2d00}', '\u{2d25}', ALetter),
    // L&       GEORGIAN SMALL LETTER YN
    ('\u{2d27}', '\u{2d27}', ALetter),
    // L&       GEORGIAN SMALL LETTER AEN
    ('\u{2d2d}', '\u{2d2d}', ALetter),
    // Lo  [56] TIFINAGH LETTER YA..TIFINAGH LETTER YO
    ('\u{2d30}', '\u{2d67}', ALetter),
    // Lm       TIFINAGH MODIFIER LETTER LABIALIZATION MARK
    ('\u{2d6f}', '\u{2d6f}', ALetter),
    // Mn       TIFINAGH CONSONANT JOINER
    ('\u{2d7f}', '\u{2d7f}', Extend),
    // Lo  [23] ETHIOPIC SYLLABLE LOA..ETHIOPIC SYLLABLE GGWE
    ('\u{2d80}', '\u{2d96}', ALetter),
    // Lo   [7] ETHIOPIC SYLLABLE SSA..ETHIOPIC SYLLABLE SSO
    ('\u{2da0}', '\u{2da6}', ALetter),
    // Lo   [7] ETHIOPIC SYLLABLE CCA..ETHIOPIC SYLLABLE CCO
    ('\u{2da8}', '\u{2dae}', ALetter),
    // Lo   [7] ETHIOPIC SYLLABLE ZZA..ETHIOPIC SYLLABLE ZZO
    ('\u{2db0}', '\u{2db6}', ALetter),
    // Lo   [7] ETHIOPIC SYLLABLE CCHA..ETHIOPIC SYLLABLE CCHO
    ('\u{2db8}', '\u{2dbe}', ALetter),
    // Lo   [7] ETHIOPIC SYLLABLE QYA..ETHIOPIC SYLLABLE QYO
    ('\u{2dc0}', '\u{2dc6}', ALetter),
    // Lo   [7] ETHIOPIC SYLLABLE KYA..ETHIOPIC SYLLABLE KYO
    ('\u{2dc8}', '\u{2dce}', ALetter),
    // Lo   [7] ETHIOPIC SYLLABLE XYA..ETHIOPIC SYLLABLE XYO
    ('\u{2dd0}', '\u{2dd6}', ALetter),
    // Lo   [7] ETHIOPIC SYLLABLE GYA..ETHIOPIC SYLLABLE GYO
    ('\u{2dd8}', '\u{2dde}', ALetter),
    // Mn  [32] COMBINING CYRILLIC LETTER BE..COMBINING CYRILLIC LETTER IOTIFIED BIG YUS
    ('\u{2de0}', '\u{2dff}', Extend),
    // Lm       VERTICAL TILDE
    ('\u{2e2f}', '\u{2e2f}', ALetter),
    // Zs       IDEOGRAPHIC SPACE
    ('\u{3000}', '\u{3000}', WSegSpace),
    // Lm       IDEOGRAPHIC ITERATION MARK
    ('\u{3005}', '\u{3005}', ALetter),
    // Mn   [4] IDEOGRAPHIC LEVEL TONE MARK..IDEOGRAPHIC ENTERING TONE MARK
    ('\u{302a}', '\u{302d}', Extend),
    // Mc   [2] HANGUL SINGLE DOT TONE MARK..HANGUL DOUBLE DOT TONE MARK
    ('\u{302e}', '\u{302f}', Extend),
    // Lm   [5] VERTICAL KANA REPEAT MARK..VERTICAL KANA REPEAT MARK LOWER HALF
    ('\u{3031}', '\u{3035}', Katakana),
    // Lm       VERTICAL IDEOGRAPHIC ITERATION MARK
    ('\u{303b}', '\u{303b}', ALetter),
    // Lo       MASU MARK
    ('\u{303c}', '\u{303c}', ALetter),
    // Mn   [2] COMBINING KATAKANA-HIRAGANA VOICED SOUND MARK..COMBINING KATAKANA-HIRAGANA SEMI-VOICED SOUND MARK
    ('\u{3099}', '\u{309a}', Extend),
    // Sk   [2] KATAKANA-HIRAGANA VOICED SOUND MARK..KATAKANA-HIRAGANA SEMI-VOICED SOUND MARK
    ('\u{309b}', '\u{309c}', Katakana),
    // Pd       KATAKANA-HIRAGANA DOUBLE HYPHEN
    ('\u{30a0}', '\u{30a0}', Katakana),
    // Lo  [90] KATAKANA LETTER SMALL A..KATAKANA LETTER VO
    ('\u{30a1}', '\u{30fa}', Katakana),
    // Lm   [3] KATAKANA-HIRAGANA PROLONGED SOUND MARK..KATAKANA VOICED ITERATION MARK
    ('\u{30fc}', '\u{30fe}', Katakana),
    // Lo       KATAKANA DIGRAPH KOTO
    ('\u{30ff}', '\u{30ff}', Katakana),
    // Lo  [43] BOPOMOFO LETTER B..BOPOMOFO LETTER NN
    ('\u{3105}', '\u{312f}', ALetter),
    // Lo  [94] HANGUL LETTER KIYEOK..HANGUL LETTER ARAEAE
    ('\u{3131}', '\u{318e}', ALetter),
    // Lo  [27] BOPOMOFO LETTER BU..BOPOMOFO LETTER ZY
    ('\u{31a0}', '\u{31ba}', ALetter),
    // Lo  [16] KATAKANA LETTER SMALL KU..KATAKANA LETTER SMALL RO
    ('\u{31f0}', '\u{31ff}', Katakana),
    // So  [47] CIRCLED KATAKANA A..CIRCLED KATAKANA WO
    ('\u{32d0}', '\u{32fe}', Katakana),
    // So  [88] SQUARE APAATO..SQUARE WATTO
    ('\u{3300}', '\u{3357}', Katakana),
    // Lo  [21] YI SYLLABLE IT..YI SYLLABLE E
    ('\u{a000}', '\u{a014}', ALetter),
    // Lm       YI SYLLABLE WU
    ('\u{a015}', '\u{a015}', ALetter),
    // Lo [1143] YI SYLLABLE BIT..YI SYLLABLE YYR
    ('\u{a016}', '\u{a48c}', ALetter),
    // Lo  [40] LISU LETTER BA..LISU LETTER OE
    ('\u{a4d0}', '\u{a4f7}', ALetter),
    // Lm   [6] LISU LETTER TONE MYA TI..LISU LETTER TONE MYA JEU
    ('\u{a4f8}', '\u{a4fd}', ALetter),
    // Lo [268] VAI SYLLABLE EE..VAI SYLLABLE NG
    ('\u{a500}', '\u{a60b}', ALetter),
    // Lm       VAI SYLLABLE LENGTHENER
    ('\u{a60c}', '\u{a60c}', ALetter),
    // Lo  [16] VAI SYLLABLE NDOLE FA..VAI SYMBOL JONG
    ('\u{a610}', '\u{a61f}', ALetter),
    // Nd  [10] VAI DIGIT ZERO..VAI DIGIT NINE
    ('\u{a620}', '\u{a629}', Numeric),
    // Lo   [2] VAI SYLLABLE NDOLE MA..VAI SYLLABLE NDOLE DO
    ('\u{a62a}', '\u{a62b}', ALetter),
    // L&  [46] CYRILLIC CAPITAL LETTER ZEMLYA..CYRILLIC SMALL LETTER DOUBLE MONOCULAR O
    ('\u{a640}', '\u{a66d}', ALetter),
    // Lo       CYRILLIC LETTER MULTIOCULAR O
    ('\u{a66e}', '\u{a66e}', ALetter),
    // Mn       COMBINING CYRILLIC VZMET
    ('\u{a66f}', '\u{a66f}', Extend),
    // Me   [3] COMBINING CYRILLIC TEN MILLIONS SIGN..COMBINING CYRILLIC THOUSAND MILLIONS SIGN
    ('\u{a670}', '\u{a672}', Extend),
    // Mn  [10] COMBINING CYRILLIC LETTER UKRAINIAN IE..COMBINING CYRILLIC PAYEROK
    ('\u{a674}', '\u{a67d}', Extend),
    // Lm       CYRILLIC PAYEROK
    ('\u{a67f}', '\u{a67f}', ALetter),
    // L&  [28] CYRILLIC CAPITAL LETTER DWE..CYRILLIC SMALL LETTER CROSSED O
    ('\u{a680}', '\u{a69b}', ALetter),
    // Lm   [2] MODIFIER LETTER CYRILLIC HARD SIGN..MODIFIER LETTER CYRILLIC SOFT SIGN
    ('\u{a69c}', '\u{a69d}', ALetter),
    // Mn   [2] COMBINING CYRILLIC LETTER EF..COMBINING CYRILLIC LETTER IOTIFIED E
    ('\u{a69e}', '\u{a69f}', Extend),
    // Lo  [70] BAMUM LETTER A..BAMUM LETTER KI
    ('\u{a6a0}', '\u{a6e5}', ALetter),
    // Nl  [10] BAMUM LETTER MO..BAMUM LETTER KOGHOM
    ('\u{a6e6}', '\u{a6ef}', ALetter),
    // Mn   [2] BAMUM COMBINING MARK KOQNDON..BAMUM COMBINING MARK TUKWENTIS
    ('\u{a6f0}', '\u{a6f1}', Extend),
    // Lm   [9] MODIFIER LETTER DOT VERTICAL BAR..MODIFIER LETTER LOW INVERTED EXCLAMATION MARK
    ('\u{a717}', '\u{a71f}', ALetter),
    // Sk   [2] MODIFIER LETTER STRESS AND HIGH TONE..MODIFIER LETTER STRESS AND LOW TONE
    ('\u{a720}', '\u{a721}', ALetter),
    // L&  [78] LATIN CAPITAL LETTER EGYPTOLOGICAL ALEF..LATIN SMALL LETTER CON
    ('\u{a722}', '\u{a76f}', ALetter),
    // Lm       MODIFIER LETTER US
    ('\u{a770}', '\u{a770}', ALetter),
    // L&  [23] LATIN SMALL LETTER DUM..LATIN SMALL LETTER INSULAR T
    ('\u{a771}', '\u{a787}', ALetter),
    // Lm       MODIFIER LETTER LOW CIRCUMFLEX ACCENT
    ('\u{a788}', '\u{a788}', ALetter),
    // Sk   [2] MODIFIER LETTER COLON..MODIFIER LETTER SHORT EQUALS SIGN
    ('\u{a789}', '\u{a78a}', ALetter),
    // L&   [4] LATIN CAPITAL LETTER SALTILLO..LATIN SMALL LETTER L WITH RETROFLEX HOOK AND BELT
    ('\u{a78b}', '\u{a78e}', ALetter),
    // Lo       LATIN LETTER SINOLOGICAL DOT
    ('\u{a78f}', '\u{a78f}', ALetter),
    // L&  [48] LATIN CAPITAL LETTER N WITH DESCENDER..LATIN SMALL LETTER GLOTTAL U
    ('\u{a790}', '\u{a7bf}', ALetter),
    // L&   [5] LATIN CAPITAL LETTER ANGLICANA W..LATIN CAPITAL LETTER Z WITH PALATAL HOOK
    ('\u{a7c2}', '\u{a7c6}', ALetter),
    // Lo       LATIN EPIGRAPHIC LETTER SIDEWAYS I
    ('\u{a7f7}', '\u{a7f7}', ALetter),
    // Lm   [2] MODIFIER LETTER CAPITAL H WITH STROKE..MODIFIER LETTER SMALL LIGATURE OE
    ('\u{a7f8}', '\u{a7f9}', ALetter),
    // L&       LATIN LETTER SMALL CAPITAL TURNED M
    ('\u{a7fa}', '\u{a7fa}', ALetter),
    // Lo   [7] LATIN EPIGRAPHIC LETTER REVERSED F..SYLOTI NAGRI LETTER I
    ('\u{a7fb}', '\u{a801}', ALetter),
    // Mn       SYLOTI NAGRI SIGN DVISVARA
    ('\u{a802}', '\u{a802}', Extend),
    // Lo   [3] SYLOTI NAGRI LETTER U..SYLOTI NAGRI LETTER O
    ('\u{a803}', '\u{a805}', ALetter),
    // Mn       SYLOTI NAGRI SIGN HASANTA
    ('\u{a806}', '\u{a806}', Extend),
    // Lo   [4] SYLOTI NAGRI LETTER KO..SYLOTI NAGRI LETTER GHO
    ('\u{a807}', '\u{a80a}', ALetter),
    // Mn       SYLOTI NAGRI SIGN ANUSVARA
    ('\u{a80b}', '\u{a80b}', Extend),
    // Lo  [23] SYLOTI NAGRI LETTER CO..SYLOTI NAGRI LETTER HO
    ('\u{a80c}', '\u{a822}', ALetter),
    // Mc   [2] SYLOTI NAGRI VOWEL SIGN A..SYLOTI NAGRI VOWEL SIGN I
    ('\u{a823}', '\u{a824}', Extend),
    // Mn   [2] SYLOTI NAGRI VOWEL SIGN U..SYLOTI NAGRI VOWEL SIGN E
    ('\u{a825}', '\u{a826}', Extend),
    // Mc       SYLOTI NAGRI VOWEL SIGN OO
    ('\u{a827}', '\u{a827}', Extend),
    // Lo  [52] PHAGS-PA LETTER KA..PHAGS-PA LETTER CANDRABINDU
    ('\u{a840}', '\u{a873}', ALetter),
    // Mc   [2] SAURASHTRA SIGN ANUSVARA..SAURASHTRA SIGN VISARGA
    ('\u{a880}', '\u{a881}', Extend),
    // Lo  [50] SAURASHTRA LETTER A..SAURASHTRA LETTER LLA
    ('\u{a882}', '\u{a8b3}', ALetter),
    // Mc  [16] SAURASHTRA CONSONANT SIGN HAARU..SAURASHTRA VOWEL SIGN AU
    ('\u{a8b4}', '\u{a8c3}', Extend),
    // Mn   [2] SAURASHTRA SIGN VIRAMA..SAURASHTRA SIGN CANDRABINDU
    ('\u{a8c4}', '\u{a8c5}', Extend),
    // Nd  [10] SAURASHTRA DIGIT ZERO..SAURASHTRA DIGIT NINE
    ('\u{a8d0}', '\u{a8d9}', Numeric),
    // Mn  [18] COMBINING DEVANAGARI DIGIT ZERO..COMBINING DEVANAGARI SIGN AVAGRAHA
    ('\u{a8e0}', '\u{a8f1}', Extend),
    // Lo   [6] DEVANAGARI SIGN SPACING CANDRABINDU..DEVANAGARI SIGN CANDRABINDU AVAGRAHA
    ('\u{a8f2}', '\u{a8f7}', ALetter),
    // Lo       DEVANAGARI HEADSTROKE
    ('\u{a8fb}', '\u{a8fb}', ALetter),
    // Lo   [2] DEVANAGARI JAIN OM..DEVANAGARI LETTER AY
    ('\u{a8fd}', '\u{a8fe}', ALetter),
    // Mn       DEVANAGARI VOWEL SIGN AY
    ('\u{a8ff}', '\u{a8ff}', Extend),
    // Nd  [10] KAYAH LI DIGIT ZERO..KAYAH LI DIGIT NINE
    ('\u{a900}', '\u{a909}', Numeric),
    // Lo  [28] KAYAH LI LETTER KA..KAYAH LI LETTER OO
    ('\u{a90a}', '\u{a925}', ALetter),
    // Mn   [8] KAYAH LI VOWEL UE..KAYAH LI TONE CALYA PLOPHU
    ('\u{a926}', '\u{a92d}', Extend),
    // Lo  [23] REJANG LETTER KA..REJANG LETTER A
    ('\u{a930}', '\u{a946}', ALetter),
    // Mn  [11] REJANG VOWEL SIGN I..REJANG CONSONANT SIGN R
    ('\u{a947}', '\u{a951}', Extend),
    // Mc   [2] REJANG CONSONANT SIGN H..REJANG VIRAMA
    ('\u{a952}', '\u{a953}', Extend),
    // Lo  [29] HANGUL CHOSEONG TIKEUT-MIEUM..HANGUL CHOSEONG SSANGYEORINHIEUH
    ('\u{a960}', '\u{a97c}', ALetter),
    // Mn   [3] JAVANESE SIGN PANYANGGA..JAVANESE SIGN LAYAR
    ('\u{a980}', '\u{a982}', Extend),
    // Mc       JAVANESE SIGN WIGNYAN
    ('\u{a983}', '\u{a983}', Extend),
    // Lo  [47] JAVANESE LETTER A..JAVANESE LETTER HA
    ('\u{a984}', '\u{a9b2}', ALetter),
    // Mn       JAVANESE SIGN CECAK TELU
    ('\u{a9b3}', '\u{a9b3}', Extend),
    // Mc   [2] JAVANESE VOWEL SIGN TARUNG..JAVANESE VOWEL SIGN TOLONG
    ('\u{a9b4}', '\u{a9b5}', Extend),
    // Mn   [4] JAVANESE VOWEL SIGN WULU..JAVANESE VOWEL SIGN SUKU MENDUT
    ('\u{a9b6}', '\u{a9b9}', Extend),
    // Mc   [2] JAVANESE VOWEL SIGN TALING..JAVANESE VOWEL SIGN DIRGA MURE
    ('\u{a9ba}', '\u{a9bb}', Extend),
    // Mn   [2] JAVANESE VOWEL SIGN PEPET..JAVANESE CONSONANT SIGN KERET
    ('\u{a9bc}', '\u{a9bd}', Extend),
    // Mc   [3] JAVANESE CONSONANT SIGN PENGKAL..JAVANESE PANGKON
    ('\u{a9be}', '\u{a9c0}', Extend),
    // Lm       JAVANESE PANGRANGKEP
    ('\u{a9cf}', '\u{a9cf}', ALetter),
    // Nd  [10] JAVANESE DIGIT ZERO..JAVANESE DIGIT NINE
    ('\u{a9d0}', '\u{a9d9}', Numeric),
    // Mn       MYANMAR SIGN SHAN SAW
    ('\u{a9e5}', '\u{a9e5}', Extend),
    // Nd  [10] MYANMAR TAI LAING DIGIT ZERO..MYANMAR TAI LAING DIGIT NINE
    ('\u{a9f0}', '\u{a9f9}', Numeric),
    // Lo  [41] CHAM LETTER A..CHAM LETTER HA
    ('\u{aa00}', '\u{aa28}', ALetter),
    // Mn   [6] CHAM VOWEL SIGN AA..CHAM VOWEL SIGN OE
    ('\u{aa29}', '\u{aa2e}', Extend),
    // Mc   [2] CHAM VOWEL SIGN O..CHAM VOWEL SIGN AI
    ('\u{aa2f}', '\u{aa30}', Extend),
    // Mn   [2] CHAM VOWEL SIGN AU..CHAM VOWEL SIGN UE
    ('\u{aa31}', '\u{aa32}', Extend),
    // Mc   [2] CHAM CONSONANT SIGN YA..CHAM CONSONANT SIGN RA
    ('\u{aa33}', '\u{aa34}', Extend),
    // Mn   [2] CHAM CONSONANT SIGN LA..CHAM CONSONANT SIGN WA
    ('\u{aa35}', '\u{aa36}', Extend),
    // Lo   [3] CHAM LETTER FINAL K..CHAM LETTER FINAL NG
    ('\u{aa40}', '\u{aa42}', ALetter),
    // Mn       CHAM CONSONANT SIGN FINAL NG
    ('\u{aa43}', '\u{aa43}', Extend),
    // Lo   [8] CHAM LETTER FINAL CH..CHAM LETTER FINAL SS
    ('\u{aa44}', '\u{aa4b}', ALetter),
    // Mn       CHAM CONSONANT SIGN FINAL M
    ('\u{aa4c}', '\u{aa4c}', Extend),
    // Mc       CHAM CONSONANT SIGN FINAL H
    ('\u{aa4d}', '\u{aa4d}', Extend),
    // Nd  [10] CHAM DIGIT ZERO..CHAM DIGIT NINE
    ('\u{aa50}', '\u{aa59}', Numeric),
    // Mc       MYANMAR SIGN PAO KAREN TONE
    ('\u{aa7b}', '\u{aa7b}', Extend),
    // Mn       MYANMAR SIGN TAI LAING TONE-2
    ('\u{aa7c}', '\u{aa7c}', Extend),
    // Mc       MYANMAR SIGN TAI LAING TONE-5
    ('\u{aa7d}', '\u{aa7d}', Extend),
    // Mn       TAI VIET MAI KANG
    ('\u{aab0}', '\u{aab0}', Extend),
    // Mn   [3] TAI VIET VOWEL I..TAI VIET VOWEL U
    ('\u{aab2}', '\u{aab4}', Extend),
    // Mn   [2] TAI VIET MAI KHIT..TAI VIET VOWEL IA
    ('\u{aab7}', '\u{aab8}', Extend),
    // Mn   [2] TAI VIET VOWEL AM..TAI VIET TONE MAI EK
    ('\u{aabe}', '\u{aabf}', Extend),
    // Mn       TAI VIET TONE MAI THO
    ('\u{aac1}', '\u{aac1}', Extend),
    // Lo  [11] MEETEI MAYEK LETTER E..MEETEI MAYEK LETTER SSA
    ('\u{aae0}', '\u{aaea}', ALetter),
    // Mc       MEETEI MAYEK VOWEL SIGN II
    ('\u{aaeb}', '\u{aaeb}', Extend),
    // Mn   [2] MEETEI MAYEK VOWEL SIGN UU..MEETEI MAYEK VOWEL SIGN AAI
    ('\u{aaec}', '\u{aaed}', Extend),
    // Mc   [2] MEETEI MAYEK VOWEL SIGN AU..MEETEI MAYEK VOWEL SIGN AAU
    ('\u{aaee}', '\u{aaef}', Extend),
    // Lo       MEETEI MAYEK ANJI
    ('\u{aaf2}', '\u{aaf2}', ALetter),
    // Lm   [2] MEETEI MAYEK SYLLABLE REPETITION MARK..MEETEI MAYEK WORD REPETITION MARK
    ('\u{aaf3}', '\u{aaf4}', ALetter),
    // Mc       MEETEI MAYEK VOWEL SIGN VISARGA
    ('\u{aaf5}', '\u{aaf5}', Extend),
    // Mn       MEETEI MAYEK VIRAMA
    ('\u{aaf6}', '\u{aaf6}', Extend),
    // Lo   [6] ETHIOPIC SYLLABLE TTHU..ETHIOPIC SYLLABLE TTHO
    ('\u{ab01}', '\u{ab06}', ALetter),
    // Lo   [6] ETHIOPIC SYLLABLE DDHU..ETHIOPIC SYLLABLE DDHO
    ('\u{ab09}', '\u{ab0e}', ALetter),
    // Lo   [6] ETHIOPIC SYLLABLE DZU..ETHIOPIC SYLLABLE DZO
    ('\u{ab11}', '\u{ab16}', ALetter),
    // Lo   [7] ETHIOPIC SYLLABLE CCHHA..ETHIOPIC SYLLABLE CCHHO
    ('\u{ab20}', '\u{ab26}', ALetter),
    // Lo   [7] ETHIOPIC SYLLABLE BBA..ETHIOPIC SYLLABLE BBO
    ('\u{ab28}', '\u{ab2e}', ALetter),
    // L&  [43] LATIN SMALL LETTER BARRED ALPHA..LATIN SMALL LETTER Y WITH SHORT RIGHT LEG
    ('\u{ab30}', '\u{ab5a}', ALetter),
    // Sk       MODIFIER BREVE WITH INVERTED BREVE
    ('\u{ab5b}', '\u{ab5b}', ALetter),
    // Lm   [4] MODIFIER LETTER SMALL HENG..MODIFIER LETTER SMALL U WITH LEFT HOOK
    ('\u{ab5c}', '\u{ab5f}', ALetter),
    // L&   [8] LATIN SMALL LETTER SAKHA YAT..LATIN SMALL LETTER TS DIGRAPH WITH RETROFLEX HOOK
    ('\u{ab60}', '\u{ab67}', ALetter),
    // L&  [80] CHEROKEE SMALL LETTER A..CHEROKEE SMALL LETTER YA
    ('\u{ab70}', '\u{abbf}', ALetter),
    // Lo  [35] MEETEI MAYEK LETTER KOK..MEETEI MAYEK LETTER I LONSUM
    ('\u{abc0}', '\u{abe2}', ALetter),
    // Mc   [2] MEETEI MAYEK VOWEL SIGN ONAP..MEETEI MAYEK VOWEL SIGN INAP
    ('\u{abe3}', '\u{abe4}', Extend),
    // Mn       MEETEI MAYEK VOWEL SIGN ANAP
    ('\u{abe5}', '\u{abe5}', Extend),
    // Mc   [2] MEETEI MAYEK VOWEL SIGN YENAP..MEETEI MAYEK VOWEL SIGN SOUNAP
    ('\u{abe6}', '\u{abe7}', Extend),
    // Mn       MEETEI MAYEK VOWEL SIGN UNAP
    ('\u{abe8}', '\u{abe8}', Extend),
    // Mc   [2] MEETEI MAYEK VOWEL SIGN CHEINAP..MEETEI MAYEK VOWEL SIGN NUNG
    ('\u{abe9}', '\u{abea}', Extend),
    // Mc       MEETEI MAYEK LUM IYEK
    ('\u{abec}', '\u{abec}', Extend),
    // Mn       MEETEI MAYEK APUN IYEK
    ('\u{abed}', '\u{abed}', Extend),
    // Nd  [10] MEETEI MAYEK DIGIT ZERO..MEETEI MAYEK DIGIT NINE
    ('\u{abf0}', '\u{abf9}', Numeric),
    // Lo [11172] HANGUL SYLLABLE GA..HANGUL SYLLABLE HIH
    ('\u{ac00}', '\u{d7a3}', ALetter),
    // Lo  [23] HANGUL JUNGSEONG O-YEO..HANGUL JUNGSEONG ARAEA-E
    ('\u{d7b0}', '\u{d7c6}', ALetter),
    // Lo  [49] HANGUL JONGSEONG NIEUN-RIEUL..HANGUL JONGSEONG PHIEUPH-THIEUTH
    ('\u{d7cb}', '\u{d7fb}', ALetter),
    // L&   [7] LATIN SMALL LIGATURE FF..LATIN SMALL LIGATURE ST
    ('\u{fb00}', '\u{fb06}', ALetter),
    // L&   [5] ARMENIAN SMALL LIGATURE MEN NOW..ARMENIAN SMALL LIGATURE MEN XEH
    ('\u{fb13}', '\u{fb17}', ALetter),
    // Lo       HEBREW LETTER YOD WITH HIRIQ
    ('\u{fb1d}', '\u{fb1d}', Hebrew_Letter),
    // Mn       HEBREW POINT JUDEO-SPANISH VARIKA
    ('\u{fb1e}', '\u{fb1e}', Extend),
    // Lo  [10] HEBREW LIGATURE YIDDISH YOD YOD PATAH..HEBREW LETTER WIDE TAV
    ('\u{fb1f}', '\u{fb28}', Hebrew_Letter),
    // Lo  [13] HEBREW LETTER SHIN WITH SHIN DOT..HEBREW LETTER ZAYIN WITH DAGESH
    ('\u{fb2a}', '\u{fb36}', Hebrew_Letter),
    // Lo   [5] HEBREW LETTER TET WITH DAGESH..HEBREW LETTER LAMED WITH DAGESH
    ('\u{fb38}', '\u{fb3c}', Hebrew_Letter),
    // Lo       HEBREW LETTER MEM WITH DAGESH
    ('\u{fb3e}', '\u{fb3e}', Hebrew_Letter),
    // Lo   [2] HEBREW LETTER NUN WITH DAGESH..HEBREW LETTER SAMEKH WITH DAGESH
    ('\u{fb40}', '\u{fb41}', Hebrew_Letter),
    // Lo   [2] HEBREW LETTER FINAL PE WITH DAGESH..HEBREW LETTER PE WITH DAGESH
    ('\u{fb43}', '\u{fb44}', Hebrew_Letter),
    // Lo  [10] HEBREW LETTER TSADI WITH DAGESH..HEBREW LIGATURE ALEF LAMED
    ('\u{fb46}', '\u{fb4f}', Hebrew_Letter),
    // Lo  [98] ARABIC LETTER ALEF WASLA ISOLATED FORM..ARABIC LETTER YEH BARREE WITH HAMZA ABOVE FINAL FORM
    ('\u{fb50}', '\u{fbb1}', ALetter),
    // Lo [363] ARABIC LETTER NG ISOLATED FORM..ARABIC LIGATURE ALEF WITH FATHATAN ISOLATED FORM
    ('\u{fbd3}', '\u{fd3d}', ALetter),
    // Lo  [64] ARABIC LIGATURE TEH WITH JEEM WITH MEEM INITIAL FORM..ARABIC LIGATURE MEEM WITH KHAH WITH MEEM INITIAL FORM
    ('\u{fd50}', '\u{fd8f}', ALetter),
    // Lo  [54] ARABIC LIGATURE MEEM WITH JEEM WITH KHAH INITIAL FORM..ARABIC LIGATURE NOON WITH JEEM WITH YEH FINAL FORM
    ('\u{fd92}', '\u{fdc7}', ALetter),
    // Lo  [12] ARABIC LIGATURE SALLA USED AS KORANIC STOP SIGN ISOLATED FORM..ARABIC LIGATURE JALLAJALALOUHOU
    ('\u{fdf0}', '\u{fdfb}', ALetter),
    // Mn  [16] VARIATION SELECTOR-1..VARIATION SELECTOR-16
    ('\u{fe00}', '\u{fe0f}', Extend),
    // Po       PRESENTATION FORM FOR VERTICAL COMMA
    ('\u{fe10}', '\u{fe10}', MidNum),
    // Po       PRESENTATION FORM FOR VERTICAL COLON
    ('\u{fe13}', '\u{fe13}', MidLetter),
    // Po       PRESENTATION FORM FOR VERTICAL SEMICOLON
    ('\u{fe14}', '\u{fe14}', MidNum),
    // Mn  [16] COMBINING LIGATURE LEFT HALF..COMBINING CYRILLIC TITLO RIGHT HALF
    ('\u{fe20}', '\u{fe2f}', Extend),
    // Pc   [2] PRESENTATION FORM FOR VERTICAL LOW LINE..PRESENTATION FORM FOR VERTICAL WAVY LOW LINE
    ('\u{fe33}', '\u{fe34}', ExtendNumLet),
    // Pc   [3] DASHED LOW LINE..WAVY LOW LINE
    ('\u{fe4d}', '\u{fe4f}', ExtendNumLet),
    // Po       SMALL COMMA
    ('\u{fe50}', '\u{fe50}', MidNum),
    // Po       SMALL FULL STOP
    ('\u{fe52}', '\u{fe52}', MidNumLet),
    // Po       SMALL SEMICOLON
    ('\u{fe54}', '\u{fe54}', MidNum),
    // Po       SMALL COLON
    ('\u{fe55}', '\u{fe55}', MidLetter),
    // Lo   [5] ARABIC FATHATAN ISOLATED FORM..ARABIC KASRATAN ISOLATED FORM
    ('\u{fe70}', '\u{fe74}', ALetter),
    // Lo [135] ARABIC FATHA ISOLATED FORM..ARABIC LIGATURE LAM WITH ALEF FINAL FORM
    ('\u{fe76}', '\u{fefc}', ALetter),
    // Cf       ZERO WIDTH NO-BREAK SPACE
    ('\u{feff}', '\u{feff}', Format),
    // Po       FULLWIDTH APOSTROPHE
    ('\u{ff07}', '\u{ff07}', MidNumLet),
    // Po       FULLWIDTH COMMA
    ('\u{ff0c}', '\u{ff0c}', MidNum),
    // Po       FULLWIDTH FULL STOP
    ('\u{ff0e}', '\u{ff0e}', MidNumLet),
    // Nd  [10] FULLWIDTH DIGIT ZERO..FULLWIDTH DIGIT NINE
    ('\u{ff10}', '\u{ff19}', Numeric),
    // Po       FULLWIDTH COLON
    ('\u{ff1a}', '\u{ff1a}', MidLetter),
    // Po       FULLWIDTH SEMICOLON
    ('\u{ff1b}', '\u{ff1b}', MidNum),
    // L&  [26] FULLWIDTH LATIN CAPITAL LETTER A..FULLWIDTH LATIN CAPITAL LETTER Z
    ('\u{ff21}', '\u{ff3a}', ALetter),
    // Pc       FULLWIDTH LOW LINE
    ('\u{ff3f}', '\u{ff3f}', ExtendNumLet),
    // L&  [26] FULLWIDTH LATIN SMALL LETTER A..FULLWIDTH LATIN SMALL LETTER Z
    ('\u{ff41}', '\u{ff5a}', ALetter),
    // Lo  [10] HALFWIDTH KATAKANA LETTER WO..HALFWIDTH KATAKANA LETTER SMALL TU
    ('\u{ff66}', '\u{ff6f}', Katakana),
    // Lm       HALFWIDTH KATAKANA-HIRAGANA PROLONGED SOUND MARK
    ('\u{ff70}', '\u{ff70}', Katakana),
    // Lo  [45] HALFWIDTH KATAKANA LETTER A..HALFWIDTH KATAKANA LETTER N
    ('\u{ff71}', '\u{ff9d}', Katakana),
    // Lm   [2] HALFWIDTH KATAKANA VOICED SOUND MARK..HALFWIDTH KATAKANA SEMI-VOICED SOUND MARK
    ('\u{ff9e}', '\u{ff9f}', Extend),
    // Lo  [31] HALFWIDTH HANGUL FILLER..HALFWIDTH HANGUL LETTER HIEUH
    ('\u{ffa0}', '\u{ffbe}', ALetter),
    // Lo   [6] HALFWIDTH HANGUL LETTER A..HALFWIDTH HANGUL LETTER E
    ('\u{ffc2}', '\u{ffc7}', ALetter),
    // Lo   [6] HALFWIDTH HANGUL LETTER YEO..HALFWIDTH HANGUL LETTER OE
    ('\u{ffca}', '\u{ffcf}', ALetter),
    // Lo   [6] HALFWIDTH HANGUL LETTER YO..HALFWIDTH HANGUL LETTER YU
    ('\u{ffd2}', '\u{ffd7}', ALetter),
    // Lo   [3] HALFWIDTH HANGUL LETTER EU..HALFWIDTH HANGUL LETTER I
    ('\u{ffda}', '\u{ffdc}', ALetter),
    // Cf   [3] INTERLINEAR ANNOTATION ANCHOR..INTERLINEAR ANNOTATION TERMINATOR
    ('\u{fff9}', '\u{fffb}', Format),
    // Lo  [12] LINEAR B SYLLABLE B008 A..LINEAR B SYLLABLE B046 JE
    ('\u{10000}', '\u{1000b}', ALetter),
    // Lo  [26] LINEAR B SYLLABLE B036 JO..LINEAR B SYLLABLE B032 QO
    ('\u{1000d}', '\u{10026}', ALetter),
    // Lo  [19] LINEAR B SYLLABLE B060 RA..LINEAR B SYLLABLE B042 WO
    ('\u{10028}', '\u{1003a}', ALetter),
    // Lo   [2] LINEAR B SYLLABLE B017 ZA..LINEAR B SYLLABLE B074 ZE
    ('\u{1003c}', '\u{1003d}', ALetter),
    // Lo  [15] LINEAR B SYLLABLE B020 ZO..LINEAR B SYLLABLE B091 TWO
    ('\u{1003f}', '\u{1004d}', ALetter),
    // Lo  [14] LINEAR B SYMBOL B018..LINEAR B SYMBOL B089
    ('\u{10050}', '\u{1005d}', ALetter),
    // Lo [123] LINEAR B IDEOGRAM B100 MAN..LINEAR B IDEOGRAM VESSEL B305
    ('\u{10080}', '\u{100fa}', ALetter),
    // Nl  [53] GREEK ACROPHONIC ATTIC ONE QUARTER..GREEK ACROPHONIC STRATIAN FIFTY MNAS
    ('\u{10140}', '\u{10174}', ALetter),
    // Mn       PHAISTOS DISC SIGN COMBINING OBLIQUE STROKE
    ('\u{101fd}', '\u{101fd}', Extend),
    // Lo  [29] LYCIAN LETTER A..LYCIAN LETTER X
    ('\u{10280}', '\u{1029c}', ALetter),
    // Lo  [49] CARIAN LETTER A..CARIAN LETTER UUU3
    ('\u{102a0}', '\u{102d0}', ALetter),
    // Mn       COPTIC EPACT THOUSANDS MARK
    ('\u{102e0}', '\u{102e0}', Extend),
    // Lo  [32] OLD ITALIC LETTER A..OLD ITALIC LETTER ESS
    ('\u{10300}', '\u{1031f}', ALetter),
    // Lo  [20] OLD ITALIC LETTER YE..GOTHIC LETTER PAIRTHRA
    ('\u{1032d}', '\u{10340}', ALetter),
    // Nl       GOTHIC LETTER NINETY
    ('\u{10341}', '\u{10341}', ALetter),
    // Lo   [8] GOTHIC LETTER RAIDA..GOTHIC LETTER OTHAL
    ('\u{10342}', '\u{10349}', ALetter),
    // Nl       GOTHIC LETTER NINE HUNDRED
    ('\u{1034a}', '\u{1034a}', ALetter),
    // Lo  [38] OLD PERMIC LETTER AN..OLD PERMIC LETTER IA
    ('\u{10350}', '\u{10375}', ALetter),
    // Mn   [5] COMBINING OLD PERMIC LETTER AN..COMBINING OLD PERMIC LETTER SII
    ('\u{10376}', '\u{1037a}', Extend),
    // Lo  [30] UGARITIC LETTER ALPA..UGARITIC LETTER SSU
    ('\u{10380}', '\u{1039d}', ALetter),
    // Lo  [36] OLD PERSIAN SIGN A..OLD PERSIAN SIGN HA
    ('\u{103a0}', '\u{103c3}', ALetter),
    // Lo   [8] OLD PERSIAN SIGN AURAMAZDAA..OLD PERSIAN SIGN BUUMISH
    ('\u{103c8}', '\u{103cf}', ALetter),
    // Nl   [5] OLD PERSIAN NUMBER ONE..OLD PERSIAN NUMBER HUNDRED
    ('\u{103d1}', '\u{103d5}', ALetter),
    // L&  [80] DESERET CAPITAL LETTER LONG I..DESERET SMALL LETTER EW
    ('\u{10400}', '\u{1044f}', ALetter),
    // Lo  [78] SHAVIAN LETTER PEEP..OSMANYA LETTER OO
    ('\u{10450}', '\u{1049d}', ALetter),
    // Nd  [10] OSMANYA DIGIT ZERO..OSMANYA DIGIT NINE
    ('\u{104a0}', '\u{104a9}', Numeric),
    // L&  [36] OSAGE CAPITAL LETTER A..OSAGE CAPITAL LETTER ZHA
    ('\u{104b0}', '\u{104d3}', ALetter),
    // L&  [36] OSAGE SMALL LETTER A..OSAGE SMALL LETTER ZHA
    ('\u{104d8}', '\u{104fb}', ALetter),
    // Lo  [40] ELBASAN LETTER A..ELBASAN LETTER KHE
    ('\u{10500}', '\u{10527}', ALetter),
    // Lo  [52] CAUCASIAN ALBANIAN LETTER ALT..CAUCASIAN ALBANIAN LETTER KIW
    ('\u{10530}', '\u{10563}', ALetter),
    // Lo [311] LINEAR A SIGN AB001..LINEAR A SIGN A664
    ('\u{10600}', '\u{10736}', ALetter),
    // Lo  [22] LINEAR A SIGN A701 A..LINEAR A SIGN A732 JE
    ('\u{10740}', '\u{10755}', ALetter),
    // Lo   [8] LINEAR A SIGN A800..LINEAR A SIGN A807
    ('\u{10760}', '\u{10767}', ALetter),
    // Lo   [6] CYPRIOT SYLLABLE A..CYPRIOT SYLLABLE JA
    ('\u{10800}', '\u{10805}', ALetter),
    // Lo       CYPRIOT SYLLABLE JO
    ('\u{10808}', '\u{10808}', ALetter),
    // Lo  [44] CYPRIOT SYLLABLE KA..CYPRIOT SYLLABLE WO
    ('\u{1080a}', '\u{10835}', ALetter),
    // Lo   [2] CYPRIOT SYLLABLE XA..CYPRIOT SYLLABLE XE
    ('\u{10837}', '\u{10838}', ALetter),
    // Lo       CYPRIOT SYLLABLE ZA
    ('\u{1083c}', '\u{1083c}', ALetter),
    // Lo  [23] CYPRIOT SYLLABLE ZO..IMPERIAL ARAMAIC LETTER TAW
    ('\u{1083f}', '\u{10855}', ALetter),
    // Lo  [23] PALMYRENE LETTER ALEPH..PALMYRENE LETTER TAW
    ('\u{10860}', '\u{10876}', ALetter),
    // Lo  [31] NABATAEAN LETTER FINAL ALEPH..NABATAEAN LETTER TAW
    ('\u{10880}', '\u{1089e}', ALetter),
    // Lo  [19] HATRAN LETTER ALEPH..HATRAN LETTER QOPH
    ('\u{108e0}', '\u{108f2}', ALetter),
    // Lo   [2] HATRAN LETTER SHIN..HATRAN LETTER TAW
    ('\u{108f4}', '\u{108f5}', ALetter),
    // Lo  [22] PHOENICIAN LETTER ALF..PHOENICIAN LETTER TAU
    ('\u{10900}', '\u{10915}', ALetter),
    // Lo  [26] LYDIAN LETTER A..LYDIAN LETTER C
    ('\u{10920}', '\u{10939}', ALetter),
    // Lo  [56] MEROITIC HIEROGLYPHIC LETTER A..MEROITIC CURSIVE LETTER DA
    ('\u{10980}', '\u{109b7}', ALetter),
    // Lo   [2] MEROITIC CURSIVE LOGOGRAM RMT..MEROITIC CURSIVE LOGOGRAM IMN
    ('\u{109be}', '\u{109bf}', ALetter),
    // Lo       KHAROSHTHI LETTER A
    ('\u{10a00}', '\u{10a00}', ALetter),
    // Mn   [3] KHAROSHTHI VOWEL SIGN I..KHAROSHTHI VOWEL SIGN VOCALIC R
    ('\u{10a01}', '\u{10a03}', Extend),
    // Mn   [2] KHAROSHTHI VOWEL SIGN E..KHAROSHTHI VOWEL SIGN O
    ('\u{10a05}', '\u{10a06}', Extend),
    // Mn   [4] KHAROSHTHI VOWEL LENGTH MARK..KHAROSHTHI SIGN VISARGA
    ('\u{10a0c}', '\u{10a0f}', Extend),
    // Lo   [4] KHAROSHTHI LETTER KA..KHAROSHTHI LETTER GHA
    ('\u{10a10}', '\u{10a13}', ALetter),
    // Lo   [3] KHAROSHTHI LETTER CA..KHAROSHTHI LETTER JA
    ('\u{10a15}', '\u{10a17}', ALetter),
    // Lo  [29] KHAROSHTHI LETTER NYA..KHAROSHTHI LETTER VHA
    ('\u{10a19}', '\u{10a35}', ALetter),
    // Mn   [3] KHAROSHTHI SIGN BAR ABOVE..KHAROSHTHI SIGN DOT BELOW
    ('\u{10a38}', '\u{10a3a}', Extend),
    // Mn       KHAROSHTHI VIRAMA
    ('\u{10a3f}', '\u{10a3f}', Extend),
    // Lo  [29] OLD SOUTH ARABIAN LETTER HE..OLD SOUTH ARABIAN LETTER THETH
    ('\u{10a60}', '\u{10a7c}', ALetter),
    // Lo  [29] OLD NORTH ARABIAN LETTER HEH..OLD NORTH ARABIAN LETTER ZAH
    ('\u{10a80}', '\u{10a9c}', ALetter),
    // Lo   [8] MANICHAEAN LETTER ALEPH..MANICHAEAN LETTER WAW
    ('\u{10ac0}', '\u{10ac7}', ALetter),
    // Lo  [28] MANICHAEAN LETTER ZAYIN..MANICHAEAN LETTER TAW
    ('\u{10ac9}', '\u{10ae4}', ALetter),
    // Mn   [2] MANICHAEAN ABBREVIATION MARK ABOVE..MANICHAEAN ABBREVIATION MARK BELOW
    ('\u{10ae5}', '\u{10ae6}', Extend),
    // Lo  [54] AVESTAN LETTER A..AVESTAN LETTER HE
    ('\u{10b00}', '\u{10b35}', ALetter),
    // Lo  [22] INSCRIPTIONAL PARTHIAN LETTER ALEPH..INSCRIPTIONAL PARTHIAN LETTER TAW
    ('\u{10b40}', '\u{10b55}', ALetter),
    // Lo  [19] INSCRIPTIONAL PAHLAVI LETTER ALEPH..INSCRIPTIONAL PAHLAVI LETTER TAW
    ('\u{10b60}', '\u{10b72}', ALetter),
    // Lo  [18] PSALTER PAHLAVI LETTER ALEPH..PSALTER PAHLAVI LETTER TAW
    ('\u{10b80}', '\u{10b91}', ALetter),
    // Lo  [73] OLD TURKIC LETTER ORKHON A..OLD TURKIC LETTER ORKHON BASH
    ('\u{10c00}', '\u{10c48}', ALetter),
    // L&  [51] OLD HUNGARIAN CAPITAL LETTER A..OLD HUNGARIAN CAPITAL LETTER US
    ('\u{10c80}', '\u{10cb2}', ALetter),
    // L&  [51] OLD HUNGARIAN SMALL LETTER A..OLD HUNGARIAN SMALL LETTER US
    ('\u{10cc0}', '\u{10cf2}', ALetter),
    // Lo  [36] HANIFI ROHINGYA LETTER A..HANIFI ROHINGYA MARK NA KHONNA
    ('\u{10d00}', '\u{10d23}', ALetter),
    // Mn   [4] HANIFI ROHINGYA SIGN HARBAHAY..HANIFI ROHINGYA SIGN TASSI
    ('\u{10d24}', '\u{10d27}', Extend),
    // Nd  [10] HANIFI ROHINGYA DIGIT ZERO..HANIFI ROHINGYA DIGIT NINE
    ('\u{10d30}', '\u{10d39}', Numeric),
    // Lo  [29] OLD SOGDIAN LETTER ALEPH..OLD SOGDIAN LETTER FINAL TAW WITH VERTICAL TAIL
    ('\u{10f00}', '\u{10f1c}', ALetter),
    // Lo       OLD SOGDIAN LIGATURE AYIN-DALETH
    ('\u{10f27}', '\u{10f27}', ALetter),
    // Lo  [22] SOGDIAN LETTER ALEPH..SOGDIAN INDEPENDENT SHIN
    ('\u{10f30}', '\u{10f45}', ALetter),
    // Mn  [11] SOGDIAN COMBINING DOT BELOW..SOGDIAN COMBINING STROKE BELOW
    ('\u{10f46}', '\u{10f50}', Extend),
    // Lo  [23] ELYMAIC LETTER ALEPH..ELYMAIC LIGATURE ZAYIN-YODH
    ('\u{10fe0}', '\u{10ff6}', ALetter),
    // Mc       BRAHMI SIGN CANDRABINDU
    ('\u{11000}', '\u{11000}', Extend),
    // Mn       BRAHMI SIGN ANUSVARA
    ('\u{11001}', '\u{11001}', Extend),
    // Mc       BRAHMI SIGN VISARGA
    ('\u{11002}', '\u{11002}', Extend),
    // Lo  [53] BRAHMI SIGN JIHVAMULIYA..BRAHMI LETTER OLD TAMIL NNNA
    ('\u{11003}', '\u{11037}', ALetter),
    // Mn  [15] BRAHMI VOWEL SIGN AA..BRAHMI VIRAMA
    ('\u{11038}', '\u{11046}', Extend),
    // Nd  [10] BRAHMI DIGIT ZERO..BRAHMI DIGIT NINE
    ('\u{11066}', '\u{1106f}', Numeric),
    // Mn   [3] BRAHMI NUMBER JOINER..KAITHI SIGN ANUSVARA
    ('\u{1107f}', '\u{11081}', Extend),
    // Mc       KAITHI SIGN VISARGA
    ('\u{11082}', '\u{11082}', Extend),
    // Lo  [45] KAITHI LETTER A..KAITHI LETTER HA
    ('\u{11083}', '\u{110af}', ALetter),
    // Mc   [3] KAITHI VOWEL SIGN AA..KAITHI VOWEL SIGN II
    ('\u{110b0}', '\u{110b2}', Extend),
    // Mn   [4] KAITHI VOWEL SIGN U..KAITHI VOWEL SIGN AI
    ('\u{110b3}', '\u{110b6}', Extend),
    // Mc   [2] KAITHI VOWEL SIGN O..KAITHI VOWEL SIGN AU
    ('\u{110b7}', '\u{110b8}', Extend),
    // Mn   [2] KAITHI SIGN VIRAMA..KAITHI SIGN NUKTA
    ('\u{110b9}', '\u{110ba}', Extend),
    // Cf       KAITHI NUMBER SIGN
    ('\u{110bd}', '\u{110bd}', Format),
    // Cf       KAITHI NUMBER SIGN ABOVE
    ('\u{110cd}', '\u{110cd}', Format),
    // Lo  [25] SORA SOMPENG LETTER SAH..SORA SOMPENG LETTER MAE
    ('\u{110d0}', '\u{110e8}', ALetter),
    // Nd  [10] SORA SOMPENG DIGIT ZERO..SORA SOMPENG DIGIT NINE
    ('\u{110f0}', '\u{110f9}', Numeric),
    // Mn   [3] CHAKMA SIGN CANDRABINDU..CHAKMA SIGN VISARGA
    ('\u{11100}', '\u{11102}', Extend),
    // Lo  [36] CHAKMA LETTER AA..CHAKMA LETTER HAA
    ('\u{11103}', '\u{11126}', ALetter),
    // Mn   [5] CHAKMA VOWEL SIGN A..CHAKMA VOWEL SIGN UU
    ('\u{11127}', '\u{1112b}', Extend),
    // Mc       CHAKMA VOWEL SIGN E
    ('\u{1112c}', '\u{1112c}', Extend),
    // Mn   [8] CHAKMA VOWEL SIGN AI..CHAKMA MAAYYAA
    ('\u{1112d}', '\u{11134}', Extend),
    // Nd  [10] CHAKMA DIGIT ZERO..CHAKMA DIGIT NINE
    ('\u{11136}', '\u{1113f}', Numeric),
    // Lo       CHAKMA LETTER LHAA
    ('\u{11144}', '\u{11144}', ALetter),
    // Mc   [2] CHAKMA VOWEL SIGN AA..CHAKMA VOWEL SIGN EI
    ('\u{11145}', '\u{11146}', Extend),
    // Lo  [35] MAHAJANI LETTER A..MAHAJANI LETTER RRA
    ('\u{11150}', '\u{11172}', ALetter),
    // Mn       MAHAJANI SIGN NUKTA
    ('\u{11173}', '\u{11173}', Extend),
    // Lo       MAHAJANI LIGATURE SHRI
    ('\u{11176}', '\u{11176}', ALetter),
    // Mn   [2] SHARADA SIGN CANDRABINDU..SHARADA SIGN ANUSVARA
    ('\u{11180}', '\u{11181}', Extend),
    // Mc       SHARADA SIGN VISARGA
    ('\u{11182}', '\u{11182}', Extend),
    // Lo  [48] SHARADA LETTER A..SHARADA LETTER HA
    ('\u{11183}', '\u{111b2}', ALetter),
    // Mc   [3] SHARADA VOWEL SIGN AA..SHARADA VOWEL SIGN II
    ('\u{111b3}', '\u{111b5}', Extend),
    // Mn   [9] SHARADA VOWEL SIGN U..SHARADA VOWEL SIGN O
    ('\u{111b6}', '\u{111be}', Extend),
    // Mc   [2] SHARADA VOWEL SIGN AU..SHARADA SIGN VIRAMA
    ('\u{111bf}', '\u{111c0}', Extend),
    // Lo   [4] SHARADA SIGN AVAGRAHA..SHARADA OM
    ('\u{111c1}', '\u{111c4}', ALetter),
    // Mn   [4] SHARADA SANDHI MARK..SHARADA EXTRA SHORT VOWEL MARK
    ('\u{111c9}', '\u{111cc}', Extend),
    // Nd  [10] SHARADA DIGIT ZERO..SHARADA DIGIT NINE
    ('\u{111d0}', '\u{111d9}', Numeric),
    // Lo       SHARADA EKAM
    ('\u{111da}', '\u{111da}', ALetter),
    // Lo       SHARADA HEADSTROKE
    ('\u{111dc}', '\u{111dc}', ALetter),
    // Lo  [18] KHOJKI LETTER A..KHOJKI LETTER JJA
    ('\u{11200}', '\u{11211}', ALetter),
    // Lo  [25] KHOJKI LETTER NYA..KHOJKI LETTER LLA
    ('\u{11213}', '\u{1122b}', ALetter),
    // Mc   [3] KHOJKI VOWEL SIGN AA..KHOJKI VOWEL SIGN II
    ('\u{1122c}', '\u{1122e}', Extend),
    // Mn   [3] KHOJKI VOWEL SIGN U..KHOJKI VOWEL SIGN AI
    ('\u{1122f}', '\u{11231}', Extend),
    // Mc   [2] KHOJKI VOWEL SIGN O..KHOJKI VOWEL SIGN AU
    ('\u{11232}', '\u{11233}', Extend),
    // Mn       KHOJKI SIGN ANUSVARA
    ('\u{11234}', '\u{11234}', Extend),
    // Mc       KHOJKI SIGN VIRAMA
    ('\u{11235}', '\u{11235}', Extend),
    // Mn   [2] KHOJKI SIGN NUKTA..KHOJKI SIGN SHADDA
    ('\u{11236}', '\u{11237}', Extend),
    // Mn       KHOJKI SIGN SUKUN
    ('\u{1123e}', '\u{1123e}', Extend),
    // Lo   [7] MULTANI LETTER A..MULTANI LETTER GA
    ('\u{11280}', '\u{11286}', ALetter),
    // Lo       MULTANI LETTER GHA
    ('\u{11288}', '\u{11288}', ALetter),
    // Lo   [4] MULTANI LETTER CA..MULTANI LETTER JJA
    ('\u{1128a}', '\u{1128d}', ALetter),
    // Lo  [15] MULTANI LETTER NYA..MULTANI LETTER BA
    ('\u{1128f}', '\u{1129d}', ALetter),
    // Lo  [10] MULTANI LETTER BHA..MULTANI LETTER RHA
    ('\u{1129f}', '\u{112a8}', ALetter),
    // Lo  [47] KHUDAWADI LETTER A..KHUDAWADI LETTER HA
    ('\u{112b0}', '\u{112de}', ALetter),
    // Mn       KHUDAWADI SIGN ANUSVARA
    ('\u{112df}', '\u{112df}', Extend),
    // Mc   [3] KHUDAWADI VOWEL SIGN AA..KHUDAWADI VOWEL SIGN II
    ('\u{112e0}', '\u{112e2}', Extend),
    // Mn   [8] KHUDAWADI VOWEL SIGN U..KHUDAWADI SIGN VIRAMA
    ('\u{112e3}', '\u{112ea}', Extend),
    // Nd  [10] KHUDAWADI DIGIT ZERO..KHUDAWADI DIGIT NINE
    ('\u{112f0}', '\u{112f9}', Numeric),
    // Mn   [2] GRANTHA SIGN COMBINING ANUSVARA ABOVE..GRANTHA SIGN CANDRABINDU
    ('\u{11300}', '\u{11301}', Extend),
    // Mc   [2] GRANTHA SIGN ANUSVARA..GRANTHA SIGN VISARGA
    ('\u{11302}', '\u{11303}', Extend),
    // Lo   [8] GRANTHA LETTER A..GRANTHA LETTER VOCALIC L
    ('\u{11305}', '\u{1130c}', ALetter),
    // Lo   [2] GRANTHA LETTER EE..GRANTHA LETTER AI
    ('\u{1130f}', '\u{11310}', ALetter),
    // Lo  [22] GRANTHA LETTER OO..GRANTHA LETTER NA
    ('\u{11313}', '\u{11328}', ALetter),
    // Lo   [7] GRANTHA LETTER PA..GRANTHA LETTER RA
    ('\u{1132a}', '\u{11330}', ALetter),
    // Lo   [2] GRANTHA LETTER LA..GRANTHA LETTER LLA
    ('\u{11332}', '\u{11333}', ALetter),
    // Lo   [5] GRANTHA LETTER VA..GRANTHA LETTER HA
    ('\u{11335}', '\u{11339}', ALetter),
    // Mn   [2] COMBINING BINDU BELOW..GRANTHA SIGN NUKTA
    ('\u{1133b}', '\u{1133c}', Extend),
    // Lo       GRANTHA SIGN AVAGRAHA
    ('\u{1133d}', '\u{1133d}', ALetter),
    // Mc   [2] GRANTHA VOWEL SIGN AA..GRANTHA VOWEL SIGN I
    ('\u{1133e}', '\u{1133f}', Extend),
    // Mn       GRANTHA VOWEL SIGN II
    ('\u{11340}', '\u{11340}', Extend),
    // Mc   [4] GRANTHA VOWEL SIGN U..GRANTHA VOWEL SIGN VOCALIC RR
    ('\u{11341}', '\u{11344}', Extend),
    // Mc   [2] GRANTHA VOWEL SIGN EE..GRANTHA VOWEL SIGN AI
    ('\u{11347}', '\u{11348}', Extend),
    // Mc   [3] GRANTHA VOWEL SIGN OO..GRANTHA SIGN VIRAMA
    ('\u{1134b}', '\u{1134d}', Extend),
    // Lo       GRANTHA OM
    ('\u{11350}', '\u{11350}', ALetter),
    // Mc       GRANTHA AU LENGTH MARK
    ('\u{11357}', '\u{11357}', Extend),
    // Lo   [5] GRANTHA SIGN PLUTA..GRANTHA LETTER VOCALIC LL
    ('\u{1135d}', '\u{11361}', ALetter),
    // Mc   [2] GRANTHA VOWEL SIGN VOCALIC L..GRANTHA VOWEL SIGN VOCALIC LL
    ('\u{11362}', '\u{11363}', Extend),
    // Mn   [7] COMBINING GRANTHA DIGIT ZERO..COMBINING GRANTHA DIGIT SIX
    ('\u{11366}', '\u{1136c}', Extend),
    // Mn   [5] COMBINING GRANTHA LETTER A..COMBINING GRANTHA LETTER PA
    ('\u{11370}', '\u{11374}', Extend),
    // Lo  [53] NEWA LETTER A..NEWA LETTER HA
    ('\u{11400}', '\u{11434}', ALetter),
    // Mc   [3] NEWA VOWEL SIGN AA..NEWA VOWEL SIGN II
    ('\u{11435}', '\u{11437}', Extend),
    // Mn   [8] NEWA VOWEL SIGN U..NEWA VOWEL SIGN AI
    ('\u{11438}', '\u{1143f}', Extend),
    // Mc   [2] NEWA VOWEL SIGN O..NEWA VOWEL SIGN AU
    ('\u{11440}', '\u{11441}', Extend),
    // Mn   [3] NEWA SIGN VIRAMA..NEWA SIGN ANUSVARA
    ('\u{11442}', '\u{11444}', Extend),
    // Mc       NEWA SIGN VISARGA
    ('\u{11445}', '\u{11445}', Extend),
    // Mn       NEWA SIGN NUKTA
    ('\u{11446}', '\u{11446}', Extend),
    // Lo   [4] NEWA SIGN AVAGRAHA..NEWA SIDDHI
    ('\u{11447}', '\u{1144a}', ALetter),
    // Nd  [10] NEWA DIGIT ZERO..NEWA DIGIT NINE
    ('\u{11450}', '\u{11459}', Numeric),
    // Mn       NEWA SANDHI MARK
    ('\u{1145e}', '\u{1145e}', Extend),
    // Lo       NEWA LETTER VEDIC ANUSVARA
    ('\u{1145f}', '\u{1145f}', ALetter),
    // Lo  [48] TIRHUTA ANJI..TIRHUTA LETTER HA
    ('\u{11480}', '\u{114af}', ALetter),
    // Mc   [3] TIRHUTA VOWEL SIGN AA..TIRHUTA VOWEL SIGN II
    ('\u{114b0}', '\u{114b2}', Extend),
    // Mn   [6] TIRHUTA VOWEL SIGN U..TIRHUTA VOWEL SIGN VOCALIC LL
    ('\u{114b3}', '\u{114b8}', Extend),
    // Mc       TIRHUTA VOWEL SIGN E
    ('\u{114b9}', '\u{114b9}', Extend),
    // Mn       TIRHUTA VOWEL SIGN SHORT E
    ('\u{114ba}', '\u{114ba}', Extend),
    // Mc   [4] TIRHUTA VOWEL SIGN AI..TIRHUTA VOWEL SIGN AU
    ('\u{114bb}', '\u{114be}', Extend),
    // Mn   [2] TIRHUTA SIGN CANDRABINDU..TIRHUTA SIGN ANUSVARA
    ('\u{114bf}', '\u{114c0}', Extend),
    // Mc       TIRHUTA SIGN VISARGA
    ('\u{114c1}', '\u{114c1}', Extend),
    // Mn   [2] TIRHUTA SIGN VIRAMA..TIRHUTA SIGN NUKTA
    ('\u{114c2}', '\u{114c3}', Extend),
    // Lo   [2] TIRHUTA SIGN AVAGRAHA..TIRHUTA GVANG
    ('\u{114c4}', '\u{114c5}', ALetter),
    // Lo       TIRHUTA OM
    ('\u{114c7}', '\u{114c7}', ALetter),
    // Nd  [10] TIRHUTA DIGIT ZERO..TIRHUTA DIGIT NINE
    ('\u{114d0}', '\u{114d9}', Numeric),
    // Lo  [47] SIDDHAM LETTER A..SIDDHAM LETTER HA
    ('\u{11580}', '\u{115ae}', ALetter),
    // Mc   [3] SIDDHAM VOWEL SIGN AA..SIDDHAM VOWEL SIGN II
    ('\u{115af}', '\u{115b1}', Extend),
    // Mn   [4] SIDDHAM VOWEL SIGN U..SIDDHAM VOWEL SIGN VOCALIC RR
    ('\u{115b2}', '\u{115b5}', Extend),
    // Mc   [4] SIDDHAM VOWEL SIGN E..SIDDHAM VOWEL SIGN AU
    ('\u{115b8}', '\u{115bb}', Extend),
    // Mn   [2] SIDDHAM SIGN CANDRABINDU..SIDDHAM SIGN ANUSVARA
    ('\u{115bc}', '\u{115bd}', Extend),
    // Mc       SIDDHAM SIGN VISARGA
    ('\u{115be}', '\u{115be}', Extend),
    // Mn   [2] SIDDHAM SIGN VIRAMA..SIDDHAM SIGN NUKTA
    ('\u{115bf}', '\u{115c0}', Extend),
    // Lo   [4] SIDDHAM LETTER THREE-CIRCLE ALTERNATE I..SIDDHAM LETTER ALTERNATE U
    ('\u{115d8}', '\u{115db}', ALetter),
    // Mn   [2] SIDDHAM VOWEL SIGN ALTERNATE U..SIDDHAM VOWEL SIGN ALTERNATE UU
    ('\u{115dc}', '\u{115dd}', Extend),
    // Lo  [48] MODI LETTER A..MODI LETTER LLA
    ('\u{11600}', '\u{1162f}', ALetter),
    // Mc   [3] MODI VOWEL SIGN AA..MODI VOWEL SIGN II
    ('\u{11630}', '\u{11632}', Extend),
    // Mn   [8] MODI VOWEL SIGN U..MODI VOWEL SIGN AI
    ('\u{11633}', '\u{1163a}', Extend),
    // Mc   [2] MODI VOWEL SIGN O..MODI VOWEL SIGN AU
    ('\u{1163b}', '\u{1163c}', Extend),
    // Mn       MODI SIGN ANUSVARA
    ('\u{1163d}', '\u{1163d}', Extend),
    // Mc       MODI SIGN VISARGA
    ('\u{1163e}', '\u{1163e}', Extend),
    // Mn   [2] MODI SIGN VIRAMA..MODI SIGN ARDHACANDRA
    ('\u{1163f}', '\u{11640}', Extend),
    // Lo       MODI SIGN HUVA
    ('\u{11644}', '\u{11644}', ALetter),
    // Nd  [10] MODI DIGIT ZERO..MODI DIGIT NINE
    ('\u{11650}', '\u{11659}', Numeric),
    // Lo  [43] TAKRI LETTER A..TAKRI LETTER RRA
    ('\u{11680}', '\u{116aa}', ALetter),
    // Mn       TAKRI SIGN ANUSVARA
    ('\u{116ab}', '\u{116ab}', Extend),
    // Mc       TAKRI SIGN VISARGA
    ('\u{116ac}', '\u{116ac}', Extend),
    // Mn       TAKRI VOWEL SIGN AA
    ('\u{116ad}', '\u{116ad}', Extend),
    // Mc   [2] TAKRI VOWEL SIGN I..TAKRI VOWEL SIGN II
    ('\u{116ae}', '\u{116af}', Extend),
    // Mn   [6] TAKRI VOWEL SIGN U..TAKRI VOWEL SIGN AU
    ('\u{116b0}', '\u{116b5}', Extend),
    // Mc       TAKRI SIGN VIRAMA
    ('\u{116b6}', '\u{116b6}', Extend),
    // Mn       TAKRI SIGN NUKTA
    ('\u{116b7}', '\u{116b7}', Extend),
    // Lo       TAKRI LETTER ARCHAIC KHA
    ('\u{116b8}', '\u{116b8}', ALetter),
    // Nd  [10] TAKRI DIGIT ZERO..TAKRI DIGIT NINE
    ('\u{116c0}', '\u{116c9}', Numeric),
    // Mn   [3] AHOM CONSONANT SIGN MEDIAL LA..AHOM CONSONANT SIGN MEDIAL LIGATING RA
    ('\u{1171d}', '\u{1171f}', Extend),
    // Mc   [2] AHOM VOWEL SIGN A..AHOM VOWEL SIGN AA
    ('\u{11720}', '\u{11721}', Extend),
    // Mn   [4] AHOM VOWEL SIGN I..AHOM VOWEL SIGN UU
    ('\u{11722}', '\u{11725}', Extend),
    // Mc       AHOM VOWEL SIGN E
    ('\u{11726}', '\u{11726}', Extend),
    // Mn   [5] AHOM VOWEL SIGN AW..AHOM SIGN KILLER
    ('\u{11727}', '\u{1172b}', Extend),
    // Nd  [10] AHOM DIGIT ZERO..AHOM DIGIT NINE
    ('\u{11730}', '\u{11739}', Numeric),
    // Lo  [44] DOGRA LETTER A..DOGRA LETTER RRA
    ('\u{11800}', '\u{1182b}', ALetter),
    // Mc   [3] DOGRA VOWEL SIGN AA..DOGRA VOWEL SIGN II
    ('\u{1182c}', '\u{1182e}', Extend),
    // Mn   [9] DOGRA VOWEL SIGN U..DOGRA SIGN ANUSVARA
    ('\u{1182f}', '\u{11837}', Extend),
    // Mc       DOGRA SIGN VISARGA
    ('\u{11838}', '\u{11838}', Extend),
    // Mn   [2] DOGRA SIGN VIRAMA..DOGRA SIGN NUKTA
    ('\u{11839}', '\u{1183a}', Extend),
    // L&  [64] WARANG CITI CAPITAL LETTER NGAA..WARANG CITI SMALL LETTER VIYO
    ('\u{118a0}', '\u{118df}', ALetter),
    // Nd  [10] WARANG CITI DIGIT ZERO..WARANG CITI DIGIT NINE
    ('\u{118e0}', '\u{118e9}', Numeric),
    // Lo       WARANG CITI OM
    ('\u{118ff}', '\u{118ff}', ALetter),
    // Lo   [8] NANDINAGARI LETTER A..NANDINAGARI LETTER VOCALIC RR
    ('\u{119a0}', '\u{119a7}', ALetter),
    // Lo  [39] NANDINAGARI LETTER E..NANDINAGARI LETTER RRA
    ('\u{119aa}', '\u{119d0}', ALetter),
    // Mc   [3] NANDINAGARI VOWEL SIGN AA..NANDINAGARI VOWEL SIGN II
    ('\u{119d1}', '\u{119d3}', Extend),
    // Mn   [4] NANDINAGARI VOWEL SIGN U..NANDINAGARI VOWEL SIGN VOCALIC RR
    ('\u{119d4}', '\u{119d7}', Extend),
    // Mn   [2] NANDINAGARI VOWEL SIGN E..NANDINAGARI VOWEL SIGN AI
    ('\u{119da}', '\u{119db}', Extend),
    // Mc   [4] NANDINAGARI VOWEL SIGN O..NANDINAGARI SIGN VISARGA
    ('\u{119dc}', '\u{119df}', Extend),
    // Mn       NANDINAGARI SIGN VIRAMA
    ('\u{119e0}', '\u{119e0}', Extend),
    // Lo       NANDINAGARI SIGN AVAGRAHA
    ('\u{119e1}', '\u{119e1}', ALetter),
    // Lo       NANDINAGARI HEADSTROKE
    ('\u{119e3}', '\u{119e3}', ALetter),
    // Mc       NANDINAGARI VOWEL SIGN PRISHTHAMATRA E
    ('\u{119e4}', '\u{119e4}', Extend),
    // Lo       ZANABAZAR SQUARE LETTER A
    ('\u{11a00}', '\u{11a00}', ALetter),
    // Mn  [10] ZANABAZAR SQUARE VOWEL SIGN I..ZANABAZAR SQUARE VOWEL LENGTH MARK
    ('\u{11a01}', '\u{11a0a}', Extend),
    // Lo  [40] ZANABAZAR SQUARE LETTER KA..ZANABAZAR SQUARE LETTER KSSA
    ('\u{11a0b}', '\u{11a32}', ALetter),
    // Mn   [6] ZANABAZAR SQUARE FINAL CONSONANT MARK..ZANABAZAR SQUARE SIGN ANUSVARA
    ('\u{11a33}', '\u{11a38}', Extend),
    // Mc       ZANABAZAR SQUARE SIGN VISARGA
    ('\u{11a39}', '\u{11a39}', Extend),
    // Lo       ZANABAZAR SQUARE CLUSTER-INITIAL LETTER RA
    ('\u{11a3a}', '\u{11a3a}', ALetter),
    // Mn   [4] ZANABAZAR SQUARE CLUSTER-FINAL LETTER YA..ZANABAZAR SQUARE CLUSTER-FINAL LETTER VA
    ('\u{11a3b}', '\u{11a3e}', Extend),
    // Mn       ZANABAZAR SQUARE SUBJOINER
    ('\u{11a47}', '\u{11a47}', Extend),
    // Lo       SOYOMBO LETTER A
    ('\u{11a50}', '\u{11a50}', ALetter),
    // Mn   [6] SOYOMBO VOWEL SIGN I..SOYOMBO VOWEL SIGN OE
    ('\u{11a51}', '\u{11a56}', Extend),
    // Mc   [2] SOYOMBO VOWEL SIGN AI..SOYOMBO VOWEL SIGN AU
    ('\u{11a57}', '\u{11a58}', Extend),
    // Mn   [3] SOYOMBO VOWEL SIGN VOCALIC R..SOYOMBO VOWEL LENGTH MARK
    ('\u{11a59}', '\u{11a5b}', Extend),
    // Lo  [46] SOYOMBO LETTER KA..SOYOMBO CLUSTER-INITIAL LETTER SA
    ('\u{11a5c}', '\u{11a89}', ALetter),
    // Mn  [13] SOYOMBO FINAL CONSONANT SIGN G..SOYOMBO SIGN ANUSVARA
    ('\u{11a8a}', '\u{11a96}', Extend),
    // Mc       SOYOMBO SIGN VISARGA
    ('\u{11a97}', '\u{11a97}', Extend),
    // Mn   [2] SOYOMBO GEMINATION MARK..SOYOMBO SUBJOINER
    ('\u{11a98}', '\u{11a99}', Extend),
    // Lo       SOYOMBO MARK PLUTA
    ('\u{11a9d}', '\u{11a9d}', ALetter),
    // Lo  [57] PAU CIN HAU LETTER PA..PAU CIN HAU GLOTTAL STOP FINAL
    ('\u{11ac0}', '\u{11af8}', ALetter),
    // Lo   [9] BHAIKSUKI LETTER A..BHAIKSUKI LETTER VOCALIC L
    ('\u{11c00}', '\u{11c08}', ALetter),
    // Lo  [37] BHAIKSUKI LETTER E..BHAIKSUKI LETTER HA
    ('\u{11c0a}', '\u{11c2e}', ALetter),
    // Mc       BHAIKSUKI VOWEL SIGN AA
    ('\u{11c2f}', '\u{11c2f}', Extend),
    // Mn   [7] BHAIKSUKI VOWEL SIGN I..BHAIKSUKI VOWEL SIGN VOCALIC L
    ('\u{11c30}', '\u{11c36}', Extend),
    // Mn   [6] BHAIKSUKI VOWEL SIGN E..BHAIKSUKI SIGN ANUSVARA
    ('\u{11c38}', '\u{11c3d}', Extend),
    // Mc       BHAIKSUKI SIGN VISARGA
    ('\u{11c3e}', '\u{11c3e}', Extend),
    // Mn       BHAIKSUKI SIGN VIRAMA
    ('\u{11c3f}', '\u{11c3f}', Extend),
    // Lo       BHAIKSUKI SIGN AVAGRAHA
    ('\u{11c40}', '\u{11c40}', ALetter),
    // Nd  [10] BHAIKSUKI DIGIT ZERO..BHAIKSUKI DIGIT NINE
    ('\u{11c50}', '\u{11c59}', Numeric),
    // Lo  [30] MARCHEN LETTER KA..MARCHEN LETTER A
    ('\u{11c72}', '\u{11c8f}', ALetter),
    // Mn  [22] MARCHEN SUBJOINED LETTER KA..MARCHEN SUBJOINED LETTER ZA
    ('\u{11c92}', '\u{11ca7}', Extend),
    // Mc       MARCHEN SUBJOINED LETTER YA
    ('\u{11ca9}', '\u{11ca9}', Extend),
    // Mn   [7] MARCHEN SUBJOINED LETTER RA..MARCHEN VOWEL SIGN AA
    ('\u{11caa}', '\u{11cb0}', Extend),
    // Mc       MARCHEN VOWEL SIGN I
    ('\u{11cb1}', '\u{11cb1}', Extend),
    // Mn   [2] MARCHEN VOWEL SIGN U..MARCHEN VOWEL SIGN E
    ('\u{11cb2}', '\u{11cb3}', Extend),
    // Mc       MARCHEN VOWEL SIGN O
    ('\u{11cb4}', '\u{11cb4}', Extend),
    // Mn   [2] MARCHEN SIGN ANUSVARA..MARCHEN SIGN CANDRABINDU
    ('\u{11cb5}', '\u{11cb6}', Extend),
    // Lo   [7] MASARAM GONDI LETTER A..MASARAM GONDI LETTER E
    ('\u{11d00}', '\u{11d06}', ALetter),
    // Lo   [2] MASARAM GONDI LETTER AI..MASARAM GONDI LETTER O
    ('\u{11d08}', '\u{11d09}', ALetter),
    // Lo  [38] MASARAM GONDI LETTER AU..MASARAM GONDI LETTER TRA
    ('\u{11d0b}', '\u{11d30}', ALetter),
    // Mn   [6] MASARAM GONDI VOWEL SIGN AA..MASARAM GONDI VOWEL SIGN VOCALIC R
    ('\u{11d31}', '\u{11d36}', Extend),
    // Mn       MASARAM GONDI VOWEL SIGN E
    ('\u{11d3a}', '\u{11d3a}', Extend),
    // Mn   [2] MASARAM GONDI VOWEL SIGN AI..MASARAM GONDI VOWEL SIGN O
    ('\u{11d3c}', '\u{11d3d}', Extend),
    // Mn   [7] MASARAM GONDI VOWEL SIGN AU..MASARAM GONDI VIRAMA
    ('\u{11d3f}', '\u{11d45}', Extend),
    // Lo       MASARAM GONDI REPHA
    ('\u{11d46}', '\u{11d46}', ALetter),
    // Mn       MASARAM GONDI RA-KARA
    ('\u{11d47}', '\u{11d47}', Extend),
    // Nd  [10] MASARAM GONDI DIGIT ZERO..MASARAM GONDI DIGIT NINE
    ('\u{11d50}', '\u{11d59}', Numeric),
    // Lo   [6] GUNJALA GONDI LETTER A..GUNJALA GONDI LETTER UU
    ('\u{11d60}', '\u{11d65}', ALetter),
    // Lo   [2] GUNJALA GONDI LETTER EE..GUNJALA GONDI LETTER AI
    ('\u{11d67}', '\u{11d68}', ALetter),
    // Lo  [32] GUNJALA GONDI LETTER OO..GUNJALA GONDI LETTER SA
    ('\u{11d6a}', '\u{11d89}', ALetter),
    // Mc   [5] GUNJALA GONDI VOWEL SIGN AA..GUNJALA GONDI VOWEL SIGN UU
    ('\u{11d8a}', '\u{11d8e}', Extend),
    // Mn   [2] GUNJALA GONDI VOWEL SIGN EE..GUNJALA GONDI VOWEL SIGN AI
    ('\u{11d90}', '\u{11d91}', Extend),
    // Mc   [2] GUNJALA GONDI VOWEL SIGN OO..GUNJALA GONDI VOWEL SIGN AU
    ('\u{11d93}', '\u{11d94}', Extend),
    // Mn       GUNJALA GONDI SIGN ANUSVARA
    ('\u{11d95}', '\u{11d95}', Extend),
    // Mc       GUNJALA GONDI SIGN VISARGA
    ('\u{11d96}', '\u{11d96}', Extend),
    // Mn       GUNJALA GONDI VIRAMA
    ('\u{11d97}', '\u{11d97}', Extend),
    // Lo       GUNJALA GONDI OM
    ('\u{11d98}', '\u{11d98}', ALetter),
    // Nd  [10] GUNJALA GONDI DIGIT ZERO..GUNJALA GONDI DIGIT NINE
    ('\u{11da0}', '\u{11da9}', Numeric),
    // Lo  [19] MAKASAR LETTER KA..MAKASAR ANGKA
    ('\u{11ee0}', '\u{11ef2}', ALetter),
    // Mn   [2] MAKASAR VOWEL SIGN I..MAKASAR VOWEL SIGN U
    ('\u{11ef3}', '\u{11ef4}', Extend),
    // Mc   [2] MAKASAR VOWEL SIGN E..MAKASAR VOWEL SIGN O
    ('\u{11ef5}', '\u{11ef6}', Extend),
    // Lo [922] CUNEIFORM SIGN A..CUNEIFORM SIGN U U
    ('\u{12000}', '\u{12399}', ALetter),
    // Nl [111] CUNEIFORM NUMERIC SIGN TWO ASH..CUNEIFORM NUMERIC SIGN NINE U VARIANT FORM
    ('\u{12400}', '\u{1246e}', ALetter),
    // Lo [196] CUNEIFORM SIGN AB TIMES NUN TENU..CUNEIFORM SIGN ZU5 TIMES THREE DISH TENU
    ('\u{12480}', '\u{12543}', ALetter),
    // Lo [1071] EGYPTIAN HIEROGLYPH A001..EGYPTIAN HIEROGLYPH AA032
    ('\u{13000}', '\u{1342e}', ALetter),
    // Cf   [9] EGYPTIAN HIEROGLYPH VERTICAL JOINER..EGYPTIAN HIEROGLYPH END SEGMENT
    ('\u{13430}', '\u{13438}', Format),
    // Lo [583] ANATOLIAN HIEROGLYPH A001..ANATOLIAN HIEROGLYPH A530
    ('\u{14400}', '\u{14646}', ALetter),
    // Lo [569] BAMUM LETTER PHASE-A NGKUE MFON..BAMUM LETTER PHASE-F VUEQ
    ('\u{16800}', '\u{16a38}', ALetter),
    // Lo  [31] MRO LETTER TA..MRO LETTER TEK
    ('\u{16a40}', '\u{16a5e}', ALetter),
    // Nd  [10] MRO DIGIT ZERO..MRO DIGIT NINE
    ('\u{16a60}', '\u{16a69}', Numeric),
    // Lo  [30] BASSA VAH LETTER ENNI..BASSA VAH LETTER I
    ('\u{16ad0}', '\u{16aed}', ALetter),
    // Mn   [5] BASSA VAH COMBINING HIGH TONE..BASSA VAH COMBINING HIGH-LOW TONE
    ('\u{16af0}', '\u{16af4}', Extend),
    // Lo  [48] PAHAWH HMONG VOWEL KEEB..PAHAWH HMONG CONSONANT CAU
    ('\u{16b00}', '\u{16b2f}', ALetter),
    // Mn   [7] PAHAWH HMONG MARK CIM TUB..PAHAWH HMONG MARK CIM TAUM
    ('\u{16b30}', '\u{16b36}', Extend),
    // Lm   [4] PAHAWH HMONG SIGN VOS SEEV..PAHAWH HMONG SIGN IB YAM
    ('\u{16b40}', '\u{16b43}', ALetter),
    // Nd  [10] PAHAWH HMONG DIGIT ZERO..PAHAWH HMONG DIGIT NINE
    ('\u{16b50}', '\u{16b59}', Numeric),
    // Lo  [21] PAHAWH HMONG SIGN VOS LUB..PAHAWH HMONG SIGN CIM NRES TOS
    ('\u{16b63}', '\u{16b77}', ALetter),
    // Lo  [19] PAHAWH HMONG CLAN SIGN TSHEEJ..PAHAWH HMONG CLAN SIGN VWJ
    ('\u{16b7d}', '\u{16b8f}', ALetter),
    // L&  [64] MEDEFAIDRIN CAPITAL LETTER M..MEDEFAIDRIN SMALL LETTER Y
    ('\u{16e40}', '\u{16e7f}', ALetter),
    // Lo  [75] MIAO LETTER PA..MIAO LETTER RTE
    ('\u{16f00}', '\u{16f4a}', ALetter),
    // Mn       MIAO SIGN CONSONANT MODIFIER BAR
    ('\u{16f4f}', '\u{16f4f}', Extend),
    // Lo       MIAO LETTER NASALIZATION
    ('\u{16f50}', '\u{16f50}', ALetter),
    // Mc  [55] MIAO SIGN ASPIRATION..MIAO VOWEL SIGN UI
    ('\u{16f51}', '\u{16f87}', Extend),
    // Mn   [4] MIAO TONE RIGHT..MIAO TONE BELOW
    ('\u{16f8f}', '\u{16f92}', Extend),
    // Lm  [13] MIAO LETTER TONE-2..MIAO LETTER REFORMED TONE-8
    ('\u{16f93}', '\u{16f9f}', ALetter),
    // Lm   [2] TANGUT ITERATION MARK..NUSHU ITERATION MARK
    ('\u{16fe0}', '\u{16fe1}', ALetter),
    // Lm       OLD CHINESE ITERATION MARK
    ('\u{16fe3}', '\u{16fe3}', ALetter),
    // Lo       KATAKANA LETTER ARCHAIC E
    ('\u{1b000}', '\u{1b000}', Katakana),
    // Lo   [4] KATAKANA LETTER SMALL WI..KATAKANA LETTER SMALL N
    ('\u{1b164}', '\u{1b167}', Katakana),
    // Lo [107] DUPLOYAN LETTER H..DUPLOYAN LETTER VOCALIC M
    ('\u{1bc00}', '\u{1bc6a}', ALetter),
    // Lo  [13] DUPLOYAN AFFIX LEFT HORIZONTAL SECANT..DUPLOYAN AFFIX ATTACHED TANGENT HOOK
    ('\u{1bc70}', '\u{1bc7c}', ALetter),
    // Lo   [9] DUPLOYAN AFFIX HIGH ACUTE..DUPLOYAN AFFIX HIGH VERTICAL
    ('\u{1bc80}', '\u{1bc88}', ALetter),
    // Lo  [10] DUPLOYAN AFFIX LOW ACUTE..DUPLOYAN AFFIX LOW ARROW
    ('\u{1bc90}', '\u{1bc99}', ALetter),
    // Mn   [2] DUPLOYAN THICK LETTER SELECTOR..DUPLOYAN DOUBLE MARK
    ('\u{1bc9d}', '\u{1bc9e}', Extend),
    // Cf   [4] SHORTHAND FORMAT LETTER OVERLAP..SHORTHAND FORMAT UP STEP
    ('\u{1bca0}', '\u{1bca3}', Format),
    // Mc   [2] MUSICAL SYMBOL COMBINING STEM..MUSICAL SYMBOL COMBINING SPRECHGESANG STEM
    ('\u{1d165}', '\u{1d166}', Extend),
    // Mn   [3] MUSICAL SYMBOL COMBINING TREMOLO-1..MUSICAL SYMBOL COMBINING TREMOLO-3
    ('\u{1d167}', '\u{1d169}', Extend),
    // Mc   [6] MUSICAL SYMBOL COMBINING AUGMENTATION DOT..MUSICAL SYMBOL COMBINING FLAG-5
    ('\u{1d16d}', '\u{1d172}', Extend),
    // Cf   [8] MUSICAL SYMBOL BEGIN BEAM..MUSICAL SYMBOL END PHRASE
    ('\u{1d173}', '\u{1d17a}', Format),
    // Mn   [8] MUSICAL SYMBOL COMBINING ACCENT..MUSICAL SYMBOL COMBINING LOURE
    ('\u{1d17b}', '\u{1d182}', Extend),
    // Mn   [7] MUSICAL SYMBOL COMBINING DOIT..MUSICAL SYMBOL COMBINING TRIPLE TONGUE
    ('\u{1d185}', '\u{1d18b}', Extend),
    // Mn   [4] MUSICAL SYMBOL COMBINING DOWN BOW..MUSICAL SYMBOL COMBINING SNAP PIZZICATO
    ('\u{1d1aa}', '\u{1d1ad}', Extend),
    // Mn   [3] COMBINING GREEK MUSICAL TRISEME..COMBINING GREEK MUSICAL PENTASEME
    ('\u{1d242}', '\u{1d244}', Extend),
    // L&  [85] MATHEMATICAL BOLD CAPITAL A..MATHEMATICAL ITALIC SMALL G
    ('\u{1d400}', '\u{1d454}', ALetter),
    // L&  [71] MATHEMATICAL ITALIC SMALL I..MATHEMATICAL SCRIPT CAPITAL A
    ('\u{1d456}', '\u{1d49c}', ALetter),
    // L&   [2] MATHEMATICAL SCRIPT CAPITAL C..MATHEMATICAL SCRIPT CAPITAL D
    ('\u{1d49e}', '\u{1d49f}', ALetter),
    // L&       MATHEMATICAL SCRIPT CAPITAL G
    ('\u{1d4a2}', '\u{1d4a2}', ALetter),
    // L&   [2] MATHEMATICAL SCRIPT CAPITAL J..MATHEMATICAL SCRIPT CAPITAL K
    ('\u{1d4a5}', '\u{1d4a6}', ALetter),
    // L&   [4] MATHEMATICAL SCRIPT CAPITAL N..MATHEMATICAL SCRIPT CAPITAL Q
    ('\u{1d4a9}', '\u{1d4ac}', ALetter),
    // L&  [12] MATHEMATICAL SCRIPT CAPITAL S..MATHEMATICAL SCRIPT SMALL D
    ('\u{1d4ae}', '\u{1d4b9}', ALetter),
    // L&       MATHEMATICAL SCRIPT SMALL F
    ('\u{1d4bb}', '\u{1d4bb}', ALetter),
    // L&   [7] MATHEMATICAL SCRIPT SMALL H..MATHEMATICAL SCRIPT SMALL N
    ('\u{1d4bd}', '\u{1d4c3}', ALetter),
    // L&  [65] MATHEMATICAL SCRIPT SMALL P..MATHEMATICAL FRAKTUR CAPITAL B
    ('\u{1d4c5}', '\u{1d505}', ALetter),
    // L&   [4] MATHEMATICAL FRAKTUR CAPITAL D..MATHEMATICAL FRAKTUR CAPITAL G
    ('\u{1d507}', '\u{1d50a}', ALetter),
    // L&   [8] MATHEMATICAL FRAKTUR CAPITAL J..MATHEMATICAL FRAKTUR CAPITAL Q
    ('\u{1d50d}', '\u{1d514}', ALetter),
    // L&   [7] MATHEMATICAL FRAKTUR CAPITAL S..MATHEMATICAL FRAKTUR CAPITAL Y
    ('\u{1d516}', '\u{1d51c}', ALetter),
    // L&  [28] MATHEMATICAL FRAKTUR SMALL A..MATHEMATICAL DOUBLE-STRUCK CAPITAL B
    ('\u{1d51e}', '\u{1d539}', ALetter),
    // L&   [4] MATHEMATICAL DOUBLE-STRUCK CAPITAL D..MATHEMATICAL DOUBLE-STRUCK CAPITAL G
    ('\u{1d53b}', '\u{1d53e}', ALetter),
    // L&   [5] MATHEMATICAL DOUBLE-STRUCK CAPITAL I..MATHEMATICAL DOUBLE-STRUCK CAPITAL M
    ('\u{1d540}', '\u{1d544}', ALetter),
    // L&       MATHEMATICAL DOUBLE-STRUCK CAPITAL O
    ('\u{1d546}', '\u{1d546}', ALetter),
    // L&   [7] MATHEMATICAL DOUBLE-STRUCK CAPITAL S..MATHEMATICAL DOUBLE-STRUCK CAPITAL Y
    ('\u{1d54a}', '\u{1d550}', ALetter),
    // L& [340] MATHEMATICAL DOUBLE-STRUCK SMALL A..MATHEMATICAL ITALIC SMALL DOTLESS J
    ('\u{1d552}', '\u{1d6a5}', ALetter),
    // L&  [25] MATHEMATICAL BOLD CAPITAL ALPHA..MATHEMATICAL BOLD CAPITAL OMEGA
    ('\u{1d6a8}', '\u{1d6c0}', ALetter),
    // L&  [25] MATHEMATICAL BOLD SMALL ALPHA..MATHEMATICAL BOLD SMALL OMEGA
    ('\u{1d6c2}', '\u{1d6da}', ALetter),
    // L&  [31] MATHEMATICAL BOLD EPSILON SYMBOL..MATHEMATICAL ITALIC CAPITAL OMEGA
    ('\u{1d6dc}', '\u{1d6fa}', ALetter),
    // L&  [25] MATHEMATICAL ITALIC SMALL ALPHA..MATHEMATICAL ITALIC SMALL OMEGA
    ('\u{1d6fc}', '\u{1d714}', ALetter),
    // L&  [31] MATHEMATICAL ITALIC EPSILON SYMBOL..MATHEMATICAL BOLD ITALIC CAPITAL OMEGA
    ('\u{1d716}', '\u{1d734}', ALetter),
    // L&  [25] MATHEMATICAL BOLD ITALIC SMALL ALPHA..MATHEMATICAL BOLD ITALIC SMALL OMEGA
    ('\u{1d736}', '\u{1d74e}', ALetter),
    // L&  [31] MATHEMATICAL BOLD ITALIC EPSILON SYMBOL..MATHEMATICAL SANS-SERIF BOLD CAPITAL OMEGA
    ('\u{1d750}', '\u{1d76e}', ALetter),
    // L&  [25] MATHEMATICAL SANS-SERIF BOLD SMALL ALPHA..MATHEMATICAL SANS-SERIF BOLD SMALL OMEGA
    ('\u{1d770}', '\u{1d788}', ALetter),
    // L&  [31] MATHEMATICAL SANS-SERIF BOLD EPSILON SYMBOL..MATHEMATICAL SANS-SERIF BOLD ITALIC CAPITAL OMEGA
    ('\u{1d78a}', '\u{1d7a8}', ALetter),
    // L&  [25] MATHEMATICAL SANS-SERIF BOLD ITALIC SMALL ALPHA..MATHEMATICAL SANS-SERIF BOLD ITALIC SMALL OMEGA
    ('\u{1d7aa}', '\u{1d7c2}', ALetter),
    // L&   [8] MATHEMATICAL SANS-SERIF BOLD ITALIC EPSILON SYMBOL..MATHEMATICAL BOLD SMALL DIGAMMA
    ('\u{1d7c4}', '\u{1d7cb}', ALetter),
    // Nd  [50] MATHEMATICAL BOLD DIGIT ZERO..MATHEMATICAL MONOSPACE DIGIT NINE
    ('\u{1d7ce}', '\u{1d7ff}', Numeric),
    // Mn  [55] SIGNWRITING HEAD RIM..SIGNWRITING AIR SUCKING IN
    ('\u{1da00}', '\u{1da36}', Extend),
    // Mn  [50] SIGNWRITING MOUTH CLOSED NEUTRAL..SIGNWRITING EXCITEMENT
    ('\u{1da3b}', '\u{1da6c}', Extend),
    // Mn       SIGNWRITING UPPER BODY TILTING FROM HIP JOINTS
    ('\u{1da75}', '\u{1da75}', Extend),
    // Mn       SIGNWRITING LOCATION HEAD NECK
    ('\u{1da84}', '\u{1da84}', Extend),
    // Mn   [5] SIGNWRITING FILL MODIFIER-2..SIGNWRITING FILL MODIFIER-6
    ('\u{1da9b}', '\u{1da9f}', Extend),
    // Mn  [15] SIGNWRITING ROTATION MODIFIER-2..SIGNWRITING ROTATION MODIFIER-16
    ('\u{1daa1}', '\u{1daaf}', Extend),
    // Mn   [7] COMBINING GLAGOLITIC LETTER AZU..COMBINING GLAGOLITIC LETTER ZHIVETE
    ('\u{1e000}', '\u{1e006}', Extend),
    // Mn  [17] COMBINING GLAGOLITIC LETTER ZEMLJA..COMBINING GLAGOLITIC LETTER HERU
    ('\u{1e008}', '\u{1e018}', Extend),
    // Mn   [7] COMBINING GLAGOLITIC LETTER SHTA..COMBINING GLAGOLITIC LETTER YATI
    ('\u{1e01b}', '\u{1e021}', Extend),
    // Mn   [2] COMBINING GLAGOLITIC LETTER YU..COMBINING GLAGOLITIC LETTER SMALL YUS
    ('\u{1e023}', '\u{1e024}', Extend),
    // Mn   [5] COMBINING GLAGOLITIC LETTER YO..COMBINING GLAGOLITIC LETTER FITA
    ('\u{1e026}', '\u{1e02a}', Extend),
    // Lo  [45] NYIAKENG PUACHUE HMONG LETTER MA..NYIAKENG PUACHUE HMONG LETTER W
    ('\u{1e100}', '\u{1e12c}', ALetter),
    // Mn   [7] NYIAKENG PUACHUE HMONG TONE-B..NYIAKENG PUACHUE HMONG TONE-D
    ('\u{1e130}', '\u{1e136}', Extend),
    // Lm   [7] NYIAKENG PUACHUE HMONG SIGN FOR PERSON..NYIAKENG PUACHUE HMONG SYLLABLE LENGTHENER
    ('\u{1e137}', '\u{1e13d}', ALetter),
    // Nd  [10] NYIAKENG PUACHUE HMONG DIGIT ZERO..NYIAKENG PUACHUE HMONG DIGIT NINE
    ('\u{1e140}', '\u{1e149}', Numeric),
    // Lo       NYIAKENG PUACHUE HMONG LOGOGRAM NYAJ
    ('\u{1e14e}', '\u{1e14e}', ALetter),
    // Lo  [44] WANCHO LETTER AA..WANCHO LETTER YIH
    ('\u{1e2c0}', '\u{1e2eb}', ALetter),
    // Mn   [4] WANCHO TONE TUP..WANCHO TONE KOINI
    ('\u{1e2ec}', '\u{1e2ef}', Extend),
    // Nd  [10] WANCHO DIGIT ZERO..WANCHO DIGIT NINE
    ('\u{1e2f0}', '\u{1e2f9}', Numeric),
    // Lo [197] MENDE KIKAKUI SYLLABLE M001 KI..MENDE KIKAKUI SYLLABLE M060 NYON
    ('\u{1e800}', '\u{1e8c4}', ALetter),
    // Mn   [7] MENDE KIKAKUI COMBINING NUMBER TEENS..MENDE KIKAKUI COMBINING NUMBER MILLIONS
    ('\u{1e8d0}', '\u{1e8d6}', Extend),
    // L&  [68] ADLAM CAPITAL LETTER ALIF..ADLAM SMALL LETTER SHA
    ('\u{1e900}', '\u{1e943}', ALetter),
    // Mn   [7] ADLAM ALIF LENGTHENER..ADLAM NUKTA
    ('\u{1e944}', '\u{1e94a}', Extend),
    // Lm       ADLAM NASALIZATION MARK
    ('\u{1e94b}', '\u{1e94b}', ALetter),
    // Nd  [10] ADLAM DIGIT ZERO..ADLAM DIGIT NINE
    ('\u{1e950}', '\u{1e959}', Numeric),
    // Lo   [4] ARABIC MATHEMATICAL ALEF..ARABIC MATHEMATICAL DAL
    ('\u{1ee00}', '\u{1ee03}', ALetter),
    // Lo  [27] ARABIC MATHEMATICAL WAW..ARABIC MATHEMATICAL DOTLESS QAF
    ('\u{1ee05}', '\u{1ee1f}', ALetter),
    // Lo   [2] ARABIC MATHEMATICAL INITIAL BEH..ARABIC MATHEMATICAL INITIAL JEEM
    ('\u{1ee21}', '\u{1ee22}', ALetter),
    // Lo       ARABIC MATHEMATICAL INITIAL HEH
    ('\u{1ee24}', '\u{1ee24}', ALetter),
    // Lo       ARABIC MATHEMATICAL INITIAL HAH
    ('\u{1ee27}', '\u{1ee27}', ALetter),
    // Lo  [10] ARABIC MATHEMATICAL INITIAL YEH..ARABIC MATHEMATICAL INITIAL QAF
    ('\u{1ee29}', '\u{1ee32}', ALetter),
    // Lo   [4] ARABIC MATHEMATICAL INITIAL SHEEN..ARABIC MATHEMATICAL INITIAL KHAH
    ('\u{1ee34}', '\u{1ee37}', ALetter),
    // Lo       ARABIC MATHEMATICAL INITIAL DAD
    ('\u{1ee39}', '\u{1ee39}', ALetter),
    // Lo       ARABIC MATHEMATICAL INITIAL GHAIN
    ('\u{1ee3b}', '\u{1ee3b}', ALetter),
    // Lo       ARABIC MATHEMATICAL TAILED JEEM
    ('\u{1ee42}', '\u{1ee42}', ALetter),
    // Lo       ARABIC MATHEMATICAL TAILED HAH
    ('\u{1ee47}', '\u{1ee47}', ALetter),
    // Lo       ARABIC MATHEMATICAL TAILED YEH
    ('\u{1ee49}', '\u{1ee49}', ALetter),
    // Lo       ARABIC MATHEMATICAL TAILED LAM
    ('\u{1ee4b}', '\u{1ee4b}', ALetter),
    // Lo   [3] ARABIC MATHEMATICAL TAILED NOON..ARABIC MATHEMATICAL TAILED AIN
    ('\u{1ee4d}', '\u{1ee4f}', ALetter),
    // Lo   [2] ARABIC MATHEMATICAL TAILED SAD..ARABIC MATHEMATICAL TAILED QAF
    ('\u{1ee51}', '\u{1ee52}', ALetter),
    // Lo       ARABIC MATHEMATICAL TAILED SHEEN
    ('\u{1ee54}', '\u{1ee54}', ALetter),
    // Lo       ARABIC MATHEMATICAL TAILED KHAH
    ('\u{1ee57}', '\u{1ee57}', ALetter),
    // Lo       ARABIC MATHEMATICAL TAILED DAD
    ('\u{1ee59}', '\u{1ee59}', ALetter),
    // Lo       ARABIC MATHEMATICAL TAILED GHAIN
    ('\u{1ee5b}', '\u{1ee5b}', ALetter),
    // Lo       ARABIC MATHEMATICAL TAILED DOTLESS NOON
    ('\u{1ee5d}', '\u{1ee5d}', ALetter),
    // Lo       ARABIC MATHEMATICAL TAILED DOTLESS QAF
    ('\u{1ee5f}', '\u{1ee5f}', ALetter),
    // Lo   [2] ARABIC MATHEMATICAL STRETCHED BEH..ARABIC MATHEMATICAL STRETCHED JEEM
    ('\u{1ee61}', '\u{1ee62}', ALetter),
    // Lo       ARABIC MATHEMATICAL STRETCHED HEH
    ('\u{1ee64}', '\u{1ee64}', ALetter),
    // Lo   [4] ARABIC MATHEMATICAL STRETCHED HAH..ARABIC MATHEMATICAL STRETCHED KAF
    ('\u{1ee67}', '\u{1ee6a}', ALetter),
    // Lo   [7] ARABIC MATHEMATICAL STRETCHED MEEM..ARABIC MATHEMATICAL STRETCHED QAF
    ('\u{1ee6c}', '\u{1ee72}', ALetter),
    // Lo   [4] ARABIC MATHEMATICAL STRETCHED SHEEN..ARABIC MATHEMATICAL STRETCHED KHAH
    ('\u{1ee74}', '\u{1ee77}', ALetter),
    // Lo   [4] ARABIC MATHEMATICAL STRETCHED DAD..ARABIC MATHEMATICAL STRETCHED DOTLESS BEH
    ('\u{1ee79}', '\u{1ee7c}', ALetter),
    // Lo       ARABIC MATHEMATICAL STRETCHED DOTLESS FEH
    ('\u{1ee7e}', '\u{1ee7e}', ALetter),
    // Lo  [10] ARABIC MATHEMATICAL LOOPED ALEF..ARABIC MATHEMATICAL LOOPED YEH
    ('\u{1ee80}', '\u{1ee89}', ALetter),
    // Lo  [17] ARABIC MATHEMATICAL LOOPED LAM..ARABIC MATHEMATICAL LOOPED GHAIN
    ('\u{1ee8b}', '\u{1ee9b}', ALetter),
    // Lo   [3] ARABIC MATHEMATICAL DOUBLE-STRUCK BEH..ARABIC MATHEMATICAL DOUBLE-STRUCK DAL
    ('\u{1eea1}', '\u{1eea3}', ALetter),
    // Lo   [5] ARABIC MATHEMATICAL DOUBLE-STRUCK WAW..ARABIC MATHEMATICAL DOUBLE-STRUCK YEH
    ('\u{1eea5}', '\u{1eea9}', ALetter),
    // Lo  [17] ARABIC MATHEMATICAL DOUBLE-STRUCK LAM..ARABIC MATHEMATICAL DOUBLE-STRUCK GHAIN
    ('\u{1eeab}', '\u{1eebb}', ALetter),
    // So  [26] SQUARED LATIN CAPITAL LETTER A..SQUARED LATIN CAPITAL LETTER Z
    ('\u{1f130}', '\u{1f149}', ALetter),
    // So  [26] NEGATIVE CIRCLED LATIN CAPITAL LETTER A..NEGATIVE CIRCLED LATIN CAPITAL LETTER Z
    ('\u{1f150}', '\u{1f169}', ALetter),
    // So  [26] NEGATIVE SQUARED LATIN CAPITAL LETTER A..NEGATIVE SQUARED LATIN CAPITAL LETTER Z
    ('\u{1f170}', '\u{1f189}', ALetter),
    // So  [26] REGIONAL INDICATOR SYMBOL LETTER A..REGIONAL INDICATOR SYMBOL LETTER Z
    ('\u{1f1e6}', '\u{1f1ff}', Regional_Indicator),
    // Sk   [5] EMOJI MODIFIER FITZPATRICK TYPE-1-2..EMOJI MODIFIER FITZPATRICK TYPE-6
    ('\u{1f3fb}', '\u{1f3ff}', Extend),
    // Cf       LANGUAGE TAG
    ('\u{e0001}', '\u{e0001}', Format),
    // Cf  [96] TAG SPACE..CANCEL TAG
    ('\u{e0020}', '\u{e007f}', Extend),
    // Mn [240] VARIATION SELECTOR-17..VARIATION SELECTOR-256
    ('\u{e0100}', '\u{e01ef}', Extend),
];
