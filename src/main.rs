fn main() {
    println!("Hello, world!");
}

fn validate_serial_length(serial_number: &str) -> bool {
    serial_number.chars().count() >= 6
}

fn validate_serial_alphanumeric(serial_number: &str) -> bool {
    serial_number.chars().all(char::is_alphanumeric)
}

fn validate_serial_unique(serial_number: &str) -> bool {
    let existing_serial_numbers = vec!["serial1", "serial2", "serial3"];
    !existing_serial_numbers.contains(&serial_number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_length_of_four_characters_as_invalid() {
        let test_serial = "i234";
        let validation_result = validate_serial_length(test_serial);
        assert_eq!(false, validation_result);
    }

    #[test]
    fn validates_length_of_six_characters_as_valid() {
        let test_serial = "i23456";
        let validation_result = validate_serial_length(test_serial);
        assert_eq!(true, validation_result);
    }

    #[test]
    fn validates_length_of_ten_characters_as_valid() {
        let test_serial = "i234567891";
        let validation_result = validate_serial_length(test_serial);
        assert_eq!(true, validation_result);
    }

    #[test]
    fn validates_string_with_numbers_as_valid() {
        let test_serial = "234567891";
        let validation_result = validate_serial_alphanumeric(test_serial);
        assert_eq!(true, validation_result);
    }

    #[test]
    fn validates_string_with_az_characters_as_valid() {
        let test_serial = "abcd1234";
        let validation_result = validate_serial_alphanumeric(test_serial);
        assert_eq!(true, validation_result);
    }

    #[test]
    fn validates_string_with_unicode_characters_as_valid() {
        let test_serial = "абвгдежзийюя1234";
        let validation_result = validate_serial_alphanumeric(test_serial);
        assert_eq!(true, validation_result);
    }

    #[test]
    fn validates_string_with_special_characters_as_invalid() {
        let test_serial = "abcd!1234";
        let validation_result = validate_serial_alphanumeric(test_serial);
        assert_eq!(false, validation_result);
    }

    #[test]
    fn validates_existing_serial1_as_invalid() {
        let test_serial = "serial1";
        let validation_result = validate_serial_unique(test_serial);
        assert_eq!(false, validation_result);
    }

    #[test]
    fn validates_new_serial4_as_valid() {
        let test_serial = "serial4";
        let validation_result = validate_serial_unique(test_serial);
        assert_eq!(true, validation_result);
    }
}