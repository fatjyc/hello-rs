pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_positive_numbers() {
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(10, 20), 30);
    }

    #[test]
    fn test_add_negative_numbers() {
        assert_eq!(add(-1, -2), -3);
        assert_eq!(add(-10, -20), -30);
    }

    #[test]
    fn test_add_zero() {
        assert_eq!(add(0, 0), 0);
        assert_eq!(add(5, 0), 5);
        assert_eq!(add(0, 5), 5);
    }

    #[test]
    fn test_add_positive_and_negative() {
        assert_eq!(add(-1, 2), 1);
        assert_eq!(add(1, -2), -1);
    }

    #[test]
    fn test_add_max_values() {
        assert_eq!(add(i32::MAX, 0), i32::MAX);
        assert_eq!(add(0, i32::MIN), i32::MIN);
    }
}
