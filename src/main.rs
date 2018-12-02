fn main() {
    println!("Hello, world!");
}

fn validate_serial_length(serial_number: &str) -> bool {
    serial_number.chars().count() >= 6
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
}