pub fn luhn(cc_number: &str) -> bool {
    let cleaned: String = cc_number.chars().filter(|c| c.is_digit(10)).collect();

    if cc_number.len() < 2 {
        return false;
    }

    let mut sum = 0;
    let mut double = false;

    for c in cleaned.chars().rev() {
        let digit = c.to_digit(10).unwrap(); // Теперь точно цифра

        let mut value = digit;
        if double {
            value *= 2;
            if value > 9 {
                value -= 9;
            }
        }

        sum += value;
        double = !double;
    }

    sum % 10 == 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_cc_number() {
        assert!(luhn("4263 9826 4026 9299"));
        assert!(luhn("4539 3195 0343 6467"));
        assert!(luhn("7992 7398 713"));
    }

    #[test]
    fn test_invalid_cc_number() {
        assert!(!luhn("4223 9826 4026 9299"));
        assert!(!luhn("4539 3195 0343 6476"));
        assert!(!luhn("8273 1232 7352 0569"));
    }
    #[test]
    fn test_invalid_ccc_number() {
        assert!(!luhn("4539 3195 0343 6476"));
        assert!(!luhn("8273 1232 7352 0569"));

        // Дополнительные тесты
        assert!(!luhn("1234 5678 9012 3456"));
        assert!(!luhn("9999 9999 9999 9999")); // Граничный случай
    }

    #[test]
    fn test_edge_cases() {
        assert!(!luhn("1")); // Одно число → должно быть невалидно
        assert!(!luhn("9")); // Одно число
        assert!(luhn("91")); // Валидный пример для 2-цифр

        assert!(luhn(" 4 9 9 2 7 3 9 8 7 1 6 ")); // Случайные пробелы
        assert!(!luhn("4992-7398-713")); // Нецифровые символы
        assert!(!luhn("4992_7398_713")); // Нецифровые символы
    }
}
