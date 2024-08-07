use crate::convert_arabic_to_roman_number::ROMAN_NUMERALS;

pub fn convert_roman_to_arabic_number(roman_number: impl ToString) -> Result<i32, &'static str> {
    let roman_number = roman_number.to_string();
    let mut result = 0;
    let mut i = 0;

    while i < roman_number.len() {
        let current = roman_number.chars().nth(i).and_then(|number| {
            ROMAN_NUMERALS
                .iter()
                .find(|numeral| numeral.numeral == number)
        });
        let next = roman_number.chars().nth(i + 1).and_then(|number| {
            ROMAN_NUMERALS
                .iter()
                .find(|numeral| numeral.numeral == number)
        });
        i += 1;

        if let Some(current) = current {
            match next {
                None => {
                    if next.is_none() {
                        result += current.value;
                    }
                }
                Some(next) => {
                    if current.value >= next.value {
                        result += current.value;
                    } else {
                        let difference = next.value - current.value;
                        result += difference;
                        i += 1;
                    }
                }
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_I_to_1() {
        assert_eq!(convert_roman_to_arabic_number("I").unwrap(), 1);
    }

    #[test]
    fn convert_III_to_3() {
        assert_eq!(convert_roman_to_arabic_number("III").unwrap(), 3);
    }

    #[test]
    fn convert_IV_to_4() {
        assert_eq!(convert_roman_to_arabic_number("IV").unwrap(), 4);
    }

    #[test]
    fn convert_V_to_5() {
        assert_eq!(convert_roman_to_arabic_number("V").unwrap(), 5);
    }

    #[test]
    fn convert_VI_to_6() {
        assert_eq!(convert_roman_to_arabic_number("VI").unwrap(), 6);
    }

    #[test]
    fn convert_IX_to_9() {
        assert_eq!(convert_roman_to_arabic_number("IX").unwrap(), 9);
    }

    #[test]
    fn convert_X_to_10() {
        assert_eq!(convert_roman_to_arabic_number("X").unwrap(), 10);
    }

    #[test]
    fn convert_XI_to_11() {
        assert_eq!(convert_roman_to_arabic_number("XI").unwrap(), 11);
    }

    #[test]
    fn convert_XLIX_to_49() {
        assert_eq!(convert_roman_to_arabic_number("XLIX").unwrap(), 49);
    }

    #[test]
    fn convert_L_to_50() {
        assert_eq!(convert_roman_to_arabic_number("L").unwrap(), 50);
    }

    #[test]
    fn convert_LXVII_to_67() {
        assert_eq!(convert_roman_to_arabic_number("LXVII").unwrap(), 67);
    }

    #[test]
    fn convert_LXIV_to_69() {
        assert_eq!(convert_roman_to_arabic_number("LXIX").unwrap(), 69);
    }

    #[test]
    fn convert_LXIV_to_89() {
        assert_eq!(convert_roman_to_arabic_number("LXXXIX").unwrap(), 89);
    }

    #[test]
    fn convert_C_to_100() {
        assert_eq!(convert_roman_to_arabic_number("C").unwrap(), 100);
    }

    #[test]
    fn convert_D_to_500() {
        assert_eq!(convert_roman_to_arabic_number("D").unwrap(), 500);
    }

    #[test]
    fn convert_CMXCIX_to_999() {
        assert_eq!(convert_roman_to_arabic_number("CMXCIX").unwrap(), 999);
    }

    #[test]
    fn convert_M_to_1000() {
        assert_eq!(convert_roman_to_arabic_number("M").unwrap(), 1000);
    }

    #[test]
    fn convert_MMCDXXI_to_2421() {
        assert_eq!(convert_roman_to_arabic_number("MMCDXXI").unwrap(), 2421);
    }

    #[test]
    fn convert_MMMCMXCIX_to_3999() {
        assert_eq!(convert_roman_to_arabic_number("MMMCMXCIX").unwrap(), 3999);
    }
}
