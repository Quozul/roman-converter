use crate::converter::roman_numerals::ROMAN_NUMERALS;

pub fn convert_arabic_to_roman_number(arabic_number: i32) -> Result<String, &'static str> {
    if arabic_number <= 0 || arabic_number > 3999 {
        return Err("Illegal roman number.");
    }

    let mut roman_number = String::new();
    let mut remaining = arabic_number;

    while remaining > 0 {
        'first: for current in ROMAN_NUMERALS {
            if current.value <= remaining {
                roman_number.push(current.numeral);
                remaining -= current.value;
                break 'first;
            }

            for next in ROMAN_NUMERALS.iter().skip(1) {
                let difference = current.value - next.value;

                if next.is_subtractive && difference <= remaining && difference > 0 {
                    roman_number.push(next.numeral);
                    roman_number.push(current.numeral);
                    remaining -= difference;
                    break 'first;
                }
            }
        }
    }

    Ok(roman_number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_1() {
        assert_eq!(convert_arabic_to_roman_number(1).unwrap(), "I");
    }

    #[test]
    fn test_convert_3() {
        assert_eq!(convert_arabic_to_roman_number(3).unwrap(), "III");
    }

    #[test]
    fn convert_1() {
        assert_eq!(convert_arabic_to_roman_number(1).unwrap(), "I");
    }

    #[test]
    fn convert_3() {
        assert_eq!(convert_arabic_to_roman_number(3).unwrap(), "III");
    }

    #[test]
    fn convert_4() {
        assert_eq!(convert_arabic_to_roman_number(4).unwrap(), "IV");
    }

    #[test]
    fn convert_5() {
        assert_eq!(convert_arabic_to_roman_number(5).unwrap(), "V");
    }

    #[test]
    fn convert_6() {
        assert_eq!(convert_arabic_to_roman_number(6).unwrap(), "VI");
    }

    #[test]
    fn convert_9() {
        assert_eq!(convert_arabic_to_roman_number(9).unwrap(), "IX");
    }

    #[test]
    fn convert_10() {
        assert_eq!(convert_arabic_to_roman_number(10).unwrap(), "X");
    }

    #[test]
    fn convert_11() {
        assert_eq!(convert_arabic_to_roman_number(11).unwrap(), "XI");
    }

    #[test]
    fn convert_49() {
        assert_eq!(convert_arabic_to_roman_number(49).unwrap(), "XLIX");
    }

    #[test]
    fn convert_50() {
        assert_eq!(convert_arabic_to_roman_number(50).unwrap(), "L");
    }

    #[test]
    fn convert_67() {
        assert_eq!(convert_arabic_to_roman_number(67).unwrap(), "LXVII");
    }

    #[test]
    fn convert_69() {
        assert_eq!(convert_arabic_to_roman_number(69).unwrap(), "LXIX");
    }

    #[test]
    fn convert_89() {
        assert_eq!(convert_arabic_to_roman_number(89).unwrap(), "LXXXIX");
    }

    #[test]
    fn convert_100() {
        assert_eq!(convert_arabic_to_roman_number(100).unwrap(), "C");
    }

    #[test]
    fn convert_500() {
        assert_eq!(convert_arabic_to_roman_number(500).unwrap(), "D");
    }

    #[test]
    fn convert_999() {
        assert_eq!(convert_arabic_to_roman_number(999).unwrap(), "CMXCIX");
    }

    #[test]
    fn convert_1000() {
        assert_eq!(convert_arabic_to_roman_number(1000).unwrap(), "M");
    }

    #[test]
    fn convert_2421() {
        assert_eq!(convert_arabic_to_roman_number(2421).unwrap(), "MMCDXXI");
    }

    #[test]
    fn convert_3999() {
        assert_eq!(convert_arabic_to_roman_number(3999).unwrap(), "MMMCMXCIX");
    }

    #[test]
    #[should_panic(expected = "Conversion not implemented")]
    fn convert_4000() {
        convert_arabic_to_roman_number(4000).expect("Conversion not implemented");
    }

    #[test]
    #[should_panic(expected = "Conversion not implemented")]
    fn convert_negative() {
        convert_arabic_to_roman_number(-1).expect("Conversion not implemented");
    }
}
