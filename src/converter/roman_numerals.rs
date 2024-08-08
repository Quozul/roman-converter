#[derive(Debug)]
pub struct RomanNumeral {
    pub numeral: char,
    pub value: i32,
    pub is_subtractive: bool,
}

pub const ROMAN_NUMERALS: [RomanNumeral; 7] = [
    RomanNumeral {
        numeral: 'M',
        value: 1000,
        is_subtractive: false,
    },
    RomanNumeral {
        numeral: 'D',
        value: 500,
        is_subtractive: false,
    },
    RomanNumeral {
        numeral: 'C',
        value: 100,
        is_subtractive: true,
    },
    RomanNumeral {
        numeral: 'L',
        value: 50,
        is_subtractive: false,
    },
    RomanNumeral {
        numeral: 'X',
        value: 10,
        is_subtractive: true,
    },
    RomanNumeral {
        numeral: 'V',
        value: 5,
        is_subtractive: false,
    },
    RomanNumeral {
        numeral: 'I',
        value: 1,
        is_subtractive: true,
    },
];
