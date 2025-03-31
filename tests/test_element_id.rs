#[cfg(test)]
mod tests {
    use std::convert::TryFrom;
    use visualife::ElementID;

    #[test]
    fn test_from_str_valid() {
        let id = ElementID::try_from("abc123").unwrap();
        assert_eq!(id.id, [b'a', b'b', b'c', b'1', b'2', b'3', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_from_str_exact_length() {
        let id = ElementID::try_from("1234567812345678").unwrap();
        assert_eq!(id.id, *b"1234567812345678");
    }


    #[test]
    fn test_from_int() {
        let id = ElementID::try_from(123).unwrap();
        assert_eq!(id.id, [b'1', b'2', b'3', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_from_negative_int() {
        let id = ElementID::try_from(-45).unwrap();
        assert_eq!(id.id, [b'-', b'4', b'5', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }
}