use crate::property_enums::Word_Break;

impl From<char> for Word_Break {
    fn from(c: char) -> Self {
        use Word_Break::*;
        if c.is_ascii() {
            return ASCII_TABLE[c as u32 as usize];
        }
        return crate::table_lookup(&START_TABLE, &(c as u32), &VALUE_TABLE);

        const ASCII_TABLE: [Word_Break; 128] = [
            Other,
            Other,
            Other,
            Other,
            Other,
            Other,
            Other,
            Other,
            Other,
            Other,
            LF, // Cc       <control-000A>
            Newline, // Cc   [2] <control-000B>..<control-000C>
            Newline, // Cc   [2] <control-000B>..<control-000C>
            CR, // Cc       <control-000D>
            Other,
            Other,
            Other,
            Other,
            Other,
            Other,
            Other,
            Other,
            Other,
            Other,
            Other,
            Other,
            Other,
            Other,
            Other,
            Other,
            Other,
            Other,
            WSegSpace, // Zs       SPACE
            Other,
            Double_Quote, // Po       QUOTATION MARK
            Other,
            Other,
            Other,
            Other,
            Single_Quote, // Po       APOSTROPHE
            Other,
            Other,
            Other,
            Other,
            MidNum, // Po       COMMA
            Other,
            MidNumLet, // Po       FULL STOP
            Other,
            Numeric, // Nd  [10] DIGIT ZERO..DIGIT NINE
            Numeric, // Nd  [10] DIGIT ZERO..DIGIT NINE
            Numeric, // Nd  [10] DIGIT ZERO..DIGIT NINE
            Numeric, // Nd  [10] DIGIT ZERO..DIGIT NINE
            Numeric, // Nd  [10] DIGIT ZERO..DIGIT NINE
            Numeric, // Nd  [10] DIGIT ZERO..DIGIT NINE
            Numeric, // Nd  [10] DIGIT ZERO..DIGIT NINE
            Numeric, // Nd  [10] DIGIT ZERO..DIGIT NINE
            Numeric, // Nd  [10] DIGIT ZERO..DIGIT NINE
            Numeric, // Nd  [10] DIGIT ZERO..DIGIT NINE
            MidLetter, // Po       COLON
            MidNum, // Po       SEMICOLON
            Other,
            Other,
            Other,
            Other,
            Other,
            ALetter, // L&  [26] LATIN CAPITAL LETTER A..LATIN CAPITAL LETTER Z
            ALetter, // L&  [26] LATIN CAPITAL LETTER A..LATIN CAPITAL LETTER Z
            ALetter, // L&  [26] LATIN CAPITAL LETTER A..LATIN CAPITAL LETTER Z
            ALetter, // L&  [26] LATIN CAPITAL LETTER A..LATIN CAPITAL LETTER Z
            ALetter, // L&  [26] LATIN CAPITAL LETTER A..LATIN CAPITAL LETTER Z
            ALetter, // L&  [26] LATIN CAPITAL LETTER A..LATIN CAPITAL LETTER Z
            ALetter, // L&  [26] LATIN CAPITAL LETTER A..LATIN CAPITAL LETTER Z
            ALetter, // L&  [26] LATIN CAPITAL LETTER A..LATIN CAPITAL LETTER Z
            ALetter, // L&  [26] LATIN CAPITAL LETTER A..LATIN CAPITAL LETTER Z
            ALetter, // L&  [26] LATIN CAPITAL LETTER A..LATIN CAPITAL LETTER Z
            ALetter, // L&  [26] LATIN CAPITAL LETTER A..LATIN CAPITAL LETTER Z
            ALetter, // L&  [26] LATIN CAPITAL LETTER A..LATIN CAPITAL LETTER Z
            ALetter, // L&  [26] LATIN CAPITAL LETTER A..LATIN CAPITAL LETTER Z
            ALetter, // L&  [26] LATIN CAPITAL LETTER A..LATIN CAPITAL LETTER Z
            ALetter, // L&  [26] LATIN CAPITAL LETTER A..LATIN CAPITAL LETTER Z
            ALetter, // L&  [26] LATIN CAPITAL LETTER A..LATIN CAPITAL LETTER Z
            ALetter, // L&  [26] LATIN CAPITAL LETTER A..LATIN CAPITAL LETTER Z
            ALetter, // L&  [26] LATIN CAPITAL LETTER A..LATIN CAPITAL LETTER Z
            ALetter, // L&  [26] LATIN CAPITAL LETTER A..LATIN CAPITAL LETTER Z
            ALetter, // L&  [26] LATIN CAPITAL LETTER A..LATIN CAPITAL LETTER Z
            ALetter, // L&  [26] LATIN CAPITAL LETTER A..LATIN CAPITAL LETTER Z
            ALetter, // L&  [26] LATIN CAPITAL LETTER A..LATIN CAPITAL LETTER Z
            ALetter, // L&  [26] LATIN CAPITAL LETTER A..LATIN CAPITAL LETTER Z
            ALetter, // L&  [26] LATIN CAPITAL LETTER A..LATIN CAPITAL LETTER Z
            ALetter, // L&  [26] LATIN CAPITAL LETTER A..LATIN CAPITAL LETTER Z
            ALetter, // L&  [26] LATIN CAPITAL LETTER A..LATIN CAPITAL LETTER Z
            Other,
            Other,
            Other,
            Other,
            ExtendNumLet, // Pc       LOW LINE
            Other,
            ALetter, // L&  [26] LATIN SMALL LETTER A..LATIN SMALL LETTER Z
            ALetter, // L&  [26] LATIN SMALL LETTER A..LATIN SMALL LETTER Z
            ALetter, // L&  [26] LATIN SMALL LETTER A..LATIN SMALL LETTER Z
            ALetter, // L&  [26] LATIN SMALL LETTER A..LATIN SMALL LETTER Z
            ALetter, // L&  [26] LATIN SMALL LETTER A..LATIN SMALL LETTER Z
            ALetter, // L&  [26] LATIN SMALL LETTER A..LATIN SMALL LETTER Z
            ALetter, // L&  [26] LATIN SMALL LETTER A..LATIN SMALL LETTER Z
            ALetter, // L&  [26] LATIN SMALL LETTER A..LATIN SMALL LETTER Z
            ALetter, // L&  [26] LATIN SMALL LETTER A..LATIN SMALL LETTER Z
            ALetter, // L&  [26] LATIN SMALL LETTER A..LATIN SMALL LETTER Z
            ALetter, // L&  [26] LATIN SMALL LETTER A..LATIN SMALL LETTER Z
            ALetter, // L&  [26] LATIN SMALL LETTER A..LATIN SMALL LETTER Z
            ALetter, // L&  [26] LATIN SMALL LETTER A..LATIN SMALL LETTER Z
            ALetter, // L&  [26] LATIN SMALL LETTER A..LATIN SMALL LETTER Z
            ALetter, // L&  [26] LATIN SMALL LETTER A..LATIN SMALL LETTER Z
            ALetter, // L&  [26] LATIN SMALL LETTER A..LATIN SMALL LETTER Z
            ALetter, // L&  [26] LATIN SMALL LETTER A..LATIN SMALL LETTER Z
            ALetter, // L&  [26] LATIN SMALL LETTER A..LATIN SMALL LETTER Z
            ALetter, // L&  [26] LATIN SMALL LETTER A..LATIN SMALL LETTER Z
            ALetter, // L&  [26] LATIN SMALL LETTER A..LATIN SMALL LETTER Z
            ALetter, // L&  [26] LATIN SMALL LETTER A..LATIN SMALL LETTER Z
            ALetter, // L&  [26] LATIN SMALL LETTER A..LATIN SMALL LETTER Z
            ALetter, // L&  [26] LATIN SMALL LETTER A..LATIN SMALL LETTER Z
            ALetter, // L&  [26] LATIN SMALL LETTER A..LATIN SMALL LETTER Z
            ALetter, // L&  [26] LATIN SMALL LETTER A..LATIN SMALL LETTER Z
            ALetter, // L&  [26] LATIN SMALL LETTER A..LATIN SMALL LETTER Z
            Other,
            Other,
            Other,
            Other,
            Other,
        ];
        const START_TABLE: [u32; 1962] = [
            0x0000,
            0x0085, // Cc       <control-0085>
            0x0086,
            0x00AA, // Lo       FEMININE ORDINAL INDICATOR
            0x00AB,
            0x00AD, // Cf       SOFT HYPHEN
            0x00AE,
            0x00B5, // L&       MICRO SIGN
            0x00B6,
            0x00B7, // Po       MIDDLE DOT
            0x00B8,
            0x00BA, // Lo       MASCULINE ORDINAL INDICATOR
            0x00BB,
            0x00C0, // L&  [23] LATIN CAPITAL LETTER A WITH GRAVE..LATIN CAPITAL LETTER O WITH DIAERESIS
            0x00D7,
            0x00D8, // L&  [31] LATIN CAPITAL LETTER O WITH STROKE..LATIN SMALL LETTER O WITH DIAERESIS
            0x00F7,
            0x00F8, // L& [195] LATIN SMALL LETTER O WITH STROKE..LATIN SMALL LETTER EZH WITH TAIL
            0x01BB, // Lo       LATIN LETTER TWO WITH STROKE
            0x01BC, // L&   [4] LATIN CAPITAL LETTER TONE FIVE..LATIN LETTER WYNN
            0x01C0, // Lo   [4] LATIN LETTER DENTAL CLICK..LATIN LETTER RETROFLEX CLICK
            0x01C4, // L& [208] LATIN CAPITAL LETTER DZ WITH CARON..LATIN SMALL LETTER EZH WITH CURL
            0x0294, // Lo       LATIN LETTER GLOTTAL STOP
            0x0295, // L&  [27] LATIN LETTER PHARYNGEAL VOICED FRICATIVE..LATIN SMALL LETTER TURNED H WITH FISHHOOK AND TAIL
            0x02B0, // Lm  [18] MODIFIER LETTER SMALL H..MODIFIER LETTER REVERSED GLOTTAL STOP
            0x02C2, // Sk   [4] MODIFIER LETTER LEFT ARROWHEAD..MODIFIER LETTER DOWN ARROWHEAD
            0x02C6, // Lm  [12] MODIFIER LETTER CIRCUMFLEX ACCENT..MODIFIER LETTER HALF TRIANGULAR COLON
            0x02D2, // Sk   [6] MODIFIER LETTER CENTRED RIGHT HALF RING..MODIFIER LETTER MINUS SIGN
            0x02D8,
            0x02DE, // Sk   [2] MODIFIER LETTER RHOTIC HOOK..MODIFIER LETTER CROSS ACCENT
            0x02E0, // Lm   [5] MODIFIER LETTER SMALL GAMMA..MODIFIER LETTER SMALL REVERSED GLOTTAL STOP
            0x02E5,
            0x02EC, // Lm       MODIFIER LETTER VOICING
            0x02ED, // Sk       MODIFIER LETTER UNASPIRATED
            0x02EE, // Lm       MODIFIER LETTER DOUBLE APOSTROPHE
            0x02EF, // Sk  [17] MODIFIER LETTER LOW DOWN ARROWHEAD..MODIFIER LETTER LOW LEFT ARROW
            0x0300, // Mn [112] COMBINING GRAVE ACCENT..COMBINING LATIN SMALL LETTER X
            0x0370, // L&   [4] GREEK CAPITAL LETTER HETA..GREEK SMALL LETTER ARCHAIC SAMPI
            0x0374, // Lm       GREEK NUMERAL SIGN
            0x0375,
            0x0376, // L&   [2] GREEK CAPITAL LETTER PAMPHYLIAN DIGAMMA..GREEK SMALL LETTER PAMPHYLIAN DIGAMMA
            0x0378,
            0x037A, // Lm       GREEK YPOGEGRAMMENI
            0x037B, // L&   [3] GREEK SMALL REVERSED LUNATE SIGMA SYMBOL..GREEK SMALL REVERSED DOTTED LUNATE SIGMA SYMBOL
            0x037E, // Po       GREEK QUESTION MARK
            0x037F, // L&       GREEK CAPITAL LETTER YOT
            0x0380,
            0x0386, // L&       GREEK CAPITAL LETTER ALPHA WITH TONOS
            0x0387, // Po       GREEK ANO TELEIA
            0x0388, // L&   [3] GREEK CAPITAL LETTER EPSILON WITH TONOS..GREEK CAPITAL LETTER IOTA WITH TONOS
            0x038B,
            0x038C, // L&       GREEK CAPITAL LETTER OMICRON WITH TONOS
            0x038D,
            0x038E, // L&  [20] GREEK CAPITAL LETTER UPSILON WITH TONOS..GREEK CAPITAL LETTER RHO
            0x03A2,
            0x03A3, // L&  [83] GREEK CAPITAL LETTER SIGMA..GREEK LUNATE EPSILON SYMBOL
            0x03F6,
            0x03F7, // L& [139] GREEK CAPITAL LETTER SHO..CYRILLIC SMALL LETTER KOPPA
            0x0482,
            0x0483, // Mn   [5] COMBINING CYRILLIC TITLO..COMBINING CYRILLIC POKRYTIE
            0x0488, // Me   [2] COMBINING CYRILLIC HUNDRED THOUSANDS SIGN..COMBINING CYRILLIC MILLIONS SIGN
            0x048A, // L& [166] CYRILLIC CAPITAL LETTER SHORT I WITH TAIL..CYRILLIC SMALL LETTER EL WITH DESCENDER
            0x0530,
            0x0531, // L&  [38] ARMENIAN CAPITAL LETTER AYB..ARMENIAN CAPITAL LETTER FEH
            0x0557,
            0x0559, // Lm       ARMENIAN MODIFIER LETTER LEFT HALF RING
            0x055A,
            0x055B, // Po   [2] ARMENIAN EMPHASIS MARK..ARMENIAN EXCLAMATION MARK
            0x055D,
            0x055E, // Po       ARMENIAN QUESTION MARK
            0x055F,
            0x0560, // L&  [41] ARMENIAN SMALL LETTER TURNED AYB..ARMENIAN SMALL LETTER YI WITH STROKE
            0x0589, // Po       ARMENIAN FULL STOP
            0x058A,
            0x0591, // Mn  [45] HEBREW ACCENT ETNAHTA..HEBREW POINT METEG
            0x05BE,
            0x05BF, // Mn       HEBREW POINT RAFE
            0x05C0,
            0x05C1, // Mn   [2] HEBREW POINT SHIN DOT..HEBREW POINT SIN DOT
            0x05C3,
            0x05C4, // Mn   [2] HEBREW MARK UPPER DOT..HEBREW MARK LOWER DOT
            0x05C6,
            0x05C7, // Mn       HEBREW POINT QAMATS QATAN
            0x05C8,
            0x05D0, // Lo  [27] HEBREW LETTER ALEF..HEBREW LETTER TAV
            0x05EB,
            0x05EF, // Lo   [4] HEBREW YOD TRIANGLE..HEBREW LIGATURE YIDDISH DOUBLE YOD
            0x05F3, // Po       HEBREW PUNCTUATION GERESH
            0x05F4, // Po       HEBREW PUNCTUATION GERSHAYIM
            0x05F5,
            0x0600, // Cf   [6] ARABIC NUMBER SIGN..ARABIC NUMBER MARK ABOVE
            0x0606,
            0x060C, // Po   [2] ARABIC COMMA..ARABIC DATE SEPARATOR
            0x060E,
            0x0610, // Mn  [11] ARABIC SIGN SALLALLAHOU ALAYHE WASSALLAM..ARABIC SMALL KASRA
            0x061B,
            0x061C, // Cf       ARABIC LETTER MARK
            0x061D,
            0x0620, // Lo  [32] ARABIC LETTER KASHMIRI YEH..ARABIC LETTER FARSI YEH WITH THREE DOTS ABOVE
            0x0640, // Lm       ARABIC TATWEEL
            0x0641, // Lo  [10] ARABIC LETTER FEH..ARABIC LETTER YEH
            0x064B, // Mn  [21] ARABIC FATHATAN..ARABIC WAVY HAMZA BELOW
            0x0660, // Nd  [10] ARABIC-INDIC DIGIT ZERO..ARABIC-INDIC DIGIT NINE
            0x066A,
            0x066B, // Po       ARABIC DECIMAL SEPARATOR
            0x066C, // Po       ARABIC THOUSANDS SEPARATOR
            0x066D,
            0x066E, // Lo   [2] ARABIC LETTER DOTLESS BEH..ARABIC LETTER DOTLESS QAF
            0x0670, // Mn       ARABIC LETTER SUPERSCRIPT ALEF
            0x0671, // Lo  [99] ARABIC LETTER ALEF WASLA..ARABIC LETTER YEH BARREE WITH HAMZA ABOVE
            0x06D4,
            0x06D5, // Lo       ARABIC LETTER AE
            0x06D6, // Mn   [7] ARABIC SMALL HIGH LIGATURE SAD WITH LAM WITH ALEF MAKSURA..ARABIC SMALL HIGH SEEN
            0x06DD, // Cf       ARABIC END OF AYAH
            0x06DE,
            0x06DF, // Mn   [6] ARABIC SMALL HIGH ROUNDED ZERO..ARABIC SMALL HIGH MADDA
            0x06E5, // Lm   [2] ARABIC SMALL WAW..ARABIC SMALL YEH
            0x06E7, // Mn   [2] ARABIC SMALL HIGH YEH..ARABIC SMALL HIGH NOON
            0x06E9,
            0x06EA, // Mn   [4] ARABIC EMPTY CENTRE LOW STOP..ARABIC SMALL LOW MEEM
            0x06EE, // Lo   [2] ARABIC LETTER DAL WITH INVERTED V..ARABIC LETTER REH WITH INVERTED V
            0x06F0, // Nd  [10] EXTENDED ARABIC-INDIC DIGIT ZERO..EXTENDED ARABIC-INDIC DIGIT NINE
            0x06FA, // Lo   [3] ARABIC LETTER SHEEN WITH DOT BELOW..ARABIC LETTER GHAIN WITH DOT BELOW
            0x06FD,
            0x06FF, // Lo       ARABIC LETTER HEH WITH INVERTED V
            0x0700,
            0x070F, // Cf       SYRIAC ABBREVIATION MARK
            0x0710, // Lo       SYRIAC LETTER ALAPH
            0x0711, // Mn       SYRIAC LETTER SUPERSCRIPT ALAPH
            0x0712, // Lo  [30] SYRIAC LETTER BETH..SYRIAC LETTER PERSIAN DHALATH
            0x0730, // Mn  [27] SYRIAC PTHAHA ABOVE..SYRIAC BARREKH
            0x074B,
            0x074D, // Lo  [89] SYRIAC LETTER SOGDIAN ZHAIN..THAANA LETTER WAAVU
            0x07A6, // Mn  [11] THAANA ABAFILI..THAANA SUKUN
            0x07B1, // Lo       THAANA LETTER NAA
            0x07B2,
            0x07C0, // Nd  [10] NKO DIGIT ZERO..NKO DIGIT NINE
            0x07CA, // Lo  [33] NKO LETTER A..NKO LETTER JONA RA
            0x07EB, // Mn   [9] NKO COMBINING SHORT HIGH TONE..NKO COMBINING DOUBLE DOT ABOVE
            0x07F4, // Lm   [2] NKO HIGH TONE APOSTROPHE..NKO LOW TONE APOSTROPHE
            0x07F6,
            0x07F8, // Po       NKO COMMA
            0x07F9,
            0x07FA, // Lm       NKO LAJANYALAN
            0x07FB,
            0x07FD, // Mn       NKO DANTAYALAN
            0x07FE,
            0x0800, // Lo  [22] SAMARITAN LETTER ALAF..SAMARITAN LETTER TAAF
            0x0816, // Mn   [4] SAMARITAN MARK IN..SAMARITAN MARK DAGESH
            0x081A, // Lm       SAMARITAN MODIFIER LETTER EPENTHETIC YUT
            0x081B, // Mn   [9] SAMARITAN MARK EPENTHETIC YUT..SAMARITAN VOWEL SIGN A
            0x0824, // Lm       SAMARITAN MODIFIER LETTER SHORT A
            0x0825, // Mn   [3] SAMARITAN VOWEL SIGN SHORT A..SAMARITAN VOWEL SIGN U
            0x0828, // Lm       SAMARITAN MODIFIER LETTER I
            0x0829, // Mn   [5] SAMARITAN VOWEL SIGN LONG I..SAMARITAN MARK NEQUDAA
            0x082E,
            0x0840, // Lo  [25] MANDAIC LETTER HALQA..MANDAIC LETTER AIN
            0x0859, // Mn   [3] MANDAIC AFFRICATION MARK..MANDAIC GEMINATION MARK
            0x085C,
            0x0860, // Lo  [11] SYRIAC LETTER MALAYALAM NGA..SYRIAC LETTER MALAYALAM SSA
            0x086B,
            0x08A0, // Lo  [21] ARABIC LETTER BEH WITH SMALL V BELOW..ARABIC LETTER KAF WITH DOT BELOW
            0x08B5,
            0x08B6, // Lo   [8] ARABIC LETTER BEH WITH SMALL MEEM ABOVE..ARABIC LETTER AFRICAN NOON
            0x08BE,
            0x08D3, // Mn  [15] ARABIC SMALL LOW WAW..ARABIC SMALL HIGH SIGN SAFHA
            0x08E2, // Cf       ARABIC DISPUTED END OF AYAH
            0x08E3, // Mn  [32] ARABIC TURNED DAMMA BELOW..DEVANAGARI SIGN ANUSVARA
            0x0903, // Mc       DEVANAGARI SIGN VISARGA
            0x0904, // Lo  [54] DEVANAGARI LETTER SHORT A..DEVANAGARI LETTER HA
            0x093A, // Mn       DEVANAGARI VOWEL SIGN OE
            0x093B, // Mc       DEVANAGARI VOWEL SIGN OOE
            0x093C, // Mn       DEVANAGARI SIGN NUKTA
            0x093D, // Lo       DEVANAGARI SIGN AVAGRAHA
            0x093E, // Mc   [3] DEVANAGARI VOWEL SIGN AA..DEVANAGARI VOWEL SIGN II
            0x0941, // Mn   [8] DEVANAGARI VOWEL SIGN U..DEVANAGARI VOWEL SIGN AI
            0x0949, // Mc   [4] DEVANAGARI VOWEL SIGN CANDRA O..DEVANAGARI VOWEL SIGN AU
            0x094D, // Mn       DEVANAGARI SIGN VIRAMA
            0x094E, // Mc   [2] DEVANAGARI VOWEL SIGN PRISHTHAMATRA E..DEVANAGARI VOWEL SIGN AW
            0x0950, // Lo       DEVANAGARI OM
            0x0951, // Mn   [7] DEVANAGARI STRESS SIGN UDATTA..DEVANAGARI VOWEL SIGN UUE
            0x0958, // Lo  [10] DEVANAGARI LETTER QA..DEVANAGARI LETTER VOCALIC LL
            0x0962, // Mn   [2] DEVANAGARI VOWEL SIGN VOCALIC L..DEVANAGARI VOWEL SIGN VOCALIC LL
            0x0964,
            0x0966, // Nd  [10] DEVANAGARI DIGIT ZERO..DEVANAGARI DIGIT NINE
            0x0970,
            0x0971, // Lm       DEVANAGARI SIGN HIGH SPACING DOT
            0x0972, // Lo  [15] DEVANAGARI LETTER CANDRA A..BENGALI ANJI
            0x0981, // Mn       BENGALI SIGN CANDRABINDU
            0x0982, // Mc   [2] BENGALI SIGN ANUSVARA..BENGALI SIGN VISARGA
            0x0984,
            0x0985, // Lo   [8] BENGALI LETTER A..BENGALI LETTER VOCALIC L
            0x098D,
            0x098F, // Lo   [2] BENGALI LETTER E..BENGALI LETTER AI
            0x0991,
            0x0993, // Lo  [22] BENGALI LETTER O..BENGALI LETTER NA
            0x09A9,
            0x09AA, // Lo   [7] BENGALI LETTER PA..BENGALI LETTER RA
            0x09B1,
            0x09B2, // Lo       BENGALI LETTER LA
            0x09B3,
            0x09B6, // Lo   [4] BENGALI LETTER SHA..BENGALI LETTER HA
            0x09BA,
            0x09BC, // Mn       BENGALI SIGN NUKTA
            0x09BD, // Lo       BENGALI SIGN AVAGRAHA
            0x09BE, // Mc   [3] BENGALI VOWEL SIGN AA..BENGALI VOWEL SIGN II
            0x09C1, // Mn   [4] BENGALI VOWEL SIGN U..BENGALI VOWEL SIGN VOCALIC RR
            0x09C5,
            0x09C7, // Mc   [2] BENGALI VOWEL SIGN E..BENGALI VOWEL SIGN AI
            0x09C9,
            0x09CB, // Mc   [2] BENGALI VOWEL SIGN O..BENGALI VOWEL SIGN AU
            0x09CD, // Mn       BENGALI SIGN VIRAMA
            0x09CE, // Lo       BENGALI LETTER KHANDA TA
            0x09CF,
            0x09D7, // Mc       BENGALI AU LENGTH MARK
            0x09D8,
            0x09DC, // Lo   [2] BENGALI LETTER RRA..BENGALI LETTER RHA
            0x09DE,
            0x09DF, // Lo   [3] BENGALI LETTER YYA..BENGALI LETTER VOCALIC LL
            0x09E2, // Mn   [2] BENGALI VOWEL SIGN VOCALIC L..BENGALI VOWEL SIGN VOCALIC LL
            0x09E4,
            0x09E6, // Nd  [10] BENGALI DIGIT ZERO..BENGALI DIGIT NINE
            0x09F0, // Lo   [2] BENGALI LETTER RA WITH MIDDLE DIAGONAL..BENGALI LETTER RA WITH LOWER DIAGONAL
            0x09F2,
            0x09FC, // Lo       BENGALI LETTER VEDIC ANUSVARA
            0x09FD,
            0x09FE, // Mn       BENGALI SANDHI MARK
            0x09FF,
            0x0A01, // Mn   [2] GURMUKHI SIGN ADAK BINDI..GURMUKHI SIGN BINDI
            0x0A03, // Mc       GURMUKHI SIGN VISARGA
            0x0A04,
            0x0A05, // Lo   [6] GURMUKHI LETTER A..GURMUKHI LETTER UU
            0x0A0B,
            0x0A0F, // Lo   [2] GURMUKHI LETTER EE..GURMUKHI LETTER AI
            0x0A11,
            0x0A13, // Lo  [22] GURMUKHI LETTER OO..GURMUKHI LETTER NA
            0x0A29,
            0x0A2A, // Lo   [7] GURMUKHI LETTER PA..GURMUKHI LETTER RA
            0x0A31,
            0x0A32, // Lo   [2] GURMUKHI LETTER LA..GURMUKHI LETTER LLA
            0x0A34,
            0x0A35, // Lo   [2] GURMUKHI LETTER VA..GURMUKHI LETTER SHA
            0x0A37,
            0x0A38, // Lo   [2] GURMUKHI LETTER SA..GURMUKHI LETTER HA
            0x0A3A,
            0x0A3C, // Mn       GURMUKHI SIGN NUKTA
            0x0A3D,
            0x0A3E, // Mc   [3] GURMUKHI VOWEL SIGN AA..GURMUKHI VOWEL SIGN II
            0x0A41, // Mn   [2] GURMUKHI VOWEL SIGN U..GURMUKHI VOWEL SIGN UU
            0x0A43,
            0x0A47, // Mn   [2] GURMUKHI VOWEL SIGN EE..GURMUKHI VOWEL SIGN AI
            0x0A49,
            0x0A4B, // Mn   [3] GURMUKHI VOWEL SIGN OO..GURMUKHI SIGN VIRAMA
            0x0A4E,
            0x0A51, // Mn       GURMUKHI SIGN UDAAT
            0x0A52,
            0x0A59, // Lo   [4] GURMUKHI LETTER KHHA..GURMUKHI LETTER RRA
            0x0A5D,
            0x0A5E, // Lo       GURMUKHI LETTER FA
            0x0A5F,
            0x0A66, // Nd  [10] GURMUKHI DIGIT ZERO..GURMUKHI DIGIT NINE
            0x0A70, // Mn   [2] GURMUKHI TIPPI..GURMUKHI ADDAK
            0x0A72, // Lo   [3] GURMUKHI IRI..GURMUKHI EK ONKAR
            0x0A75, // Mn       GURMUKHI SIGN YAKASH
            0x0A76,
            0x0A81, // Mn   [2] GUJARATI SIGN CANDRABINDU..GUJARATI SIGN ANUSVARA
            0x0A83, // Mc       GUJARATI SIGN VISARGA
            0x0A84,
            0x0A85, // Lo   [9] GUJARATI LETTER A..GUJARATI VOWEL CANDRA E
            0x0A8E,
            0x0A8F, // Lo   [3] GUJARATI LETTER E..GUJARATI VOWEL CANDRA O
            0x0A92,
            0x0A93, // Lo  [22] GUJARATI LETTER O..GUJARATI LETTER NA
            0x0AA9,
            0x0AAA, // Lo   [7] GUJARATI LETTER PA..GUJARATI LETTER RA
            0x0AB1,
            0x0AB2, // Lo   [2] GUJARATI LETTER LA..GUJARATI LETTER LLA
            0x0AB4,
            0x0AB5, // Lo   [5] GUJARATI LETTER VA..GUJARATI LETTER HA
            0x0ABA,
            0x0ABC, // Mn       GUJARATI SIGN NUKTA
            0x0ABD, // Lo       GUJARATI SIGN AVAGRAHA
            0x0ABE, // Mc   [3] GUJARATI VOWEL SIGN AA..GUJARATI VOWEL SIGN II
            0x0AC1, // Mn   [5] GUJARATI VOWEL SIGN U..GUJARATI VOWEL SIGN CANDRA E
            0x0AC6,
            0x0AC7, // Mn   [2] GUJARATI VOWEL SIGN E..GUJARATI VOWEL SIGN AI
            0x0AC9, // Mc       GUJARATI VOWEL SIGN CANDRA O
            0x0ACA,
            0x0ACB, // Mc   [2] GUJARATI VOWEL SIGN O..GUJARATI VOWEL SIGN AU
            0x0ACD, // Mn       GUJARATI SIGN VIRAMA
            0x0ACE,
            0x0AD0, // Lo       GUJARATI OM
            0x0AD1,
            0x0AE0, // Lo   [2] GUJARATI LETTER VOCALIC RR..GUJARATI LETTER VOCALIC LL
            0x0AE2, // Mn   [2] GUJARATI VOWEL SIGN VOCALIC L..GUJARATI VOWEL SIGN VOCALIC LL
            0x0AE4,
            0x0AE6, // Nd  [10] GUJARATI DIGIT ZERO..GUJARATI DIGIT NINE
            0x0AF0,
            0x0AF9, // Lo       GUJARATI LETTER ZHA
            0x0AFA, // Mn   [6] GUJARATI SIGN SUKUN..GUJARATI SIGN TWO-CIRCLE NUKTA ABOVE
            0x0B00,
            0x0B01, // Mn       ORIYA SIGN CANDRABINDU
            0x0B02, // Mc   [2] ORIYA SIGN ANUSVARA..ORIYA SIGN VISARGA
            0x0B04,
            0x0B05, // Lo   [8] ORIYA LETTER A..ORIYA LETTER VOCALIC L
            0x0B0D,
            0x0B0F, // Lo   [2] ORIYA LETTER E..ORIYA LETTER AI
            0x0B11,
            0x0B13, // Lo  [22] ORIYA LETTER O..ORIYA LETTER NA
            0x0B29,
            0x0B2A, // Lo   [7] ORIYA LETTER PA..ORIYA LETTER RA
            0x0B31,
            0x0B32, // Lo   [2] ORIYA LETTER LA..ORIYA LETTER LLA
            0x0B34,
            0x0B35, // Lo   [5] ORIYA LETTER VA..ORIYA LETTER HA
            0x0B3A,
            0x0B3C, // Mn       ORIYA SIGN NUKTA
            0x0B3D, // Lo       ORIYA SIGN AVAGRAHA
            0x0B3E, // Mc       ORIYA VOWEL SIGN AA
            0x0B3F, // Mn       ORIYA VOWEL SIGN I
            0x0B40, // Mc       ORIYA VOWEL SIGN II
            0x0B41, // Mn   [4] ORIYA VOWEL SIGN U..ORIYA VOWEL SIGN VOCALIC RR
            0x0B45,
            0x0B47, // Mc   [2] ORIYA VOWEL SIGN E..ORIYA VOWEL SIGN AI
            0x0B49,
            0x0B4B, // Mc   [2] ORIYA VOWEL SIGN O..ORIYA VOWEL SIGN AU
            0x0B4D, // Mn       ORIYA SIGN VIRAMA
            0x0B4E,
            0x0B56, // Mn       ORIYA AI LENGTH MARK
            0x0B57, // Mc       ORIYA AU LENGTH MARK
            0x0B58,
            0x0B5C, // Lo   [2] ORIYA LETTER RRA..ORIYA LETTER RHA
            0x0B5E,
            0x0B5F, // Lo   [3] ORIYA LETTER YYA..ORIYA LETTER VOCALIC LL
            0x0B62, // Mn   [2] ORIYA VOWEL SIGN VOCALIC L..ORIYA VOWEL SIGN VOCALIC LL
            0x0B64,
            0x0B66, // Nd  [10] ORIYA DIGIT ZERO..ORIYA DIGIT NINE
            0x0B70,
            0x0B71, // Lo       ORIYA LETTER WA
            0x0B72,
            0x0B82, // Mn       TAMIL SIGN ANUSVARA
            0x0B83, // Lo       TAMIL SIGN VISARGA
            0x0B84,
            0x0B85, // Lo   [6] TAMIL LETTER A..TAMIL LETTER UU
            0x0B8B,
            0x0B8E, // Lo   [3] TAMIL LETTER E..TAMIL LETTER AI
            0x0B91,
            0x0B92, // Lo   [4] TAMIL LETTER O..TAMIL LETTER KA
            0x0B96,
            0x0B99, // Lo   [2] TAMIL LETTER NGA..TAMIL LETTER CA
            0x0B9B,
            0x0B9C, // Lo       TAMIL LETTER JA
            0x0B9D,
            0x0B9E, // Lo   [2] TAMIL LETTER NYA..TAMIL LETTER TTA
            0x0BA0,
            0x0BA3, // Lo   [2] TAMIL LETTER NNA..TAMIL LETTER TA
            0x0BA5,
            0x0BA8, // Lo   [3] TAMIL LETTER NA..TAMIL LETTER PA
            0x0BAB,
            0x0BAE, // Lo  [12] TAMIL LETTER MA..TAMIL LETTER HA
            0x0BBA,
            0x0BBE, // Mc   [2] TAMIL VOWEL SIGN AA..TAMIL VOWEL SIGN I
            0x0BC0, // Mn       TAMIL VOWEL SIGN II
            0x0BC1, // Mc   [2] TAMIL VOWEL SIGN U..TAMIL VOWEL SIGN UU
            0x0BC3,
            0x0BC6, // Mc   [3] TAMIL VOWEL SIGN E..TAMIL VOWEL SIGN AI
            0x0BC9,
            0x0BCA, // Mc   [3] TAMIL VOWEL SIGN O..TAMIL VOWEL SIGN AU
            0x0BCD, // Mn       TAMIL SIGN VIRAMA
            0x0BCE,
            0x0BD0, // Lo       TAMIL OM
            0x0BD1,
            0x0BD7, // Mc       TAMIL AU LENGTH MARK
            0x0BD8,
            0x0BE6, // Nd  [10] TAMIL DIGIT ZERO..TAMIL DIGIT NINE
            0x0BF0,
            0x0C00, // Mn       TELUGU SIGN COMBINING CANDRABINDU ABOVE
            0x0C01, // Mc   [3] TELUGU SIGN CANDRABINDU..TELUGU SIGN VISARGA
            0x0C04, // Mn       TELUGU SIGN COMBINING ANUSVARA ABOVE
            0x0C05, // Lo   [8] TELUGU LETTER A..TELUGU LETTER VOCALIC L
            0x0C0D,
            0x0C0E, // Lo   [3] TELUGU LETTER E..TELUGU LETTER AI
            0x0C11,
            0x0C12, // Lo  [23] TELUGU LETTER O..TELUGU LETTER NA
            0x0C29,
            0x0C2A, // Lo  [16] TELUGU LETTER PA..TELUGU LETTER HA
            0x0C3A,
            0x0C3D, // Lo       TELUGU SIGN AVAGRAHA
            0x0C3E, // Mn   [3] TELUGU VOWEL SIGN AA..TELUGU VOWEL SIGN II
            0x0C41, // Mc   [4] TELUGU VOWEL SIGN U..TELUGU VOWEL SIGN VOCALIC RR
            0x0C45,
            0x0C46, // Mn   [3] TELUGU VOWEL SIGN E..TELUGU VOWEL SIGN AI
            0x0C49,
            0x0C4A, // Mn   [4] TELUGU VOWEL SIGN O..TELUGU SIGN VIRAMA
            0x0C4E,
            0x0C55, // Mn   [2] TELUGU LENGTH MARK..TELUGU AI LENGTH MARK
            0x0C57,
            0x0C58, // Lo   [3] TELUGU LETTER TSA..TELUGU LETTER RRRA
            0x0C5B,
            0x0C60, // Lo   [2] TELUGU LETTER VOCALIC RR..TELUGU LETTER VOCALIC LL
            0x0C62, // Mn   [2] TELUGU VOWEL SIGN VOCALIC L..TELUGU VOWEL SIGN VOCALIC LL
            0x0C64,
            0x0C66, // Nd  [10] TELUGU DIGIT ZERO..TELUGU DIGIT NINE
            0x0C70,
            0x0C80, // Lo       KANNADA SIGN SPACING CANDRABINDU
            0x0C81, // Mn       KANNADA SIGN CANDRABINDU
            0x0C82, // Mc   [2] KANNADA SIGN ANUSVARA..KANNADA SIGN VISARGA
            0x0C84,
            0x0C85, // Lo   [8] KANNADA LETTER A..KANNADA LETTER VOCALIC L
            0x0C8D,
            0x0C8E, // Lo   [3] KANNADA LETTER E..KANNADA LETTER AI
            0x0C91,
            0x0C92, // Lo  [23] KANNADA LETTER O..KANNADA LETTER NA
            0x0CA9,
            0x0CAA, // Lo  [10] KANNADA LETTER PA..KANNADA LETTER LLA
            0x0CB4,
            0x0CB5, // Lo   [5] KANNADA LETTER VA..KANNADA LETTER HA
            0x0CBA,
            0x0CBC, // Mn       KANNADA SIGN NUKTA
            0x0CBD, // Lo       KANNADA SIGN AVAGRAHA
            0x0CBE, // Mc       KANNADA VOWEL SIGN AA
            0x0CBF, // Mn       KANNADA VOWEL SIGN I
            0x0CC0, // Mc   [5] KANNADA VOWEL SIGN II..KANNADA VOWEL SIGN VOCALIC RR
            0x0CC5,
            0x0CC6, // Mn       KANNADA VOWEL SIGN E
            0x0CC7, // Mc   [2] KANNADA VOWEL SIGN EE..KANNADA VOWEL SIGN AI
            0x0CC9,
            0x0CCA, // Mc   [2] KANNADA VOWEL SIGN O..KANNADA VOWEL SIGN OO
            0x0CCC, // Mn   [2] KANNADA VOWEL SIGN AU..KANNADA SIGN VIRAMA
            0x0CCE,
            0x0CD5, // Mc   [2] KANNADA LENGTH MARK..KANNADA AI LENGTH MARK
            0x0CD7,
            0x0CDE, // Lo       KANNADA LETTER FA
            0x0CDF,
            0x0CE0, // Lo   [2] KANNADA LETTER VOCALIC RR..KANNADA LETTER VOCALIC LL
            0x0CE2, // Mn   [2] KANNADA VOWEL SIGN VOCALIC L..KANNADA VOWEL SIGN VOCALIC LL
            0x0CE4,
            0x0CE6, // Nd  [10] KANNADA DIGIT ZERO..KANNADA DIGIT NINE
            0x0CF0,
            0x0CF1, // Lo   [2] KANNADA SIGN JIHVAMULIYA..KANNADA SIGN UPADHMANIYA
            0x0CF3,
            0x0D00, // Mn   [2] MALAYALAM SIGN COMBINING ANUSVARA ABOVE..MALAYALAM SIGN CANDRABINDU
            0x0D02, // Mc   [2] MALAYALAM SIGN ANUSVARA..MALAYALAM SIGN VISARGA
            0x0D04,
            0x0D05, // Lo   [8] MALAYALAM LETTER A..MALAYALAM LETTER VOCALIC L
            0x0D0D,
            0x0D0E, // Lo   [3] MALAYALAM LETTER E..MALAYALAM LETTER AI
            0x0D11,
            0x0D12, // Lo  [41] MALAYALAM LETTER O..MALAYALAM LETTER TTTA
            0x0D3B, // Mn   [2] MALAYALAM SIGN VERTICAL BAR VIRAMA..MALAYALAM SIGN CIRCULAR VIRAMA
            0x0D3D, // Lo       MALAYALAM SIGN AVAGRAHA
            0x0D3E, // Mc   [3] MALAYALAM VOWEL SIGN AA..MALAYALAM VOWEL SIGN II
            0x0D41, // Mn   [4] MALAYALAM VOWEL SIGN U..MALAYALAM VOWEL SIGN VOCALIC RR
            0x0D45,
            0x0D46, // Mc   [3] MALAYALAM VOWEL SIGN E..MALAYALAM VOWEL SIGN AI
            0x0D49,
            0x0D4A, // Mc   [3] MALAYALAM VOWEL SIGN O..MALAYALAM VOWEL SIGN AU
            0x0D4D, // Mn       MALAYALAM SIGN VIRAMA
            0x0D4E, // Lo       MALAYALAM LETTER DOT REPH
            0x0D4F,
            0x0D54, // Lo   [3] MALAYALAM LETTER CHILLU M..MALAYALAM LETTER CHILLU LLL
            0x0D57, // Mc       MALAYALAM AU LENGTH MARK
            0x0D58,
            0x0D5F, // Lo   [3] MALAYALAM LETTER ARCHAIC II..MALAYALAM LETTER VOCALIC LL
            0x0D62, // Mn   [2] MALAYALAM VOWEL SIGN VOCALIC L..MALAYALAM VOWEL SIGN VOCALIC LL
            0x0D64,
            0x0D66, // Nd  [10] MALAYALAM DIGIT ZERO..MALAYALAM DIGIT NINE
            0x0D70,
            0x0D7A, // Lo   [6] MALAYALAM LETTER CHILLU NN..MALAYALAM LETTER CHILLU K
            0x0D80,
            0x0D82, // Mc   [2] SINHALA SIGN ANUSVARAYA..SINHALA SIGN VISARGAYA
            0x0D84,
            0x0D85, // Lo  [18] SINHALA LETTER AYANNA..SINHALA LETTER AUYANNA
            0x0D97,
            0x0D9A, // Lo  [24] SINHALA LETTER ALPAPRAANA KAYANNA..SINHALA LETTER DANTAJA NAYANNA
            0x0DB2,
            0x0DB3, // Lo   [9] SINHALA LETTER SANYAKA DAYANNA..SINHALA LETTER RAYANNA
            0x0DBC,
            0x0DBD, // Lo       SINHALA LETTER DANTAJA LAYANNA
            0x0DBE,
            0x0DC0, // Lo   [7] SINHALA LETTER VAYANNA..SINHALA LETTER FAYANNA
            0x0DC7,
            0x0DCA, // Mn       SINHALA SIGN AL-LAKUNA
            0x0DCB,
            0x0DCF, // Mc   [3] SINHALA VOWEL SIGN AELA-PILLA..SINHALA VOWEL SIGN DIGA AEDA-PILLA
            0x0DD2, // Mn   [3] SINHALA VOWEL SIGN KETTI IS-PILLA..SINHALA VOWEL SIGN KETTI PAA-PILLA
            0x0DD5,
            0x0DD6, // Mn       SINHALA VOWEL SIGN DIGA PAA-PILLA
            0x0DD7,
            0x0DD8, // Mc   [8] SINHALA VOWEL SIGN GAETTA-PILLA..SINHALA VOWEL SIGN GAYANUKITTA
            0x0DE0,
            0x0DE6, // Nd  [10] SINHALA LITH DIGIT ZERO..SINHALA LITH DIGIT NINE
            0x0DF0,
            0x0DF2, // Mc   [2] SINHALA VOWEL SIGN DIGA GAETTA-PILLA..SINHALA VOWEL SIGN DIGA GAYANUKITTA
            0x0DF4,
            0x0E31, // Mn       THAI CHARACTER MAI HAN-AKAT
            0x0E32,
            0x0E34, // Mn   [7] THAI CHARACTER SARA I..THAI CHARACTER PHINTHU
            0x0E3B,
            0x0E47, // Mn   [8] THAI CHARACTER MAITAIKHU..THAI CHARACTER YAMAKKAN
            0x0E4F,
            0x0E50, // Nd  [10] THAI DIGIT ZERO..THAI DIGIT NINE
            0x0E5A,
            0x0EB1, // Mn       LAO VOWEL SIGN MAI KAN
            0x0EB2,
            0x0EB4, // Mn   [9] LAO VOWEL SIGN I..LAO SEMIVOWEL SIGN LO
            0x0EBD,
            0x0EC8, // Mn   [6] LAO TONE MAI EK..LAO NIGGAHITA
            0x0ECE,
            0x0ED0, // Nd  [10] LAO DIGIT ZERO..LAO DIGIT NINE
            0x0EDA,
            0x0F00, // Lo       TIBETAN SYLLABLE OM
            0x0F01,
            0x0F18, // Mn   [2] TIBETAN ASTROLOGICAL SIGN -KHYUD PA..TIBETAN ASTROLOGICAL SIGN SDONG TSHUGS
            0x0F1A,
            0x0F20, // Nd  [10] TIBETAN DIGIT ZERO..TIBETAN DIGIT NINE
            0x0F2A,
            0x0F35, // Mn       TIBETAN MARK NGAS BZUNG NYI ZLA
            0x0F36,
            0x0F37, // Mn       TIBETAN MARK NGAS BZUNG SGOR RTAGS
            0x0F38,
            0x0F39, // Mn       TIBETAN MARK TSA -PHRU
            0x0F3A,
            0x0F3E, // Mc   [2] TIBETAN SIGN YAR TSHES..TIBETAN SIGN MAR TSHES
            0x0F40, // Lo   [8] TIBETAN LETTER KA..TIBETAN LETTER JA
            0x0F48,
            0x0F49, // Lo  [36] TIBETAN LETTER NYA..TIBETAN LETTER RRA
            0x0F6D,
            0x0F71, // Mn  [14] TIBETAN VOWEL SIGN AA..TIBETAN SIGN RJES SU NGA RO
            0x0F7F, // Mc       TIBETAN SIGN RNAM BCAD
            0x0F80, // Mn   [5] TIBETAN VOWEL SIGN REVERSED I..TIBETAN MARK HALANTA
            0x0F85,
            0x0F86, // Mn   [2] TIBETAN SIGN LCI RTAGS..TIBETAN SIGN YANG RTAGS
            0x0F88, // Lo   [5] TIBETAN SIGN LCE TSA CAN..TIBETAN SIGN INVERTED MCHU CAN
            0x0F8D, // Mn  [11] TIBETAN SUBJOINED SIGN LCE TSA CAN..TIBETAN SUBJOINED LETTER JA
            0x0F98,
            0x0F99, // Mn  [36] TIBETAN SUBJOINED LETTER NYA..TIBETAN SUBJOINED LETTER FIXED-FORM RA
            0x0FBD,
            0x0FC6, // Mn       TIBETAN SYMBOL PADMA GDAN
            0x0FC7,
            0x102B, // Mc   [2] MYANMAR VOWEL SIGN TALL AA..MYANMAR VOWEL SIGN AA
            0x102D, // Mn   [4] MYANMAR VOWEL SIGN I..MYANMAR VOWEL SIGN UU
            0x1031, // Mc       MYANMAR VOWEL SIGN E
            0x1032, // Mn   [6] MYANMAR VOWEL SIGN AI..MYANMAR SIGN DOT BELOW
            0x1038, // Mc       MYANMAR SIGN VISARGA
            0x1039, // Mn   [2] MYANMAR SIGN VIRAMA..MYANMAR SIGN ASAT
            0x103B, // Mc   [2] MYANMAR CONSONANT SIGN MEDIAL YA..MYANMAR CONSONANT SIGN MEDIAL RA
            0x103D, // Mn   [2] MYANMAR CONSONANT SIGN MEDIAL WA..MYANMAR CONSONANT SIGN MEDIAL HA
            0x103F,
            0x1040, // Nd  [10] MYANMAR DIGIT ZERO..MYANMAR DIGIT NINE
            0x104A,
            0x1056, // Mc   [2] MYANMAR VOWEL SIGN VOCALIC R..MYANMAR VOWEL SIGN VOCALIC RR
            0x1058, // Mn   [2] MYANMAR VOWEL SIGN VOCALIC L..MYANMAR VOWEL SIGN VOCALIC LL
            0x105A,
            0x105E, // Mn   [3] MYANMAR CONSONANT SIGN MON MEDIAL NA..MYANMAR CONSONANT SIGN MON MEDIAL LA
            0x1061,
            0x1062, // Mc   [3] MYANMAR VOWEL SIGN SGAW KAREN EU..MYANMAR TONE MARK SGAW KAREN KE PHO
            0x1065,
            0x1067, // Mc   [7] MYANMAR VOWEL SIGN WESTERN PWO KAREN EU..MYANMAR SIGN WESTERN PWO KAREN TONE-5
            0x106E,
            0x1071, // Mn   [4] MYANMAR VOWEL SIGN GEBA KAREN I..MYANMAR VOWEL SIGN KAYAH EE
            0x1075,
            0x1082, // Mn       MYANMAR CONSONANT SIGN SHAN MEDIAL WA
            0x1083, // Mc   [2] MYANMAR VOWEL SIGN SHAN AA..MYANMAR VOWEL SIGN SHAN E
            0x1085, // Mn   [2] MYANMAR VOWEL SIGN SHAN E ABOVE..MYANMAR VOWEL SIGN SHAN FINAL Y
            0x1087, // Mc   [6] MYANMAR SIGN SHAN TONE-2..MYANMAR SIGN SHAN COUNCIL TONE-3
            0x108D, // Mn       MYANMAR SIGN SHAN COUNCIL EMPHATIC TONE
            0x108E,
            0x108F, // Mc       MYANMAR SIGN RUMAI PALAUNG TONE-5
            0x1090, // Nd  [10] MYANMAR SHAN DIGIT ZERO..MYANMAR SHAN DIGIT NINE
            0x109A, // Mc   [3] MYANMAR SIGN KHAMTI TONE-1..MYANMAR VOWEL SIGN AITON A
            0x109D, // Mn       MYANMAR VOWEL SIGN AITON AI
            0x109E,
            0x10A0, // L&  [38] GEORGIAN CAPITAL LETTER AN..GEORGIAN CAPITAL LETTER HOE
            0x10C6,
            0x10C7, // L&       GEORGIAN CAPITAL LETTER YN
            0x10C8,
            0x10CD, // L&       GEORGIAN CAPITAL LETTER AEN
            0x10CE,
            0x10D0, // L&  [43] GEORGIAN LETTER AN..GEORGIAN LETTER AIN
            0x10FB,
            0x10FC, // Lm       MODIFIER LETTER GEORGIAN NAR
            0x10FD, // L&   [3] GEORGIAN LETTER AEN..GEORGIAN LETTER LABIAL SIGN
            0x1100, // Lo [329] HANGUL CHOSEONG KIYEOK..ETHIOPIC SYLLABLE QWA
            0x1249,
            0x124A, // Lo   [4] ETHIOPIC SYLLABLE QWI..ETHIOPIC SYLLABLE QWE
            0x124E,
            0x1250, // Lo   [7] ETHIOPIC SYLLABLE QHA..ETHIOPIC SYLLABLE QHO
            0x1257,
            0x1258, // Lo       ETHIOPIC SYLLABLE QHWA
            0x1259,
            0x125A, // Lo   [4] ETHIOPIC SYLLABLE QHWI..ETHIOPIC SYLLABLE QHWE
            0x125E,
            0x1260, // Lo  [41] ETHIOPIC SYLLABLE BA..ETHIOPIC SYLLABLE XWA
            0x1289,
            0x128A, // Lo   [4] ETHIOPIC SYLLABLE XWI..ETHIOPIC SYLLABLE XWE
            0x128E,
            0x1290, // Lo  [33] ETHIOPIC SYLLABLE NA..ETHIOPIC SYLLABLE KWA
            0x12B1,
            0x12B2, // Lo   [4] ETHIOPIC SYLLABLE KWI..ETHIOPIC SYLLABLE KWE
            0x12B6,
            0x12B8, // Lo   [7] ETHIOPIC SYLLABLE KXA..ETHIOPIC SYLLABLE KXO
            0x12BF,
            0x12C0, // Lo       ETHIOPIC SYLLABLE KXWA
            0x12C1,
            0x12C2, // Lo   [4] ETHIOPIC SYLLABLE KXWI..ETHIOPIC SYLLABLE KXWE
            0x12C6,
            0x12C8, // Lo  [15] ETHIOPIC SYLLABLE WA..ETHIOPIC SYLLABLE PHARYNGEAL O
            0x12D7,
            0x12D8, // Lo  [57] ETHIOPIC SYLLABLE ZA..ETHIOPIC SYLLABLE GWA
            0x1311,
            0x1312, // Lo   [4] ETHIOPIC SYLLABLE GWI..ETHIOPIC SYLLABLE GWE
            0x1316,
            0x1318, // Lo  [67] ETHIOPIC SYLLABLE GGA..ETHIOPIC SYLLABLE FYA
            0x135B,
            0x135D, // Mn   [3] ETHIOPIC COMBINING GEMINATION AND VOWEL LENGTH MARK..ETHIOPIC COMBINING GEMINATION MARK
            0x1360,
            0x1380, // Lo  [16] ETHIOPIC SYLLABLE SEBATBEIT MWA..ETHIOPIC SYLLABLE PWE
            0x1390,
            0x13A0, // L&  [86] CHEROKEE LETTER A..CHEROKEE LETTER MV
            0x13F6,
            0x13F8, // L&   [6] CHEROKEE SMALL LETTER YE..CHEROKEE SMALL LETTER MV
            0x13FE,
            0x1401, // Lo [620] CANADIAN SYLLABICS E..CANADIAN SYLLABICS CARRIER TTSA
            0x166D,
            0x166F, // Lo  [17] CANADIAN SYLLABICS QAI..CANADIAN SYLLABICS BLACKFOOT W
            0x1680, // Zs       OGHAM SPACE MARK
            0x1681, // Lo  [26] OGHAM LETTER BEITH..OGHAM LETTER PEITH
            0x169B,
            0x16A0, // Lo  [75] RUNIC LETTER FEHU FEOH FE F..RUNIC LETTER X
            0x16EB,
            0x16EE, // Nl   [3] RUNIC ARLAUG SYMBOL..RUNIC BELGTHOR SYMBOL
            0x16F1, // Lo   [8] RUNIC LETTER K..RUNIC LETTER FRANKS CASKET AESC
            0x16F9,
            0x1700, // Lo  [13] TAGALOG LETTER A..TAGALOG LETTER YA
            0x170D,
            0x170E, // Lo   [4] TAGALOG LETTER LA..TAGALOG LETTER HA
            0x1712, // Mn   [3] TAGALOG VOWEL SIGN I..TAGALOG SIGN VIRAMA
            0x1715,
            0x1720, // Lo  [18] HANUNOO LETTER A..HANUNOO LETTER HA
            0x1732, // Mn   [3] HANUNOO VOWEL SIGN I..HANUNOO SIGN PAMUDPOD
            0x1735,
            0x1740, // Lo  [18] BUHID LETTER A..BUHID LETTER HA
            0x1752, // Mn   [2] BUHID VOWEL SIGN I..BUHID VOWEL SIGN U
            0x1754,
            0x1760, // Lo  [13] TAGBANWA LETTER A..TAGBANWA LETTER YA
            0x176D,
            0x176E, // Lo   [3] TAGBANWA LETTER LA..TAGBANWA LETTER SA
            0x1771,
            0x1772, // Mn   [2] TAGBANWA VOWEL SIGN I..TAGBANWA VOWEL SIGN U
            0x1774,
            0x17B4, // Mn   [2] KHMER VOWEL INHERENT AQ..KHMER VOWEL INHERENT AA
            0x17B6, // Mc       KHMER VOWEL SIGN AA
            0x17B7, // Mn   [7] KHMER VOWEL SIGN I..KHMER VOWEL SIGN UA
            0x17BE, // Mc   [8] KHMER VOWEL SIGN OE..KHMER VOWEL SIGN AU
            0x17C6, // Mn       KHMER SIGN NIKAHIT
            0x17C7, // Mc   [2] KHMER SIGN REAHMUK..KHMER SIGN YUUKALEAPINTU
            0x17C9, // Mn  [11] KHMER SIGN MUUSIKATOAN..KHMER SIGN BATHAMASAT
            0x17D4,
            0x17DD, // Mn       KHMER SIGN ATTHACAN
            0x17DE,
            0x17E0, // Nd  [10] KHMER DIGIT ZERO..KHMER DIGIT NINE
            0x17EA,
            0x180B, // Mn   [3] MONGOLIAN FREE VARIATION SELECTOR ONE..MONGOLIAN FREE VARIATION SELECTOR THREE
            0x180E, // Cf       MONGOLIAN VOWEL SEPARATOR
            0x180F,
            0x1810, // Nd  [10] MONGOLIAN DIGIT ZERO..MONGOLIAN DIGIT NINE
            0x181A,
            0x1820, // Lo  [35] MONGOLIAN LETTER A..MONGOLIAN LETTER CHI
            0x1843, // Lm       MONGOLIAN LETTER TODO LONG VOWEL SIGN
            0x1844, // Lo  [53] MONGOLIAN LETTER TODO E..MONGOLIAN LETTER CHA WITH TWO DOTS
            0x1879,
            0x1880, // Lo   [5] MONGOLIAN LETTER ALI GALI ANUSVARA ONE..MONGOLIAN LETTER ALI GALI INVERTED UBADAMA
            0x1885, // Mn   [2] MONGOLIAN LETTER ALI GALI BALUDA..MONGOLIAN LETTER ALI GALI THREE BALUDA
            0x1887, // Lo  [34] MONGOLIAN LETTER ALI GALI A..MONGOLIAN LETTER MANCHU ALI GALI BHA
            0x18A9, // Mn       MONGOLIAN LETTER ALI GALI DAGALGA
            0x18AA, // Lo       MONGOLIAN LETTER MANCHU ALI GALI LHA
            0x18AB,
            0x18B0, // Lo  [70] CANADIAN SYLLABICS OY..CANADIAN SYLLABICS CARRIER DENTAL S
            0x18F6,
            0x1900, // Lo  [31] LIMBU VOWEL-CARRIER LETTER..LIMBU LETTER TRA
            0x191F,
            0x1920, // Mn   [3] LIMBU VOWEL SIGN A..LIMBU VOWEL SIGN U
            0x1923, // Mc   [4] LIMBU VOWEL SIGN EE..LIMBU VOWEL SIGN AU
            0x1927, // Mn   [2] LIMBU VOWEL SIGN E..LIMBU VOWEL SIGN O
            0x1929, // Mc   [3] LIMBU SUBJOINED LETTER YA..LIMBU SUBJOINED LETTER WA
            0x192C,
            0x1930, // Mc   [2] LIMBU SMALL LETTER KA..LIMBU SMALL LETTER NGA
            0x1932, // Mn       LIMBU SMALL LETTER ANUSVARA
            0x1933, // Mc   [6] LIMBU SMALL LETTER TA..LIMBU SMALL LETTER LA
            0x1939, // Mn   [3] LIMBU SIGN MUKPHRENG..LIMBU SIGN SA-I
            0x193C,
            0x1946, // Nd  [10] LIMBU DIGIT ZERO..LIMBU DIGIT NINE
            0x1950,
            0x19D0, // Nd  [10] NEW TAI LUE DIGIT ZERO..NEW TAI LUE DIGIT NINE
            0x19DA,
            0x1A00, // Lo  [23] BUGINESE LETTER KA..BUGINESE LETTER HA
            0x1A17, // Mn   [2] BUGINESE VOWEL SIGN I..BUGINESE VOWEL SIGN U
            0x1A19, // Mc   [2] BUGINESE VOWEL SIGN E..BUGINESE VOWEL SIGN O
            0x1A1B, // Mn       BUGINESE VOWEL SIGN AE
            0x1A1C,
            0x1A55, // Mc       TAI THAM CONSONANT SIGN MEDIAL RA
            0x1A56, // Mn       TAI THAM CONSONANT SIGN MEDIAL LA
            0x1A57, // Mc       TAI THAM CONSONANT SIGN LA TANG LAI
            0x1A58, // Mn   [7] TAI THAM SIGN MAI KANG LAI..TAI THAM CONSONANT SIGN SA
            0x1A5F,
            0x1A60, // Mn       TAI THAM SIGN SAKOT
            0x1A61, // Mc       TAI THAM VOWEL SIGN A
            0x1A62, // Mn       TAI THAM VOWEL SIGN MAI SAT
            0x1A63, // Mc   [2] TAI THAM VOWEL SIGN AA..TAI THAM VOWEL SIGN TALL AA
            0x1A65, // Mn   [8] TAI THAM VOWEL SIGN I..TAI THAM VOWEL SIGN OA BELOW
            0x1A6D, // Mc   [6] TAI THAM VOWEL SIGN OY..TAI THAM VOWEL SIGN THAM AI
            0x1A73, // Mn  [10] TAI THAM VOWEL SIGN OA ABOVE..TAI THAM SIGN KHUEN-LUE KARAN
            0x1A7D,
            0x1A7F, // Mn       TAI THAM COMBINING CRYPTOGRAMMIC DOT
            0x1A80, // Nd  [10] TAI THAM HORA DIGIT ZERO..TAI THAM HORA DIGIT NINE
            0x1A8A,
            0x1A90, // Nd  [10] TAI THAM THAM DIGIT ZERO..TAI THAM THAM DIGIT NINE
            0x1A9A,
            0x1AB0, // Mn  [14] COMBINING DOUBLED CIRCUMFLEX ACCENT..COMBINING PARENTHESES BELOW
            0x1ABE, // Me       COMBINING PARENTHESES OVERLAY
            0x1ABF,
            0x1B00, // Mn   [4] BALINESE SIGN ULU RICEM..BALINESE SIGN SURANG
            0x1B04, // Mc       BALINESE SIGN BISAH
            0x1B05, // Lo  [47] BALINESE LETTER AKARA..BALINESE LETTER HA
            0x1B34, // Mn       BALINESE SIGN REREKAN
            0x1B35, // Mc       BALINESE VOWEL SIGN TEDUNG
            0x1B36, // Mn   [5] BALINESE VOWEL SIGN ULU..BALINESE VOWEL SIGN RA REPA
            0x1B3B, // Mc       BALINESE VOWEL SIGN RA REPA TEDUNG
            0x1B3C, // Mn       BALINESE VOWEL SIGN LA LENGA
            0x1B3D, // Mc   [5] BALINESE VOWEL SIGN LA LENGA TEDUNG..BALINESE VOWEL SIGN TALING REPA TEDUNG
            0x1B42, // Mn       BALINESE VOWEL SIGN PEPET
            0x1B43, // Mc   [2] BALINESE VOWEL SIGN PEPET TEDUNG..BALINESE ADEG ADEG
            0x1B45, // Lo   [7] BALINESE LETTER KAF SASAK..BALINESE LETTER ASYURA SASAK
            0x1B4C,
            0x1B50, // Nd  [10] BALINESE DIGIT ZERO..BALINESE DIGIT NINE
            0x1B5A,
            0x1B6B, // Mn   [9] BALINESE MUSICAL SYMBOL COMBINING TEGEH..BALINESE MUSICAL SYMBOL COMBINING GONG
            0x1B74,
            0x1B80, // Mn   [2] SUNDANESE SIGN PANYECEK..SUNDANESE SIGN PANGLAYAR
            0x1B82, // Mc       SUNDANESE SIGN PANGWISAD
            0x1B83, // Lo  [30] SUNDANESE LETTER A..SUNDANESE LETTER HA
            0x1BA1, // Mc       SUNDANESE CONSONANT SIGN PAMINGKAL
            0x1BA2, // Mn   [4] SUNDANESE CONSONANT SIGN PANYAKRA..SUNDANESE VOWEL SIGN PANYUKU
            0x1BA6, // Mc   [2] SUNDANESE VOWEL SIGN PANAELAENG..SUNDANESE VOWEL SIGN PANOLONG
            0x1BA8, // Mn   [2] SUNDANESE VOWEL SIGN PAMEPET..SUNDANESE VOWEL SIGN PANEULEUNG
            0x1BAA, // Mc       SUNDANESE SIGN PAMAAEH
            0x1BAB, // Mn   [3] SUNDANESE SIGN VIRAMA..SUNDANESE CONSONANT SIGN PASANGAN WA
            0x1BAE, // Lo   [2] SUNDANESE LETTER KHA..SUNDANESE LETTER SYA
            0x1BB0, // Nd  [10] SUNDANESE DIGIT ZERO..SUNDANESE DIGIT NINE
            0x1BBA, // Lo  [44] SUNDANESE AVAGRAHA..BATAK LETTER U
            0x1BE6, // Mn       BATAK SIGN TOMPI
            0x1BE7, // Mc       BATAK VOWEL SIGN E
            0x1BE8, // Mn   [2] BATAK VOWEL SIGN PAKPAK E..BATAK VOWEL SIGN EE
            0x1BEA, // Mc   [3] BATAK VOWEL SIGN I..BATAK VOWEL SIGN O
            0x1BED, // Mn       BATAK VOWEL SIGN KARO O
            0x1BEE, // Mc       BATAK VOWEL SIGN U
            0x1BEF, // Mn   [3] BATAK VOWEL SIGN U FOR SIMALUNGUN SA..BATAK CONSONANT SIGN H
            0x1BF2, // Mc   [2] BATAK PANGOLAT..BATAK PANONGONAN
            0x1BF4,
            0x1C00, // Lo  [36] LEPCHA LETTER KA..LEPCHA LETTER A
            0x1C24, // Mc   [8] LEPCHA SUBJOINED LETTER YA..LEPCHA VOWEL SIGN UU
            0x1C2C, // Mn   [8] LEPCHA VOWEL SIGN E..LEPCHA CONSONANT SIGN T
            0x1C34, // Mc   [2] LEPCHA CONSONANT SIGN NYIN-DO..LEPCHA CONSONANT SIGN KANG
            0x1C36, // Mn   [2] LEPCHA SIGN RAN..LEPCHA SIGN NUKTA
            0x1C38,
            0x1C40, // Nd  [10] LEPCHA DIGIT ZERO..LEPCHA DIGIT NINE
            0x1C4A,
            0x1C4D, // Lo   [3] LEPCHA LETTER TTA..LEPCHA LETTER DDA
            0x1C50, // Nd  [10] OL CHIKI DIGIT ZERO..OL CHIKI DIGIT NINE
            0x1C5A, // Lo  [30] OL CHIKI LETTER LA..OL CHIKI LETTER OH
            0x1C78, // Lm   [6] OL CHIKI MU TTUDDAG..OL CHIKI AHAD
            0x1C7E,
            0x1C80, // L&   [9] CYRILLIC SMALL LETTER ROUNDED VE..CYRILLIC SMALL LETTER UNBLENDED UK
            0x1C89,
            0x1C90, // L&  [43] GEORGIAN MTAVRULI CAPITAL LETTER AN..GEORGIAN MTAVRULI CAPITAL LETTER AIN
            0x1CBB,
            0x1CBD, // L&   [3] GEORGIAN MTAVRULI CAPITAL LETTER AEN..GEORGIAN MTAVRULI CAPITAL LETTER LABIAL SIGN
            0x1CC0,
            0x1CD0, // Mn   [3] VEDIC TONE KARSHANA..VEDIC TONE PRENKHA
            0x1CD3,
            0x1CD4, // Mn  [13] VEDIC SIGN YAJURVEDIC MIDLINE SVARITA..VEDIC TONE RIGVEDIC KASHMIRI INDEPENDENT SVARITA
            0x1CE1, // Mc       VEDIC TONE ATHARVAVEDIC INDEPENDENT SVARITA
            0x1CE2, // Mn   [7] VEDIC SIGN VISARGA SVARITA..VEDIC SIGN VISARGA ANUDATTA WITH TAIL
            0x1CE9, // Lo   [4] VEDIC SIGN ANUSVARA ANTARGOMUKHA..VEDIC SIGN ANUSVARA VAMAGOMUKHA WITH TAIL
            0x1CED, // Mn       VEDIC SIGN TIRYAK
            0x1CEE, // Lo   [6] VEDIC SIGN HEXIFORM LONG ANUSVARA..VEDIC SIGN ROTATED ARDHAVISARGA
            0x1CF4, // Mn       VEDIC TONE CANDRA ABOVE
            0x1CF5, // Lo   [2] VEDIC SIGN JIHVAMULIYA..VEDIC SIGN UPADHMANIYA
            0x1CF7, // Mc       VEDIC SIGN ATIKRAMA
            0x1CF8, // Mn   [2] VEDIC TONE RING ABOVE..VEDIC TONE DOUBLE RING ABOVE
            0x1CFA, // Lo       VEDIC SIGN DOUBLE ANUSVARA ANTARGOMUKHA
            0x1CFB,
            0x1D00, // L&  [44] LATIN LETTER SMALL CAPITAL A..CYRILLIC LETTER SMALL CAPITAL EL
            0x1D2C, // Lm  [63] MODIFIER LETTER CAPITAL A..GREEK SUBSCRIPT SMALL LETTER CHI
            0x1D6B, // L&  [13] LATIN SMALL LETTER UE..LATIN SMALL LETTER TURNED G
            0x1D78, // Lm       MODIFIER LETTER CYRILLIC EN
            0x1D79, // L&  [34] LATIN SMALL LETTER INSULAR G..LATIN SMALL LETTER EZH WITH RETROFLEX HOOK
            0x1D9B, // Lm  [37] MODIFIER LETTER SMALL TURNED ALPHA..MODIFIER LETTER SMALL THETA
            0x1DC0, // Mn  [58] COMBINING DOTTED GRAVE ACCENT..COMBINING WIDE INVERTED BRIDGE BELOW
            0x1DFA,
            0x1DFB, // Mn   [5] COMBINING DELETION MARK..COMBINING RIGHT ARROWHEAD AND DOWN ARROWHEAD BELOW
            0x1E00, // L& [278] LATIN CAPITAL LETTER A WITH RING BELOW..GREEK SMALL LETTER EPSILON WITH DASIA AND OXIA
            0x1F16,
            0x1F18, // L&   [6] GREEK CAPITAL LETTER EPSILON WITH PSILI..GREEK CAPITAL LETTER EPSILON WITH DASIA AND OXIA
            0x1F1E,
            0x1F20, // L&  [38] GREEK SMALL LETTER ETA WITH PSILI..GREEK SMALL LETTER OMICRON WITH DASIA AND OXIA
            0x1F46,
            0x1F48, // L&   [6] GREEK CAPITAL LETTER OMICRON WITH PSILI..GREEK CAPITAL LETTER OMICRON WITH DASIA AND OXIA
            0x1F4E,
            0x1F50, // L&   [8] GREEK SMALL LETTER UPSILON WITH PSILI..GREEK SMALL LETTER UPSILON WITH DASIA AND PERISPOMENI
            0x1F58,
            0x1F59, // L&       GREEK CAPITAL LETTER UPSILON WITH DASIA
            0x1F5A,
            0x1F5B, // L&       GREEK CAPITAL LETTER UPSILON WITH DASIA AND VARIA
            0x1F5C,
            0x1F5D, // L&       GREEK CAPITAL LETTER UPSILON WITH DASIA AND OXIA
            0x1F5E,
            0x1F5F, // L&  [31] GREEK CAPITAL LETTER UPSILON WITH DASIA AND PERISPOMENI..GREEK SMALL LETTER OMEGA WITH OXIA
            0x1F7E,
            0x1F80, // L&  [53] GREEK SMALL LETTER ALPHA WITH PSILI AND YPOGEGRAMMENI..GREEK SMALL LETTER ALPHA WITH OXIA AND YPOGEGRAMMENI
            0x1FB5,
            0x1FB6, // L&   [7] GREEK SMALL LETTER ALPHA WITH PERISPOMENI..GREEK CAPITAL LETTER ALPHA WITH PROSGEGRAMMENI
            0x1FBD,
            0x1FBE, // L&       GREEK PROSGEGRAMMENI
            0x1FBF,
            0x1FC2, // L&   [3] GREEK SMALL LETTER ETA WITH VARIA AND YPOGEGRAMMENI..GREEK SMALL LETTER ETA WITH OXIA AND YPOGEGRAMMENI
            0x1FC5,
            0x1FC6, // L&   [7] GREEK SMALL LETTER ETA WITH PERISPOMENI..GREEK CAPITAL LETTER ETA WITH PROSGEGRAMMENI
            0x1FCD,
            0x1FD0, // L&   [4] GREEK SMALL LETTER IOTA WITH VRACHY..GREEK SMALL LETTER IOTA WITH DIALYTIKA AND OXIA
            0x1FD4,
            0x1FD6, // L&   [6] GREEK SMALL LETTER IOTA WITH PERISPOMENI..GREEK CAPITAL LETTER IOTA WITH OXIA
            0x1FDC,
            0x1FE0, // L&  [13] GREEK SMALL LETTER UPSILON WITH VRACHY..GREEK CAPITAL LETTER RHO WITH DASIA
            0x1FED,
            0x1FF2, // L&   [3] GREEK SMALL LETTER OMEGA WITH VARIA AND YPOGEGRAMMENI..GREEK SMALL LETTER OMEGA WITH OXIA AND YPOGEGRAMMENI
            0x1FF5,
            0x1FF6, // L&   [7] GREEK SMALL LETTER OMEGA WITH PERISPOMENI..GREEK CAPITAL LETTER OMEGA WITH PROSGEGRAMMENI
            0x1FFD,
            0x2000, // Zs   [7] EN QUAD..SIX-PER-EM SPACE
            0x2007,
            0x2008, // Zs   [3] PUNCTUATION SPACE..HAIR SPACE
            0x200B,
            0x200C, // Cf       ZERO WIDTH NON-JOINER
            0x200D, // Cf       ZERO WIDTH JOINER
            0x200E, // Cf   [2] LEFT-TO-RIGHT MARK..RIGHT-TO-LEFT MARK
            0x2010,
            0x2018, // Pi       LEFT SINGLE QUOTATION MARK
            0x2019, // Pf       RIGHT SINGLE QUOTATION MARK
            0x201A,
            0x2024, // Po       ONE DOT LEADER
            0x2025,
            0x2027, // Po       HYPHENATION POINT
            0x2028, // Zl       LINE SEPARATOR
            0x2029, // Zp       PARAGRAPH SEPARATOR
            0x202A, // Cf   [5] LEFT-TO-RIGHT EMBEDDING..RIGHT-TO-LEFT OVERRIDE
            0x202F, // Zs       NARROW NO-BREAK SPACE
            0x2030,
            0x203F, // Pc   [2] UNDERTIE..CHARACTER TIE
            0x2041,
            0x2044, // Sm       FRACTION SLASH
            0x2045,
            0x2054, // Pc       INVERTED UNDERTIE
            0x2055,
            0x205F, // Zs       MEDIUM MATHEMATICAL SPACE
            0x2060, // Cf   [5] WORD JOINER..INVISIBLE PLUS
            0x2065,
            0x2066, // Cf  [10] LEFT-TO-RIGHT ISOLATE..NOMINAL DIGIT SHAPES
            0x2070,
            0x2071, // Lm       SUPERSCRIPT LATIN SMALL LETTER I
            0x2072,
            0x207F, // Lm       SUPERSCRIPT LATIN SMALL LETTER N
            0x2080,
            0x2090, // Lm  [13] LATIN SUBSCRIPT SMALL LETTER A..LATIN SUBSCRIPT SMALL LETTER T
            0x209D,
            0x20D0, // Mn  [13] COMBINING LEFT HARPOON ABOVE..COMBINING FOUR DOTS ABOVE
            0x20DD, // Me   [4] COMBINING ENCLOSING CIRCLE..COMBINING ENCLOSING CIRCLE BACKSLASH
            0x20E1, // Mn       COMBINING LEFT RIGHT ARROW ABOVE
            0x20E2, // Me   [3] COMBINING ENCLOSING SCREEN..COMBINING ENCLOSING UPWARD POINTING TRIANGLE
            0x20E5, // Mn  [12] COMBINING REVERSE SOLIDUS OVERLAY..COMBINING ASTERISK ABOVE
            0x20F1,
            0x2102, // L&       DOUBLE-STRUCK CAPITAL C
            0x2103,
            0x2107, // L&       EULER CONSTANT
            0x2108,
            0x210A, // L&  [10] SCRIPT SMALL G..SCRIPT SMALL L
            0x2114,
            0x2115, // L&       DOUBLE-STRUCK CAPITAL N
            0x2116,
            0x2119, // L&   [5] DOUBLE-STRUCK CAPITAL P..DOUBLE-STRUCK CAPITAL R
            0x211E,
            0x2124, // L&       DOUBLE-STRUCK CAPITAL Z
            0x2125,
            0x2126, // L&       OHM SIGN
            0x2127,
            0x2128, // L&       BLACK-LETTER CAPITAL Z
            0x2129,
            0x212A, // L&   [4] KELVIN SIGN..BLACK-LETTER CAPITAL C
            0x212E,
            0x212F, // L&   [6] SCRIPT SMALL E..SCRIPT SMALL O
            0x2135, // Lo   [4] ALEF SYMBOL..DALET SYMBOL
            0x2139, // L&       INFORMATION SOURCE
            0x213A,
            0x213C, // L&   [4] DOUBLE-STRUCK SMALL PI..DOUBLE-STRUCK CAPITAL PI
            0x2140,
            0x2145, // L&   [5] DOUBLE-STRUCK ITALIC CAPITAL D..DOUBLE-STRUCK ITALIC SMALL J
            0x214A,
            0x214E, // L&       TURNED SMALL F
            0x214F,
            0x2160, // Nl  [35] ROMAN NUMERAL ONE..ROMAN NUMERAL TEN THOUSAND
            0x2183, // L&   [2] ROMAN NUMERAL REVERSED ONE HUNDRED..LATIN SMALL LETTER REVERSED C
            0x2185, // Nl   [4] ROMAN NUMERAL SIX LATE FORM..ROMAN NUMERAL ONE HUNDRED THOUSAND
            0x2189,
            0x24B6, // So  [52] CIRCLED LATIN CAPITAL LETTER A..CIRCLED LATIN SMALL LETTER Z
            0x24EA,
            0x2C00, // L&  [47] GLAGOLITIC CAPITAL LETTER AZU..GLAGOLITIC CAPITAL LETTER LATINATE MYSLITE
            0x2C2F,
            0x2C30, // L&  [47] GLAGOLITIC SMALL LETTER AZU..GLAGOLITIC SMALL LETTER LATINATE MYSLITE
            0x2C5F,
            0x2C60, // L&  [28] LATIN CAPITAL LETTER L WITH DOUBLE BAR..LATIN LETTER SMALL CAPITAL TURNED E
            0x2C7C, // Lm   [2] LATIN SUBSCRIPT SMALL LETTER J..MODIFIER LETTER CAPITAL V
            0x2C7E, // L& [103] LATIN CAPITAL LETTER S WITH SWASH TAIL..COPTIC SYMBOL KAI
            0x2CE5,
            0x2CEB, // L&   [4] COPTIC CAPITAL LETTER CRYPTOGRAMMIC SHEI..COPTIC SMALL LETTER CRYPTOGRAMMIC GANGIA
            0x2CEF, // Mn   [3] COPTIC COMBINING NI ABOVE..COPTIC COMBINING SPIRITUS LENIS
            0x2CF2, // L&   [2] COPTIC CAPITAL LETTER BOHAIRIC KHEI..COPTIC SMALL LETTER BOHAIRIC KHEI
            0x2CF4,
            0x2D00, // L&  [38] GEORGIAN SMALL LETTER AN..GEORGIAN SMALL LETTER HOE
            0x2D26,
            0x2D27, // L&       GEORGIAN SMALL LETTER YN
            0x2D28,
            0x2D2D, // L&       GEORGIAN SMALL LETTER AEN
            0x2D2E,
            0x2D30, // Lo  [56] TIFINAGH LETTER YA..TIFINAGH LETTER YO
            0x2D68,
            0x2D6F, // Lm       TIFINAGH MODIFIER LETTER LABIALIZATION MARK
            0x2D70,
            0x2D7F, // Mn       TIFINAGH CONSONANT JOINER
            0x2D80, // Lo  [23] ETHIOPIC SYLLABLE LOA..ETHIOPIC SYLLABLE GGWE
            0x2D97,
            0x2DA0, // Lo   [7] ETHIOPIC SYLLABLE SSA..ETHIOPIC SYLLABLE SSO
            0x2DA7,
            0x2DA8, // Lo   [7] ETHIOPIC SYLLABLE CCA..ETHIOPIC SYLLABLE CCO
            0x2DAF,
            0x2DB0, // Lo   [7] ETHIOPIC SYLLABLE ZZA..ETHIOPIC SYLLABLE ZZO
            0x2DB7,
            0x2DB8, // Lo   [7] ETHIOPIC SYLLABLE CCHA..ETHIOPIC SYLLABLE CCHO
            0x2DBF,
            0x2DC0, // Lo   [7] ETHIOPIC SYLLABLE QYA..ETHIOPIC SYLLABLE QYO
            0x2DC7,
            0x2DC8, // Lo   [7] ETHIOPIC SYLLABLE KYA..ETHIOPIC SYLLABLE KYO
            0x2DCF,
            0x2DD0, // Lo   [7] ETHIOPIC SYLLABLE XYA..ETHIOPIC SYLLABLE XYO
            0x2DD7,
            0x2DD8, // Lo   [7] ETHIOPIC SYLLABLE GYA..ETHIOPIC SYLLABLE GYO
            0x2DDF,
            0x2DE0, // Mn  [32] COMBINING CYRILLIC LETTER BE..COMBINING CYRILLIC LETTER IOTIFIED BIG YUS
            0x2E00,
            0x2E2F, // Lm       VERTICAL TILDE
            0x2E30,
            0x3000, // Zs       IDEOGRAPHIC SPACE
            0x3001,
            0x3005, // Lm       IDEOGRAPHIC ITERATION MARK
            0x3006,
            0x302A, // Mn   [4] IDEOGRAPHIC LEVEL TONE MARK..IDEOGRAPHIC ENTERING TONE MARK
            0x302E, // Mc   [2] HANGUL SINGLE DOT TONE MARK..HANGUL DOUBLE DOT TONE MARK
            0x3030,
            0x3031, // Lm   [5] VERTICAL KANA REPEAT MARK..VERTICAL KANA REPEAT MARK LOWER HALF
            0x3036,
            0x303B, // Lm       VERTICAL IDEOGRAPHIC ITERATION MARK
            0x303C, // Lo       MASU MARK
            0x303D,
            0x3099, // Mn   [2] COMBINING KATAKANA-HIRAGANA VOICED SOUND MARK..COMBINING KATAKANA-HIRAGANA SEMI-VOICED SOUND MARK
            0x309B, // Sk   [2] KATAKANA-HIRAGANA VOICED SOUND MARK..KATAKANA-HIRAGANA SEMI-VOICED SOUND MARK
            0x309D,
            0x30A0, // Pd       KATAKANA-HIRAGANA DOUBLE HYPHEN
            0x30A1, // Lo  [90] KATAKANA LETTER SMALL A..KATAKANA LETTER VO
            0x30FB,
            0x30FC, // Lm   [3] KATAKANA-HIRAGANA PROLONGED SOUND MARK..KATAKANA VOICED ITERATION MARK
            0x30FF, // Lo       KATAKANA DIGRAPH KOTO
            0x3100,
            0x3105, // Lo  [43] BOPOMOFO LETTER B..BOPOMOFO LETTER NN
            0x3130,
            0x3131, // Lo  [94] HANGUL LETTER KIYEOK..HANGUL LETTER ARAEAE
            0x318F,
            0x31A0, // Lo  [27] BOPOMOFO LETTER BU..BOPOMOFO LETTER ZY
            0x31BB,
            0x31F0, // Lo  [16] KATAKANA LETTER SMALL KU..KATAKANA LETTER SMALL RO
            0x3200,
            0x32D0, // So  [47] CIRCLED KATAKANA A..CIRCLED KATAKANA WO
            0x32FF,
            0x3300, // So  [88] SQUARE APAATO..SQUARE WATTO
            0x3358,
            0xA000, // Lo  [21] YI SYLLABLE IT..YI SYLLABLE E
            0xA015, // Lm       YI SYLLABLE WU
            0xA016, // Lo [1143] YI SYLLABLE BIT..YI SYLLABLE YYR
            0xA48D,
            0xA4D0, // Lo  [40] LISU LETTER BA..LISU LETTER OE
            0xA4F8, // Lm   [6] LISU LETTER TONE MYA TI..LISU LETTER TONE MYA JEU
            0xA4FE,
            0xA500, // Lo [268] VAI SYLLABLE EE..VAI SYLLABLE NG
            0xA60C, // Lm       VAI SYLLABLE LENGTHENER
            0xA60D,
            0xA610, // Lo  [16] VAI SYLLABLE NDOLE FA..VAI SYMBOL JONG
            0xA620, // Nd  [10] VAI DIGIT ZERO..VAI DIGIT NINE
            0xA62A, // Lo   [2] VAI SYLLABLE NDOLE MA..VAI SYLLABLE NDOLE DO
            0xA62C,
            0xA640, // L&  [46] CYRILLIC CAPITAL LETTER ZEMLYA..CYRILLIC SMALL LETTER DOUBLE MONOCULAR O
            0xA66E, // Lo       CYRILLIC LETTER MULTIOCULAR O
            0xA66F, // Mn       COMBINING CYRILLIC VZMET
            0xA670, // Me   [3] COMBINING CYRILLIC TEN MILLIONS SIGN..COMBINING CYRILLIC THOUSAND MILLIONS SIGN
            0xA673,
            0xA674, // Mn  [10] COMBINING CYRILLIC LETTER UKRAINIAN IE..COMBINING CYRILLIC PAYEROK
            0xA67E,
            0xA67F, // Lm       CYRILLIC PAYEROK
            0xA680, // L&  [28] CYRILLIC CAPITAL LETTER DWE..CYRILLIC SMALL LETTER CROSSED O
            0xA69C, // Lm   [2] MODIFIER LETTER CYRILLIC HARD SIGN..MODIFIER LETTER CYRILLIC SOFT SIGN
            0xA69E, // Mn   [2] COMBINING CYRILLIC LETTER EF..COMBINING CYRILLIC LETTER IOTIFIED E
            0xA6A0, // Lo  [70] BAMUM LETTER A..BAMUM LETTER KI
            0xA6E6, // Nl  [10] BAMUM LETTER MO..BAMUM LETTER KOGHOM
            0xA6F0, // Mn   [2] BAMUM COMBINING MARK KOQNDON..BAMUM COMBINING MARK TUKWENTIS
            0xA6F2,
            0xA717, // Lm   [9] MODIFIER LETTER DOT VERTICAL BAR..MODIFIER LETTER LOW INVERTED EXCLAMATION MARK
            0xA720, // Sk   [2] MODIFIER LETTER STRESS AND HIGH TONE..MODIFIER LETTER STRESS AND LOW TONE
            0xA722, // L&  [78] LATIN CAPITAL LETTER EGYPTOLOGICAL ALEF..LATIN SMALL LETTER CON
            0xA770, // Lm       MODIFIER LETTER US
            0xA771, // L&  [23] LATIN SMALL LETTER DUM..LATIN SMALL LETTER INSULAR T
            0xA788, // Lm       MODIFIER LETTER LOW CIRCUMFLEX ACCENT
            0xA789, // Sk   [2] MODIFIER LETTER COLON..MODIFIER LETTER SHORT EQUALS SIGN
            0xA78B, // L&   [4] LATIN CAPITAL LETTER SALTILLO..LATIN SMALL LETTER L WITH RETROFLEX HOOK AND BELT
            0xA78F, // Lo       LATIN LETTER SINOLOGICAL DOT
            0xA790, // L&  [48] LATIN CAPITAL LETTER N WITH DESCENDER..LATIN SMALL LETTER GLOTTAL U
            0xA7C0,
            0xA7C2, // L&   [5] LATIN CAPITAL LETTER ANGLICANA W..LATIN CAPITAL LETTER Z WITH PALATAL HOOK
            0xA7C7,
            0xA7F7, // Lo       LATIN EPIGRAPHIC LETTER SIDEWAYS I
            0xA7F8, // Lm   [2] MODIFIER LETTER CAPITAL H WITH STROKE..MODIFIER LETTER SMALL LIGATURE OE
            0xA7FA, // L&       LATIN LETTER SMALL CAPITAL TURNED M
            0xA7FB, // Lo   [7] LATIN EPIGRAPHIC LETTER REVERSED F..SYLOTI NAGRI LETTER I
            0xA802, // Mn       SYLOTI NAGRI SIGN DVISVARA
            0xA803, // Lo   [3] SYLOTI NAGRI LETTER U..SYLOTI NAGRI LETTER O
            0xA806, // Mn       SYLOTI NAGRI SIGN HASANTA
            0xA807, // Lo   [4] SYLOTI NAGRI LETTER KO..SYLOTI NAGRI LETTER GHO
            0xA80B, // Mn       SYLOTI NAGRI SIGN ANUSVARA
            0xA80C, // Lo  [23] SYLOTI NAGRI LETTER CO..SYLOTI NAGRI LETTER HO
            0xA823, // Mc   [2] SYLOTI NAGRI VOWEL SIGN A..SYLOTI NAGRI VOWEL SIGN I
            0xA825, // Mn   [2] SYLOTI NAGRI VOWEL SIGN U..SYLOTI NAGRI VOWEL SIGN E
            0xA827, // Mc       SYLOTI NAGRI VOWEL SIGN OO
            0xA828,
            0xA840, // Lo  [52] PHAGS-PA LETTER KA..PHAGS-PA LETTER CANDRABINDU
            0xA874,
            0xA880, // Mc   [2] SAURASHTRA SIGN ANUSVARA..SAURASHTRA SIGN VISARGA
            0xA882, // Lo  [50] SAURASHTRA LETTER A..SAURASHTRA LETTER LLA
            0xA8B4, // Mc  [16] SAURASHTRA CONSONANT SIGN HAARU..SAURASHTRA VOWEL SIGN AU
            0xA8C4, // Mn   [2] SAURASHTRA SIGN VIRAMA..SAURASHTRA SIGN CANDRABINDU
            0xA8C6,
            0xA8D0, // Nd  [10] SAURASHTRA DIGIT ZERO..SAURASHTRA DIGIT NINE
            0xA8DA,
            0xA8E0, // Mn  [18] COMBINING DEVANAGARI DIGIT ZERO..COMBINING DEVANAGARI SIGN AVAGRAHA
            0xA8F2, // Lo   [6] DEVANAGARI SIGN SPACING CANDRABINDU..DEVANAGARI SIGN CANDRABINDU AVAGRAHA
            0xA8F8,
            0xA8FB, // Lo       DEVANAGARI HEADSTROKE
            0xA8FC,
            0xA8FD, // Lo   [2] DEVANAGARI JAIN OM..DEVANAGARI LETTER AY
            0xA8FF, // Mn       DEVANAGARI VOWEL SIGN AY
            0xA900, // Nd  [10] KAYAH LI DIGIT ZERO..KAYAH LI DIGIT NINE
            0xA90A, // Lo  [28] KAYAH LI LETTER KA..KAYAH LI LETTER OO
            0xA926, // Mn   [8] KAYAH LI VOWEL UE..KAYAH LI TONE CALYA PLOPHU
            0xA92E,
            0xA930, // Lo  [23] REJANG LETTER KA..REJANG LETTER A
            0xA947, // Mn  [11] REJANG VOWEL SIGN I..REJANG CONSONANT SIGN R
            0xA952, // Mc   [2] REJANG CONSONANT SIGN H..REJANG VIRAMA
            0xA954,
            0xA960, // Lo  [29] HANGUL CHOSEONG TIKEUT-MIEUM..HANGUL CHOSEONG SSANGYEORINHIEUH
            0xA97D,
            0xA980, // Mn   [3] JAVANESE SIGN PANYANGGA..JAVANESE SIGN LAYAR
            0xA983, // Mc       JAVANESE SIGN WIGNYAN
            0xA984, // Lo  [47] JAVANESE LETTER A..JAVANESE LETTER HA
            0xA9B3, // Mn       JAVANESE SIGN CECAK TELU
            0xA9B4, // Mc   [2] JAVANESE VOWEL SIGN TARUNG..JAVANESE VOWEL SIGN TOLONG
            0xA9B6, // Mn   [4] JAVANESE VOWEL SIGN WULU..JAVANESE VOWEL SIGN SUKU MENDUT
            0xA9BA, // Mc   [2] JAVANESE VOWEL SIGN TALING..JAVANESE VOWEL SIGN DIRGA MURE
            0xA9BC, // Mn   [2] JAVANESE VOWEL SIGN PEPET..JAVANESE CONSONANT SIGN KERET
            0xA9BE, // Mc   [3] JAVANESE CONSONANT SIGN PENGKAL..JAVANESE PANGKON
            0xA9C1,
            0xA9CF, // Lm       JAVANESE PANGRANGKEP
            0xA9D0, // Nd  [10] JAVANESE DIGIT ZERO..JAVANESE DIGIT NINE
            0xA9DA,
            0xA9E5, // Mn       MYANMAR SIGN SHAN SAW
            0xA9E6,
            0xA9F0, // Nd  [10] MYANMAR TAI LAING DIGIT ZERO..MYANMAR TAI LAING DIGIT NINE
            0xA9FA,
            0xAA00, // Lo  [41] CHAM LETTER A..CHAM LETTER HA
            0xAA29, // Mn   [6] CHAM VOWEL SIGN AA..CHAM VOWEL SIGN OE
            0xAA2F, // Mc   [2] CHAM VOWEL SIGN O..CHAM VOWEL SIGN AI
            0xAA31, // Mn   [2] CHAM VOWEL SIGN AU..CHAM VOWEL SIGN UE
            0xAA33, // Mc   [2] CHAM CONSONANT SIGN YA..CHAM CONSONANT SIGN RA
            0xAA35, // Mn   [2] CHAM CONSONANT SIGN LA..CHAM CONSONANT SIGN WA
            0xAA37,
            0xAA40, // Lo   [3] CHAM LETTER FINAL K..CHAM LETTER FINAL NG
            0xAA43, // Mn       CHAM CONSONANT SIGN FINAL NG
            0xAA44, // Lo   [8] CHAM LETTER FINAL CH..CHAM LETTER FINAL SS
            0xAA4C, // Mn       CHAM CONSONANT SIGN FINAL M
            0xAA4D, // Mc       CHAM CONSONANT SIGN FINAL H
            0xAA4E,
            0xAA50, // Nd  [10] CHAM DIGIT ZERO..CHAM DIGIT NINE
            0xAA5A,
            0xAA7B, // Mc       MYANMAR SIGN PAO KAREN TONE
            0xAA7C, // Mn       MYANMAR SIGN TAI LAING TONE-2
            0xAA7D, // Mc       MYANMAR SIGN TAI LAING TONE-5
            0xAA7E,
            0xAAB0, // Mn       TAI VIET MAI KANG
            0xAAB1,
            0xAAB2, // Mn   [3] TAI VIET VOWEL I..TAI VIET VOWEL U
            0xAAB5,
            0xAAB7, // Mn   [2] TAI VIET MAI KHIT..TAI VIET VOWEL IA
            0xAAB9,
            0xAABE, // Mn   [2] TAI VIET VOWEL AM..TAI VIET TONE MAI EK
            0xAAC0,
            0xAAC1, // Mn       TAI VIET TONE MAI THO
            0xAAC2,
            0xAAE0, // Lo  [11] MEETEI MAYEK LETTER E..MEETEI MAYEK LETTER SSA
            0xAAEB, // Mc       MEETEI MAYEK VOWEL SIGN II
            0xAAEC, // Mn   [2] MEETEI MAYEK VOWEL SIGN UU..MEETEI MAYEK VOWEL SIGN AAI
            0xAAEE, // Mc   [2] MEETEI MAYEK VOWEL SIGN AU..MEETEI MAYEK VOWEL SIGN AAU
            0xAAF0,
            0xAAF2, // Lo       MEETEI MAYEK ANJI
            0xAAF3, // Lm   [2] MEETEI MAYEK SYLLABLE REPETITION MARK..MEETEI MAYEK WORD REPETITION MARK
            0xAAF5, // Mc       MEETEI MAYEK VOWEL SIGN VISARGA
            0xAAF6, // Mn       MEETEI MAYEK VIRAMA
            0xAAF7,
            0xAB01, // Lo   [6] ETHIOPIC SYLLABLE TTHU..ETHIOPIC SYLLABLE TTHO
            0xAB07,
            0xAB09, // Lo   [6] ETHIOPIC SYLLABLE DDHU..ETHIOPIC SYLLABLE DDHO
            0xAB0F,
            0xAB11, // Lo   [6] ETHIOPIC SYLLABLE DZU..ETHIOPIC SYLLABLE DZO
            0xAB17,
            0xAB20, // Lo   [7] ETHIOPIC SYLLABLE CCHHA..ETHIOPIC SYLLABLE CCHHO
            0xAB27,
            0xAB28, // Lo   [7] ETHIOPIC SYLLABLE BBA..ETHIOPIC SYLLABLE BBO
            0xAB2F,
            0xAB30, // L&  [43] LATIN SMALL LETTER BARRED ALPHA..LATIN SMALL LETTER Y WITH SHORT RIGHT LEG
            0xAB5B, // Sk       MODIFIER BREVE WITH INVERTED BREVE
            0xAB5C, // Lm   [4] MODIFIER LETTER SMALL HENG..MODIFIER LETTER SMALL U WITH LEFT HOOK
            0xAB60, // L&   [8] LATIN SMALL LETTER SAKHA YAT..LATIN SMALL LETTER TS DIGRAPH WITH RETROFLEX HOOK
            0xAB68,
            0xAB70, // L&  [80] CHEROKEE SMALL LETTER A..CHEROKEE SMALL LETTER YA
            0xABC0, // Lo  [35] MEETEI MAYEK LETTER KOK..MEETEI MAYEK LETTER I LONSUM
            0xABE3, // Mc   [2] MEETEI MAYEK VOWEL SIGN ONAP..MEETEI MAYEK VOWEL SIGN INAP
            0xABE5, // Mn       MEETEI MAYEK VOWEL SIGN ANAP
            0xABE6, // Mc   [2] MEETEI MAYEK VOWEL SIGN YENAP..MEETEI MAYEK VOWEL SIGN SOUNAP
            0xABE8, // Mn       MEETEI MAYEK VOWEL SIGN UNAP
            0xABE9, // Mc   [2] MEETEI MAYEK VOWEL SIGN CHEINAP..MEETEI MAYEK VOWEL SIGN NUNG
            0xABEB,
            0xABEC, // Mc       MEETEI MAYEK LUM IYEK
            0xABED, // Mn       MEETEI MAYEK APUN IYEK
            0xABEE,
            0xABF0, // Nd  [10] MEETEI MAYEK DIGIT ZERO..MEETEI MAYEK DIGIT NINE
            0xABFA,
            0xAC00, // Lo [11172] HANGUL SYLLABLE GA..HANGUL SYLLABLE HIH
            0xD7A4,
            0xD7B0, // Lo  [23] HANGUL JUNGSEONG O-YEO..HANGUL JUNGSEONG ARAEA-E
            0xD7C7,
            0xD7CB, // Lo  [49] HANGUL JONGSEONG NIEUN-RIEUL..HANGUL JONGSEONG PHIEUPH-THIEUTH
            0xD7FC,
            0xFB00, // L&   [7] LATIN SMALL LIGATURE FF..LATIN SMALL LIGATURE ST
            0xFB07,
            0xFB13, // L&   [5] ARMENIAN SMALL LIGATURE MEN NOW..ARMENIAN SMALL LIGATURE MEN XEH
            0xFB18,
            0xFB1D, // Lo       HEBREW LETTER YOD WITH HIRIQ
            0xFB1E, // Mn       HEBREW POINT JUDEO-SPANISH VARIKA
            0xFB1F, // Lo  [10] HEBREW LIGATURE YIDDISH YOD YOD PATAH..HEBREW LETTER WIDE TAV
            0xFB29,
            0xFB2A, // Lo  [13] HEBREW LETTER SHIN WITH SHIN DOT..HEBREW LETTER ZAYIN WITH DAGESH
            0xFB37,
            0xFB38, // Lo   [5] HEBREW LETTER TET WITH DAGESH..HEBREW LETTER LAMED WITH DAGESH
            0xFB3D,
            0xFB3E, // Lo       HEBREW LETTER MEM WITH DAGESH
            0xFB3F,
            0xFB40, // Lo   [2] HEBREW LETTER NUN WITH DAGESH..HEBREW LETTER SAMEKH WITH DAGESH
            0xFB42,
            0xFB43, // Lo   [2] HEBREW LETTER FINAL PE WITH DAGESH..HEBREW LETTER PE WITH DAGESH
            0xFB45,
            0xFB46, // Lo  [10] HEBREW LETTER TSADI WITH DAGESH..HEBREW LIGATURE ALEF LAMED
            0xFB50, // Lo  [98] ARABIC LETTER ALEF WASLA ISOLATED FORM..ARABIC LETTER YEH BARREE WITH HAMZA ABOVE FINAL FORM
            0xFBB2,
            0xFBD3, // Lo [363] ARABIC LETTER NG ISOLATED FORM..ARABIC LIGATURE ALEF WITH FATHATAN ISOLATED FORM
            0xFD3E,
            0xFD50, // Lo  [64] ARABIC LIGATURE TEH WITH JEEM WITH MEEM INITIAL FORM..ARABIC LIGATURE MEEM WITH KHAH WITH MEEM INITIAL FORM
            0xFD90,
            0xFD92, // Lo  [54] ARABIC LIGATURE MEEM WITH JEEM WITH KHAH INITIAL FORM..ARABIC LIGATURE NOON WITH JEEM WITH YEH FINAL FORM
            0xFDC8,
            0xFDF0, // Lo  [12] ARABIC LIGATURE SALLA USED AS KORANIC STOP SIGN ISOLATED FORM..ARABIC LIGATURE JALLAJALALOUHOU
            0xFDFC,
            0xFE00, // Mn  [16] VARIATION SELECTOR-1..VARIATION SELECTOR-16
            0xFE10, // Po       PRESENTATION FORM FOR VERTICAL COMMA
            0xFE11,
            0xFE13, // Po       PRESENTATION FORM FOR VERTICAL COLON
            0xFE14, // Po       PRESENTATION FORM FOR VERTICAL SEMICOLON
            0xFE15,
            0xFE20, // Mn  [16] COMBINING LIGATURE LEFT HALF..COMBINING CYRILLIC TITLO RIGHT HALF
            0xFE30,
            0xFE33, // Pc   [2] PRESENTATION FORM FOR VERTICAL LOW LINE..PRESENTATION FORM FOR VERTICAL WAVY LOW LINE
            0xFE35,
            0xFE4D, // Pc   [3] DASHED LOW LINE..WAVY LOW LINE
            0xFE50, // Po       SMALL COMMA
            0xFE51,
            0xFE52, // Po       SMALL FULL STOP
            0xFE53,
            0xFE54, // Po       SMALL SEMICOLON
            0xFE55, // Po       SMALL COLON
            0xFE56,
            0xFE70, // Lo   [5] ARABIC FATHATAN ISOLATED FORM..ARABIC KASRATAN ISOLATED FORM
            0xFE75,
            0xFE76, // Lo [135] ARABIC FATHA ISOLATED FORM..ARABIC LIGATURE LAM WITH ALEF FINAL FORM
            0xFEFD,
            0xFEFF, // Cf       ZERO WIDTH NO-BREAK SPACE
            0xFF00,
            0xFF07, // Po       FULLWIDTH APOSTROPHE
            0xFF08,
            0xFF0C, // Po       FULLWIDTH COMMA
            0xFF0D,
            0xFF0E, // Po       FULLWIDTH FULL STOP
            0xFF0F,
            0xFF10, // Nd  [10] FULLWIDTH DIGIT ZERO..FULLWIDTH DIGIT NINE
            0xFF1A, // Po       FULLWIDTH COLON
            0xFF1B, // Po       FULLWIDTH SEMICOLON
            0xFF1C,
            0xFF21, // L&  [26] FULLWIDTH LATIN CAPITAL LETTER A..FULLWIDTH LATIN CAPITAL LETTER Z
            0xFF3B,
            0xFF3F, // Pc       FULLWIDTH LOW LINE
            0xFF40,
            0xFF41, // L&  [26] FULLWIDTH LATIN SMALL LETTER A..FULLWIDTH LATIN SMALL LETTER Z
            0xFF5B,
            0xFF66, // Lo  [10] HALFWIDTH KATAKANA LETTER WO..HALFWIDTH KATAKANA LETTER SMALL TU
            0xFF70, // Lm       HALFWIDTH KATAKANA-HIRAGANA PROLONGED SOUND MARK
            0xFF71, // Lo  [45] HALFWIDTH KATAKANA LETTER A..HALFWIDTH KATAKANA LETTER N
            0xFF9E, // Lm   [2] HALFWIDTH KATAKANA VOICED SOUND MARK..HALFWIDTH KATAKANA SEMI-VOICED SOUND MARK
            0xFFA0, // Lo  [31] HALFWIDTH HANGUL FILLER..HALFWIDTH HANGUL LETTER HIEUH
            0xFFBF,
            0xFFC2, // Lo   [6] HALFWIDTH HANGUL LETTER A..HALFWIDTH HANGUL LETTER E
            0xFFC8,
            0xFFCA, // Lo   [6] HALFWIDTH HANGUL LETTER YEO..HALFWIDTH HANGUL LETTER OE
            0xFFD0,
            0xFFD2, // Lo   [6] HALFWIDTH HANGUL LETTER YO..HALFWIDTH HANGUL LETTER YU
            0xFFD8,
            0xFFDA, // Lo   [3] HALFWIDTH HANGUL LETTER EU..HALFWIDTH HANGUL LETTER I
            0xFFDD,
            0xFFF9, // Cf   [3] INTERLINEAR ANNOTATION ANCHOR..INTERLINEAR ANNOTATION TERMINATOR
            0xFFFC,
            0x10000, // Lo  [12] LINEAR B SYLLABLE B008 A..LINEAR B SYLLABLE B046 JE
            0x1000C,
            0x1000D, // Lo  [26] LINEAR B SYLLABLE B036 JO..LINEAR B SYLLABLE B032 QO
            0x10027,
            0x10028, // Lo  [19] LINEAR B SYLLABLE B060 RA..LINEAR B SYLLABLE B042 WO
            0x1003B,
            0x1003C, // Lo   [2] LINEAR B SYLLABLE B017 ZA..LINEAR B SYLLABLE B074 ZE
            0x1003E,
            0x1003F, // Lo  [15] LINEAR B SYLLABLE B020 ZO..LINEAR B SYLLABLE B091 TWO
            0x1004E,
            0x10050, // Lo  [14] LINEAR B SYMBOL B018..LINEAR B SYMBOL B089
            0x1005E,
            0x10080, // Lo [123] LINEAR B IDEOGRAM B100 MAN..LINEAR B IDEOGRAM VESSEL B305
            0x100FB,
            0x10140, // Nl  [53] GREEK ACROPHONIC ATTIC ONE QUARTER..GREEK ACROPHONIC STRATIAN FIFTY MNAS
            0x10175,
            0x101FD, // Mn       PHAISTOS DISC SIGN COMBINING OBLIQUE STROKE
            0x101FE,
            0x10280, // Lo  [29] LYCIAN LETTER A..LYCIAN LETTER X
            0x1029D,
            0x102A0, // Lo  [49] CARIAN LETTER A..CARIAN LETTER UUU3
            0x102D1,
            0x102E0, // Mn       COPTIC EPACT THOUSANDS MARK
            0x102E1,
            0x10300, // Lo  [32] OLD ITALIC LETTER A..OLD ITALIC LETTER ESS
            0x10320,
            0x1032D, // Lo  [20] OLD ITALIC LETTER YE..GOTHIC LETTER PAIRTHRA
            0x10341, // Nl       GOTHIC LETTER NINETY
            0x10342, // Lo   [8] GOTHIC LETTER RAIDA..GOTHIC LETTER OTHAL
            0x1034A, // Nl       GOTHIC LETTER NINE HUNDRED
            0x1034B,
            0x10350, // Lo  [38] OLD PERMIC LETTER AN..OLD PERMIC LETTER IA
            0x10376, // Mn   [5] COMBINING OLD PERMIC LETTER AN..COMBINING OLD PERMIC LETTER SII
            0x1037B,
            0x10380, // Lo  [30] UGARITIC LETTER ALPA..UGARITIC LETTER SSU
            0x1039E,
            0x103A0, // Lo  [36] OLD PERSIAN SIGN A..OLD PERSIAN SIGN HA
            0x103C4,
            0x103C8, // Lo   [8] OLD PERSIAN SIGN AURAMAZDAA..OLD PERSIAN SIGN BUUMISH
            0x103D0,
            0x103D1, // Nl   [5] OLD PERSIAN NUMBER ONE..OLD PERSIAN NUMBER HUNDRED
            0x103D6,
            0x10400, // L&  [80] DESERET CAPITAL LETTER LONG I..DESERET SMALL LETTER EW
            0x10450, // Lo  [78] SHAVIAN LETTER PEEP..OSMANYA LETTER OO
            0x1049E,
            0x104A0, // Nd  [10] OSMANYA DIGIT ZERO..OSMANYA DIGIT NINE
            0x104AA,
            0x104B0, // L&  [36] OSAGE CAPITAL LETTER A..OSAGE CAPITAL LETTER ZHA
            0x104D4,
            0x104D8, // L&  [36] OSAGE SMALL LETTER A..OSAGE SMALL LETTER ZHA
            0x104FC,
            0x10500, // Lo  [40] ELBASAN LETTER A..ELBASAN LETTER KHE
            0x10528,
            0x10530, // Lo  [52] CAUCASIAN ALBANIAN LETTER ALT..CAUCASIAN ALBANIAN LETTER KIW
            0x10564,
            0x10600, // Lo [311] LINEAR A SIGN AB001..LINEAR A SIGN A664
            0x10737,
            0x10740, // Lo  [22] LINEAR A SIGN A701 A..LINEAR A SIGN A732 JE
            0x10756,
            0x10760, // Lo   [8] LINEAR A SIGN A800..LINEAR A SIGN A807
            0x10768,
            0x10800, // Lo   [6] CYPRIOT SYLLABLE A..CYPRIOT SYLLABLE JA
            0x10806,
            0x10808, // Lo       CYPRIOT SYLLABLE JO
            0x10809,
            0x1080A, // Lo  [44] CYPRIOT SYLLABLE KA..CYPRIOT SYLLABLE WO
            0x10836,
            0x10837, // Lo   [2] CYPRIOT SYLLABLE XA..CYPRIOT SYLLABLE XE
            0x10839,
            0x1083C, // Lo       CYPRIOT SYLLABLE ZA
            0x1083D,
            0x1083F, // Lo  [23] CYPRIOT SYLLABLE ZO..IMPERIAL ARAMAIC LETTER TAW
            0x10856,
            0x10860, // Lo  [23] PALMYRENE LETTER ALEPH..PALMYRENE LETTER TAW
            0x10877,
            0x10880, // Lo  [31] NABATAEAN LETTER FINAL ALEPH..NABATAEAN LETTER TAW
            0x1089F,
            0x108E0, // Lo  [19] HATRAN LETTER ALEPH..HATRAN LETTER QOPH
            0x108F3,
            0x108F4, // Lo   [2] HATRAN LETTER SHIN..HATRAN LETTER TAW
            0x108F6,
            0x10900, // Lo  [22] PHOENICIAN LETTER ALF..PHOENICIAN LETTER TAU
            0x10916,
            0x10920, // Lo  [26] LYDIAN LETTER A..LYDIAN LETTER C
            0x1093A,
            0x10980, // Lo  [56] MEROITIC HIEROGLYPHIC LETTER A..MEROITIC CURSIVE LETTER DA
            0x109B8,
            0x109BE, // Lo   [2] MEROITIC CURSIVE LOGOGRAM RMT..MEROITIC CURSIVE LOGOGRAM IMN
            0x109C0,
            0x10A00, // Lo       KHAROSHTHI LETTER A
            0x10A01, // Mn   [3] KHAROSHTHI VOWEL SIGN I..KHAROSHTHI VOWEL SIGN VOCALIC R
            0x10A04,
            0x10A05, // Mn   [2] KHAROSHTHI VOWEL SIGN E..KHAROSHTHI VOWEL SIGN O
            0x10A07,
            0x10A0C, // Mn   [4] KHAROSHTHI VOWEL LENGTH MARK..KHAROSHTHI SIGN VISARGA
            0x10A10, // Lo   [4] KHAROSHTHI LETTER KA..KHAROSHTHI LETTER GHA
            0x10A14,
            0x10A15, // Lo   [3] KHAROSHTHI LETTER CA..KHAROSHTHI LETTER JA
            0x10A18,
            0x10A19, // Lo  [29] KHAROSHTHI LETTER NYA..KHAROSHTHI LETTER VHA
            0x10A36,
            0x10A38, // Mn   [3] KHAROSHTHI SIGN BAR ABOVE..KHAROSHTHI SIGN DOT BELOW
            0x10A3B,
            0x10A3F, // Mn       KHAROSHTHI VIRAMA
            0x10A40,
            0x10A60, // Lo  [29] OLD SOUTH ARABIAN LETTER HE..OLD SOUTH ARABIAN LETTER THETH
            0x10A7D,
            0x10A80, // Lo  [29] OLD NORTH ARABIAN LETTER HEH..OLD NORTH ARABIAN LETTER ZAH
            0x10A9D,
            0x10AC0, // Lo   [8] MANICHAEAN LETTER ALEPH..MANICHAEAN LETTER WAW
            0x10AC8,
            0x10AC9, // Lo  [28] MANICHAEAN LETTER ZAYIN..MANICHAEAN LETTER TAW
            0x10AE5, // Mn   [2] MANICHAEAN ABBREVIATION MARK ABOVE..MANICHAEAN ABBREVIATION MARK BELOW
            0x10AE7,
            0x10B00, // Lo  [54] AVESTAN LETTER A..AVESTAN LETTER HE
            0x10B36,
            0x10B40, // Lo  [22] INSCRIPTIONAL PARTHIAN LETTER ALEPH..INSCRIPTIONAL PARTHIAN LETTER TAW
            0x10B56,
            0x10B60, // Lo  [19] INSCRIPTIONAL PAHLAVI LETTER ALEPH..INSCRIPTIONAL PAHLAVI LETTER TAW
            0x10B73,
            0x10B80, // Lo  [18] PSALTER PAHLAVI LETTER ALEPH..PSALTER PAHLAVI LETTER TAW
            0x10B92,
            0x10C00, // Lo  [73] OLD TURKIC LETTER ORKHON A..OLD TURKIC LETTER ORKHON BASH
            0x10C49,
            0x10C80, // L&  [51] OLD HUNGARIAN CAPITAL LETTER A..OLD HUNGARIAN CAPITAL LETTER US
            0x10CB3,
            0x10CC0, // L&  [51] OLD HUNGARIAN SMALL LETTER A..OLD HUNGARIAN SMALL LETTER US
            0x10CF3,
            0x10D00, // Lo  [36] HANIFI ROHINGYA LETTER A..HANIFI ROHINGYA MARK NA KHONNA
            0x10D24, // Mn   [4] HANIFI ROHINGYA SIGN HARBAHAY..HANIFI ROHINGYA SIGN TASSI
            0x10D28,
            0x10D30, // Nd  [10] HANIFI ROHINGYA DIGIT ZERO..HANIFI ROHINGYA DIGIT NINE
            0x10D3A,
            0x10F00, // Lo  [29] OLD SOGDIAN LETTER ALEPH..OLD SOGDIAN LETTER FINAL TAW WITH VERTICAL TAIL
            0x10F1D,
            0x10F27, // Lo       OLD SOGDIAN LIGATURE AYIN-DALETH
            0x10F28,
            0x10F30, // Lo  [22] SOGDIAN LETTER ALEPH..SOGDIAN INDEPENDENT SHIN
            0x10F46, // Mn  [11] SOGDIAN COMBINING DOT BELOW..SOGDIAN COMBINING STROKE BELOW
            0x10F51,
            0x10FE0, // Lo  [23] ELYMAIC LETTER ALEPH..ELYMAIC LIGATURE ZAYIN-YODH
            0x10FF7,
            0x11000, // Mc       BRAHMI SIGN CANDRABINDU
            0x11001, // Mn       BRAHMI SIGN ANUSVARA
            0x11002, // Mc       BRAHMI SIGN VISARGA
            0x11003, // Lo  [53] BRAHMI SIGN JIHVAMULIYA..BRAHMI LETTER OLD TAMIL NNNA
            0x11038, // Mn  [15] BRAHMI VOWEL SIGN AA..BRAHMI VIRAMA
            0x11047,
            0x11066, // Nd  [10] BRAHMI DIGIT ZERO..BRAHMI DIGIT NINE
            0x11070,
            0x1107F, // Mn   [3] BRAHMI NUMBER JOINER..KAITHI SIGN ANUSVARA
            0x11082, // Mc       KAITHI SIGN VISARGA
            0x11083, // Lo  [45] KAITHI LETTER A..KAITHI LETTER HA
            0x110B0, // Mc   [3] KAITHI VOWEL SIGN AA..KAITHI VOWEL SIGN II
            0x110B3, // Mn   [4] KAITHI VOWEL SIGN U..KAITHI VOWEL SIGN AI
            0x110B7, // Mc   [2] KAITHI VOWEL SIGN O..KAITHI VOWEL SIGN AU
            0x110B9, // Mn   [2] KAITHI SIGN VIRAMA..KAITHI SIGN NUKTA
            0x110BB,
            0x110BD, // Cf       KAITHI NUMBER SIGN
            0x110BE,
            0x110CD, // Cf       KAITHI NUMBER SIGN ABOVE
            0x110CE,
            0x110D0, // Lo  [25] SORA SOMPENG LETTER SAH..SORA SOMPENG LETTER MAE
            0x110E9,
            0x110F0, // Nd  [10] SORA SOMPENG DIGIT ZERO..SORA SOMPENG DIGIT NINE
            0x110FA,
            0x11100, // Mn   [3] CHAKMA SIGN CANDRABINDU..CHAKMA SIGN VISARGA
            0x11103, // Lo  [36] CHAKMA LETTER AA..CHAKMA LETTER HAA
            0x11127, // Mn   [5] CHAKMA VOWEL SIGN A..CHAKMA VOWEL SIGN UU
            0x1112C, // Mc       CHAKMA VOWEL SIGN E
            0x1112D, // Mn   [8] CHAKMA VOWEL SIGN AI..CHAKMA MAAYYAA
            0x11135,
            0x11136, // Nd  [10] CHAKMA DIGIT ZERO..CHAKMA DIGIT NINE
            0x11140,
            0x11144, // Lo       CHAKMA LETTER LHAA
            0x11145, // Mc   [2] CHAKMA VOWEL SIGN AA..CHAKMA VOWEL SIGN EI
            0x11147,
            0x11150, // Lo  [35] MAHAJANI LETTER A..MAHAJANI LETTER RRA
            0x11173, // Mn       MAHAJANI SIGN NUKTA
            0x11174,
            0x11176, // Lo       MAHAJANI LIGATURE SHRI
            0x11177,
            0x11180, // Mn   [2] SHARADA SIGN CANDRABINDU..SHARADA SIGN ANUSVARA
            0x11182, // Mc       SHARADA SIGN VISARGA
            0x11183, // Lo  [48] SHARADA LETTER A..SHARADA LETTER HA
            0x111B3, // Mc   [3] SHARADA VOWEL SIGN AA..SHARADA VOWEL SIGN II
            0x111B6, // Mn   [9] SHARADA VOWEL SIGN U..SHARADA VOWEL SIGN O
            0x111BF, // Mc   [2] SHARADA VOWEL SIGN AU..SHARADA SIGN VIRAMA
            0x111C1, // Lo   [4] SHARADA SIGN AVAGRAHA..SHARADA OM
            0x111C5,
            0x111C9, // Mn   [4] SHARADA SANDHI MARK..SHARADA EXTRA SHORT VOWEL MARK
            0x111CD,
            0x111D0, // Nd  [10] SHARADA DIGIT ZERO..SHARADA DIGIT NINE
            0x111DA, // Lo       SHARADA EKAM
            0x111DB,
            0x111DC, // Lo       SHARADA HEADSTROKE
            0x111DD,
            0x11200, // Lo  [18] KHOJKI LETTER A..KHOJKI LETTER JJA
            0x11212,
            0x11213, // Lo  [25] KHOJKI LETTER NYA..KHOJKI LETTER LLA
            0x1122C, // Mc   [3] KHOJKI VOWEL SIGN AA..KHOJKI VOWEL SIGN II
            0x1122F, // Mn   [3] KHOJKI VOWEL SIGN U..KHOJKI VOWEL SIGN AI
            0x11232, // Mc   [2] KHOJKI VOWEL SIGN O..KHOJKI VOWEL SIGN AU
            0x11234, // Mn       KHOJKI SIGN ANUSVARA
            0x11235, // Mc       KHOJKI SIGN VIRAMA
            0x11236, // Mn   [2] KHOJKI SIGN NUKTA..KHOJKI SIGN SHADDA
            0x11238,
            0x1123E, // Mn       KHOJKI SIGN SUKUN
            0x1123F,
            0x11280, // Lo   [7] MULTANI LETTER A..MULTANI LETTER GA
            0x11287,
            0x11288, // Lo       MULTANI LETTER GHA
            0x11289,
            0x1128A, // Lo   [4] MULTANI LETTER CA..MULTANI LETTER JJA
            0x1128E,
            0x1128F, // Lo  [15] MULTANI LETTER NYA..MULTANI LETTER BA
            0x1129E,
            0x1129F, // Lo  [10] MULTANI LETTER BHA..MULTANI LETTER RHA
            0x112A9,
            0x112B0, // Lo  [47] KHUDAWADI LETTER A..KHUDAWADI LETTER HA
            0x112DF, // Mn       KHUDAWADI SIGN ANUSVARA
            0x112E0, // Mc   [3] KHUDAWADI VOWEL SIGN AA..KHUDAWADI VOWEL SIGN II
            0x112E3, // Mn   [8] KHUDAWADI VOWEL SIGN U..KHUDAWADI SIGN VIRAMA
            0x112EB,
            0x112F0, // Nd  [10] KHUDAWADI DIGIT ZERO..KHUDAWADI DIGIT NINE
            0x112FA,
            0x11300, // Mn   [2] GRANTHA SIGN COMBINING ANUSVARA ABOVE..GRANTHA SIGN CANDRABINDU
            0x11302, // Mc   [2] GRANTHA SIGN ANUSVARA..GRANTHA SIGN VISARGA
            0x11304,
            0x11305, // Lo   [8] GRANTHA LETTER A..GRANTHA LETTER VOCALIC L
            0x1130D,
            0x1130F, // Lo   [2] GRANTHA LETTER EE..GRANTHA LETTER AI
            0x11311,
            0x11313, // Lo  [22] GRANTHA LETTER OO..GRANTHA LETTER NA
            0x11329,
            0x1132A, // Lo   [7] GRANTHA LETTER PA..GRANTHA LETTER RA
            0x11331,
            0x11332, // Lo   [2] GRANTHA LETTER LA..GRANTHA LETTER LLA
            0x11334,
            0x11335, // Lo   [5] GRANTHA LETTER VA..GRANTHA LETTER HA
            0x1133A,
            0x1133B, // Mn   [2] COMBINING BINDU BELOW..GRANTHA SIGN NUKTA
            0x1133D, // Lo       GRANTHA SIGN AVAGRAHA
            0x1133E, // Mc   [2] GRANTHA VOWEL SIGN AA..GRANTHA VOWEL SIGN I
            0x11340, // Mn       GRANTHA VOWEL SIGN II
            0x11341, // Mc   [4] GRANTHA VOWEL SIGN U..GRANTHA VOWEL SIGN VOCALIC RR
            0x11345,
            0x11347, // Mc   [2] GRANTHA VOWEL SIGN EE..GRANTHA VOWEL SIGN AI
            0x11349,
            0x1134B, // Mc   [3] GRANTHA VOWEL SIGN OO..GRANTHA SIGN VIRAMA
            0x1134E,
            0x11350, // Lo       GRANTHA OM
            0x11351,
            0x11357, // Mc       GRANTHA AU LENGTH MARK
            0x11358,
            0x1135D, // Lo   [5] GRANTHA SIGN PLUTA..GRANTHA LETTER VOCALIC LL
            0x11362, // Mc   [2] GRANTHA VOWEL SIGN VOCALIC L..GRANTHA VOWEL SIGN VOCALIC LL
            0x11364,
            0x11366, // Mn   [7] COMBINING GRANTHA DIGIT ZERO..COMBINING GRANTHA DIGIT SIX
            0x1136D,
            0x11370, // Mn   [5] COMBINING GRANTHA LETTER A..COMBINING GRANTHA LETTER PA
            0x11375,
            0x11400, // Lo  [53] NEWA LETTER A..NEWA LETTER HA
            0x11435, // Mc   [3] NEWA VOWEL SIGN AA..NEWA VOWEL SIGN II
            0x11438, // Mn   [8] NEWA VOWEL SIGN U..NEWA VOWEL SIGN AI
            0x11440, // Mc   [2] NEWA VOWEL SIGN O..NEWA VOWEL SIGN AU
            0x11442, // Mn   [3] NEWA SIGN VIRAMA..NEWA SIGN ANUSVARA
            0x11445, // Mc       NEWA SIGN VISARGA
            0x11446, // Mn       NEWA SIGN NUKTA
            0x11447, // Lo   [4] NEWA SIGN AVAGRAHA..NEWA SIDDHI
            0x1144B,
            0x11450, // Nd  [10] NEWA DIGIT ZERO..NEWA DIGIT NINE
            0x1145A,
            0x1145E, // Mn       NEWA SANDHI MARK
            0x1145F, // Lo       NEWA LETTER VEDIC ANUSVARA
            0x11460,
            0x11480, // Lo  [48] TIRHUTA ANJI..TIRHUTA LETTER HA
            0x114B0, // Mc   [3] TIRHUTA VOWEL SIGN AA..TIRHUTA VOWEL SIGN II
            0x114B3, // Mn   [6] TIRHUTA VOWEL SIGN U..TIRHUTA VOWEL SIGN VOCALIC LL
            0x114B9, // Mc       TIRHUTA VOWEL SIGN E
            0x114BA, // Mn       TIRHUTA VOWEL SIGN SHORT E
            0x114BB, // Mc   [4] TIRHUTA VOWEL SIGN AI..TIRHUTA VOWEL SIGN AU
            0x114BF, // Mn   [2] TIRHUTA SIGN CANDRABINDU..TIRHUTA SIGN ANUSVARA
            0x114C1, // Mc       TIRHUTA SIGN VISARGA
            0x114C2, // Mn   [2] TIRHUTA SIGN VIRAMA..TIRHUTA SIGN NUKTA
            0x114C4, // Lo   [2] TIRHUTA SIGN AVAGRAHA..TIRHUTA GVANG
            0x114C6,
            0x114C7, // Lo       TIRHUTA OM
            0x114C8,
            0x114D0, // Nd  [10] TIRHUTA DIGIT ZERO..TIRHUTA DIGIT NINE
            0x114DA,
            0x11580, // Lo  [47] SIDDHAM LETTER A..SIDDHAM LETTER HA
            0x115AF, // Mc   [3] SIDDHAM VOWEL SIGN AA..SIDDHAM VOWEL SIGN II
            0x115B2, // Mn   [4] SIDDHAM VOWEL SIGN U..SIDDHAM VOWEL SIGN VOCALIC RR
            0x115B6,
            0x115B8, // Mc   [4] SIDDHAM VOWEL SIGN E..SIDDHAM VOWEL SIGN AU
            0x115BC, // Mn   [2] SIDDHAM SIGN CANDRABINDU..SIDDHAM SIGN ANUSVARA
            0x115BE, // Mc       SIDDHAM SIGN VISARGA
            0x115BF, // Mn   [2] SIDDHAM SIGN VIRAMA..SIDDHAM SIGN NUKTA
            0x115C1,
            0x115D8, // Lo   [4] SIDDHAM LETTER THREE-CIRCLE ALTERNATE I..SIDDHAM LETTER ALTERNATE U
            0x115DC, // Mn   [2] SIDDHAM VOWEL SIGN ALTERNATE U..SIDDHAM VOWEL SIGN ALTERNATE UU
            0x115DE,
            0x11600, // Lo  [48] MODI LETTER A..MODI LETTER LLA
            0x11630, // Mc   [3] MODI VOWEL SIGN AA..MODI VOWEL SIGN II
            0x11633, // Mn   [8] MODI VOWEL SIGN U..MODI VOWEL SIGN AI
            0x1163B, // Mc   [2] MODI VOWEL SIGN O..MODI VOWEL SIGN AU
            0x1163D, // Mn       MODI SIGN ANUSVARA
            0x1163E, // Mc       MODI SIGN VISARGA
            0x1163F, // Mn   [2] MODI SIGN VIRAMA..MODI SIGN ARDHACANDRA
            0x11641,
            0x11644, // Lo       MODI SIGN HUVA
            0x11645,
            0x11650, // Nd  [10] MODI DIGIT ZERO..MODI DIGIT NINE
            0x1165A,
            0x11680, // Lo  [43] TAKRI LETTER A..TAKRI LETTER RRA
            0x116AB, // Mn       TAKRI SIGN ANUSVARA
            0x116AC, // Mc       TAKRI SIGN VISARGA
            0x116AD, // Mn       TAKRI VOWEL SIGN AA
            0x116AE, // Mc   [2] TAKRI VOWEL SIGN I..TAKRI VOWEL SIGN II
            0x116B0, // Mn   [6] TAKRI VOWEL SIGN U..TAKRI VOWEL SIGN AU
            0x116B6, // Mc       TAKRI SIGN VIRAMA
            0x116B7, // Mn       TAKRI SIGN NUKTA
            0x116B8, // Lo       TAKRI LETTER ARCHAIC KHA
            0x116B9,
            0x116C0, // Nd  [10] TAKRI DIGIT ZERO..TAKRI DIGIT NINE
            0x116CA,
            0x1171D, // Mn   [3] AHOM CONSONANT SIGN MEDIAL LA..AHOM CONSONANT SIGN MEDIAL LIGATING RA
            0x11720, // Mc   [2] AHOM VOWEL SIGN A..AHOM VOWEL SIGN AA
            0x11722, // Mn   [4] AHOM VOWEL SIGN I..AHOM VOWEL SIGN UU
            0x11726, // Mc       AHOM VOWEL SIGN E
            0x11727, // Mn   [5] AHOM VOWEL SIGN AW..AHOM SIGN KILLER
            0x1172C,
            0x11730, // Nd  [10] AHOM DIGIT ZERO..AHOM DIGIT NINE
            0x1173A,
            0x11800, // Lo  [44] DOGRA LETTER A..DOGRA LETTER RRA
            0x1182C, // Mc   [3] DOGRA VOWEL SIGN AA..DOGRA VOWEL SIGN II
            0x1182F, // Mn   [9] DOGRA VOWEL SIGN U..DOGRA SIGN ANUSVARA
            0x11838, // Mc       DOGRA SIGN VISARGA
            0x11839, // Mn   [2] DOGRA SIGN VIRAMA..DOGRA SIGN NUKTA
            0x1183B,
            0x118A0, // L&  [64] WARANG CITI CAPITAL LETTER NGAA..WARANG CITI SMALL LETTER VIYO
            0x118E0, // Nd  [10] WARANG CITI DIGIT ZERO..WARANG CITI DIGIT NINE
            0x118EA,
            0x118FF, // Lo       WARANG CITI OM
            0x11900,
            0x119A0, // Lo   [8] NANDINAGARI LETTER A..NANDINAGARI LETTER VOCALIC RR
            0x119A8,
            0x119AA, // Lo  [39] NANDINAGARI LETTER E..NANDINAGARI LETTER RRA
            0x119D1, // Mc   [3] NANDINAGARI VOWEL SIGN AA..NANDINAGARI VOWEL SIGN II
            0x119D4, // Mn   [4] NANDINAGARI VOWEL SIGN U..NANDINAGARI VOWEL SIGN VOCALIC RR
            0x119D8,
            0x119DA, // Mn   [2] NANDINAGARI VOWEL SIGN E..NANDINAGARI VOWEL SIGN AI
            0x119DC, // Mc   [4] NANDINAGARI VOWEL SIGN O..NANDINAGARI SIGN VISARGA
            0x119E0, // Mn       NANDINAGARI SIGN VIRAMA
            0x119E1, // Lo       NANDINAGARI SIGN AVAGRAHA
            0x119E2,
            0x119E3, // Lo       NANDINAGARI HEADSTROKE
            0x119E4, // Mc       NANDINAGARI VOWEL SIGN PRISHTHAMATRA E
            0x119E5,
            0x11A00, // Lo       ZANABAZAR SQUARE LETTER A
            0x11A01, // Mn  [10] ZANABAZAR SQUARE VOWEL SIGN I..ZANABAZAR SQUARE VOWEL LENGTH MARK
            0x11A0B, // Lo  [40] ZANABAZAR SQUARE LETTER KA..ZANABAZAR SQUARE LETTER KSSA
            0x11A33, // Mn   [6] ZANABAZAR SQUARE FINAL CONSONANT MARK..ZANABAZAR SQUARE SIGN ANUSVARA
            0x11A39, // Mc       ZANABAZAR SQUARE SIGN VISARGA
            0x11A3A, // Lo       ZANABAZAR SQUARE CLUSTER-INITIAL LETTER RA
            0x11A3B, // Mn   [4] ZANABAZAR SQUARE CLUSTER-FINAL LETTER YA..ZANABAZAR SQUARE CLUSTER-FINAL LETTER VA
            0x11A3F,
            0x11A47, // Mn       ZANABAZAR SQUARE SUBJOINER
            0x11A48,
            0x11A50, // Lo       SOYOMBO LETTER A
            0x11A51, // Mn   [6] SOYOMBO VOWEL SIGN I..SOYOMBO VOWEL SIGN OE
            0x11A57, // Mc   [2] SOYOMBO VOWEL SIGN AI..SOYOMBO VOWEL SIGN AU
            0x11A59, // Mn   [3] SOYOMBO VOWEL SIGN VOCALIC R..SOYOMBO VOWEL LENGTH MARK
            0x11A5C, // Lo  [46] SOYOMBO LETTER KA..SOYOMBO CLUSTER-INITIAL LETTER SA
            0x11A8A, // Mn  [13] SOYOMBO FINAL CONSONANT SIGN G..SOYOMBO SIGN ANUSVARA
            0x11A97, // Mc       SOYOMBO SIGN VISARGA
            0x11A98, // Mn   [2] SOYOMBO GEMINATION MARK..SOYOMBO SUBJOINER
            0x11A9A,
            0x11A9D, // Lo       SOYOMBO MARK PLUTA
            0x11A9E,
            0x11AC0, // Lo  [57] PAU CIN HAU LETTER PA..PAU CIN HAU GLOTTAL STOP FINAL
            0x11AF9,
            0x11C00, // Lo   [9] BHAIKSUKI LETTER A..BHAIKSUKI LETTER VOCALIC L
            0x11C09,
            0x11C0A, // Lo  [37] BHAIKSUKI LETTER E..BHAIKSUKI LETTER HA
            0x11C2F, // Mc       BHAIKSUKI VOWEL SIGN AA
            0x11C30, // Mn   [7] BHAIKSUKI VOWEL SIGN I..BHAIKSUKI VOWEL SIGN VOCALIC L
            0x11C37,
            0x11C38, // Mn   [6] BHAIKSUKI VOWEL SIGN E..BHAIKSUKI SIGN ANUSVARA
            0x11C3E, // Mc       BHAIKSUKI SIGN VISARGA
            0x11C3F, // Mn       BHAIKSUKI SIGN VIRAMA
            0x11C40, // Lo       BHAIKSUKI SIGN AVAGRAHA
            0x11C41,
            0x11C50, // Nd  [10] BHAIKSUKI DIGIT ZERO..BHAIKSUKI DIGIT NINE
            0x11C5A,
            0x11C72, // Lo  [30] MARCHEN LETTER KA..MARCHEN LETTER A
            0x11C90,
            0x11C92, // Mn  [22] MARCHEN SUBJOINED LETTER KA..MARCHEN SUBJOINED LETTER ZA
            0x11CA8,
            0x11CA9, // Mc       MARCHEN SUBJOINED LETTER YA
            0x11CAA, // Mn   [7] MARCHEN SUBJOINED LETTER RA..MARCHEN VOWEL SIGN AA
            0x11CB1, // Mc       MARCHEN VOWEL SIGN I
            0x11CB2, // Mn   [2] MARCHEN VOWEL SIGN U..MARCHEN VOWEL SIGN E
            0x11CB4, // Mc       MARCHEN VOWEL SIGN O
            0x11CB5, // Mn   [2] MARCHEN SIGN ANUSVARA..MARCHEN SIGN CANDRABINDU
            0x11CB7,
            0x11D00, // Lo   [7] MASARAM GONDI LETTER A..MASARAM GONDI LETTER E
            0x11D07,
            0x11D08, // Lo   [2] MASARAM GONDI LETTER AI..MASARAM GONDI LETTER O
            0x11D0A,
            0x11D0B, // Lo  [38] MASARAM GONDI LETTER AU..MASARAM GONDI LETTER TRA
            0x11D31, // Mn   [6] MASARAM GONDI VOWEL SIGN AA..MASARAM GONDI VOWEL SIGN VOCALIC R
            0x11D37,
            0x11D3A, // Mn       MASARAM GONDI VOWEL SIGN E
            0x11D3B,
            0x11D3C, // Mn   [2] MASARAM GONDI VOWEL SIGN AI..MASARAM GONDI VOWEL SIGN O
            0x11D3E,
            0x11D3F, // Mn   [7] MASARAM GONDI VOWEL SIGN AU..MASARAM GONDI VIRAMA
            0x11D46, // Lo       MASARAM GONDI REPHA
            0x11D47, // Mn       MASARAM GONDI RA-KARA
            0x11D48,
            0x11D50, // Nd  [10] MASARAM GONDI DIGIT ZERO..MASARAM GONDI DIGIT NINE
            0x11D5A,
            0x11D60, // Lo   [6] GUNJALA GONDI LETTER A..GUNJALA GONDI LETTER UU
            0x11D66,
            0x11D67, // Lo   [2] GUNJALA GONDI LETTER EE..GUNJALA GONDI LETTER AI
            0x11D69,
            0x11D6A, // Lo  [32] GUNJALA GONDI LETTER OO..GUNJALA GONDI LETTER SA
            0x11D8A, // Mc   [5] GUNJALA GONDI VOWEL SIGN AA..GUNJALA GONDI VOWEL SIGN UU
            0x11D8F,
            0x11D90, // Mn   [2] GUNJALA GONDI VOWEL SIGN EE..GUNJALA GONDI VOWEL SIGN AI
            0x11D92,
            0x11D93, // Mc   [2] GUNJALA GONDI VOWEL SIGN OO..GUNJALA GONDI VOWEL SIGN AU
            0x11D95, // Mn       GUNJALA GONDI SIGN ANUSVARA
            0x11D96, // Mc       GUNJALA GONDI SIGN VISARGA
            0x11D97, // Mn       GUNJALA GONDI VIRAMA
            0x11D98, // Lo       GUNJALA GONDI OM
            0x11D99,
            0x11DA0, // Nd  [10] GUNJALA GONDI DIGIT ZERO..GUNJALA GONDI DIGIT NINE
            0x11DAA,
            0x11EE0, // Lo  [19] MAKASAR LETTER KA..MAKASAR ANGKA
            0x11EF3, // Mn   [2] MAKASAR VOWEL SIGN I..MAKASAR VOWEL SIGN U
            0x11EF5, // Mc   [2] MAKASAR VOWEL SIGN E..MAKASAR VOWEL SIGN O
            0x11EF7,
            0x12000, // Lo [922] CUNEIFORM SIGN A..CUNEIFORM SIGN U U
            0x1239A,
            0x12400, // Nl [111] CUNEIFORM NUMERIC SIGN TWO ASH..CUNEIFORM NUMERIC SIGN NINE U VARIANT FORM
            0x1246F,
            0x12480, // Lo [196] CUNEIFORM SIGN AB TIMES NUN TENU..CUNEIFORM SIGN ZU5 TIMES THREE DISH TENU
            0x12544,
            0x13000, // Lo [1071] EGYPTIAN HIEROGLYPH A001..EGYPTIAN HIEROGLYPH AA032
            0x1342F,
            0x13430, // Cf   [9] EGYPTIAN HIEROGLYPH VERTICAL JOINER..EGYPTIAN HIEROGLYPH END SEGMENT
            0x13439,
            0x14400, // Lo [583] ANATOLIAN HIEROGLYPH A001..ANATOLIAN HIEROGLYPH A530
            0x14647,
            0x16800, // Lo [569] BAMUM LETTER PHASE-A NGKUE MFON..BAMUM LETTER PHASE-F VUEQ
            0x16A39,
            0x16A40, // Lo  [31] MRO LETTER TA..MRO LETTER TEK
            0x16A5F,
            0x16A60, // Nd  [10] MRO DIGIT ZERO..MRO DIGIT NINE
            0x16A6A,
            0x16AD0, // Lo  [30] BASSA VAH LETTER ENNI..BASSA VAH LETTER I
            0x16AEE,
            0x16AF0, // Mn   [5] BASSA VAH COMBINING HIGH TONE..BASSA VAH COMBINING HIGH-LOW TONE
            0x16AF5,
            0x16B00, // Lo  [48] PAHAWH HMONG VOWEL KEEB..PAHAWH HMONG CONSONANT CAU
            0x16B30, // Mn   [7] PAHAWH HMONG MARK CIM TUB..PAHAWH HMONG MARK CIM TAUM
            0x16B37,
            0x16B40, // Lm   [4] PAHAWH HMONG SIGN VOS SEEV..PAHAWH HMONG SIGN IB YAM
            0x16B44,
            0x16B50, // Nd  [10] PAHAWH HMONG DIGIT ZERO..PAHAWH HMONG DIGIT NINE
            0x16B5A,
            0x16B63, // Lo  [21] PAHAWH HMONG SIGN VOS LUB..PAHAWH HMONG SIGN CIM NRES TOS
            0x16B78,
            0x16B7D, // Lo  [19] PAHAWH HMONG CLAN SIGN TSHEEJ..PAHAWH HMONG CLAN SIGN VWJ
            0x16B90,
            0x16E40, // L&  [64] MEDEFAIDRIN CAPITAL LETTER M..MEDEFAIDRIN SMALL LETTER Y
            0x16E80,
            0x16F00, // Lo  [75] MIAO LETTER PA..MIAO LETTER RTE
            0x16F4B,
            0x16F4F, // Mn       MIAO SIGN CONSONANT MODIFIER BAR
            0x16F50, // Lo       MIAO LETTER NASALIZATION
            0x16F51, // Mc  [55] MIAO SIGN ASPIRATION..MIAO VOWEL SIGN UI
            0x16F88,
            0x16F8F, // Mn   [4] MIAO TONE RIGHT..MIAO TONE BELOW
            0x16F93, // Lm  [13] MIAO LETTER TONE-2..MIAO LETTER REFORMED TONE-8
            0x16FA0,
            0x16FE0, // Lm   [2] TANGUT ITERATION MARK..NUSHU ITERATION MARK
            0x16FE2,
            0x16FE3, // Lm       OLD CHINESE ITERATION MARK
            0x16FE4,
            0x1B000, // Lo       KATAKANA LETTER ARCHAIC E
            0x1B001,
            0x1B164, // Lo   [4] KATAKANA LETTER SMALL WI..KATAKANA LETTER SMALL N
            0x1B168,
            0x1BC00, // Lo [107] DUPLOYAN LETTER H..DUPLOYAN LETTER VOCALIC M
            0x1BC6B,
            0x1BC70, // Lo  [13] DUPLOYAN AFFIX LEFT HORIZONTAL SECANT..DUPLOYAN AFFIX ATTACHED TANGENT HOOK
            0x1BC7D,
            0x1BC80, // Lo   [9] DUPLOYAN AFFIX HIGH ACUTE..DUPLOYAN AFFIX HIGH VERTICAL
            0x1BC89,
            0x1BC90, // Lo  [10] DUPLOYAN AFFIX LOW ACUTE..DUPLOYAN AFFIX LOW ARROW
            0x1BC9A,
            0x1BC9D, // Mn   [2] DUPLOYAN THICK LETTER SELECTOR..DUPLOYAN DOUBLE MARK
            0x1BC9F,
            0x1BCA0, // Cf   [4] SHORTHAND FORMAT LETTER OVERLAP..SHORTHAND FORMAT UP STEP
            0x1BCA4,
            0x1D165, // Mc   [2] MUSICAL SYMBOL COMBINING STEM..MUSICAL SYMBOL COMBINING SPRECHGESANG STEM
            0x1D167, // Mn   [3] MUSICAL SYMBOL COMBINING TREMOLO-1..MUSICAL SYMBOL COMBINING TREMOLO-3
            0x1D16A,
            0x1D16D, // Mc   [6] MUSICAL SYMBOL COMBINING AUGMENTATION DOT..MUSICAL SYMBOL COMBINING FLAG-5
            0x1D173, // Cf   [8] MUSICAL SYMBOL BEGIN BEAM..MUSICAL SYMBOL END PHRASE
            0x1D17B, // Mn   [8] MUSICAL SYMBOL COMBINING ACCENT..MUSICAL SYMBOL COMBINING LOURE
            0x1D183,
            0x1D185, // Mn   [7] MUSICAL SYMBOL COMBINING DOIT..MUSICAL SYMBOL COMBINING TRIPLE TONGUE
            0x1D18C,
            0x1D1AA, // Mn   [4] MUSICAL SYMBOL COMBINING DOWN BOW..MUSICAL SYMBOL COMBINING SNAP PIZZICATO
            0x1D1AE,
            0x1D242, // Mn   [3] COMBINING GREEK MUSICAL TRISEME..COMBINING GREEK MUSICAL PENTASEME
            0x1D245,
            0x1D400, // L&  [85] MATHEMATICAL BOLD CAPITAL A..MATHEMATICAL ITALIC SMALL G
            0x1D455,
            0x1D456, // L&  [71] MATHEMATICAL ITALIC SMALL I..MATHEMATICAL SCRIPT CAPITAL A
            0x1D49D,
            0x1D49E, // L&   [2] MATHEMATICAL SCRIPT CAPITAL C..MATHEMATICAL SCRIPT CAPITAL D
            0x1D4A0,
            0x1D4A2, // L&       MATHEMATICAL SCRIPT CAPITAL G
            0x1D4A3,
            0x1D4A5, // L&   [2] MATHEMATICAL SCRIPT CAPITAL J..MATHEMATICAL SCRIPT CAPITAL K
            0x1D4A7,
            0x1D4A9, // L&   [4] MATHEMATICAL SCRIPT CAPITAL N..MATHEMATICAL SCRIPT CAPITAL Q
            0x1D4AD,
            0x1D4AE, // L&  [12] MATHEMATICAL SCRIPT CAPITAL S..MATHEMATICAL SCRIPT SMALL D
            0x1D4BA,
            0x1D4BB, // L&       MATHEMATICAL SCRIPT SMALL F
            0x1D4BC,
            0x1D4BD, // L&   [7] MATHEMATICAL SCRIPT SMALL H..MATHEMATICAL SCRIPT SMALL N
            0x1D4C4,
            0x1D4C5, // L&  [65] MATHEMATICAL SCRIPT SMALL P..MATHEMATICAL FRAKTUR CAPITAL B
            0x1D506,
            0x1D507, // L&   [4] MATHEMATICAL FRAKTUR CAPITAL D..MATHEMATICAL FRAKTUR CAPITAL G
            0x1D50B,
            0x1D50D, // L&   [8] MATHEMATICAL FRAKTUR CAPITAL J..MATHEMATICAL FRAKTUR CAPITAL Q
            0x1D515,
            0x1D516, // L&   [7] MATHEMATICAL FRAKTUR CAPITAL S..MATHEMATICAL FRAKTUR CAPITAL Y
            0x1D51D,
            0x1D51E, // L&  [28] MATHEMATICAL FRAKTUR SMALL A..MATHEMATICAL DOUBLE-STRUCK CAPITAL B
            0x1D53A,
            0x1D53B, // L&   [4] MATHEMATICAL DOUBLE-STRUCK CAPITAL D..MATHEMATICAL DOUBLE-STRUCK CAPITAL G
            0x1D53F,
            0x1D540, // L&   [5] MATHEMATICAL DOUBLE-STRUCK CAPITAL I..MATHEMATICAL DOUBLE-STRUCK CAPITAL M
            0x1D545,
            0x1D546, // L&       MATHEMATICAL DOUBLE-STRUCK CAPITAL O
            0x1D547,
            0x1D54A, // L&   [7] MATHEMATICAL DOUBLE-STRUCK CAPITAL S..MATHEMATICAL DOUBLE-STRUCK CAPITAL Y
            0x1D551,
            0x1D552, // L& [340] MATHEMATICAL DOUBLE-STRUCK SMALL A..MATHEMATICAL ITALIC SMALL DOTLESS J
            0x1D6A6,
            0x1D6A8, // L&  [25] MATHEMATICAL BOLD CAPITAL ALPHA..MATHEMATICAL BOLD CAPITAL OMEGA
            0x1D6C1,
            0x1D6C2, // L&  [25] MATHEMATICAL BOLD SMALL ALPHA..MATHEMATICAL BOLD SMALL OMEGA
            0x1D6DB,
            0x1D6DC, // L&  [31] MATHEMATICAL BOLD EPSILON SYMBOL..MATHEMATICAL ITALIC CAPITAL OMEGA
            0x1D6FB,
            0x1D6FC, // L&  [25] MATHEMATICAL ITALIC SMALL ALPHA..MATHEMATICAL ITALIC SMALL OMEGA
            0x1D715,
            0x1D716, // L&  [31] MATHEMATICAL ITALIC EPSILON SYMBOL..MATHEMATICAL BOLD ITALIC CAPITAL OMEGA
            0x1D735,
            0x1D736, // L&  [25] MATHEMATICAL BOLD ITALIC SMALL ALPHA..MATHEMATICAL BOLD ITALIC SMALL OMEGA
            0x1D74F,
            0x1D750, // L&  [31] MATHEMATICAL BOLD ITALIC EPSILON SYMBOL..MATHEMATICAL SANS-SERIF BOLD CAPITAL OMEGA
            0x1D76F,
            0x1D770, // L&  [25] MATHEMATICAL SANS-SERIF BOLD SMALL ALPHA..MATHEMATICAL SANS-SERIF BOLD SMALL OMEGA
            0x1D789,
            0x1D78A, // L&  [31] MATHEMATICAL SANS-SERIF BOLD EPSILON SYMBOL..MATHEMATICAL SANS-SERIF BOLD ITALIC CAPITAL OMEGA
            0x1D7A9,
            0x1D7AA, // L&  [25] MATHEMATICAL SANS-SERIF BOLD ITALIC SMALL ALPHA..MATHEMATICAL SANS-SERIF BOLD ITALIC SMALL OMEGA
            0x1D7C3,
            0x1D7C4, // L&   [8] MATHEMATICAL SANS-SERIF BOLD ITALIC EPSILON SYMBOL..MATHEMATICAL BOLD SMALL DIGAMMA
            0x1D7CC,
            0x1D7CE, // Nd  [50] MATHEMATICAL BOLD DIGIT ZERO..MATHEMATICAL MONOSPACE DIGIT NINE
            0x1D800,
            0x1DA00, // Mn  [55] SIGNWRITING HEAD RIM..SIGNWRITING AIR SUCKING IN
            0x1DA37,
            0x1DA3B, // Mn  [50] SIGNWRITING MOUTH CLOSED NEUTRAL..SIGNWRITING EXCITEMENT
            0x1DA6D,
            0x1DA75, // Mn       SIGNWRITING UPPER BODY TILTING FROM HIP JOINTS
            0x1DA76,
            0x1DA84, // Mn       SIGNWRITING LOCATION HEAD NECK
            0x1DA85,
            0x1DA9B, // Mn   [5] SIGNWRITING FILL MODIFIER-2..SIGNWRITING FILL MODIFIER-6
            0x1DAA0,
            0x1DAA1, // Mn  [15] SIGNWRITING ROTATION MODIFIER-2..SIGNWRITING ROTATION MODIFIER-16
            0x1DAB0,
            0x1E000, // Mn   [7] COMBINING GLAGOLITIC LETTER AZU..COMBINING GLAGOLITIC LETTER ZHIVETE
            0x1E007,
            0x1E008, // Mn  [17] COMBINING GLAGOLITIC LETTER ZEMLJA..COMBINING GLAGOLITIC LETTER HERU
            0x1E019,
            0x1E01B, // Mn   [7] COMBINING GLAGOLITIC LETTER SHTA..COMBINING GLAGOLITIC LETTER YATI
            0x1E022,
            0x1E023, // Mn   [2] COMBINING GLAGOLITIC LETTER YU..COMBINING GLAGOLITIC LETTER SMALL YUS
            0x1E025,
            0x1E026, // Mn   [5] COMBINING GLAGOLITIC LETTER YO..COMBINING GLAGOLITIC LETTER FITA
            0x1E02B,
            0x1E100, // Lo  [45] NYIAKENG PUACHUE HMONG LETTER MA..NYIAKENG PUACHUE HMONG LETTER W
            0x1E12D,
            0x1E130, // Mn   [7] NYIAKENG PUACHUE HMONG TONE-B..NYIAKENG PUACHUE HMONG TONE-D
            0x1E137, // Lm   [7] NYIAKENG PUACHUE HMONG SIGN FOR PERSON..NYIAKENG PUACHUE HMONG SYLLABLE LENGTHENER
            0x1E13E,
            0x1E140, // Nd  [10] NYIAKENG PUACHUE HMONG DIGIT ZERO..NYIAKENG PUACHUE HMONG DIGIT NINE
            0x1E14A,
            0x1E14E, // Lo       NYIAKENG PUACHUE HMONG LOGOGRAM NYAJ
            0x1E14F,
            0x1E2C0, // Lo  [44] WANCHO LETTER AA..WANCHO LETTER YIH
            0x1E2EC, // Mn   [4] WANCHO TONE TUP..WANCHO TONE KOINI
            0x1E2F0, // Nd  [10] WANCHO DIGIT ZERO..WANCHO DIGIT NINE
            0x1E2FA,
            0x1E800, // Lo [197] MENDE KIKAKUI SYLLABLE M001 KI..MENDE KIKAKUI SYLLABLE M060 NYON
            0x1E8C5,
            0x1E8D0, // Mn   [7] MENDE KIKAKUI COMBINING NUMBER TEENS..MENDE KIKAKUI COMBINING NUMBER MILLIONS
            0x1E8D7,
            0x1E900, // L&  [68] ADLAM CAPITAL LETTER ALIF..ADLAM SMALL LETTER SHA
            0x1E944, // Mn   [7] ADLAM ALIF LENGTHENER..ADLAM NUKTA
            0x1E94B, // Lm       ADLAM NASALIZATION MARK
            0x1E94C,
            0x1E950, // Nd  [10] ADLAM DIGIT ZERO..ADLAM DIGIT NINE
            0x1E95A,
            0x1EE00, // Lo   [4] ARABIC MATHEMATICAL ALEF..ARABIC MATHEMATICAL DAL
            0x1EE04,
            0x1EE05, // Lo  [27] ARABIC MATHEMATICAL WAW..ARABIC MATHEMATICAL DOTLESS QAF
            0x1EE20,
            0x1EE21, // Lo   [2] ARABIC MATHEMATICAL INITIAL BEH..ARABIC MATHEMATICAL INITIAL JEEM
            0x1EE23,
            0x1EE24, // Lo       ARABIC MATHEMATICAL INITIAL HEH
            0x1EE25,
            0x1EE27, // Lo       ARABIC MATHEMATICAL INITIAL HAH
            0x1EE28,
            0x1EE29, // Lo  [10] ARABIC MATHEMATICAL INITIAL YEH..ARABIC MATHEMATICAL INITIAL QAF
            0x1EE33,
            0x1EE34, // Lo   [4] ARABIC MATHEMATICAL INITIAL SHEEN..ARABIC MATHEMATICAL INITIAL KHAH
            0x1EE38,
            0x1EE39, // Lo       ARABIC MATHEMATICAL INITIAL DAD
            0x1EE3A,
            0x1EE3B, // Lo       ARABIC MATHEMATICAL INITIAL GHAIN
            0x1EE3C,
            0x1EE42, // Lo       ARABIC MATHEMATICAL TAILED JEEM
            0x1EE43,
            0x1EE47, // Lo       ARABIC MATHEMATICAL TAILED HAH
            0x1EE48,
            0x1EE49, // Lo       ARABIC MATHEMATICAL TAILED YEH
            0x1EE4A,
            0x1EE4B, // Lo       ARABIC MATHEMATICAL TAILED LAM
            0x1EE4C,
            0x1EE4D, // Lo   [3] ARABIC MATHEMATICAL TAILED NOON..ARABIC MATHEMATICAL TAILED AIN
            0x1EE50,
            0x1EE51, // Lo   [2] ARABIC MATHEMATICAL TAILED SAD..ARABIC MATHEMATICAL TAILED QAF
            0x1EE53,
            0x1EE54, // Lo       ARABIC MATHEMATICAL TAILED SHEEN
            0x1EE55,
            0x1EE57, // Lo       ARABIC MATHEMATICAL TAILED KHAH
            0x1EE58,
            0x1EE59, // Lo       ARABIC MATHEMATICAL TAILED DAD
            0x1EE5A,
            0x1EE5B, // Lo       ARABIC MATHEMATICAL TAILED GHAIN
            0x1EE5C,
            0x1EE5D, // Lo       ARABIC MATHEMATICAL TAILED DOTLESS NOON
            0x1EE5E,
            0x1EE5F, // Lo       ARABIC MATHEMATICAL TAILED DOTLESS QAF
            0x1EE60,
            0x1EE61, // Lo   [2] ARABIC MATHEMATICAL STRETCHED BEH..ARABIC MATHEMATICAL STRETCHED JEEM
            0x1EE63,
            0x1EE64, // Lo       ARABIC MATHEMATICAL STRETCHED HEH
            0x1EE65,
            0x1EE67, // Lo   [4] ARABIC MATHEMATICAL STRETCHED HAH..ARABIC MATHEMATICAL STRETCHED KAF
            0x1EE6B,
            0x1EE6C, // Lo   [7] ARABIC MATHEMATICAL STRETCHED MEEM..ARABIC MATHEMATICAL STRETCHED QAF
            0x1EE73,
            0x1EE74, // Lo   [4] ARABIC MATHEMATICAL STRETCHED SHEEN..ARABIC MATHEMATICAL STRETCHED KHAH
            0x1EE78,
            0x1EE79, // Lo   [4] ARABIC MATHEMATICAL STRETCHED DAD..ARABIC MATHEMATICAL STRETCHED DOTLESS BEH
            0x1EE7D,
            0x1EE7E, // Lo       ARABIC MATHEMATICAL STRETCHED DOTLESS FEH
            0x1EE7F,
            0x1EE80, // Lo  [10] ARABIC MATHEMATICAL LOOPED ALEF..ARABIC MATHEMATICAL LOOPED YEH
            0x1EE8A,
            0x1EE8B, // Lo  [17] ARABIC MATHEMATICAL LOOPED LAM..ARABIC MATHEMATICAL LOOPED GHAIN
            0x1EE9C,
            0x1EEA1, // Lo   [3] ARABIC MATHEMATICAL DOUBLE-STRUCK BEH..ARABIC MATHEMATICAL DOUBLE-STRUCK DAL
            0x1EEA4,
            0x1EEA5, // Lo   [5] ARABIC MATHEMATICAL DOUBLE-STRUCK WAW..ARABIC MATHEMATICAL DOUBLE-STRUCK YEH
            0x1EEAA,
            0x1EEAB, // Lo  [17] ARABIC MATHEMATICAL DOUBLE-STRUCK LAM..ARABIC MATHEMATICAL DOUBLE-STRUCK GHAIN
            0x1EEBC,
            0x1F130, // So  [26] SQUARED LATIN CAPITAL LETTER A..SQUARED LATIN CAPITAL LETTER Z
            0x1F14A,
            0x1F150, // So  [26] NEGATIVE CIRCLED LATIN CAPITAL LETTER A..NEGATIVE CIRCLED LATIN CAPITAL LETTER Z
            0x1F16A,
            0x1F170, // So  [26] NEGATIVE SQUARED LATIN CAPITAL LETTER A..NEGATIVE SQUARED LATIN CAPITAL LETTER Z
            0x1F18A,
            0x1F1E6, // So  [26] REGIONAL INDICATOR SYMBOL LETTER A..REGIONAL INDICATOR SYMBOL LETTER Z
            0x1F200,
            0x1F3FB, // Sk   [5] EMOJI MODIFIER FITZPATRICK TYPE-1-2..EMOJI MODIFIER FITZPATRICK TYPE-6
            0x1F400,
            0xE0001, // Cf       LANGUAGE TAG
            0xE0002,
            0xE0020, // Cf  [96] TAG SPACE..CANCEL TAG
            0xE0080,
            0xE0100, // Mn [240] VARIATION SELECTOR-17..VARIATION SELECTOR-256
            0xE01F0,
        ];
        const VALUE_TABLE: [Word_Break; 1962] = [
            Other,
            Newline, // Cc       <control-0085>
            Other,
            ALetter, // Lo       FEMININE ORDINAL INDICATOR
            Other,
            Format, // Cf       SOFT HYPHEN
            Other,
            ALetter, // L&       MICRO SIGN
            Other,
            MidLetter, // Po       MIDDLE DOT
            Other,
            ALetter, // Lo       MASCULINE ORDINAL INDICATOR
            Other,
            ALetter, // L&  [23] LATIN CAPITAL LETTER A WITH GRAVE..LATIN CAPITAL LETTER O WITH DIAERESIS
            Other,
            ALetter, // L&  [31] LATIN CAPITAL LETTER O WITH STROKE..LATIN SMALL LETTER O WITH DIAERESIS
            Other,
            ALetter, // L& [195] LATIN SMALL LETTER O WITH STROKE..LATIN SMALL LETTER EZH WITH TAIL
            ALetter, // Lo       LATIN LETTER TWO WITH STROKE
            ALetter, // L&   [4] LATIN CAPITAL LETTER TONE FIVE..LATIN LETTER WYNN
            ALetter, // Lo   [4] LATIN LETTER DENTAL CLICK..LATIN LETTER RETROFLEX CLICK
            ALetter, // L& [208] LATIN CAPITAL LETTER DZ WITH CARON..LATIN SMALL LETTER EZH WITH CURL
            ALetter, // Lo       LATIN LETTER GLOTTAL STOP
            ALetter, // L&  [27] LATIN LETTER PHARYNGEAL VOICED FRICATIVE..LATIN SMALL LETTER TURNED H WITH FISHHOOK AND TAIL
            ALetter, // Lm  [18] MODIFIER LETTER SMALL H..MODIFIER LETTER REVERSED GLOTTAL STOP
            ALetter, // Sk   [4] MODIFIER LETTER LEFT ARROWHEAD..MODIFIER LETTER DOWN ARROWHEAD
            ALetter, // Lm  [12] MODIFIER LETTER CIRCUMFLEX ACCENT..MODIFIER LETTER HALF TRIANGULAR COLON
            ALetter, // Sk   [6] MODIFIER LETTER CENTRED RIGHT HALF RING..MODIFIER LETTER MINUS SIGN
            Other,
            ALetter, // Sk   [2] MODIFIER LETTER RHOTIC HOOK..MODIFIER LETTER CROSS ACCENT
            ALetter, // Lm   [5] MODIFIER LETTER SMALL GAMMA..MODIFIER LETTER SMALL REVERSED GLOTTAL STOP
            Other,
            ALetter, // Lm       MODIFIER LETTER VOICING
            ALetter, // Sk       MODIFIER LETTER UNASPIRATED
            ALetter, // Lm       MODIFIER LETTER DOUBLE APOSTROPHE
            ALetter, // Sk  [17] MODIFIER LETTER LOW DOWN ARROWHEAD..MODIFIER LETTER LOW LEFT ARROW
            Extend, // Mn [112] COMBINING GRAVE ACCENT..COMBINING LATIN SMALL LETTER X
            ALetter, // L&   [4] GREEK CAPITAL LETTER HETA..GREEK SMALL LETTER ARCHAIC SAMPI
            ALetter, // Lm       GREEK NUMERAL SIGN
            Other,
            ALetter, // L&   [2] GREEK CAPITAL LETTER PAMPHYLIAN DIGAMMA..GREEK SMALL LETTER PAMPHYLIAN DIGAMMA
            Other,
            ALetter, // Lm       GREEK YPOGEGRAMMENI
            ALetter, // L&   [3] GREEK SMALL REVERSED LUNATE SIGMA SYMBOL..GREEK SMALL REVERSED DOTTED LUNATE SIGMA SYMBOL
            MidNum, // Po       GREEK QUESTION MARK
            ALetter, // L&       GREEK CAPITAL LETTER YOT
            Other,
            ALetter, // L&       GREEK CAPITAL LETTER ALPHA WITH TONOS
            MidLetter, // Po       GREEK ANO TELEIA
            ALetter, // L&   [3] GREEK CAPITAL LETTER EPSILON WITH TONOS..GREEK CAPITAL LETTER IOTA WITH TONOS
            Other,
            ALetter, // L&       GREEK CAPITAL LETTER OMICRON WITH TONOS
            Other,
            ALetter, // L&  [20] GREEK CAPITAL LETTER UPSILON WITH TONOS..GREEK CAPITAL LETTER RHO
            Other,
            ALetter, // L&  [83] GREEK CAPITAL LETTER SIGMA..GREEK LUNATE EPSILON SYMBOL
            Other,
            ALetter, // L& [139] GREEK CAPITAL LETTER SHO..CYRILLIC SMALL LETTER KOPPA
            Other,
            Extend, // Mn   [5] COMBINING CYRILLIC TITLO..COMBINING CYRILLIC POKRYTIE
            Extend, // Me   [2] COMBINING CYRILLIC HUNDRED THOUSANDS SIGN..COMBINING CYRILLIC MILLIONS SIGN
            ALetter, // L& [166] CYRILLIC CAPITAL LETTER SHORT I WITH TAIL..CYRILLIC SMALL LETTER EL WITH DESCENDER
            Other,
            ALetter, // L&  [38] ARMENIAN CAPITAL LETTER AYB..ARMENIAN CAPITAL LETTER FEH
            Other,
            ALetter, // Lm       ARMENIAN MODIFIER LETTER LEFT HALF RING
            Other,
            ALetter, // Po   [2] ARMENIAN EMPHASIS MARK..ARMENIAN EXCLAMATION MARK
            Other,
            ALetter, // Po       ARMENIAN QUESTION MARK
            Other,
            ALetter, // L&  [41] ARMENIAN SMALL LETTER TURNED AYB..ARMENIAN SMALL LETTER YI WITH STROKE
            MidNum, // Po       ARMENIAN FULL STOP
            Other,
            Extend, // Mn  [45] HEBREW ACCENT ETNAHTA..HEBREW POINT METEG
            Other,
            Extend, // Mn       HEBREW POINT RAFE
            Other,
            Extend, // Mn   [2] HEBREW POINT SHIN DOT..HEBREW POINT SIN DOT
            Other,
            Extend, // Mn   [2] HEBREW MARK UPPER DOT..HEBREW MARK LOWER DOT
            Other,
            Extend, // Mn       HEBREW POINT QAMATS QATAN
            Other,
            Hebrew_Letter, // Lo  [27] HEBREW LETTER ALEF..HEBREW LETTER TAV
            Other,
            Hebrew_Letter, // Lo   [4] HEBREW YOD TRIANGLE..HEBREW LIGATURE YIDDISH DOUBLE YOD
            ALetter, // Po       HEBREW PUNCTUATION GERESH
            MidLetter, // Po       HEBREW PUNCTUATION GERSHAYIM
            Other,
            Format, // Cf   [6] ARABIC NUMBER SIGN..ARABIC NUMBER MARK ABOVE
            Other,
            MidNum, // Po   [2] ARABIC COMMA..ARABIC DATE SEPARATOR
            Other,
            Extend, // Mn  [11] ARABIC SIGN SALLALLAHOU ALAYHE WASSALLAM..ARABIC SMALL KASRA
            Other,
            Format, // Cf       ARABIC LETTER MARK
            Other,
            ALetter, // Lo  [32] ARABIC LETTER KASHMIRI YEH..ARABIC LETTER FARSI YEH WITH THREE DOTS ABOVE
            ALetter, // Lm       ARABIC TATWEEL
            ALetter, // Lo  [10] ARABIC LETTER FEH..ARABIC LETTER YEH
            Extend, // Mn  [21] ARABIC FATHATAN..ARABIC WAVY HAMZA BELOW
            Numeric, // Nd  [10] ARABIC-INDIC DIGIT ZERO..ARABIC-INDIC DIGIT NINE
            Other,
            Numeric, // Po       ARABIC DECIMAL SEPARATOR
            MidNum, // Po       ARABIC THOUSANDS SEPARATOR
            Other,
            ALetter, // Lo   [2] ARABIC LETTER DOTLESS BEH..ARABIC LETTER DOTLESS QAF
            Extend, // Mn       ARABIC LETTER SUPERSCRIPT ALEF
            ALetter, // Lo  [99] ARABIC LETTER ALEF WASLA..ARABIC LETTER YEH BARREE WITH HAMZA ABOVE
            Other,
            ALetter, // Lo       ARABIC LETTER AE
            Extend, // Mn   [7] ARABIC SMALL HIGH LIGATURE SAD WITH LAM WITH ALEF MAKSURA..ARABIC SMALL HIGH SEEN
            Format, // Cf       ARABIC END OF AYAH
            Other,
            Extend, // Mn   [6] ARABIC SMALL HIGH ROUNDED ZERO..ARABIC SMALL HIGH MADDA
            ALetter, // Lm   [2] ARABIC SMALL WAW..ARABIC SMALL YEH
            Extend, // Mn   [2] ARABIC SMALL HIGH YEH..ARABIC SMALL HIGH NOON
            Other,
            Extend, // Mn   [4] ARABIC EMPTY CENTRE LOW STOP..ARABIC SMALL LOW MEEM
            ALetter, // Lo   [2] ARABIC LETTER DAL WITH INVERTED V..ARABIC LETTER REH WITH INVERTED V
            Numeric, // Nd  [10] EXTENDED ARABIC-INDIC DIGIT ZERO..EXTENDED ARABIC-INDIC DIGIT NINE
            ALetter, // Lo   [3] ARABIC LETTER SHEEN WITH DOT BELOW..ARABIC LETTER GHAIN WITH DOT BELOW
            Other,
            ALetter, // Lo       ARABIC LETTER HEH WITH INVERTED V
            Other,
            Format, // Cf       SYRIAC ABBREVIATION MARK
            ALetter, // Lo       SYRIAC LETTER ALAPH
            Extend, // Mn       SYRIAC LETTER SUPERSCRIPT ALAPH
            ALetter, // Lo  [30] SYRIAC LETTER BETH..SYRIAC LETTER PERSIAN DHALATH
            Extend, // Mn  [27] SYRIAC PTHAHA ABOVE..SYRIAC BARREKH
            Other,
            ALetter, // Lo  [89] SYRIAC LETTER SOGDIAN ZHAIN..THAANA LETTER WAAVU
            Extend, // Mn  [11] THAANA ABAFILI..THAANA SUKUN
            ALetter, // Lo       THAANA LETTER NAA
            Other,
            Numeric, // Nd  [10] NKO DIGIT ZERO..NKO DIGIT NINE
            ALetter, // Lo  [33] NKO LETTER A..NKO LETTER JONA RA
            Extend, // Mn   [9] NKO COMBINING SHORT HIGH TONE..NKO COMBINING DOUBLE DOT ABOVE
            ALetter, // Lm   [2] NKO HIGH TONE APOSTROPHE..NKO LOW TONE APOSTROPHE
            Other,
            MidNum, // Po       NKO COMMA
            Other,
            ALetter, // Lm       NKO LAJANYALAN
            Other,
            Extend, // Mn       NKO DANTAYALAN
            Other,
            ALetter, // Lo  [22] SAMARITAN LETTER ALAF..SAMARITAN LETTER TAAF
            Extend, // Mn   [4] SAMARITAN MARK IN..SAMARITAN MARK DAGESH
            ALetter, // Lm       SAMARITAN MODIFIER LETTER EPENTHETIC YUT
            Extend, // Mn   [9] SAMARITAN MARK EPENTHETIC YUT..SAMARITAN VOWEL SIGN A
            ALetter, // Lm       SAMARITAN MODIFIER LETTER SHORT A
            Extend, // Mn   [3] SAMARITAN VOWEL SIGN SHORT A..SAMARITAN VOWEL SIGN U
            ALetter, // Lm       SAMARITAN MODIFIER LETTER I
            Extend, // Mn   [5] SAMARITAN VOWEL SIGN LONG I..SAMARITAN MARK NEQUDAA
            Other,
            ALetter, // Lo  [25] MANDAIC LETTER HALQA..MANDAIC LETTER AIN
            Extend, // Mn   [3] MANDAIC AFFRICATION MARK..MANDAIC GEMINATION MARK
            Other,
            ALetter, // Lo  [11] SYRIAC LETTER MALAYALAM NGA..SYRIAC LETTER MALAYALAM SSA
            Other,
            ALetter, // Lo  [21] ARABIC LETTER BEH WITH SMALL V BELOW..ARABIC LETTER KAF WITH DOT BELOW
            Other,
            ALetter, // Lo   [8] ARABIC LETTER BEH WITH SMALL MEEM ABOVE..ARABIC LETTER AFRICAN NOON
            Other,
            Extend, // Mn  [15] ARABIC SMALL LOW WAW..ARABIC SMALL HIGH SIGN SAFHA
            Format, // Cf       ARABIC DISPUTED END OF AYAH
            Extend, // Mn  [32] ARABIC TURNED DAMMA BELOW..DEVANAGARI SIGN ANUSVARA
            Extend, // Mc       DEVANAGARI SIGN VISARGA
            ALetter, // Lo  [54] DEVANAGARI LETTER SHORT A..DEVANAGARI LETTER HA
            Extend, // Mn       DEVANAGARI VOWEL SIGN OE
            Extend, // Mc       DEVANAGARI VOWEL SIGN OOE
            Extend, // Mn       DEVANAGARI SIGN NUKTA
            ALetter, // Lo       DEVANAGARI SIGN AVAGRAHA
            Extend, // Mc   [3] DEVANAGARI VOWEL SIGN AA..DEVANAGARI VOWEL SIGN II
            Extend, // Mn   [8] DEVANAGARI VOWEL SIGN U..DEVANAGARI VOWEL SIGN AI
            Extend, // Mc   [4] DEVANAGARI VOWEL SIGN CANDRA O..DEVANAGARI VOWEL SIGN AU
            Extend, // Mn       DEVANAGARI SIGN VIRAMA
            Extend, // Mc   [2] DEVANAGARI VOWEL SIGN PRISHTHAMATRA E..DEVANAGARI VOWEL SIGN AW
            ALetter, // Lo       DEVANAGARI OM
            Extend, // Mn   [7] DEVANAGARI STRESS SIGN UDATTA..DEVANAGARI VOWEL SIGN UUE
            ALetter, // Lo  [10] DEVANAGARI LETTER QA..DEVANAGARI LETTER VOCALIC LL
            Extend, // Mn   [2] DEVANAGARI VOWEL SIGN VOCALIC L..DEVANAGARI VOWEL SIGN VOCALIC LL
            Other,
            Numeric, // Nd  [10] DEVANAGARI DIGIT ZERO..DEVANAGARI DIGIT NINE
            Other,
            ALetter, // Lm       DEVANAGARI SIGN HIGH SPACING DOT
            ALetter, // Lo  [15] DEVANAGARI LETTER CANDRA A..BENGALI ANJI
            Extend, // Mn       BENGALI SIGN CANDRABINDU
            Extend, // Mc   [2] BENGALI SIGN ANUSVARA..BENGALI SIGN VISARGA
            Other,
            ALetter, // Lo   [8] BENGALI LETTER A..BENGALI LETTER VOCALIC L
            Other,
            ALetter, // Lo   [2] BENGALI LETTER E..BENGALI LETTER AI
            Other,
            ALetter, // Lo  [22] BENGALI LETTER O..BENGALI LETTER NA
            Other,
            ALetter, // Lo   [7] BENGALI LETTER PA..BENGALI LETTER RA
            Other,
            ALetter, // Lo       BENGALI LETTER LA
            Other,
            ALetter, // Lo   [4] BENGALI LETTER SHA..BENGALI LETTER HA
            Other,
            Extend, // Mn       BENGALI SIGN NUKTA
            ALetter, // Lo       BENGALI SIGN AVAGRAHA
            Extend, // Mc   [3] BENGALI VOWEL SIGN AA..BENGALI VOWEL SIGN II
            Extend, // Mn   [4] BENGALI VOWEL SIGN U..BENGALI VOWEL SIGN VOCALIC RR
            Other,
            Extend, // Mc   [2] BENGALI VOWEL SIGN E..BENGALI VOWEL SIGN AI
            Other,
            Extend, // Mc   [2] BENGALI VOWEL SIGN O..BENGALI VOWEL SIGN AU
            Extend, // Mn       BENGALI SIGN VIRAMA
            ALetter, // Lo       BENGALI LETTER KHANDA TA
            Other,
            Extend, // Mc       BENGALI AU LENGTH MARK
            Other,
            ALetter, // Lo   [2] BENGALI LETTER RRA..BENGALI LETTER RHA
            Other,
            ALetter, // Lo   [3] BENGALI LETTER YYA..BENGALI LETTER VOCALIC LL
            Extend, // Mn   [2] BENGALI VOWEL SIGN VOCALIC L..BENGALI VOWEL SIGN VOCALIC LL
            Other,
            Numeric, // Nd  [10] BENGALI DIGIT ZERO..BENGALI DIGIT NINE
            ALetter, // Lo   [2] BENGALI LETTER RA WITH MIDDLE DIAGONAL..BENGALI LETTER RA WITH LOWER DIAGONAL
            Other,
            ALetter, // Lo       BENGALI LETTER VEDIC ANUSVARA
            Other,
            Extend, // Mn       BENGALI SANDHI MARK
            Other,
            Extend, // Mn   [2] GURMUKHI SIGN ADAK BINDI..GURMUKHI SIGN BINDI
            Extend, // Mc       GURMUKHI SIGN VISARGA
            Other,
            ALetter, // Lo   [6] GURMUKHI LETTER A..GURMUKHI LETTER UU
            Other,
            ALetter, // Lo   [2] GURMUKHI LETTER EE..GURMUKHI LETTER AI
            Other,
            ALetter, // Lo  [22] GURMUKHI LETTER OO..GURMUKHI LETTER NA
            Other,
            ALetter, // Lo   [7] GURMUKHI LETTER PA..GURMUKHI LETTER RA
            Other,
            ALetter, // Lo   [2] GURMUKHI LETTER LA..GURMUKHI LETTER LLA
            Other,
            ALetter, // Lo   [2] GURMUKHI LETTER VA..GURMUKHI LETTER SHA
            Other,
            ALetter, // Lo   [2] GURMUKHI LETTER SA..GURMUKHI LETTER HA
            Other,
            Extend, // Mn       GURMUKHI SIGN NUKTA
            Other,
            Extend, // Mc   [3] GURMUKHI VOWEL SIGN AA..GURMUKHI VOWEL SIGN II
            Extend, // Mn   [2] GURMUKHI VOWEL SIGN U..GURMUKHI VOWEL SIGN UU
            Other,
            Extend, // Mn   [2] GURMUKHI VOWEL SIGN EE..GURMUKHI VOWEL SIGN AI
            Other,
            Extend, // Mn   [3] GURMUKHI VOWEL SIGN OO..GURMUKHI SIGN VIRAMA
            Other,
            Extend, // Mn       GURMUKHI SIGN UDAAT
            Other,
            ALetter, // Lo   [4] GURMUKHI LETTER KHHA..GURMUKHI LETTER RRA
            Other,
            ALetter, // Lo       GURMUKHI LETTER FA
            Other,
            Numeric, // Nd  [10] GURMUKHI DIGIT ZERO..GURMUKHI DIGIT NINE
            Extend, // Mn   [2] GURMUKHI TIPPI..GURMUKHI ADDAK
            ALetter, // Lo   [3] GURMUKHI IRI..GURMUKHI EK ONKAR
            Extend, // Mn       GURMUKHI SIGN YAKASH
            Other,
            Extend, // Mn   [2] GUJARATI SIGN CANDRABINDU..GUJARATI SIGN ANUSVARA
            Extend, // Mc       GUJARATI SIGN VISARGA
            Other,
            ALetter, // Lo   [9] GUJARATI LETTER A..GUJARATI VOWEL CANDRA E
            Other,
            ALetter, // Lo   [3] GUJARATI LETTER E..GUJARATI VOWEL CANDRA O
            Other,
            ALetter, // Lo  [22] GUJARATI LETTER O..GUJARATI LETTER NA
            Other,
            ALetter, // Lo   [7] GUJARATI LETTER PA..GUJARATI LETTER RA
            Other,
            ALetter, // Lo   [2] GUJARATI LETTER LA..GUJARATI LETTER LLA
            Other,
            ALetter, // Lo   [5] GUJARATI LETTER VA..GUJARATI LETTER HA
            Other,
            Extend, // Mn       GUJARATI SIGN NUKTA
            ALetter, // Lo       GUJARATI SIGN AVAGRAHA
            Extend, // Mc   [3] GUJARATI VOWEL SIGN AA..GUJARATI VOWEL SIGN II
            Extend, // Mn   [5] GUJARATI VOWEL SIGN U..GUJARATI VOWEL SIGN CANDRA E
            Other,
            Extend, // Mn   [2] GUJARATI VOWEL SIGN E..GUJARATI VOWEL SIGN AI
            Extend, // Mc       GUJARATI VOWEL SIGN CANDRA O
            Other,
            Extend, // Mc   [2] GUJARATI VOWEL SIGN O..GUJARATI VOWEL SIGN AU
            Extend, // Mn       GUJARATI SIGN VIRAMA
            Other,
            ALetter, // Lo       GUJARATI OM
            Other,
            ALetter, // Lo   [2] GUJARATI LETTER VOCALIC RR..GUJARATI LETTER VOCALIC LL
            Extend, // Mn   [2] GUJARATI VOWEL SIGN VOCALIC L..GUJARATI VOWEL SIGN VOCALIC LL
            Other,
            Numeric, // Nd  [10] GUJARATI DIGIT ZERO..GUJARATI DIGIT NINE
            Other,
            ALetter, // Lo       GUJARATI LETTER ZHA
            Extend, // Mn   [6] GUJARATI SIGN SUKUN..GUJARATI SIGN TWO-CIRCLE NUKTA ABOVE
            Other,
            Extend, // Mn       ORIYA SIGN CANDRABINDU
            Extend, // Mc   [2] ORIYA SIGN ANUSVARA..ORIYA SIGN VISARGA
            Other,
            ALetter, // Lo   [8] ORIYA LETTER A..ORIYA LETTER VOCALIC L
            Other,
            ALetter, // Lo   [2] ORIYA LETTER E..ORIYA LETTER AI
            Other,
            ALetter, // Lo  [22] ORIYA LETTER O..ORIYA LETTER NA
            Other,
            ALetter, // Lo   [7] ORIYA LETTER PA..ORIYA LETTER RA
            Other,
            ALetter, // Lo   [2] ORIYA LETTER LA..ORIYA LETTER LLA
            Other,
            ALetter, // Lo   [5] ORIYA LETTER VA..ORIYA LETTER HA
            Other,
            Extend, // Mn       ORIYA SIGN NUKTA
            ALetter, // Lo       ORIYA SIGN AVAGRAHA
            Extend, // Mc       ORIYA VOWEL SIGN AA
            Extend, // Mn       ORIYA VOWEL SIGN I
            Extend, // Mc       ORIYA VOWEL SIGN II
            Extend, // Mn   [4] ORIYA VOWEL SIGN U..ORIYA VOWEL SIGN VOCALIC RR
            Other,
            Extend, // Mc   [2] ORIYA VOWEL SIGN E..ORIYA VOWEL SIGN AI
            Other,
            Extend, // Mc   [2] ORIYA VOWEL SIGN O..ORIYA VOWEL SIGN AU
            Extend, // Mn       ORIYA SIGN VIRAMA
            Other,
            Extend, // Mn       ORIYA AI LENGTH MARK
            Extend, // Mc       ORIYA AU LENGTH MARK
            Other,
            ALetter, // Lo   [2] ORIYA LETTER RRA..ORIYA LETTER RHA
            Other,
            ALetter, // Lo   [3] ORIYA LETTER YYA..ORIYA LETTER VOCALIC LL
            Extend, // Mn   [2] ORIYA VOWEL SIGN VOCALIC L..ORIYA VOWEL SIGN VOCALIC LL
            Other,
            Numeric, // Nd  [10] ORIYA DIGIT ZERO..ORIYA DIGIT NINE
            Other,
            ALetter, // Lo       ORIYA LETTER WA
            Other,
            Extend, // Mn       TAMIL SIGN ANUSVARA
            ALetter, // Lo       TAMIL SIGN VISARGA
            Other,
            ALetter, // Lo   [6] TAMIL LETTER A..TAMIL LETTER UU
            Other,
            ALetter, // Lo   [3] TAMIL LETTER E..TAMIL LETTER AI
            Other,
            ALetter, // Lo   [4] TAMIL LETTER O..TAMIL LETTER KA
            Other,
            ALetter, // Lo   [2] TAMIL LETTER NGA..TAMIL LETTER CA
            Other,
            ALetter, // Lo       TAMIL LETTER JA
            Other,
            ALetter, // Lo   [2] TAMIL LETTER NYA..TAMIL LETTER TTA
            Other,
            ALetter, // Lo   [2] TAMIL LETTER NNA..TAMIL LETTER TA
            Other,
            ALetter, // Lo   [3] TAMIL LETTER NA..TAMIL LETTER PA
            Other,
            ALetter, // Lo  [12] TAMIL LETTER MA..TAMIL LETTER HA
            Other,
            Extend, // Mc   [2] TAMIL VOWEL SIGN AA..TAMIL VOWEL SIGN I
            Extend, // Mn       TAMIL VOWEL SIGN II
            Extend, // Mc   [2] TAMIL VOWEL SIGN U..TAMIL VOWEL SIGN UU
            Other,
            Extend, // Mc   [3] TAMIL VOWEL SIGN E..TAMIL VOWEL SIGN AI
            Other,
            Extend, // Mc   [3] TAMIL VOWEL SIGN O..TAMIL VOWEL SIGN AU
            Extend, // Mn       TAMIL SIGN VIRAMA
            Other,
            ALetter, // Lo       TAMIL OM
            Other,
            Extend, // Mc       TAMIL AU LENGTH MARK
            Other,
            Numeric, // Nd  [10] TAMIL DIGIT ZERO..TAMIL DIGIT NINE
            Other,
            Extend, // Mn       TELUGU SIGN COMBINING CANDRABINDU ABOVE
            Extend, // Mc   [3] TELUGU SIGN CANDRABINDU..TELUGU SIGN VISARGA
            Extend, // Mn       TELUGU SIGN COMBINING ANUSVARA ABOVE
            ALetter, // Lo   [8] TELUGU LETTER A..TELUGU LETTER VOCALIC L
            Other,
            ALetter, // Lo   [3] TELUGU LETTER E..TELUGU LETTER AI
            Other,
            ALetter, // Lo  [23] TELUGU LETTER O..TELUGU LETTER NA
            Other,
            ALetter, // Lo  [16] TELUGU LETTER PA..TELUGU LETTER HA
            Other,
            ALetter, // Lo       TELUGU SIGN AVAGRAHA
            Extend, // Mn   [3] TELUGU VOWEL SIGN AA..TELUGU VOWEL SIGN II
            Extend, // Mc   [4] TELUGU VOWEL SIGN U..TELUGU VOWEL SIGN VOCALIC RR
            Other,
            Extend, // Mn   [3] TELUGU VOWEL SIGN E..TELUGU VOWEL SIGN AI
            Other,
            Extend, // Mn   [4] TELUGU VOWEL SIGN O..TELUGU SIGN VIRAMA
            Other,
            Extend, // Mn   [2] TELUGU LENGTH MARK..TELUGU AI LENGTH MARK
            Other,
            ALetter, // Lo   [3] TELUGU LETTER TSA..TELUGU LETTER RRRA
            Other,
            ALetter, // Lo   [2] TELUGU LETTER VOCALIC RR..TELUGU LETTER VOCALIC LL
            Extend, // Mn   [2] TELUGU VOWEL SIGN VOCALIC L..TELUGU VOWEL SIGN VOCALIC LL
            Other,
            Numeric, // Nd  [10] TELUGU DIGIT ZERO..TELUGU DIGIT NINE
            Other,
            ALetter, // Lo       KANNADA SIGN SPACING CANDRABINDU
            Extend, // Mn       KANNADA SIGN CANDRABINDU
            Extend, // Mc   [2] KANNADA SIGN ANUSVARA..KANNADA SIGN VISARGA
            Other,
            ALetter, // Lo   [8] KANNADA LETTER A..KANNADA LETTER VOCALIC L
            Other,
            ALetter, // Lo   [3] KANNADA LETTER E..KANNADA LETTER AI
            Other,
            ALetter, // Lo  [23] KANNADA LETTER O..KANNADA LETTER NA
            Other,
            ALetter, // Lo  [10] KANNADA LETTER PA..KANNADA LETTER LLA
            Other,
            ALetter, // Lo   [5] KANNADA LETTER VA..KANNADA LETTER HA
            Other,
            Extend, // Mn       KANNADA SIGN NUKTA
            ALetter, // Lo       KANNADA SIGN AVAGRAHA
            Extend, // Mc       KANNADA VOWEL SIGN AA
            Extend, // Mn       KANNADA VOWEL SIGN I
            Extend, // Mc   [5] KANNADA VOWEL SIGN II..KANNADA VOWEL SIGN VOCALIC RR
            Other,
            Extend, // Mn       KANNADA VOWEL SIGN E
            Extend, // Mc   [2] KANNADA VOWEL SIGN EE..KANNADA VOWEL SIGN AI
            Other,
            Extend, // Mc   [2] KANNADA VOWEL SIGN O..KANNADA VOWEL SIGN OO
            Extend, // Mn   [2] KANNADA VOWEL SIGN AU..KANNADA SIGN VIRAMA
            Other,
            Extend, // Mc   [2] KANNADA LENGTH MARK..KANNADA AI LENGTH MARK
            Other,
            ALetter, // Lo       KANNADA LETTER FA
            Other,
            ALetter, // Lo   [2] KANNADA LETTER VOCALIC RR..KANNADA LETTER VOCALIC LL
            Extend, // Mn   [2] KANNADA VOWEL SIGN VOCALIC L..KANNADA VOWEL SIGN VOCALIC LL
            Other,
            Numeric, // Nd  [10] KANNADA DIGIT ZERO..KANNADA DIGIT NINE
            Other,
            ALetter, // Lo   [2] KANNADA SIGN JIHVAMULIYA..KANNADA SIGN UPADHMANIYA
            Other,
            Extend, // Mn   [2] MALAYALAM SIGN COMBINING ANUSVARA ABOVE..MALAYALAM SIGN CANDRABINDU
            Extend, // Mc   [2] MALAYALAM SIGN ANUSVARA..MALAYALAM SIGN VISARGA
            Other,
            ALetter, // Lo   [8] MALAYALAM LETTER A..MALAYALAM LETTER VOCALIC L
            Other,
            ALetter, // Lo   [3] MALAYALAM LETTER E..MALAYALAM LETTER AI
            Other,
            ALetter, // Lo  [41] MALAYALAM LETTER O..MALAYALAM LETTER TTTA
            Extend, // Mn   [2] MALAYALAM SIGN VERTICAL BAR VIRAMA..MALAYALAM SIGN CIRCULAR VIRAMA
            ALetter, // Lo       MALAYALAM SIGN AVAGRAHA
            Extend, // Mc   [3] MALAYALAM VOWEL SIGN AA..MALAYALAM VOWEL SIGN II
            Extend, // Mn   [4] MALAYALAM VOWEL SIGN U..MALAYALAM VOWEL SIGN VOCALIC RR
            Other,
            Extend, // Mc   [3] MALAYALAM VOWEL SIGN E..MALAYALAM VOWEL SIGN AI
            Other,
            Extend, // Mc   [3] MALAYALAM VOWEL SIGN O..MALAYALAM VOWEL SIGN AU
            Extend, // Mn       MALAYALAM SIGN VIRAMA
            ALetter, // Lo       MALAYALAM LETTER DOT REPH
            Other,
            ALetter, // Lo   [3] MALAYALAM LETTER CHILLU M..MALAYALAM LETTER CHILLU LLL
            Extend, // Mc       MALAYALAM AU LENGTH MARK
            Other,
            ALetter, // Lo   [3] MALAYALAM LETTER ARCHAIC II..MALAYALAM LETTER VOCALIC LL
            Extend, // Mn   [2] MALAYALAM VOWEL SIGN VOCALIC L..MALAYALAM VOWEL SIGN VOCALIC LL
            Other,
            Numeric, // Nd  [10] MALAYALAM DIGIT ZERO..MALAYALAM DIGIT NINE
            Other,
            ALetter, // Lo   [6] MALAYALAM LETTER CHILLU NN..MALAYALAM LETTER CHILLU K
            Other,
            Extend, // Mc   [2] SINHALA SIGN ANUSVARAYA..SINHALA SIGN VISARGAYA
            Other,
            ALetter, // Lo  [18] SINHALA LETTER AYANNA..SINHALA LETTER AUYANNA
            Other,
            ALetter, // Lo  [24] SINHALA LETTER ALPAPRAANA KAYANNA..SINHALA LETTER DANTAJA NAYANNA
            Other,
            ALetter, // Lo   [9] SINHALA LETTER SANYAKA DAYANNA..SINHALA LETTER RAYANNA
            Other,
            ALetter, // Lo       SINHALA LETTER DANTAJA LAYANNA
            Other,
            ALetter, // Lo   [7] SINHALA LETTER VAYANNA..SINHALA LETTER FAYANNA
            Other,
            Extend, // Mn       SINHALA SIGN AL-LAKUNA
            Other,
            Extend, // Mc   [3] SINHALA VOWEL SIGN AELA-PILLA..SINHALA VOWEL SIGN DIGA AEDA-PILLA
            Extend, // Mn   [3] SINHALA VOWEL SIGN KETTI IS-PILLA..SINHALA VOWEL SIGN KETTI PAA-PILLA
            Other,
            Extend, // Mn       SINHALA VOWEL SIGN DIGA PAA-PILLA
            Other,
            Extend, // Mc   [8] SINHALA VOWEL SIGN GAETTA-PILLA..SINHALA VOWEL SIGN GAYANUKITTA
            Other,
            Numeric, // Nd  [10] SINHALA LITH DIGIT ZERO..SINHALA LITH DIGIT NINE
            Other,
            Extend, // Mc   [2] SINHALA VOWEL SIGN DIGA GAETTA-PILLA..SINHALA VOWEL SIGN DIGA GAYANUKITTA
            Other,
            Extend, // Mn       THAI CHARACTER MAI HAN-AKAT
            Other,
            Extend, // Mn   [7] THAI CHARACTER SARA I..THAI CHARACTER PHINTHU
            Other,
            Extend, // Mn   [8] THAI CHARACTER MAITAIKHU..THAI CHARACTER YAMAKKAN
            Other,
            Numeric, // Nd  [10] THAI DIGIT ZERO..THAI DIGIT NINE
            Other,
            Extend, // Mn       LAO VOWEL SIGN MAI KAN
            Other,
            Extend, // Mn   [9] LAO VOWEL SIGN I..LAO SEMIVOWEL SIGN LO
            Other,
            Extend, // Mn   [6] LAO TONE MAI EK..LAO NIGGAHITA
            Other,
            Numeric, // Nd  [10] LAO DIGIT ZERO..LAO DIGIT NINE
            Other,
            ALetter, // Lo       TIBETAN SYLLABLE OM
            Other,
            Extend, // Mn   [2] TIBETAN ASTROLOGICAL SIGN -KHYUD PA..TIBETAN ASTROLOGICAL SIGN SDONG TSHUGS
            Other,
            Numeric, // Nd  [10] TIBETAN DIGIT ZERO..TIBETAN DIGIT NINE
            Other,
            Extend, // Mn       TIBETAN MARK NGAS BZUNG NYI ZLA
            Other,
            Extend, // Mn       TIBETAN MARK NGAS BZUNG SGOR RTAGS
            Other,
            Extend, // Mn       TIBETAN MARK TSA -PHRU
            Other,
            Extend, // Mc   [2] TIBETAN SIGN YAR TSHES..TIBETAN SIGN MAR TSHES
            ALetter, // Lo   [8] TIBETAN LETTER KA..TIBETAN LETTER JA
            Other,
            ALetter, // Lo  [36] TIBETAN LETTER NYA..TIBETAN LETTER RRA
            Other,
            Extend, // Mn  [14] TIBETAN VOWEL SIGN AA..TIBETAN SIGN RJES SU NGA RO
            Extend, // Mc       TIBETAN SIGN RNAM BCAD
            Extend, // Mn   [5] TIBETAN VOWEL SIGN REVERSED I..TIBETAN MARK HALANTA
            Other,
            Extend, // Mn   [2] TIBETAN SIGN LCI RTAGS..TIBETAN SIGN YANG RTAGS
            ALetter, // Lo   [5] TIBETAN SIGN LCE TSA CAN..TIBETAN SIGN INVERTED MCHU CAN
            Extend, // Mn  [11] TIBETAN SUBJOINED SIGN LCE TSA CAN..TIBETAN SUBJOINED LETTER JA
            Other,
            Extend, // Mn  [36] TIBETAN SUBJOINED LETTER NYA..TIBETAN SUBJOINED LETTER FIXED-FORM RA
            Other,
            Extend, // Mn       TIBETAN SYMBOL PADMA GDAN
            Other,
            Extend, // Mc   [2] MYANMAR VOWEL SIGN TALL AA..MYANMAR VOWEL SIGN AA
            Extend, // Mn   [4] MYANMAR VOWEL SIGN I..MYANMAR VOWEL SIGN UU
            Extend, // Mc       MYANMAR VOWEL SIGN E
            Extend, // Mn   [6] MYANMAR VOWEL SIGN AI..MYANMAR SIGN DOT BELOW
            Extend, // Mc       MYANMAR SIGN VISARGA
            Extend, // Mn   [2] MYANMAR SIGN VIRAMA..MYANMAR SIGN ASAT
            Extend, // Mc   [2] MYANMAR CONSONANT SIGN MEDIAL YA..MYANMAR CONSONANT SIGN MEDIAL RA
            Extend, // Mn   [2] MYANMAR CONSONANT SIGN MEDIAL WA..MYANMAR CONSONANT SIGN MEDIAL HA
            Other,
            Numeric, // Nd  [10] MYANMAR DIGIT ZERO..MYANMAR DIGIT NINE
            Other,
            Extend, // Mc   [2] MYANMAR VOWEL SIGN VOCALIC R..MYANMAR VOWEL SIGN VOCALIC RR
            Extend, // Mn   [2] MYANMAR VOWEL SIGN VOCALIC L..MYANMAR VOWEL SIGN VOCALIC LL
            Other,
            Extend, // Mn   [3] MYANMAR CONSONANT SIGN MON MEDIAL NA..MYANMAR CONSONANT SIGN MON MEDIAL LA
            Other,
            Extend, // Mc   [3] MYANMAR VOWEL SIGN SGAW KAREN EU..MYANMAR TONE MARK SGAW KAREN KE PHO
            Other,
            Extend, // Mc   [7] MYANMAR VOWEL SIGN WESTERN PWO KAREN EU..MYANMAR SIGN WESTERN PWO KAREN TONE-5
            Other,
            Extend, // Mn   [4] MYANMAR VOWEL SIGN GEBA KAREN I..MYANMAR VOWEL SIGN KAYAH EE
            Other,
            Extend, // Mn       MYANMAR CONSONANT SIGN SHAN MEDIAL WA
            Extend, // Mc   [2] MYANMAR VOWEL SIGN SHAN AA..MYANMAR VOWEL SIGN SHAN E
            Extend, // Mn   [2] MYANMAR VOWEL SIGN SHAN E ABOVE..MYANMAR VOWEL SIGN SHAN FINAL Y
            Extend, // Mc   [6] MYANMAR SIGN SHAN TONE-2..MYANMAR SIGN SHAN COUNCIL TONE-3
            Extend, // Mn       MYANMAR SIGN SHAN COUNCIL EMPHATIC TONE
            Other,
            Extend, // Mc       MYANMAR SIGN RUMAI PALAUNG TONE-5
            Numeric, // Nd  [10] MYANMAR SHAN DIGIT ZERO..MYANMAR SHAN DIGIT NINE
            Extend, // Mc   [3] MYANMAR SIGN KHAMTI TONE-1..MYANMAR VOWEL SIGN AITON A
            Extend, // Mn       MYANMAR VOWEL SIGN AITON AI
            Other,
            ALetter, // L&  [38] GEORGIAN CAPITAL LETTER AN..GEORGIAN CAPITAL LETTER HOE
            Other,
            ALetter, // L&       GEORGIAN CAPITAL LETTER YN
            Other,
            ALetter, // L&       GEORGIAN CAPITAL LETTER AEN
            Other,
            ALetter, // L&  [43] GEORGIAN LETTER AN..GEORGIAN LETTER AIN
            Other,
            ALetter, // Lm       MODIFIER LETTER GEORGIAN NAR
            ALetter, // L&   [3] GEORGIAN LETTER AEN..GEORGIAN LETTER LABIAL SIGN
            ALetter, // Lo [329] HANGUL CHOSEONG KIYEOK..ETHIOPIC SYLLABLE QWA
            Other,
            ALetter, // Lo   [4] ETHIOPIC SYLLABLE QWI..ETHIOPIC SYLLABLE QWE
            Other,
            ALetter, // Lo   [7] ETHIOPIC SYLLABLE QHA..ETHIOPIC SYLLABLE QHO
            Other,
            ALetter, // Lo       ETHIOPIC SYLLABLE QHWA
            Other,
            ALetter, // Lo   [4] ETHIOPIC SYLLABLE QHWI..ETHIOPIC SYLLABLE QHWE
            Other,
            ALetter, // Lo  [41] ETHIOPIC SYLLABLE BA..ETHIOPIC SYLLABLE XWA
            Other,
            ALetter, // Lo   [4] ETHIOPIC SYLLABLE XWI..ETHIOPIC SYLLABLE XWE
            Other,
            ALetter, // Lo  [33] ETHIOPIC SYLLABLE NA..ETHIOPIC SYLLABLE KWA
            Other,
            ALetter, // Lo   [4] ETHIOPIC SYLLABLE KWI..ETHIOPIC SYLLABLE KWE
            Other,
            ALetter, // Lo   [7] ETHIOPIC SYLLABLE KXA..ETHIOPIC SYLLABLE KXO
            Other,
            ALetter, // Lo       ETHIOPIC SYLLABLE KXWA
            Other,
            ALetter, // Lo   [4] ETHIOPIC SYLLABLE KXWI..ETHIOPIC SYLLABLE KXWE
            Other,
            ALetter, // Lo  [15] ETHIOPIC SYLLABLE WA..ETHIOPIC SYLLABLE PHARYNGEAL O
            Other,
            ALetter, // Lo  [57] ETHIOPIC SYLLABLE ZA..ETHIOPIC SYLLABLE GWA
            Other,
            ALetter, // Lo   [4] ETHIOPIC SYLLABLE GWI..ETHIOPIC SYLLABLE GWE
            Other,
            ALetter, // Lo  [67] ETHIOPIC SYLLABLE GGA..ETHIOPIC SYLLABLE FYA
            Other,
            Extend, // Mn   [3] ETHIOPIC COMBINING GEMINATION AND VOWEL LENGTH MARK..ETHIOPIC COMBINING GEMINATION MARK
            Other,
            ALetter, // Lo  [16] ETHIOPIC SYLLABLE SEBATBEIT MWA..ETHIOPIC SYLLABLE PWE
            Other,
            ALetter, // L&  [86] CHEROKEE LETTER A..CHEROKEE LETTER MV
            Other,
            ALetter, // L&   [6] CHEROKEE SMALL LETTER YE..CHEROKEE SMALL LETTER MV
            Other,
            ALetter, // Lo [620] CANADIAN SYLLABICS E..CANADIAN SYLLABICS CARRIER TTSA
            Other,
            ALetter, // Lo  [17] CANADIAN SYLLABICS QAI..CANADIAN SYLLABICS BLACKFOOT W
            WSegSpace, // Zs       OGHAM SPACE MARK
            ALetter, // Lo  [26] OGHAM LETTER BEITH..OGHAM LETTER PEITH
            Other,
            ALetter, // Lo  [75] RUNIC LETTER FEHU FEOH FE F..RUNIC LETTER X
            Other,
            ALetter, // Nl   [3] RUNIC ARLAUG SYMBOL..RUNIC BELGTHOR SYMBOL
            ALetter, // Lo   [8] RUNIC LETTER K..RUNIC LETTER FRANKS CASKET AESC
            Other,
            ALetter, // Lo  [13] TAGALOG LETTER A..TAGALOG LETTER YA
            Other,
            ALetter, // Lo   [4] TAGALOG LETTER LA..TAGALOG LETTER HA
            Extend, // Mn   [3] TAGALOG VOWEL SIGN I..TAGALOG SIGN VIRAMA
            Other,
            ALetter, // Lo  [18] HANUNOO LETTER A..HANUNOO LETTER HA
            Extend, // Mn   [3] HANUNOO VOWEL SIGN I..HANUNOO SIGN PAMUDPOD
            Other,
            ALetter, // Lo  [18] BUHID LETTER A..BUHID LETTER HA
            Extend, // Mn   [2] BUHID VOWEL SIGN I..BUHID VOWEL SIGN U
            Other,
            ALetter, // Lo  [13] TAGBANWA LETTER A..TAGBANWA LETTER YA
            Other,
            ALetter, // Lo   [3] TAGBANWA LETTER LA..TAGBANWA LETTER SA
            Other,
            Extend, // Mn   [2] TAGBANWA VOWEL SIGN I..TAGBANWA VOWEL SIGN U
            Other,
            Extend, // Mn   [2] KHMER VOWEL INHERENT AQ..KHMER VOWEL INHERENT AA
            Extend, // Mc       KHMER VOWEL SIGN AA
            Extend, // Mn   [7] KHMER VOWEL SIGN I..KHMER VOWEL SIGN UA
            Extend, // Mc   [8] KHMER VOWEL SIGN OE..KHMER VOWEL SIGN AU
            Extend, // Mn       KHMER SIGN NIKAHIT
            Extend, // Mc   [2] KHMER SIGN REAHMUK..KHMER SIGN YUUKALEAPINTU
            Extend, // Mn  [11] KHMER SIGN MUUSIKATOAN..KHMER SIGN BATHAMASAT
            Other,
            Extend, // Mn       KHMER SIGN ATTHACAN
            Other,
            Numeric, // Nd  [10] KHMER DIGIT ZERO..KHMER DIGIT NINE
            Other,
            Extend, // Mn   [3] MONGOLIAN FREE VARIATION SELECTOR ONE..MONGOLIAN FREE VARIATION SELECTOR THREE
            Format, // Cf       MONGOLIAN VOWEL SEPARATOR
            Other,
            Numeric, // Nd  [10] MONGOLIAN DIGIT ZERO..MONGOLIAN DIGIT NINE
            Other,
            ALetter, // Lo  [35] MONGOLIAN LETTER A..MONGOLIAN LETTER CHI
            ALetter, // Lm       MONGOLIAN LETTER TODO LONG VOWEL SIGN
            ALetter, // Lo  [53] MONGOLIAN LETTER TODO E..MONGOLIAN LETTER CHA WITH TWO DOTS
            Other,
            ALetter, // Lo   [5] MONGOLIAN LETTER ALI GALI ANUSVARA ONE..MONGOLIAN LETTER ALI GALI INVERTED UBADAMA
            Extend, // Mn   [2] MONGOLIAN LETTER ALI GALI BALUDA..MONGOLIAN LETTER ALI GALI THREE BALUDA
            ALetter, // Lo  [34] MONGOLIAN LETTER ALI GALI A..MONGOLIAN LETTER MANCHU ALI GALI BHA
            Extend, // Mn       MONGOLIAN LETTER ALI GALI DAGALGA
            ALetter, // Lo       MONGOLIAN LETTER MANCHU ALI GALI LHA
            Other,
            ALetter, // Lo  [70] CANADIAN SYLLABICS OY..CANADIAN SYLLABICS CARRIER DENTAL S
            Other,
            ALetter, // Lo  [31] LIMBU VOWEL-CARRIER LETTER..LIMBU LETTER TRA
            Other,
            Extend, // Mn   [3] LIMBU VOWEL SIGN A..LIMBU VOWEL SIGN U
            Extend, // Mc   [4] LIMBU VOWEL SIGN EE..LIMBU VOWEL SIGN AU
            Extend, // Mn   [2] LIMBU VOWEL SIGN E..LIMBU VOWEL SIGN O
            Extend, // Mc   [3] LIMBU SUBJOINED LETTER YA..LIMBU SUBJOINED LETTER WA
            Other,
            Extend, // Mc   [2] LIMBU SMALL LETTER KA..LIMBU SMALL LETTER NGA
            Extend, // Mn       LIMBU SMALL LETTER ANUSVARA
            Extend, // Mc   [6] LIMBU SMALL LETTER TA..LIMBU SMALL LETTER LA
            Extend, // Mn   [3] LIMBU SIGN MUKPHRENG..LIMBU SIGN SA-I
            Other,
            Numeric, // Nd  [10] LIMBU DIGIT ZERO..LIMBU DIGIT NINE
            Other,
            Numeric, // Nd  [10] NEW TAI LUE DIGIT ZERO..NEW TAI LUE DIGIT NINE
            Other,
            ALetter, // Lo  [23] BUGINESE LETTER KA..BUGINESE LETTER HA
            Extend, // Mn   [2] BUGINESE VOWEL SIGN I..BUGINESE VOWEL SIGN U
            Extend, // Mc   [2] BUGINESE VOWEL SIGN E..BUGINESE VOWEL SIGN O
            Extend, // Mn       BUGINESE VOWEL SIGN AE
            Other,
            Extend, // Mc       TAI THAM CONSONANT SIGN MEDIAL RA
            Extend, // Mn       TAI THAM CONSONANT SIGN MEDIAL LA
            Extend, // Mc       TAI THAM CONSONANT SIGN LA TANG LAI
            Extend, // Mn   [7] TAI THAM SIGN MAI KANG LAI..TAI THAM CONSONANT SIGN SA
            Other,
            Extend, // Mn       TAI THAM SIGN SAKOT
            Extend, // Mc       TAI THAM VOWEL SIGN A
            Extend, // Mn       TAI THAM VOWEL SIGN MAI SAT
            Extend, // Mc   [2] TAI THAM VOWEL SIGN AA..TAI THAM VOWEL SIGN TALL AA
            Extend, // Mn   [8] TAI THAM VOWEL SIGN I..TAI THAM VOWEL SIGN OA BELOW
            Extend, // Mc   [6] TAI THAM VOWEL SIGN OY..TAI THAM VOWEL SIGN THAM AI
            Extend, // Mn  [10] TAI THAM VOWEL SIGN OA ABOVE..TAI THAM SIGN KHUEN-LUE KARAN
            Other,
            Extend, // Mn       TAI THAM COMBINING CRYPTOGRAMMIC DOT
            Numeric, // Nd  [10] TAI THAM HORA DIGIT ZERO..TAI THAM HORA DIGIT NINE
            Other,
            Numeric, // Nd  [10] TAI THAM THAM DIGIT ZERO..TAI THAM THAM DIGIT NINE
            Other,
            Extend, // Mn  [14] COMBINING DOUBLED CIRCUMFLEX ACCENT..COMBINING PARENTHESES BELOW
            Extend, // Me       COMBINING PARENTHESES OVERLAY
            Other,
            Extend, // Mn   [4] BALINESE SIGN ULU RICEM..BALINESE SIGN SURANG
            Extend, // Mc       BALINESE SIGN BISAH
            ALetter, // Lo  [47] BALINESE LETTER AKARA..BALINESE LETTER HA
            Extend, // Mn       BALINESE SIGN REREKAN
            Extend, // Mc       BALINESE VOWEL SIGN TEDUNG
            Extend, // Mn   [5] BALINESE VOWEL SIGN ULU..BALINESE VOWEL SIGN RA REPA
            Extend, // Mc       BALINESE VOWEL SIGN RA REPA TEDUNG
            Extend, // Mn       BALINESE VOWEL SIGN LA LENGA
            Extend, // Mc   [5] BALINESE VOWEL SIGN LA LENGA TEDUNG..BALINESE VOWEL SIGN TALING REPA TEDUNG
            Extend, // Mn       BALINESE VOWEL SIGN PEPET
            Extend, // Mc   [2] BALINESE VOWEL SIGN PEPET TEDUNG..BALINESE ADEG ADEG
            ALetter, // Lo   [7] BALINESE LETTER KAF SASAK..BALINESE LETTER ASYURA SASAK
            Other,
            Numeric, // Nd  [10] BALINESE DIGIT ZERO..BALINESE DIGIT NINE
            Other,
            Extend, // Mn   [9] BALINESE MUSICAL SYMBOL COMBINING TEGEH..BALINESE MUSICAL SYMBOL COMBINING GONG
            Other,
            Extend, // Mn   [2] SUNDANESE SIGN PANYECEK..SUNDANESE SIGN PANGLAYAR
            Extend, // Mc       SUNDANESE SIGN PANGWISAD
            ALetter, // Lo  [30] SUNDANESE LETTER A..SUNDANESE LETTER HA
            Extend, // Mc       SUNDANESE CONSONANT SIGN PAMINGKAL
            Extend, // Mn   [4] SUNDANESE CONSONANT SIGN PANYAKRA..SUNDANESE VOWEL SIGN PANYUKU
            Extend, // Mc   [2] SUNDANESE VOWEL SIGN PANAELAENG..SUNDANESE VOWEL SIGN PANOLONG
            Extend, // Mn   [2] SUNDANESE VOWEL SIGN PAMEPET..SUNDANESE VOWEL SIGN PANEULEUNG
            Extend, // Mc       SUNDANESE SIGN PAMAAEH
            Extend, // Mn   [3] SUNDANESE SIGN VIRAMA..SUNDANESE CONSONANT SIGN PASANGAN WA
            ALetter, // Lo   [2] SUNDANESE LETTER KHA..SUNDANESE LETTER SYA
            Numeric, // Nd  [10] SUNDANESE DIGIT ZERO..SUNDANESE DIGIT NINE
            ALetter, // Lo  [44] SUNDANESE AVAGRAHA..BATAK LETTER U
            Extend, // Mn       BATAK SIGN TOMPI
            Extend, // Mc       BATAK VOWEL SIGN E
            Extend, // Mn   [2] BATAK VOWEL SIGN PAKPAK E..BATAK VOWEL SIGN EE
            Extend, // Mc   [3] BATAK VOWEL SIGN I..BATAK VOWEL SIGN O
            Extend, // Mn       BATAK VOWEL SIGN KARO O
            Extend, // Mc       BATAK VOWEL SIGN U
            Extend, // Mn   [3] BATAK VOWEL SIGN U FOR SIMALUNGUN SA..BATAK CONSONANT SIGN H
            Extend, // Mc   [2] BATAK PANGOLAT..BATAK PANONGONAN
            Other,
            ALetter, // Lo  [36] LEPCHA LETTER KA..LEPCHA LETTER A
            Extend, // Mc   [8] LEPCHA SUBJOINED LETTER YA..LEPCHA VOWEL SIGN UU
            Extend, // Mn   [8] LEPCHA VOWEL SIGN E..LEPCHA CONSONANT SIGN T
            Extend, // Mc   [2] LEPCHA CONSONANT SIGN NYIN-DO..LEPCHA CONSONANT SIGN KANG
            Extend, // Mn   [2] LEPCHA SIGN RAN..LEPCHA SIGN NUKTA
            Other,
            Numeric, // Nd  [10] LEPCHA DIGIT ZERO..LEPCHA DIGIT NINE
            Other,
            ALetter, // Lo   [3] LEPCHA LETTER TTA..LEPCHA LETTER DDA
            Numeric, // Nd  [10] OL CHIKI DIGIT ZERO..OL CHIKI DIGIT NINE
            ALetter, // Lo  [30] OL CHIKI LETTER LA..OL CHIKI LETTER OH
            ALetter, // Lm   [6] OL CHIKI MU TTUDDAG..OL CHIKI AHAD
            Other,
            ALetter, // L&   [9] CYRILLIC SMALL LETTER ROUNDED VE..CYRILLIC SMALL LETTER UNBLENDED UK
            Other,
            ALetter, // L&  [43] GEORGIAN MTAVRULI CAPITAL LETTER AN..GEORGIAN MTAVRULI CAPITAL LETTER AIN
            Other,
            ALetter, // L&   [3] GEORGIAN MTAVRULI CAPITAL LETTER AEN..GEORGIAN MTAVRULI CAPITAL LETTER LABIAL SIGN
            Other,
            Extend, // Mn   [3] VEDIC TONE KARSHANA..VEDIC TONE PRENKHA
            Other,
            Extend, // Mn  [13] VEDIC SIGN YAJURVEDIC MIDLINE SVARITA..VEDIC TONE RIGVEDIC KASHMIRI INDEPENDENT SVARITA
            Extend, // Mc       VEDIC TONE ATHARVAVEDIC INDEPENDENT SVARITA
            Extend, // Mn   [7] VEDIC SIGN VISARGA SVARITA..VEDIC SIGN VISARGA ANUDATTA WITH TAIL
            ALetter, // Lo   [4] VEDIC SIGN ANUSVARA ANTARGOMUKHA..VEDIC SIGN ANUSVARA VAMAGOMUKHA WITH TAIL
            Extend, // Mn       VEDIC SIGN TIRYAK
            ALetter, // Lo   [6] VEDIC SIGN HEXIFORM LONG ANUSVARA..VEDIC SIGN ROTATED ARDHAVISARGA
            Extend, // Mn       VEDIC TONE CANDRA ABOVE
            ALetter, // Lo   [2] VEDIC SIGN JIHVAMULIYA..VEDIC SIGN UPADHMANIYA
            Extend, // Mc       VEDIC SIGN ATIKRAMA
            Extend, // Mn   [2] VEDIC TONE RING ABOVE..VEDIC TONE DOUBLE RING ABOVE
            ALetter, // Lo       VEDIC SIGN DOUBLE ANUSVARA ANTARGOMUKHA
            Other,
            ALetter, // L&  [44] LATIN LETTER SMALL CAPITAL A..CYRILLIC LETTER SMALL CAPITAL EL
            ALetter, // Lm  [63] MODIFIER LETTER CAPITAL A..GREEK SUBSCRIPT SMALL LETTER CHI
            ALetter, // L&  [13] LATIN SMALL LETTER UE..LATIN SMALL LETTER TURNED G
            ALetter, // Lm       MODIFIER LETTER CYRILLIC EN
            ALetter, // L&  [34] LATIN SMALL LETTER INSULAR G..LATIN SMALL LETTER EZH WITH RETROFLEX HOOK
            ALetter, // Lm  [37] MODIFIER LETTER SMALL TURNED ALPHA..MODIFIER LETTER SMALL THETA
            Extend, // Mn  [58] COMBINING DOTTED GRAVE ACCENT..COMBINING WIDE INVERTED BRIDGE BELOW
            Other,
            Extend, // Mn   [5] COMBINING DELETION MARK..COMBINING RIGHT ARROWHEAD AND DOWN ARROWHEAD BELOW
            ALetter, // L& [278] LATIN CAPITAL LETTER A WITH RING BELOW..GREEK SMALL LETTER EPSILON WITH DASIA AND OXIA
            Other,
            ALetter, // L&   [6] GREEK CAPITAL LETTER EPSILON WITH PSILI..GREEK CAPITAL LETTER EPSILON WITH DASIA AND OXIA
            Other,
            ALetter, // L&  [38] GREEK SMALL LETTER ETA WITH PSILI..GREEK SMALL LETTER OMICRON WITH DASIA AND OXIA
            Other,
            ALetter, // L&   [6] GREEK CAPITAL LETTER OMICRON WITH PSILI..GREEK CAPITAL LETTER OMICRON WITH DASIA AND OXIA
            Other,
            ALetter, // L&   [8] GREEK SMALL LETTER UPSILON WITH PSILI..GREEK SMALL LETTER UPSILON WITH DASIA AND PERISPOMENI
            Other,
            ALetter, // L&       GREEK CAPITAL LETTER UPSILON WITH DASIA
            Other,
            ALetter, // L&       GREEK CAPITAL LETTER UPSILON WITH DASIA AND VARIA
            Other,
            ALetter, // L&       GREEK CAPITAL LETTER UPSILON WITH DASIA AND OXIA
            Other,
            ALetter, // L&  [31] GREEK CAPITAL LETTER UPSILON WITH DASIA AND PERISPOMENI..GREEK SMALL LETTER OMEGA WITH OXIA
            Other,
            ALetter, // L&  [53] GREEK SMALL LETTER ALPHA WITH PSILI AND YPOGEGRAMMENI..GREEK SMALL LETTER ALPHA WITH OXIA AND YPOGEGRAMMENI
            Other,
            ALetter, // L&   [7] GREEK SMALL LETTER ALPHA WITH PERISPOMENI..GREEK CAPITAL LETTER ALPHA WITH PROSGEGRAMMENI
            Other,
            ALetter, // L&       GREEK PROSGEGRAMMENI
            Other,
            ALetter, // L&   [3] GREEK SMALL LETTER ETA WITH VARIA AND YPOGEGRAMMENI..GREEK SMALL LETTER ETA WITH OXIA AND YPOGEGRAMMENI
            Other,
            ALetter, // L&   [7] GREEK SMALL LETTER ETA WITH PERISPOMENI..GREEK CAPITAL LETTER ETA WITH PROSGEGRAMMENI
            Other,
            ALetter, // L&   [4] GREEK SMALL LETTER IOTA WITH VRACHY..GREEK SMALL LETTER IOTA WITH DIALYTIKA AND OXIA
            Other,
            ALetter, // L&   [6] GREEK SMALL LETTER IOTA WITH PERISPOMENI..GREEK CAPITAL LETTER IOTA WITH OXIA
            Other,
            ALetter, // L&  [13] GREEK SMALL LETTER UPSILON WITH VRACHY..GREEK CAPITAL LETTER RHO WITH DASIA
            Other,
            ALetter, // L&   [3] GREEK SMALL LETTER OMEGA WITH VARIA AND YPOGEGRAMMENI..GREEK SMALL LETTER OMEGA WITH OXIA AND YPOGEGRAMMENI
            Other,
            ALetter, // L&   [7] GREEK SMALL LETTER OMEGA WITH PERISPOMENI..GREEK CAPITAL LETTER OMEGA WITH PROSGEGRAMMENI
            Other,
            WSegSpace, // Zs   [7] EN QUAD..SIX-PER-EM SPACE
            Other,
            WSegSpace, // Zs   [3] PUNCTUATION SPACE..HAIR SPACE
            Other,
            Extend, // Cf       ZERO WIDTH NON-JOINER
            ZWJ, // Cf       ZERO WIDTH JOINER
            Format, // Cf   [2] LEFT-TO-RIGHT MARK..RIGHT-TO-LEFT MARK
            Other,
            MidNumLet, // Pi       LEFT SINGLE QUOTATION MARK
            MidNumLet, // Pf       RIGHT SINGLE QUOTATION MARK
            Other,
            MidNumLet, // Po       ONE DOT LEADER
            Other,
            MidLetter, // Po       HYPHENATION POINT
            Newline, // Zl       LINE SEPARATOR
            Newline, // Zp       PARAGRAPH SEPARATOR
            Format, // Cf   [5] LEFT-TO-RIGHT EMBEDDING..RIGHT-TO-LEFT OVERRIDE
            ExtendNumLet, // Zs       NARROW NO-BREAK SPACE
            Other,
            ExtendNumLet, // Pc   [2] UNDERTIE..CHARACTER TIE
            Other,
            MidNum, // Sm       FRACTION SLASH
            Other,
            ExtendNumLet, // Pc       INVERTED UNDERTIE
            Other,
            WSegSpace, // Zs       MEDIUM MATHEMATICAL SPACE
            Format, // Cf   [5] WORD JOINER..INVISIBLE PLUS
            Other,
            Format, // Cf  [10] LEFT-TO-RIGHT ISOLATE..NOMINAL DIGIT SHAPES
            Other,
            ALetter, // Lm       SUPERSCRIPT LATIN SMALL LETTER I
            Other,
            ALetter, // Lm       SUPERSCRIPT LATIN SMALL LETTER N
            Other,
            ALetter, // Lm  [13] LATIN SUBSCRIPT SMALL LETTER A..LATIN SUBSCRIPT SMALL LETTER T
            Other,
            Extend, // Mn  [13] COMBINING LEFT HARPOON ABOVE..COMBINING FOUR DOTS ABOVE
            Extend, // Me   [4] COMBINING ENCLOSING CIRCLE..COMBINING ENCLOSING CIRCLE BACKSLASH
            Extend, // Mn       COMBINING LEFT RIGHT ARROW ABOVE
            Extend, // Me   [3] COMBINING ENCLOSING SCREEN..COMBINING ENCLOSING UPWARD POINTING TRIANGLE
            Extend, // Mn  [12] COMBINING REVERSE SOLIDUS OVERLAY..COMBINING ASTERISK ABOVE
            Other,
            ALetter, // L&       DOUBLE-STRUCK CAPITAL C
            Other,
            ALetter, // L&       EULER CONSTANT
            Other,
            ALetter, // L&  [10] SCRIPT SMALL G..SCRIPT SMALL L
            Other,
            ALetter, // L&       DOUBLE-STRUCK CAPITAL N
            Other,
            ALetter, // L&   [5] DOUBLE-STRUCK CAPITAL P..DOUBLE-STRUCK CAPITAL R
            Other,
            ALetter, // L&       DOUBLE-STRUCK CAPITAL Z
            Other,
            ALetter, // L&       OHM SIGN
            Other,
            ALetter, // L&       BLACK-LETTER CAPITAL Z
            Other,
            ALetter, // L&   [4] KELVIN SIGN..BLACK-LETTER CAPITAL C
            Other,
            ALetter, // L&   [6] SCRIPT SMALL E..SCRIPT SMALL O
            ALetter, // Lo   [4] ALEF SYMBOL..DALET SYMBOL
            ALetter, // L&       INFORMATION SOURCE
            Other,
            ALetter, // L&   [4] DOUBLE-STRUCK SMALL PI..DOUBLE-STRUCK CAPITAL PI
            Other,
            ALetter, // L&   [5] DOUBLE-STRUCK ITALIC CAPITAL D..DOUBLE-STRUCK ITALIC SMALL J
            Other,
            ALetter, // L&       TURNED SMALL F
            Other,
            ALetter, // Nl  [35] ROMAN NUMERAL ONE..ROMAN NUMERAL TEN THOUSAND
            ALetter, // L&   [2] ROMAN NUMERAL REVERSED ONE HUNDRED..LATIN SMALL LETTER REVERSED C
            ALetter, // Nl   [4] ROMAN NUMERAL SIX LATE FORM..ROMAN NUMERAL ONE HUNDRED THOUSAND
            Other,
            ALetter, // So  [52] CIRCLED LATIN CAPITAL LETTER A..CIRCLED LATIN SMALL LETTER Z
            Other,
            ALetter, // L&  [47] GLAGOLITIC CAPITAL LETTER AZU..GLAGOLITIC CAPITAL LETTER LATINATE MYSLITE
            Other,
            ALetter, // L&  [47] GLAGOLITIC SMALL LETTER AZU..GLAGOLITIC SMALL LETTER LATINATE MYSLITE
            Other,
            ALetter, // L&  [28] LATIN CAPITAL LETTER L WITH DOUBLE BAR..LATIN LETTER SMALL CAPITAL TURNED E
            ALetter, // Lm   [2] LATIN SUBSCRIPT SMALL LETTER J..MODIFIER LETTER CAPITAL V
            ALetter, // L& [103] LATIN CAPITAL LETTER S WITH SWASH TAIL..COPTIC SYMBOL KAI
            Other,
            ALetter, // L&   [4] COPTIC CAPITAL LETTER CRYPTOGRAMMIC SHEI..COPTIC SMALL LETTER CRYPTOGRAMMIC GANGIA
            Extend, // Mn   [3] COPTIC COMBINING NI ABOVE..COPTIC COMBINING SPIRITUS LENIS
            ALetter, // L&   [2] COPTIC CAPITAL LETTER BOHAIRIC KHEI..COPTIC SMALL LETTER BOHAIRIC KHEI
            Other,
            ALetter, // L&  [38] GEORGIAN SMALL LETTER AN..GEORGIAN SMALL LETTER HOE
            Other,
            ALetter, // L&       GEORGIAN SMALL LETTER YN
            Other,
            ALetter, // L&       GEORGIAN SMALL LETTER AEN
            Other,
            ALetter, // Lo  [56] TIFINAGH LETTER YA..TIFINAGH LETTER YO
            Other,
            ALetter, // Lm       TIFINAGH MODIFIER LETTER LABIALIZATION MARK
            Other,
            Extend, // Mn       TIFINAGH CONSONANT JOINER
            ALetter, // Lo  [23] ETHIOPIC SYLLABLE LOA..ETHIOPIC SYLLABLE GGWE
            Other,
            ALetter, // Lo   [7] ETHIOPIC SYLLABLE SSA..ETHIOPIC SYLLABLE SSO
            Other,
            ALetter, // Lo   [7] ETHIOPIC SYLLABLE CCA..ETHIOPIC SYLLABLE CCO
            Other,
            ALetter, // Lo   [7] ETHIOPIC SYLLABLE ZZA..ETHIOPIC SYLLABLE ZZO
            Other,
            ALetter, // Lo   [7] ETHIOPIC SYLLABLE CCHA..ETHIOPIC SYLLABLE CCHO
            Other,
            ALetter, // Lo   [7] ETHIOPIC SYLLABLE QYA..ETHIOPIC SYLLABLE QYO
            Other,
            ALetter, // Lo   [7] ETHIOPIC SYLLABLE KYA..ETHIOPIC SYLLABLE KYO
            Other,
            ALetter, // Lo   [7] ETHIOPIC SYLLABLE XYA..ETHIOPIC SYLLABLE XYO
            Other,
            ALetter, // Lo   [7] ETHIOPIC SYLLABLE GYA..ETHIOPIC SYLLABLE GYO
            Other,
            Extend, // Mn  [32] COMBINING CYRILLIC LETTER BE..COMBINING CYRILLIC LETTER IOTIFIED BIG YUS
            Other,
            ALetter, // Lm       VERTICAL TILDE
            Other,
            WSegSpace, // Zs       IDEOGRAPHIC SPACE
            Other,
            ALetter, // Lm       IDEOGRAPHIC ITERATION MARK
            Other,
            Extend, // Mn   [4] IDEOGRAPHIC LEVEL TONE MARK..IDEOGRAPHIC ENTERING TONE MARK
            Extend, // Mc   [2] HANGUL SINGLE DOT TONE MARK..HANGUL DOUBLE DOT TONE MARK
            Other,
            Katakana, // Lm   [5] VERTICAL KANA REPEAT MARK..VERTICAL KANA REPEAT MARK LOWER HALF
            Other,
            ALetter, // Lm       VERTICAL IDEOGRAPHIC ITERATION MARK
            ALetter, // Lo       MASU MARK
            Other,
            Extend, // Mn   [2] COMBINING KATAKANA-HIRAGANA VOICED SOUND MARK..COMBINING KATAKANA-HIRAGANA SEMI-VOICED SOUND MARK
            Katakana, // Sk   [2] KATAKANA-HIRAGANA VOICED SOUND MARK..KATAKANA-HIRAGANA SEMI-VOICED SOUND MARK
            Other,
            Katakana, // Pd       KATAKANA-HIRAGANA DOUBLE HYPHEN
            Katakana, // Lo  [90] KATAKANA LETTER SMALL A..KATAKANA LETTER VO
            Other,
            Katakana, // Lm   [3] KATAKANA-HIRAGANA PROLONGED SOUND MARK..KATAKANA VOICED ITERATION MARK
            Katakana, // Lo       KATAKANA DIGRAPH KOTO
            Other,
            ALetter, // Lo  [43] BOPOMOFO LETTER B..BOPOMOFO LETTER NN
            Other,
            ALetter, // Lo  [94] HANGUL LETTER KIYEOK..HANGUL LETTER ARAEAE
            Other,
            ALetter, // Lo  [27] BOPOMOFO LETTER BU..BOPOMOFO LETTER ZY
            Other,
            Katakana, // Lo  [16] KATAKANA LETTER SMALL KU..KATAKANA LETTER SMALL RO
            Other,
            Katakana, // So  [47] CIRCLED KATAKANA A..CIRCLED KATAKANA WO
            Other,
            Katakana, // So  [88] SQUARE APAATO..SQUARE WATTO
            Other,
            ALetter, // Lo  [21] YI SYLLABLE IT..YI SYLLABLE E
            ALetter, // Lm       YI SYLLABLE WU
            ALetter, // Lo [1143] YI SYLLABLE BIT..YI SYLLABLE YYR
            Other,
            ALetter, // Lo  [40] LISU LETTER BA..LISU LETTER OE
            ALetter, // Lm   [6] LISU LETTER TONE MYA TI..LISU LETTER TONE MYA JEU
            Other,
            ALetter, // Lo [268] VAI SYLLABLE EE..VAI SYLLABLE NG
            ALetter, // Lm       VAI SYLLABLE LENGTHENER
            Other,
            ALetter, // Lo  [16] VAI SYLLABLE NDOLE FA..VAI SYMBOL JONG
            Numeric, // Nd  [10] VAI DIGIT ZERO..VAI DIGIT NINE
            ALetter, // Lo   [2] VAI SYLLABLE NDOLE MA..VAI SYLLABLE NDOLE DO
            Other,
            ALetter, // L&  [46] CYRILLIC CAPITAL LETTER ZEMLYA..CYRILLIC SMALL LETTER DOUBLE MONOCULAR O
            ALetter, // Lo       CYRILLIC LETTER MULTIOCULAR O
            Extend, // Mn       COMBINING CYRILLIC VZMET
            Extend, // Me   [3] COMBINING CYRILLIC TEN MILLIONS SIGN..COMBINING CYRILLIC THOUSAND MILLIONS SIGN
            Other,
            Extend, // Mn  [10] COMBINING CYRILLIC LETTER UKRAINIAN IE..COMBINING CYRILLIC PAYEROK
            Other,
            ALetter, // Lm       CYRILLIC PAYEROK
            ALetter, // L&  [28] CYRILLIC CAPITAL LETTER DWE..CYRILLIC SMALL LETTER CROSSED O
            ALetter, // Lm   [2] MODIFIER LETTER CYRILLIC HARD SIGN..MODIFIER LETTER CYRILLIC SOFT SIGN
            Extend, // Mn   [2] COMBINING CYRILLIC LETTER EF..COMBINING CYRILLIC LETTER IOTIFIED E
            ALetter, // Lo  [70] BAMUM LETTER A..BAMUM LETTER KI
            ALetter, // Nl  [10] BAMUM LETTER MO..BAMUM LETTER KOGHOM
            Extend, // Mn   [2] BAMUM COMBINING MARK KOQNDON..BAMUM COMBINING MARK TUKWENTIS
            Other,
            ALetter, // Lm   [9] MODIFIER LETTER DOT VERTICAL BAR..MODIFIER LETTER LOW INVERTED EXCLAMATION MARK
            ALetter, // Sk   [2] MODIFIER LETTER STRESS AND HIGH TONE..MODIFIER LETTER STRESS AND LOW TONE
            ALetter, // L&  [78] LATIN CAPITAL LETTER EGYPTOLOGICAL ALEF..LATIN SMALL LETTER CON
            ALetter, // Lm       MODIFIER LETTER US
            ALetter, // L&  [23] LATIN SMALL LETTER DUM..LATIN SMALL LETTER INSULAR T
            ALetter, // Lm       MODIFIER LETTER LOW CIRCUMFLEX ACCENT
            ALetter, // Sk   [2] MODIFIER LETTER COLON..MODIFIER LETTER SHORT EQUALS SIGN
            ALetter, // L&   [4] LATIN CAPITAL LETTER SALTILLO..LATIN SMALL LETTER L WITH RETROFLEX HOOK AND BELT
            ALetter, // Lo       LATIN LETTER SINOLOGICAL DOT
            ALetter, // L&  [48] LATIN CAPITAL LETTER N WITH DESCENDER..LATIN SMALL LETTER GLOTTAL U
            Other,
            ALetter, // L&   [5] LATIN CAPITAL LETTER ANGLICANA W..LATIN CAPITAL LETTER Z WITH PALATAL HOOK
            Other,
            ALetter, // Lo       LATIN EPIGRAPHIC LETTER SIDEWAYS I
            ALetter, // Lm   [2] MODIFIER LETTER CAPITAL H WITH STROKE..MODIFIER LETTER SMALL LIGATURE OE
            ALetter, // L&       LATIN LETTER SMALL CAPITAL TURNED M
            ALetter, // Lo   [7] LATIN EPIGRAPHIC LETTER REVERSED F..SYLOTI NAGRI LETTER I
            Extend, // Mn       SYLOTI NAGRI SIGN DVISVARA
            ALetter, // Lo   [3] SYLOTI NAGRI LETTER U..SYLOTI NAGRI LETTER O
            Extend, // Mn       SYLOTI NAGRI SIGN HASANTA
            ALetter, // Lo   [4] SYLOTI NAGRI LETTER KO..SYLOTI NAGRI LETTER GHO
            Extend, // Mn       SYLOTI NAGRI SIGN ANUSVARA
            ALetter, // Lo  [23] SYLOTI NAGRI LETTER CO..SYLOTI NAGRI LETTER HO
            Extend, // Mc   [2] SYLOTI NAGRI VOWEL SIGN A..SYLOTI NAGRI VOWEL SIGN I
            Extend, // Mn   [2] SYLOTI NAGRI VOWEL SIGN U..SYLOTI NAGRI VOWEL SIGN E
            Extend, // Mc       SYLOTI NAGRI VOWEL SIGN OO
            Other,
            ALetter, // Lo  [52] PHAGS-PA LETTER KA..PHAGS-PA LETTER CANDRABINDU
            Other,
            Extend, // Mc   [2] SAURASHTRA SIGN ANUSVARA..SAURASHTRA SIGN VISARGA
            ALetter, // Lo  [50] SAURASHTRA LETTER A..SAURASHTRA LETTER LLA
            Extend, // Mc  [16] SAURASHTRA CONSONANT SIGN HAARU..SAURASHTRA VOWEL SIGN AU
            Extend, // Mn   [2] SAURASHTRA SIGN VIRAMA..SAURASHTRA SIGN CANDRABINDU
            Other,
            Numeric, // Nd  [10] SAURASHTRA DIGIT ZERO..SAURASHTRA DIGIT NINE
            Other,
            Extend, // Mn  [18] COMBINING DEVANAGARI DIGIT ZERO..COMBINING DEVANAGARI SIGN AVAGRAHA
            ALetter, // Lo   [6] DEVANAGARI SIGN SPACING CANDRABINDU..DEVANAGARI SIGN CANDRABINDU AVAGRAHA
            Other,
            ALetter, // Lo       DEVANAGARI HEADSTROKE
            Other,
            ALetter, // Lo   [2] DEVANAGARI JAIN OM..DEVANAGARI LETTER AY
            Extend, // Mn       DEVANAGARI VOWEL SIGN AY
            Numeric, // Nd  [10] KAYAH LI DIGIT ZERO..KAYAH LI DIGIT NINE
            ALetter, // Lo  [28] KAYAH LI LETTER KA..KAYAH LI LETTER OO
            Extend, // Mn   [8] KAYAH LI VOWEL UE..KAYAH LI TONE CALYA PLOPHU
            Other,
            ALetter, // Lo  [23] REJANG LETTER KA..REJANG LETTER A
            Extend, // Mn  [11] REJANG VOWEL SIGN I..REJANG CONSONANT SIGN R
            Extend, // Mc   [2] REJANG CONSONANT SIGN H..REJANG VIRAMA
            Other,
            ALetter, // Lo  [29] HANGUL CHOSEONG TIKEUT-MIEUM..HANGUL CHOSEONG SSANGYEORINHIEUH
            Other,
            Extend, // Mn   [3] JAVANESE SIGN PANYANGGA..JAVANESE SIGN LAYAR
            Extend, // Mc       JAVANESE SIGN WIGNYAN
            ALetter, // Lo  [47] JAVANESE LETTER A..JAVANESE LETTER HA
            Extend, // Mn       JAVANESE SIGN CECAK TELU
            Extend, // Mc   [2] JAVANESE VOWEL SIGN TARUNG..JAVANESE VOWEL SIGN TOLONG
            Extend, // Mn   [4] JAVANESE VOWEL SIGN WULU..JAVANESE VOWEL SIGN SUKU MENDUT
            Extend, // Mc   [2] JAVANESE VOWEL SIGN TALING..JAVANESE VOWEL SIGN DIRGA MURE
            Extend, // Mn   [2] JAVANESE VOWEL SIGN PEPET..JAVANESE CONSONANT SIGN KERET
            Extend, // Mc   [3] JAVANESE CONSONANT SIGN PENGKAL..JAVANESE PANGKON
            Other,
            ALetter, // Lm       JAVANESE PANGRANGKEP
            Numeric, // Nd  [10] JAVANESE DIGIT ZERO..JAVANESE DIGIT NINE
            Other,
            Extend, // Mn       MYANMAR SIGN SHAN SAW
            Other,
            Numeric, // Nd  [10] MYANMAR TAI LAING DIGIT ZERO..MYANMAR TAI LAING DIGIT NINE
            Other,
            ALetter, // Lo  [41] CHAM LETTER A..CHAM LETTER HA
            Extend, // Mn   [6] CHAM VOWEL SIGN AA..CHAM VOWEL SIGN OE
            Extend, // Mc   [2] CHAM VOWEL SIGN O..CHAM VOWEL SIGN AI
            Extend, // Mn   [2] CHAM VOWEL SIGN AU..CHAM VOWEL SIGN UE
            Extend, // Mc   [2] CHAM CONSONANT SIGN YA..CHAM CONSONANT SIGN RA
            Extend, // Mn   [2] CHAM CONSONANT SIGN LA..CHAM CONSONANT SIGN WA
            Other,
            ALetter, // Lo   [3] CHAM LETTER FINAL K..CHAM LETTER FINAL NG
            Extend, // Mn       CHAM CONSONANT SIGN FINAL NG
            ALetter, // Lo   [8] CHAM LETTER FINAL CH..CHAM LETTER FINAL SS
            Extend, // Mn       CHAM CONSONANT SIGN FINAL M
            Extend, // Mc       CHAM CONSONANT SIGN FINAL H
            Other,
            Numeric, // Nd  [10] CHAM DIGIT ZERO..CHAM DIGIT NINE
            Other,
            Extend, // Mc       MYANMAR SIGN PAO KAREN TONE
            Extend, // Mn       MYANMAR SIGN TAI LAING TONE-2
            Extend, // Mc       MYANMAR SIGN TAI LAING TONE-5
            Other,
            Extend, // Mn       TAI VIET MAI KANG
            Other,
            Extend, // Mn   [3] TAI VIET VOWEL I..TAI VIET VOWEL U
            Other,
            Extend, // Mn   [2] TAI VIET MAI KHIT..TAI VIET VOWEL IA
            Other,
            Extend, // Mn   [2] TAI VIET VOWEL AM..TAI VIET TONE MAI EK
            Other,
            Extend, // Mn       TAI VIET TONE MAI THO
            Other,
            ALetter, // Lo  [11] MEETEI MAYEK LETTER E..MEETEI MAYEK LETTER SSA
            Extend, // Mc       MEETEI MAYEK VOWEL SIGN II
            Extend, // Mn   [2] MEETEI MAYEK VOWEL SIGN UU..MEETEI MAYEK VOWEL SIGN AAI
            Extend, // Mc   [2] MEETEI MAYEK VOWEL SIGN AU..MEETEI MAYEK VOWEL SIGN AAU
            Other,
            ALetter, // Lo       MEETEI MAYEK ANJI
            ALetter, // Lm   [2] MEETEI MAYEK SYLLABLE REPETITION MARK..MEETEI MAYEK WORD REPETITION MARK
            Extend, // Mc       MEETEI MAYEK VOWEL SIGN VISARGA
            Extend, // Mn       MEETEI MAYEK VIRAMA
            Other,
            ALetter, // Lo   [6] ETHIOPIC SYLLABLE TTHU..ETHIOPIC SYLLABLE TTHO
            Other,
            ALetter, // Lo   [6] ETHIOPIC SYLLABLE DDHU..ETHIOPIC SYLLABLE DDHO
            Other,
            ALetter, // Lo   [6] ETHIOPIC SYLLABLE DZU..ETHIOPIC SYLLABLE DZO
            Other,
            ALetter, // Lo   [7] ETHIOPIC SYLLABLE CCHHA..ETHIOPIC SYLLABLE CCHHO
            Other,
            ALetter, // Lo   [7] ETHIOPIC SYLLABLE BBA..ETHIOPIC SYLLABLE BBO
            Other,
            ALetter, // L&  [43] LATIN SMALL LETTER BARRED ALPHA..LATIN SMALL LETTER Y WITH SHORT RIGHT LEG
            ALetter, // Sk       MODIFIER BREVE WITH INVERTED BREVE
            ALetter, // Lm   [4] MODIFIER LETTER SMALL HENG..MODIFIER LETTER SMALL U WITH LEFT HOOK
            ALetter, // L&   [8] LATIN SMALL LETTER SAKHA YAT..LATIN SMALL LETTER TS DIGRAPH WITH RETROFLEX HOOK
            Other,
            ALetter, // L&  [80] CHEROKEE SMALL LETTER A..CHEROKEE SMALL LETTER YA
            ALetter, // Lo  [35] MEETEI MAYEK LETTER KOK..MEETEI MAYEK LETTER I LONSUM
            Extend, // Mc   [2] MEETEI MAYEK VOWEL SIGN ONAP..MEETEI MAYEK VOWEL SIGN INAP
            Extend, // Mn       MEETEI MAYEK VOWEL SIGN ANAP
            Extend, // Mc   [2] MEETEI MAYEK VOWEL SIGN YENAP..MEETEI MAYEK VOWEL SIGN SOUNAP
            Extend, // Mn       MEETEI MAYEK VOWEL SIGN UNAP
            Extend, // Mc   [2] MEETEI MAYEK VOWEL SIGN CHEINAP..MEETEI MAYEK VOWEL SIGN NUNG
            Other,
            Extend, // Mc       MEETEI MAYEK LUM IYEK
            Extend, // Mn       MEETEI MAYEK APUN IYEK
            Other,
            Numeric, // Nd  [10] MEETEI MAYEK DIGIT ZERO..MEETEI MAYEK DIGIT NINE
            Other,
            ALetter, // Lo [11172] HANGUL SYLLABLE GA..HANGUL SYLLABLE HIH
            Other,
            ALetter, // Lo  [23] HANGUL JUNGSEONG O-YEO..HANGUL JUNGSEONG ARAEA-E
            Other,
            ALetter, // Lo  [49] HANGUL JONGSEONG NIEUN-RIEUL..HANGUL JONGSEONG PHIEUPH-THIEUTH
            Other,
            ALetter, // L&   [7] LATIN SMALL LIGATURE FF..LATIN SMALL LIGATURE ST
            Other,
            ALetter, // L&   [5] ARMENIAN SMALL LIGATURE MEN NOW..ARMENIAN SMALL LIGATURE MEN XEH
            Other,
            Hebrew_Letter, // Lo       HEBREW LETTER YOD WITH HIRIQ
            Extend, // Mn       HEBREW POINT JUDEO-SPANISH VARIKA
            Hebrew_Letter, // Lo  [10] HEBREW LIGATURE YIDDISH YOD YOD PATAH..HEBREW LETTER WIDE TAV
            Other,
            Hebrew_Letter, // Lo  [13] HEBREW LETTER SHIN WITH SHIN DOT..HEBREW LETTER ZAYIN WITH DAGESH
            Other,
            Hebrew_Letter, // Lo   [5] HEBREW LETTER TET WITH DAGESH..HEBREW LETTER LAMED WITH DAGESH
            Other,
            Hebrew_Letter, // Lo       HEBREW LETTER MEM WITH DAGESH
            Other,
            Hebrew_Letter, // Lo   [2] HEBREW LETTER NUN WITH DAGESH..HEBREW LETTER SAMEKH WITH DAGESH
            Other,
            Hebrew_Letter, // Lo   [2] HEBREW LETTER FINAL PE WITH DAGESH..HEBREW LETTER PE WITH DAGESH
            Other,
            Hebrew_Letter, // Lo  [10] HEBREW LETTER TSADI WITH DAGESH..HEBREW LIGATURE ALEF LAMED
            ALetter, // Lo  [98] ARABIC LETTER ALEF WASLA ISOLATED FORM..ARABIC LETTER YEH BARREE WITH HAMZA ABOVE FINAL FORM
            Other,
            ALetter, // Lo [363] ARABIC LETTER NG ISOLATED FORM..ARABIC LIGATURE ALEF WITH FATHATAN ISOLATED FORM
            Other,
            ALetter, // Lo  [64] ARABIC LIGATURE TEH WITH JEEM WITH MEEM INITIAL FORM..ARABIC LIGATURE MEEM WITH KHAH WITH MEEM INITIAL FORM
            Other,
            ALetter, // Lo  [54] ARABIC LIGATURE MEEM WITH JEEM WITH KHAH INITIAL FORM..ARABIC LIGATURE NOON WITH JEEM WITH YEH FINAL FORM
            Other,
            ALetter, // Lo  [12] ARABIC LIGATURE SALLA USED AS KORANIC STOP SIGN ISOLATED FORM..ARABIC LIGATURE JALLAJALALOUHOU
            Other,
            Extend, // Mn  [16] VARIATION SELECTOR-1..VARIATION SELECTOR-16
            MidNum, // Po       PRESENTATION FORM FOR VERTICAL COMMA
            Other,
            MidLetter, // Po       PRESENTATION FORM FOR VERTICAL COLON
            MidNum, // Po       PRESENTATION FORM FOR VERTICAL SEMICOLON
            Other,
            Extend, // Mn  [16] COMBINING LIGATURE LEFT HALF..COMBINING CYRILLIC TITLO RIGHT HALF
            Other,
            ExtendNumLet, // Pc   [2] PRESENTATION FORM FOR VERTICAL LOW LINE..PRESENTATION FORM FOR VERTICAL WAVY LOW LINE
            Other,
            ExtendNumLet, // Pc   [3] DASHED LOW LINE..WAVY LOW LINE
            MidNum, // Po       SMALL COMMA
            Other,
            MidNumLet, // Po       SMALL FULL STOP
            Other,
            MidNum, // Po       SMALL SEMICOLON
            MidLetter, // Po       SMALL COLON
            Other,
            ALetter, // Lo   [5] ARABIC FATHATAN ISOLATED FORM..ARABIC KASRATAN ISOLATED FORM
            Other,
            ALetter, // Lo [135] ARABIC FATHA ISOLATED FORM..ARABIC LIGATURE LAM WITH ALEF FINAL FORM
            Other,
            Format, // Cf       ZERO WIDTH NO-BREAK SPACE
            Other,
            MidNumLet, // Po       FULLWIDTH APOSTROPHE
            Other,
            MidNum, // Po       FULLWIDTH COMMA
            Other,
            MidNumLet, // Po       FULLWIDTH FULL STOP
            Other,
            Numeric, // Nd  [10] FULLWIDTH DIGIT ZERO..FULLWIDTH DIGIT NINE
            MidLetter, // Po       FULLWIDTH COLON
            MidNum, // Po       FULLWIDTH SEMICOLON
            Other,
            ALetter, // L&  [26] FULLWIDTH LATIN CAPITAL LETTER A..FULLWIDTH LATIN CAPITAL LETTER Z
            Other,
            ExtendNumLet, // Pc       FULLWIDTH LOW LINE
            Other,
            ALetter, // L&  [26] FULLWIDTH LATIN SMALL LETTER A..FULLWIDTH LATIN SMALL LETTER Z
            Other,
            Katakana, // Lo  [10] HALFWIDTH KATAKANA LETTER WO..HALFWIDTH KATAKANA LETTER SMALL TU
            Katakana, // Lm       HALFWIDTH KATAKANA-HIRAGANA PROLONGED SOUND MARK
            Katakana, // Lo  [45] HALFWIDTH KATAKANA LETTER A..HALFWIDTH KATAKANA LETTER N
            Extend, // Lm   [2] HALFWIDTH KATAKANA VOICED SOUND MARK..HALFWIDTH KATAKANA SEMI-VOICED SOUND MARK
            ALetter, // Lo  [31] HALFWIDTH HANGUL FILLER..HALFWIDTH HANGUL LETTER HIEUH
            Other,
            ALetter, // Lo   [6] HALFWIDTH HANGUL LETTER A..HALFWIDTH HANGUL LETTER E
            Other,
            ALetter, // Lo   [6] HALFWIDTH HANGUL LETTER YEO..HALFWIDTH HANGUL LETTER OE
            Other,
            ALetter, // Lo   [6] HALFWIDTH HANGUL LETTER YO..HALFWIDTH HANGUL LETTER YU
            Other,
            ALetter, // Lo   [3] HALFWIDTH HANGUL LETTER EU..HALFWIDTH HANGUL LETTER I
            Other,
            Format, // Cf   [3] INTERLINEAR ANNOTATION ANCHOR..INTERLINEAR ANNOTATION TERMINATOR
            Other,
            ALetter, // Lo  [12] LINEAR B SYLLABLE B008 A..LINEAR B SYLLABLE B046 JE
            Other,
            ALetter, // Lo  [26] LINEAR B SYLLABLE B036 JO..LINEAR B SYLLABLE B032 QO
            Other,
            ALetter, // Lo  [19] LINEAR B SYLLABLE B060 RA..LINEAR B SYLLABLE B042 WO
            Other,
            ALetter, // Lo   [2] LINEAR B SYLLABLE B017 ZA..LINEAR B SYLLABLE B074 ZE
            Other,
            ALetter, // Lo  [15] LINEAR B SYLLABLE B020 ZO..LINEAR B SYLLABLE B091 TWO
            Other,
            ALetter, // Lo  [14] LINEAR B SYMBOL B018..LINEAR B SYMBOL B089
            Other,
            ALetter, // Lo [123] LINEAR B IDEOGRAM B100 MAN..LINEAR B IDEOGRAM VESSEL B305
            Other,
            ALetter, // Nl  [53] GREEK ACROPHONIC ATTIC ONE QUARTER..GREEK ACROPHONIC STRATIAN FIFTY MNAS
            Other,
            Extend, // Mn       PHAISTOS DISC SIGN COMBINING OBLIQUE STROKE
            Other,
            ALetter, // Lo  [29] LYCIAN LETTER A..LYCIAN LETTER X
            Other,
            ALetter, // Lo  [49] CARIAN LETTER A..CARIAN LETTER UUU3
            Other,
            Extend, // Mn       COPTIC EPACT THOUSANDS MARK
            Other,
            ALetter, // Lo  [32] OLD ITALIC LETTER A..OLD ITALIC LETTER ESS
            Other,
            ALetter, // Lo  [20] OLD ITALIC LETTER YE..GOTHIC LETTER PAIRTHRA
            ALetter, // Nl       GOTHIC LETTER NINETY
            ALetter, // Lo   [8] GOTHIC LETTER RAIDA..GOTHIC LETTER OTHAL
            ALetter, // Nl       GOTHIC LETTER NINE HUNDRED
            Other,
            ALetter, // Lo  [38] OLD PERMIC LETTER AN..OLD PERMIC LETTER IA
            Extend, // Mn   [5] COMBINING OLD PERMIC LETTER AN..COMBINING OLD PERMIC LETTER SII
            Other,
            ALetter, // Lo  [30] UGARITIC LETTER ALPA..UGARITIC LETTER SSU
            Other,
            ALetter, // Lo  [36] OLD PERSIAN SIGN A..OLD PERSIAN SIGN HA
            Other,
            ALetter, // Lo   [8] OLD PERSIAN SIGN AURAMAZDAA..OLD PERSIAN SIGN BUUMISH
            Other,
            ALetter, // Nl   [5] OLD PERSIAN NUMBER ONE..OLD PERSIAN NUMBER HUNDRED
            Other,
            ALetter, // L&  [80] DESERET CAPITAL LETTER LONG I..DESERET SMALL LETTER EW
            ALetter, // Lo  [78] SHAVIAN LETTER PEEP..OSMANYA LETTER OO
            Other,
            Numeric, // Nd  [10] OSMANYA DIGIT ZERO..OSMANYA DIGIT NINE
            Other,
            ALetter, // L&  [36] OSAGE CAPITAL LETTER A..OSAGE CAPITAL LETTER ZHA
            Other,
            ALetter, // L&  [36] OSAGE SMALL LETTER A..OSAGE SMALL LETTER ZHA
            Other,
            ALetter, // Lo  [40] ELBASAN LETTER A..ELBASAN LETTER KHE
            Other,
            ALetter, // Lo  [52] CAUCASIAN ALBANIAN LETTER ALT..CAUCASIAN ALBANIAN LETTER KIW
            Other,
            ALetter, // Lo [311] LINEAR A SIGN AB001..LINEAR A SIGN A664
            Other,
            ALetter, // Lo  [22] LINEAR A SIGN A701 A..LINEAR A SIGN A732 JE
            Other,
            ALetter, // Lo   [8] LINEAR A SIGN A800..LINEAR A SIGN A807
            Other,
            ALetter, // Lo   [6] CYPRIOT SYLLABLE A..CYPRIOT SYLLABLE JA
            Other,
            ALetter, // Lo       CYPRIOT SYLLABLE JO
            Other,
            ALetter, // Lo  [44] CYPRIOT SYLLABLE KA..CYPRIOT SYLLABLE WO
            Other,
            ALetter, // Lo   [2] CYPRIOT SYLLABLE XA..CYPRIOT SYLLABLE XE
            Other,
            ALetter, // Lo       CYPRIOT SYLLABLE ZA
            Other,
            ALetter, // Lo  [23] CYPRIOT SYLLABLE ZO..IMPERIAL ARAMAIC LETTER TAW
            Other,
            ALetter, // Lo  [23] PALMYRENE LETTER ALEPH..PALMYRENE LETTER TAW
            Other,
            ALetter, // Lo  [31] NABATAEAN LETTER FINAL ALEPH..NABATAEAN LETTER TAW
            Other,
            ALetter, // Lo  [19] HATRAN LETTER ALEPH..HATRAN LETTER QOPH
            Other,
            ALetter, // Lo   [2] HATRAN LETTER SHIN..HATRAN LETTER TAW
            Other,
            ALetter, // Lo  [22] PHOENICIAN LETTER ALF..PHOENICIAN LETTER TAU
            Other,
            ALetter, // Lo  [26] LYDIAN LETTER A..LYDIAN LETTER C
            Other,
            ALetter, // Lo  [56] MEROITIC HIEROGLYPHIC LETTER A..MEROITIC CURSIVE LETTER DA
            Other,
            ALetter, // Lo   [2] MEROITIC CURSIVE LOGOGRAM RMT..MEROITIC CURSIVE LOGOGRAM IMN
            Other,
            ALetter, // Lo       KHAROSHTHI LETTER A
            Extend, // Mn   [3] KHAROSHTHI VOWEL SIGN I..KHAROSHTHI VOWEL SIGN VOCALIC R
            Other,
            Extend, // Mn   [2] KHAROSHTHI VOWEL SIGN E..KHAROSHTHI VOWEL SIGN O
            Other,
            Extend, // Mn   [4] KHAROSHTHI VOWEL LENGTH MARK..KHAROSHTHI SIGN VISARGA
            ALetter, // Lo   [4] KHAROSHTHI LETTER KA..KHAROSHTHI LETTER GHA
            Other,
            ALetter, // Lo   [3] KHAROSHTHI LETTER CA..KHAROSHTHI LETTER JA
            Other,
            ALetter, // Lo  [29] KHAROSHTHI LETTER NYA..KHAROSHTHI LETTER VHA
            Other,
            Extend, // Mn   [3] KHAROSHTHI SIGN BAR ABOVE..KHAROSHTHI SIGN DOT BELOW
            Other,
            Extend, // Mn       KHAROSHTHI VIRAMA
            Other,
            ALetter, // Lo  [29] OLD SOUTH ARABIAN LETTER HE..OLD SOUTH ARABIAN LETTER THETH
            Other,
            ALetter, // Lo  [29] OLD NORTH ARABIAN LETTER HEH..OLD NORTH ARABIAN LETTER ZAH
            Other,
            ALetter, // Lo   [8] MANICHAEAN LETTER ALEPH..MANICHAEAN LETTER WAW
            Other,
            ALetter, // Lo  [28] MANICHAEAN LETTER ZAYIN..MANICHAEAN LETTER TAW
            Extend, // Mn   [2] MANICHAEAN ABBREVIATION MARK ABOVE..MANICHAEAN ABBREVIATION MARK BELOW
            Other,
            ALetter, // Lo  [54] AVESTAN LETTER A..AVESTAN LETTER HE
            Other,
            ALetter, // Lo  [22] INSCRIPTIONAL PARTHIAN LETTER ALEPH..INSCRIPTIONAL PARTHIAN LETTER TAW
            Other,
            ALetter, // Lo  [19] INSCRIPTIONAL PAHLAVI LETTER ALEPH..INSCRIPTIONAL PAHLAVI LETTER TAW
            Other,
            ALetter, // Lo  [18] PSALTER PAHLAVI LETTER ALEPH..PSALTER PAHLAVI LETTER TAW
            Other,
            ALetter, // Lo  [73] OLD TURKIC LETTER ORKHON A..OLD TURKIC LETTER ORKHON BASH
            Other,
            ALetter, // L&  [51] OLD HUNGARIAN CAPITAL LETTER A..OLD HUNGARIAN CAPITAL LETTER US
            Other,
            ALetter, // L&  [51] OLD HUNGARIAN SMALL LETTER A..OLD HUNGARIAN SMALL LETTER US
            Other,
            ALetter, // Lo  [36] HANIFI ROHINGYA LETTER A..HANIFI ROHINGYA MARK NA KHONNA
            Extend, // Mn   [4] HANIFI ROHINGYA SIGN HARBAHAY..HANIFI ROHINGYA SIGN TASSI
            Other,
            Numeric, // Nd  [10] HANIFI ROHINGYA DIGIT ZERO..HANIFI ROHINGYA DIGIT NINE
            Other,
            ALetter, // Lo  [29] OLD SOGDIAN LETTER ALEPH..OLD SOGDIAN LETTER FINAL TAW WITH VERTICAL TAIL
            Other,
            ALetter, // Lo       OLD SOGDIAN LIGATURE AYIN-DALETH
            Other,
            ALetter, // Lo  [22] SOGDIAN LETTER ALEPH..SOGDIAN INDEPENDENT SHIN
            Extend, // Mn  [11] SOGDIAN COMBINING DOT BELOW..SOGDIAN COMBINING STROKE BELOW
            Other,
            ALetter, // Lo  [23] ELYMAIC LETTER ALEPH..ELYMAIC LIGATURE ZAYIN-YODH
            Other,
            Extend, // Mc       BRAHMI SIGN CANDRABINDU
            Extend, // Mn       BRAHMI SIGN ANUSVARA
            Extend, // Mc       BRAHMI SIGN VISARGA
            ALetter, // Lo  [53] BRAHMI SIGN JIHVAMULIYA..BRAHMI LETTER OLD TAMIL NNNA
            Extend, // Mn  [15] BRAHMI VOWEL SIGN AA..BRAHMI VIRAMA
            Other,
            Numeric, // Nd  [10] BRAHMI DIGIT ZERO..BRAHMI DIGIT NINE
            Other,
            Extend, // Mn   [3] BRAHMI NUMBER JOINER..KAITHI SIGN ANUSVARA
            Extend, // Mc       KAITHI SIGN VISARGA
            ALetter, // Lo  [45] KAITHI LETTER A..KAITHI LETTER HA
            Extend, // Mc   [3] KAITHI VOWEL SIGN AA..KAITHI VOWEL SIGN II
            Extend, // Mn   [4] KAITHI VOWEL SIGN U..KAITHI VOWEL SIGN AI
            Extend, // Mc   [2] KAITHI VOWEL SIGN O..KAITHI VOWEL SIGN AU
            Extend, // Mn   [2] KAITHI SIGN VIRAMA..KAITHI SIGN NUKTA
            Other,
            Format, // Cf       KAITHI NUMBER SIGN
            Other,
            Format, // Cf       KAITHI NUMBER SIGN ABOVE
            Other,
            ALetter, // Lo  [25] SORA SOMPENG LETTER SAH..SORA SOMPENG LETTER MAE
            Other,
            Numeric, // Nd  [10] SORA SOMPENG DIGIT ZERO..SORA SOMPENG DIGIT NINE
            Other,
            Extend, // Mn   [3] CHAKMA SIGN CANDRABINDU..CHAKMA SIGN VISARGA
            ALetter, // Lo  [36] CHAKMA LETTER AA..CHAKMA LETTER HAA
            Extend, // Mn   [5] CHAKMA VOWEL SIGN A..CHAKMA VOWEL SIGN UU
            Extend, // Mc       CHAKMA VOWEL SIGN E
            Extend, // Mn   [8] CHAKMA VOWEL SIGN AI..CHAKMA MAAYYAA
            Other,
            Numeric, // Nd  [10] CHAKMA DIGIT ZERO..CHAKMA DIGIT NINE
            Other,
            ALetter, // Lo       CHAKMA LETTER LHAA
            Extend, // Mc   [2] CHAKMA VOWEL SIGN AA..CHAKMA VOWEL SIGN EI
            Other,
            ALetter, // Lo  [35] MAHAJANI LETTER A..MAHAJANI LETTER RRA
            Extend, // Mn       MAHAJANI SIGN NUKTA
            Other,
            ALetter, // Lo       MAHAJANI LIGATURE SHRI
            Other,
            Extend, // Mn   [2] SHARADA SIGN CANDRABINDU..SHARADA SIGN ANUSVARA
            Extend, // Mc       SHARADA SIGN VISARGA
            ALetter, // Lo  [48] SHARADA LETTER A..SHARADA LETTER HA
            Extend, // Mc   [3] SHARADA VOWEL SIGN AA..SHARADA VOWEL SIGN II
            Extend, // Mn   [9] SHARADA VOWEL SIGN U..SHARADA VOWEL SIGN O
            Extend, // Mc   [2] SHARADA VOWEL SIGN AU..SHARADA SIGN VIRAMA
            ALetter, // Lo   [4] SHARADA SIGN AVAGRAHA..SHARADA OM
            Other,
            Extend, // Mn   [4] SHARADA SANDHI MARK..SHARADA EXTRA SHORT VOWEL MARK
            Other,
            Numeric, // Nd  [10] SHARADA DIGIT ZERO..SHARADA DIGIT NINE
            ALetter, // Lo       SHARADA EKAM
            Other,
            ALetter, // Lo       SHARADA HEADSTROKE
            Other,
            ALetter, // Lo  [18] KHOJKI LETTER A..KHOJKI LETTER JJA
            Other,
            ALetter, // Lo  [25] KHOJKI LETTER NYA..KHOJKI LETTER LLA
            Extend, // Mc   [3] KHOJKI VOWEL SIGN AA..KHOJKI VOWEL SIGN II
            Extend, // Mn   [3] KHOJKI VOWEL SIGN U..KHOJKI VOWEL SIGN AI
            Extend, // Mc   [2] KHOJKI VOWEL SIGN O..KHOJKI VOWEL SIGN AU
            Extend, // Mn       KHOJKI SIGN ANUSVARA
            Extend, // Mc       KHOJKI SIGN VIRAMA
            Extend, // Mn   [2] KHOJKI SIGN NUKTA..KHOJKI SIGN SHADDA
            Other,
            Extend, // Mn       KHOJKI SIGN SUKUN
            Other,
            ALetter, // Lo   [7] MULTANI LETTER A..MULTANI LETTER GA
            Other,
            ALetter, // Lo       MULTANI LETTER GHA
            Other,
            ALetter, // Lo   [4] MULTANI LETTER CA..MULTANI LETTER JJA
            Other,
            ALetter, // Lo  [15] MULTANI LETTER NYA..MULTANI LETTER BA
            Other,
            ALetter, // Lo  [10] MULTANI LETTER BHA..MULTANI LETTER RHA
            Other,
            ALetter, // Lo  [47] KHUDAWADI LETTER A..KHUDAWADI LETTER HA
            Extend, // Mn       KHUDAWADI SIGN ANUSVARA
            Extend, // Mc   [3] KHUDAWADI VOWEL SIGN AA..KHUDAWADI VOWEL SIGN II
            Extend, // Mn   [8] KHUDAWADI VOWEL SIGN U..KHUDAWADI SIGN VIRAMA
            Other,
            Numeric, // Nd  [10] KHUDAWADI DIGIT ZERO..KHUDAWADI DIGIT NINE
            Other,
            Extend, // Mn   [2] GRANTHA SIGN COMBINING ANUSVARA ABOVE..GRANTHA SIGN CANDRABINDU
            Extend, // Mc   [2] GRANTHA SIGN ANUSVARA..GRANTHA SIGN VISARGA
            Other,
            ALetter, // Lo   [8] GRANTHA LETTER A..GRANTHA LETTER VOCALIC L
            Other,
            ALetter, // Lo   [2] GRANTHA LETTER EE..GRANTHA LETTER AI
            Other,
            ALetter, // Lo  [22] GRANTHA LETTER OO..GRANTHA LETTER NA
            Other,
            ALetter, // Lo   [7] GRANTHA LETTER PA..GRANTHA LETTER RA
            Other,
            ALetter, // Lo   [2] GRANTHA LETTER LA..GRANTHA LETTER LLA
            Other,
            ALetter, // Lo   [5] GRANTHA LETTER VA..GRANTHA LETTER HA
            Other,
            Extend, // Mn   [2] COMBINING BINDU BELOW..GRANTHA SIGN NUKTA
            ALetter, // Lo       GRANTHA SIGN AVAGRAHA
            Extend, // Mc   [2] GRANTHA VOWEL SIGN AA..GRANTHA VOWEL SIGN I
            Extend, // Mn       GRANTHA VOWEL SIGN II
            Extend, // Mc   [4] GRANTHA VOWEL SIGN U..GRANTHA VOWEL SIGN VOCALIC RR
            Other,
            Extend, // Mc   [2] GRANTHA VOWEL SIGN EE..GRANTHA VOWEL SIGN AI
            Other,
            Extend, // Mc   [3] GRANTHA VOWEL SIGN OO..GRANTHA SIGN VIRAMA
            Other,
            ALetter, // Lo       GRANTHA OM
            Other,
            Extend, // Mc       GRANTHA AU LENGTH MARK
            Other,
            ALetter, // Lo   [5] GRANTHA SIGN PLUTA..GRANTHA LETTER VOCALIC LL
            Extend, // Mc   [2] GRANTHA VOWEL SIGN VOCALIC L..GRANTHA VOWEL SIGN VOCALIC LL
            Other,
            Extend, // Mn   [7] COMBINING GRANTHA DIGIT ZERO..COMBINING GRANTHA DIGIT SIX
            Other,
            Extend, // Mn   [5] COMBINING GRANTHA LETTER A..COMBINING GRANTHA LETTER PA
            Other,
            ALetter, // Lo  [53] NEWA LETTER A..NEWA LETTER HA
            Extend, // Mc   [3] NEWA VOWEL SIGN AA..NEWA VOWEL SIGN II
            Extend, // Mn   [8] NEWA VOWEL SIGN U..NEWA VOWEL SIGN AI
            Extend, // Mc   [2] NEWA VOWEL SIGN O..NEWA VOWEL SIGN AU
            Extend, // Mn   [3] NEWA SIGN VIRAMA..NEWA SIGN ANUSVARA
            Extend, // Mc       NEWA SIGN VISARGA
            Extend, // Mn       NEWA SIGN NUKTA
            ALetter, // Lo   [4] NEWA SIGN AVAGRAHA..NEWA SIDDHI
            Other,
            Numeric, // Nd  [10] NEWA DIGIT ZERO..NEWA DIGIT NINE
            Other,
            Extend, // Mn       NEWA SANDHI MARK
            ALetter, // Lo       NEWA LETTER VEDIC ANUSVARA
            Other,
            ALetter, // Lo  [48] TIRHUTA ANJI..TIRHUTA LETTER HA
            Extend, // Mc   [3] TIRHUTA VOWEL SIGN AA..TIRHUTA VOWEL SIGN II
            Extend, // Mn   [6] TIRHUTA VOWEL SIGN U..TIRHUTA VOWEL SIGN VOCALIC LL
            Extend, // Mc       TIRHUTA VOWEL SIGN E
            Extend, // Mn       TIRHUTA VOWEL SIGN SHORT E
            Extend, // Mc   [4] TIRHUTA VOWEL SIGN AI..TIRHUTA VOWEL SIGN AU
            Extend, // Mn   [2] TIRHUTA SIGN CANDRABINDU..TIRHUTA SIGN ANUSVARA
            Extend, // Mc       TIRHUTA SIGN VISARGA
            Extend, // Mn   [2] TIRHUTA SIGN VIRAMA..TIRHUTA SIGN NUKTA
            ALetter, // Lo   [2] TIRHUTA SIGN AVAGRAHA..TIRHUTA GVANG
            Other,
            ALetter, // Lo       TIRHUTA OM
            Other,
            Numeric, // Nd  [10] TIRHUTA DIGIT ZERO..TIRHUTA DIGIT NINE
            Other,
            ALetter, // Lo  [47] SIDDHAM LETTER A..SIDDHAM LETTER HA
            Extend, // Mc   [3] SIDDHAM VOWEL SIGN AA..SIDDHAM VOWEL SIGN II
            Extend, // Mn   [4] SIDDHAM VOWEL SIGN U..SIDDHAM VOWEL SIGN VOCALIC RR
            Other,
            Extend, // Mc   [4] SIDDHAM VOWEL SIGN E..SIDDHAM VOWEL SIGN AU
            Extend, // Mn   [2] SIDDHAM SIGN CANDRABINDU..SIDDHAM SIGN ANUSVARA
            Extend, // Mc       SIDDHAM SIGN VISARGA
            Extend, // Mn   [2] SIDDHAM SIGN VIRAMA..SIDDHAM SIGN NUKTA
            Other,
            ALetter, // Lo   [4] SIDDHAM LETTER THREE-CIRCLE ALTERNATE I..SIDDHAM LETTER ALTERNATE U
            Extend, // Mn   [2] SIDDHAM VOWEL SIGN ALTERNATE U..SIDDHAM VOWEL SIGN ALTERNATE UU
            Other,
            ALetter, // Lo  [48] MODI LETTER A..MODI LETTER LLA
            Extend, // Mc   [3] MODI VOWEL SIGN AA..MODI VOWEL SIGN II
            Extend, // Mn   [8] MODI VOWEL SIGN U..MODI VOWEL SIGN AI
            Extend, // Mc   [2] MODI VOWEL SIGN O..MODI VOWEL SIGN AU
            Extend, // Mn       MODI SIGN ANUSVARA
            Extend, // Mc       MODI SIGN VISARGA
            Extend, // Mn   [2] MODI SIGN VIRAMA..MODI SIGN ARDHACANDRA
            Other,
            ALetter, // Lo       MODI SIGN HUVA
            Other,
            Numeric, // Nd  [10] MODI DIGIT ZERO..MODI DIGIT NINE
            Other,
            ALetter, // Lo  [43] TAKRI LETTER A..TAKRI LETTER RRA
            Extend, // Mn       TAKRI SIGN ANUSVARA
            Extend, // Mc       TAKRI SIGN VISARGA
            Extend, // Mn       TAKRI VOWEL SIGN AA
            Extend, // Mc   [2] TAKRI VOWEL SIGN I..TAKRI VOWEL SIGN II
            Extend, // Mn   [6] TAKRI VOWEL SIGN U..TAKRI VOWEL SIGN AU
            Extend, // Mc       TAKRI SIGN VIRAMA
            Extend, // Mn       TAKRI SIGN NUKTA
            ALetter, // Lo       TAKRI LETTER ARCHAIC KHA
            Other,
            Numeric, // Nd  [10] TAKRI DIGIT ZERO..TAKRI DIGIT NINE
            Other,
            Extend, // Mn   [3] AHOM CONSONANT SIGN MEDIAL LA..AHOM CONSONANT SIGN MEDIAL LIGATING RA
            Extend, // Mc   [2] AHOM VOWEL SIGN A..AHOM VOWEL SIGN AA
            Extend, // Mn   [4] AHOM VOWEL SIGN I..AHOM VOWEL SIGN UU
            Extend, // Mc       AHOM VOWEL SIGN E
            Extend, // Mn   [5] AHOM VOWEL SIGN AW..AHOM SIGN KILLER
            Other,
            Numeric, // Nd  [10] AHOM DIGIT ZERO..AHOM DIGIT NINE
            Other,
            ALetter, // Lo  [44] DOGRA LETTER A..DOGRA LETTER RRA
            Extend, // Mc   [3] DOGRA VOWEL SIGN AA..DOGRA VOWEL SIGN II
            Extend, // Mn   [9] DOGRA VOWEL SIGN U..DOGRA SIGN ANUSVARA
            Extend, // Mc       DOGRA SIGN VISARGA
            Extend, // Mn   [2] DOGRA SIGN VIRAMA..DOGRA SIGN NUKTA
            Other,
            ALetter, // L&  [64] WARANG CITI CAPITAL LETTER NGAA..WARANG CITI SMALL LETTER VIYO
            Numeric, // Nd  [10] WARANG CITI DIGIT ZERO..WARANG CITI DIGIT NINE
            Other,
            ALetter, // Lo       WARANG CITI OM
            Other,
            ALetter, // Lo   [8] NANDINAGARI LETTER A..NANDINAGARI LETTER VOCALIC RR
            Other,
            ALetter, // Lo  [39] NANDINAGARI LETTER E..NANDINAGARI LETTER RRA
            Extend, // Mc   [3] NANDINAGARI VOWEL SIGN AA..NANDINAGARI VOWEL SIGN II
            Extend, // Mn   [4] NANDINAGARI VOWEL SIGN U..NANDINAGARI VOWEL SIGN VOCALIC RR
            Other,
            Extend, // Mn   [2] NANDINAGARI VOWEL SIGN E..NANDINAGARI VOWEL SIGN AI
            Extend, // Mc   [4] NANDINAGARI VOWEL SIGN O..NANDINAGARI SIGN VISARGA
            Extend, // Mn       NANDINAGARI SIGN VIRAMA
            ALetter, // Lo       NANDINAGARI SIGN AVAGRAHA
            Other,
            ALetter, // Lo       NANDINAGARI HEADSTROKE
            Extend, // Mc       NANDINAGARI VOWEL SIGN PRISHTHAMATRA E
            Other,
            ALetter, // Lo       ZANABAZAR SQUARE LETTER A
            Extend, // Mn  [10] ZANABAZAR SQUARE VOWEL SIGN I..ZANABAZAR SQUARE VOWEL LENGTH MARK
            ALetter, // Lo  [40] ZANABAZAR SQUARE LETTER KA..ZANABAZAR SQUARE LETTER KSSA
            Extend, // Mn   [6] ZANABAZAR SQUARE FINAL CONSONANT MARK..ZANABAZAR SQUARE SIGN ANUSVARA
            Extend, // Mc       ZANABAZAR SQUARE SIGN VISARGA
            ALetter, // Lo       ZANABAZAR SQUARE CLUSTER-INITIAL LETTER RA
            Extend, // Mn   [4] ZANABAZAR SQUARE CLUSTER-FINAL LETTER YA..ZANABAZAR SQUARE CLUSTER-FINAL LETTER VA
            Other,
            Extend, // Mn       ZANABAZAR SQUARE SUBJOINER
            Other,
            ALetter, // Lo       SOYOMBO LETTER A
            Extend, // Mn   [6] SOYOMBO VOWEL SIGN I..SOYOMBO VOWEL SIGN OE
            Extend, // Mc   [2] SOYOMBO VOWEL SIGN AI..SOYOMBO VOWEL SIGN AU
            Extend, // Mn   [3] SOYOMBO VOWEL SIGN VOCALIC R..SOYOMBO VOWEL LENGTH MARK
            ALetter, // Lo  [46] SOYOMBO LETTER KA..SOYOMBO CLUSTER-INITIAL LETTER SA
            Extend, // Mn  [13] SOYOMBO FINAL CONSONANT SIGN G..SOYOMBO SIGN ANUSVARA
            Extend, // Mc       SOYOMBO SIGN VISARGA
            Extend, // Mn   [2] SOYOMBO GEMINATION MARK..SOYOMBO SUBJOINER
            Other,
            ALetter, // Lo       SOYOMBO MARK PLUTA
            Other,
            ALetter, // Lo  [57] PAU CIN HAU LETTER PA..PAU CIN HAU GLOTTAL STOP FINAL
            Other,
            ALetter, // Lo   [9] BHAIKSUKI LETTER A..BHAIKSUKI LETTER VOCALIC L
            Other,
            ALetter, // Lo  [37] BHAIKSUKI LETTER E..BHAIKSUKI LETTER HA
            Extend, // Mc       BHAIKSUKI VOWEL SIGN AA
            Extend, // Mn   [7] BHAIKSUKI VOWEL SIGN I..BHAIKSUKI VOWEL SIGN VOCALIC L
            Other,
            Extend, // Mn   [6] BHAIKSUKI VOWEL SIGN E..BHAIKSUKI SIGN ANUSVARA
            Extend, // Mc       BHAIKSUKI SIGN VISARGA
            Extend, // Mn       BHAIKSUKI SIGN VIRAMA
            ALetter, // Lo       BHAIKSUKI SIGN AVAGRAHA
            Other,
            Numeric, // Nd  [10] BHAIKSUKI DIGIT ZERO..BHAIKSUKI DIGIT NINE
            Other,
            ALetter, // Lo  [30] MARCHEN LETTER KA..MARCHEN LETTER A
            Other,
            Extend, // Mn  [22] MARCHEN SUBJOINED LETTER KA..MARCHEN SUBJOINED LETTER ZA
            Other,
            Extend, // Mc       MARCHEN SUBJOINED LETTER YA
            Extend, // Mn   [7] MARCHEN SUBJOINED LETTER RA..MARCHEN VOWEL SIGN AA
            Extend, // Mc       MARCHEN VOWEL SIGN I
            Extend, // Mn   [2] MARCHEN VOWEL SIGN U..MARCHEN VOWEL SIGN E
            Extend, // Mc       MARCHEN VOWEL SIGN O
            Extend, // Mn   [2] MARCHEN SIGN ANUSVARA..MARCHEN SIGN CANDRABINDU
            Other,
            ALetter, // Lo   [7] MASARAM GONDI LETTER A..MASARAM GONDI LETTER E
            Other,
            ALetter, // Lo   [2] MASARAM GONDI LETTER AI..MASARAM GONDI LETTER O
            Other,
            ALetter, // Lo  [38] MASARAM GONDI LETTER AU..MASARAM GONDI LETTER TRA
            Extend, // Mn   [6] MASARAM GONDI VOWEL SIGN AA..MASARAM GONDI VOWEL SIGN VOCALIC R
            Other,
            Extend, // Mn       MASARAM GONDI VOWEL SIGN E
            Other,
            Extend, // Mn   [2] MASARAM GONDI VOWEL SIGN AI..MASARAM GONDI VOWEL SIGN O
            Other,
            Extend, // Mn   [7] MASARAM GONDI VOWEL SIGN AU..MASARAM GONDI VIRAMA
            ALetter, // Lo       MASARAM GONDI REPHA
            Extend, // Mn       MASARAM GONDI RA-KARA
            Other,
            Numeric, // Nd  [10] MASARAM GONDI DIGIT ZERO..MASARAM GONDI DIGIT NINE
            Other,
            ALetter, // Lo   [6] GUNJALA GONDI LETTER A..GUNJALA GONDI LETTER UU
            Other,
            ALetter, // Lo   [2] GUNJALA GONDI LETTER EE..GUNJALA GONDI LETTER AI
            Other,
            ALetter, // Lo  [32] GUNJALA GONDI LETTER OO..GUNJALA GONDI LETTER SA
            Extend, // Mc   [5] GUNJALA GONDI VOWEL SIGN AA..GUNJALA GONDI VOWEL SIGN UU
            Other,
            Extend, // Mn   [2] GUNJALA GONDI VOWEL SIGN EE..GUNJALA GONDI VOWEL SIGN AI
            Other,
            Extend, // Mc   [2] GUNJALA GONDI VOWEL SIGN OO..GUNJALA GONDI VOWEL SIGN AU
            Extend, // Mn       GUNJALA GONDI SIGN ANUSVARA
            Extend, // Mc       GUNJALA GONDI SIGN VISARGA
            Extend, // Mn       GUNJALA GONDI VIRAMA
            ALetter, // Lo       GUNJALA GONDI OM
            Other,
            Numeric, // Nd  [10] GUNJALA GONDI DIGIT ZERO..GUNJALA GONDI DIGIT NINE
            Other,
            ALetter, // Lo  [19] MAKASAR LETTER KA..MAKASAR ANGKA
            Extend, // Mn   [2] MAKASAR VOWEL SIGN I..MAKASAR VOWEL SIGN U
            Extend, // Mc   [2] MAKASAR VOWEL SIGN E..MAKASAR VOWEL SIGN O
            Other,
            ALetter, // Lo [922] CUNEIFORM SIGN A..CUNEIFORM SIGN U U
            Other,
            ALetter, // Nl [111] CUNEIFORM NUMERIC SIGN TWO ASH..CUNEIFORM NUMERIC SIGN NINE U VARIANT FORM
            Other,
            ALetter, // Lo [196] CUNEIFORM SIGN AB TIMES NUN TENU..CUNEIFORM SIGN ZU5 TIMES THREE DISH TENU
            Other,
            ALetter, // Lo [1071] EGYPTIAN HIEROGLYPH A001..EGYPTIAN HIEROGLYPH AA032
            Other,
            Format, // Cf   [9] EGYPTIAN HIEROGLYPH VERTICAL JOINER..EGYPTIAN HIEROGLYPH END SEGMENT
            Other,
            ALetter, // Lo [583] ANATOLIAN HIEROGLYPH A001..ANATOLIAN HIEROGLYPH A530
            Other,
            ALetter, // Lo [569] BAMUM LETTER PHASE-A NGKUE MFON..BAMUM LETTER PHASE-F VUEQ
            Other,
            ALetter, // Lo  [31] MRO LETTER TA..MRO LETTER TEK
            Other,
            Numeric, // Nd  [10] MRO DIGIT ZERO..MRO DIGIT NINE
            Other,
            ALetter, // Lo  [30] BASSA VAH LETTER ENNI..BASSA VAH LETTER I
            Other,
            Extend, // Mn   [5] BASSA VAH COMBINING HIGH TONE..BASSA VAH COMBINING HIGH-LOW TONE
            Other,
            ALetter, // Lo  [48] PAHAWH HMONG VOWEL KEEB..PAHAWH HMONG CONSONANT CAU
            Extend, // Mn   [7] PAHAWH HMONG MARK CIM TUB..PAHAWH HMONG MARK CIM TAUM
            Other,
            ALetter, // Lm   [4] PAHAWH HMONG SIGN VOS SEEV..PAHAWH HMONG SIGN IB YAM
            Other,
            Numeric, // Nd  [10] PAHAWH HMONG DIGIT ZERO..PAHAWH HMONG DIGIT NINE
            Other,
            ALetter, // Lo  [21] PAHAWH HMONG SIGN VOS LUB..PAHAWH HMONG SIGN CIM NRES TOS
            Other,
            ALetter, // Lo  [19] PAHAWH HMONG CLAN SIGN TSHEEJ..PAHAWH HMONG CLAN SIGN VWJ
            Other,
            ALetter, // L&  [64] MEDEFAIDRIN CAPITAL LETTER M..MEDEFAIDRIN SMALL LETTER Y
            Other,
            ALetter, // Lo  [75] MIAO LETTER PA..MIAO LETTER RTE
            Other,
            Extend, // Mn       MIAO SIGN CONSONANT MODIFIER BAR
            ALetter, // Lo       MIAO LETTER NASALIZATION
            Extend, // Mc  [55] MIAO SIGN ASPIRATION..MIAO VOWEL SIGN UI
            Other,
            Extend, // Mn   [4] MIAO TONE RIGHT..MIAO TONE BELOW
            ALetter, // Lm  [13] MIAO LETTER TONE-2..MIAO LETTER REFORMED TONE-8
            Other,
            ALetter, // Lm   [2] TANGUT ITERATION MARK..NUSHU ITERATION MARK
            Other,
            ALetter, // Lm       OLD CHINESE ITERATION MARK
            Other,
            Katakana, // Lo       KATAKANA LETTER ARCHAIC E
            Other,
            Katakana, // Lo   [4] KATAKANA LETTER SMALL WI..KATAKANA LETTER SMALL N
            Other,
            ALetter, // Lo [107] DUPLOYAN LETTER H..DUPLOYAN LETTER VOCALIC M
            Other,
            ALetter, // Lo  [13] DUPLOYAN AFFIX LEFT HORIZONTAL SECANT..DUPLOYAN AFFIX ATTACHED TANGENT HOOK
            Other,
            ALetter, // Lo   [9] DUPLOYAN AFFIX HIGH ACUTE..DUPLOYAN AFFIX HIGH VERTICAL
            Other,
            ALetter, // Lo  [10] DUPLOYAN AFFIX LOW ACUTE..DUPLOYAN AFFIX LOW ARROW
            Other,
            Extend, // Mn   [2] DUPLOYAN THICK LETTER SELECTOR..DUPLOYAN DOUBLE MARK
            Other,
            Format, // Cf   [4] SHORTHAND FORMAT LETTER OVERLAP..SHORTHAND FORMAT UP STEP
            Other,
            Extend, // Mc   [2] MUSICAL SYMBOL COMBINING STEM..MUSICAL SYMBOL COMBINING SPRECHGESANG STEM
            Extend, // Mn   [3] MUSICAL SYMBOL COMBINING TREMOLO-1..MUSICAL SYMBOL COMBINING TREMOLO-3
            Other,
            Extend, // Mc   [6] MUSICAL SYMBOL COMBINING AUGMENTATION DOT..MUSICAL SYMBOL COMBINING FLAG-5
            Format, // Cf   [8] MUSICAL SYMBOL BEGIN BEAM..MUSICAL SYMBOL END PHRASE
            Extend, // Mn   [8] MUSICAL SYMBOL COMBINING ACCENT..MUSICAL SYMBOL COMBINING LOURE
            Other,
            Extend, // Mn   [7] MUSICAL SYMBOL COMBINING DOIT..MUSICAL SYMBOL COMBINING TRIPLE TONGUE
            Other,
            Extend, // Mn   [4] MUSICAL SYMBOL COMBINING DOWN BOW..MUSICAL SYMBOL COMBINING SNAP PIZZICATO
            Other,
            Extend, // Mn   [3] COMBINING GREEK MUSICAL TRISEME..COMBINING GREEK MUSICAL PENTASEME
            Other,
            ALetter, // L&  [85] MATHEMATICAL BOLD CAPITAL A..MATHEMATICAL ITALIC SMALL G
            Other,
            ALetter, // L&  [71] MATHEMATICAL ITALIC SMALL I..MATHEMATICAL SCRIPT CAPITAL A
            Other,
            ALetter, // L&   [2] MATHEMATICAL SCRIPT CAPITAL C..MATHEMATICAL SCRIPT CAPITAL D
            Other,
            ALetter, // L&       MATHEMATICAL SCRIPT CAPITAL G
            Other,
            ALetter, // L&   [2] MATHEMATICAL SCRIPT CAPITAL J..MATHEMATICAL SCRIPT CAPITAL K
            Other,
            ALetter, // L&   [4] MATHEMATICAL SCRIPT CAPITAL N..MATHEMATICAL SCRIPT CAPITAL Q
            Other,
            ALetter, // L&  [12] MATHEMATICAL SCRIPT CAPITAL S..MATHEMATICAL SCRIPT SMALL D
            Other,
            ALetter, // L&       MATHEMATICAL SCRIPT SMALL F
            Other,
            ALetter, // L&   [7] MATHEMATICAL SCRIPT SMALL H..MATHEMATICAL SCRIPT SMALL N
            Other,
            ALetter, // L&  [65] MATHEMATICAL SCRIPT SMALL P..MATHEMATICAL FRAKTUR CAPITAL B
            Other,
            ALetter, // L&   [4] MATHEMATICAL FRAKTUR CAPITAL D..MATHEMATICAL FRAKTUR CAPITAL G
            Other,
            ALetter, // L&   [8] MATHEMATICAL FRAKTUR CAPITAL J..MATHEMATICAL FRAKTUR CAPITAL Q
            Other,
            ALetter, // L&   [7] MATHEMATICAL FRAKTUR CAPITAL S..MATHEMATICAL FRAKTUR CAPITAL Y
            Other,
            ALetter, // L&  [28] MATHEMATICAL FRAKTUR SMALL A..MATHEMATICAL DOUBLE-STRUCK CAPITAL B
            Other,
            ALetter, // L&   [4] MATHEMATICAL DOUBLE-STRUCK CAPITAL D..MATHEMATICAL DOUBLE-STRUCK CAPITAL G
            Other,
            ALetter, // L&   [5] MATHEMATICAL DOUBLE-STRUCK CAPITAL I..MATHEMATICAL DOUBLE-STRUCK CAPITAL M
            Other,
            ALetter, // L&       MATHEMATICAL DOUBLE-STRUCK CAPITAL O
            Other,
            ALetter, // L&   [7] MATHEMATICAL DOUBLE-STRUCK CAPITAL S..MATHEMATICAL DOUBLE-STRUCK CAPITAL Y
            Other,
            ALetter, // L& [340] MATHEMATICAL DOUBLE-STRUCK SMALL A..MATHEMATICAL ITALIC SMALL DOTLESS J
            Other,
            ALetter, // L&  [25] MATHEMATICAL BOLD CAPITAL ALPHA..MATHEMATICAL BOLD CAPITAL OMEGA
            Other,
            ALetter, // L&  [25] MATHEMATICAL BOLD SMALL ALPHA..MATHEMATICAL BOLD SMALL OMEGA
            Other,
            ALetter, // L&  [31] MATHEMATICAL BOLD EPSILON SYMBOL..MATHEMATICAL ITALIC CAPITAL OMEGA
            Other,
            ALetter, // L&  [25] MATHEMATICAL ITALIC SMALL ALPHA..MATHEMATICAL ITALIC SMALL OMEGA
            Other,
            ALetter, // L&  [31] MATHEMATICAL ITALIC EPSILON SYMBOL..MATHEMATICAL BOLD ITALIC CAPITAL OMEGA
            Other,
            ALetter, // L&  [25] MATHEMATICAL BOLD ITALIC SMALL ALPHA..MATHEMATICAL BOLD ITALIC SMALL OMEGA
            Other,
            ALetter, // L&  [31] MATHEMATICAL BOLD ITALIC EPSILON SYMBOL..MATHEMATICAL SANS-SERIF BOLD CAPITAL OMEGA
            Other,
            ALetter, // L&  [25] MATHEMATICAL SANS-SERIF BOLD SMALL ALPHA..MATHEMATICAL SANS-SERIF BOLD SMALL OMEGA
            Other,
            ALetter, // L&  [31] MATHEMATICAL SANS-SERIF BOLD EPSILON SYMBOL..MATHEMATICAL SANS-SERIF BOLD ITALIC CAPITAL OMEGA
            Other,
            ALetter, // L&  [25] MATHEMATICAL SANS-SERIF BOLD ITALIC SMALL ALPHA..MATHEMATICAL SANS-SERIF BOLD ITALIC SMALL OMEGA
            Other,
            ALetter, // L&   [8] MATHEMATICAL SANS-SERIF BOLD ITALIC EPSILON SYMBOL..MATHEMATICAL BOLD SMALL DIGAMMA
            Other,
            Numeric, // Nd  [50] MATHEMATICAL BOLD DIGIT ZERO..MATHEMATICAL MONOSPACE DIGIT NINE
            Other,
            Extend, // Mn  [55] SIGNWRITING HEAD RIM..SIGNWRITING AIR SUCKING IN
            Other,
            Extend, // Mn  [50] SIGNWRITING MOUTH CLOSED NEUTRAL..SIGNWRITING EXCITEMENT
            Other,
            Extend, // Mn       SIGNWRITING UPPER BODY TILTING FROM HIP JOINTS
            Other,
            Extend, // Mn       SIGNWRITING LOCATION HEAD NECK
            Other,
            Extend, // Mn   [5] SIGNWRITING FILL MODIFIER-2..SIGNWRITING FILL MODIFIER-6
            Other,
            Extend, // Mn  [15] SIGNWRITING ROTATION MODIFIER-2..SIGNWRITING ROTATION MODIFIER-16
            Other,
            Extend, // Mn   [7] COMBINING GLAGOLITIC LETTER AZU..COMBINING GLAGOLITIC LETTER ZHIVETE
            Other,
            Extend, // Mn  [17] COMBINING GLAGOLITIC LETTER ZEMLJA..COMBINING GLAGOLITIC LETTER HERU
            Other,
            Extend, // Mn   [7] COMBINING GLAGOLITIC LETTER SHTA..COMBINING GLAGOLITIC LETTER YATI
            Other,
            Extend, // Mn   [2] COMBINING GLAGOLITIC LETTER YU..COMBINING GLAGOLITIC LETTER SMALL YUS
            Other,
            Extend, // Mn   [5] COMBINING GLAGOLITIC LETTER YO..COMBINING GLAGOLITIC LETTER FITA
            Other,
            ALetter, // Lo  [45] NYIAKENG PUACHUE HMONG LETTER MA..NYIAKENG PUACHUE HMONG LETTER W
            Other,
            Extend, // Mn   [7] NYIAKENG PUACHUE HMONG TONE-B..NYIAKENG PUACHUE HMONG TONE-D
            ALetter, // Lm   [7] NYIAKENG PUACHUE HMONG SIGN FOR PERSON..NYIAKENG PUACHUE HMONG SYLLABLE LENGTHENER
            Other,
            Numeric, // Nd  [10] NYIAKENG PUACHUE HMONG DIGIT ZERO..NYIAKENG PUACHUE HMONG DIGIT NINE
            Other,
            ALetter, // Lo       NYIAKENG PUACHUE HMONG LOGOGRAM NYAJ
            Other,
            ALetter, // Lo  [44] WANCHO LETTER AA..WANCHO LETTER YIH
            Extend, // Mn   [4] WANCHO TONE TUP..WANCHO TONE KOINI
            Numeric, // Nd  [10] WANCHO DIGIT ZERO..WANCHO DIGIT NINE
            Other,
            ALetter, // Lo [197] MENDE KIKAKUI SYLLABLE M001 KI..MENDE KIKAKUI SYLLABLE M060 NYON
            Other,
            Extend, // Mn   [7] MENDE KIKAKUI COMBINING NUMBER TEENS..MENDE KIKAKUI COMBINING NUMBER MILLIONS
            Other,
            ALetter, // L&  [68] ADLAM CAPITAL LETTER ALIF..ADLAM SMALL LETTER SHA
            Extend, // Mn   [7] ADLAM ALIF LENGTHENER..ADLAM NUKTA
            ALetter, // Lm       ADLAM NASALIZATION MARK
            Other,
            Numeric, // Nd  [10] ADLAM DIGIT ZERO..ADLAM DIGIT NINE
            Other,
            ALetter, // Lo   [4] ARABIC MATHEMATICAL ALEF..ARABIC MATHEMATICAL DAL
            Other,
            ALetter, // Lo  [27] ARABIC MATHEMATICAL WAW..ARABIC MATHEMATICAL DOTLESS QAF
            Other,
            ALetter, // Lo   [2] ARABIC MATHEMATICAL INITIAL BEH..ARABIC MATHEMATICAL INITIAL JEEM
            Other,
            ALetter, // Lo       ARABIC MATHEMATICAL INITIAL HEH
            Other,
            ALetter, // Lo       ARABIC MATHEMATICAL INITIAL HAH
            Other,
            ALetter, // Lo  [10] ARABIC MATHEMATICAL INITIAL YEH..ARABIC MATHEMATICAL INITIAL QAF
            Other,
            ALetter, // Lo   [4] ARABIC MATHEMATICAL INITIAL SHEEN..ARABIC MATHEMATICAL INITIAL KHAH
            Other,
            ALetter, // Lo       ARABIC MATHEMATICAL INITIAL DAD
            Other,
            ALetter, // Lo       ARABIC MATHEMATICAL INITIAL GHAIN
            Other,
            ALetter, // Lo       ARABIC MATHEMATICAL TAILED JEEM
            Other,
            ALetter, // Lo       ARABIC MATHEMATICAL TAILED HAH
            Other,
            ALetter, // Lo       ARABIC MATHEMATICAL TAILED YEH
            Other,
            ALetter, // Lo       ARABIC MATHEMATICAL TAILED LAM
            Other,
            ALetter, // Lo   [3] ARABIC MATHEMATICAL TAILED NOON..ARABIC MATHEMATICAL TAILED AIN
            Other,
            ALetter, // Lo   [2] ARABIC MATHEMATICAL TAILED SAD..ARABIC MATHEMATICAL TAILED QAF
            Other,
            ALetter, // Lo       ARABIC MATHEMATICAL TAILED SHEEN
            Other,
            ALetter, // Lo       ARABIC MATHEMATICAL TAILED KHAH
            Other,
            ALetter, // Lo       ARABIC MATHEMATICAL TAILED DAD
            Other,
            ALetter, // Lo       ARABIC MATHEMATICAL TAILED GHAIN
            Other,
            ALetter, // Lo       ARABIC MATHEMATICAL TAILED DOTLESS NOON
            Other,
            ALetter, // Lo       ARABIC MATHEMATICAL TAILED DOTLESS QAF
            Other,
            ALetter, // Lo   [2] ARABIC MATHEMATICAL STRETCHED BEH..ARABIC MATHEMATICAL STRETCHED JEEM
            Other,
            ALetter, // Lo       ARABIC MATHEMATICAL STRETCHED HEH
            Other,
            ALetter, // Lo   [4] ARABIC MATHEMATICAL STRETCHED HAH..ARABIC MATHEMATICAL STRETCHED KAF
            Other,
            ALetter, // Lo   [7] ARABIC MATHEMATICAL STRETCHED MEEM..ARABIC MATHEMATICAL STRETCHED QAF
            Other,
            ALetter, // Lo   [4] ARABIC MATHEMATICAL STRETCHED SHEEN..ARABIC MATHEMATICAL STRETCHED KHAH
            Other,
            ALetter, // Lo   [4] ARABIC MATHEMATICAL STRETCHED DAD..ARABIC MATHEMATICAL STRETCHED DOTLESS BEH
            Other,
            ALetter, // Lo       ARABIC MATHEMATICAL STRETCHED DOTLESS FEH
            Other,
            ALetter, // Lo  [10] ARABIC MATHEMATICAL LOOPED ALEF..ARABIC MATHEMATICAL LOOPED YEH
            Other,
            ALetter, // Lo  [17] ARABIC MATHEMATICAL LOOPED LAM..ARABIC MATHEMATICAL LOOPED GHAIN
            Other,
            ALetter, // Lo   [3] ARABIC MATHEMATICAL DOUBLE-STRUCK BEH..ARABIC MATHEMATICAL DOUBLE-STRUCK DAL
            Other,
            ALetter, // Lo   [5] ARABIC MATHEMATICAL DOUBLE-STRUCK WAW..ARABIC MATHEMATICAL DOUBLE-STRUCK YEH
            Other,
            ALetter, // Lo  [17] ARABIC MATHEMATICAL DOUBLE-STRUCK LAM..ARABIC MATHEMATICAL DOUBLE-STRUCK GHAIN
            Other,
            ALetter, // So  [26] SQUARED LATIN CAPITAL LETTER A..SQUARED LATIN CAPITAL LETTER Z
            Other,
            ALetter, // So  [26] NEGATIVE CIRCLED LATIN CAPITAL LETTER A..NEGATIVE CIRCLED LATIN CAPITAL LETTER Z
            Other,
            ALetter, // So  [26] NEGATIVE SQUARED LATIN CAPITAL LETTER A..NEGATIVE SQUARED LATIN CAPITAL LETTER Z
            Other,
            Regional_Indicator, // So  [26] REGIONAL INDICATOR SYMBOL LETTER A..REGIONAL INDICATOR SYMBOL LETTER Z
            Other,
            Extend, // Sk   [5] EMOJI MODIFIER FITZPATRICK TYPE-1-2..EMOJI MODIFIER FITZPATRICK TYPE-6
            Other,
            Format, // Cf       LANGUAGE TAG
            Other,
            Extend, // Cf  [96] TAG SPACE..CANCEL TAG
            Other,
            Extend, // Mn [240] VARIATION SELECTOR-17..VARIATION SELECTOR-256
            Other,
        ];
    }
}
