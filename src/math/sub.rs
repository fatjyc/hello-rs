pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub_positive_numbers() {
        assert_eq!(sub(5, 3), 2);
    }

    #[test]
    fn test_sub_negative_numbers() {
        assert_eq!(sub(-5, -3), -2);
    }

    #[test]
    fn test_sub_mixed_numbers() {
        assert_eq!(sub(5, -3), 8);
        assert_eq!(sub(-5, 3), -8);
    }

    #[test]
    fn test_sub_zero() {
        assert_eq!(sub(5, 0), 5);
        assert_eq!(sub(0, 5), -5);
        assert_eq!(sub(0, 0), 0);
    }

    #[test]
    fn test_sub_large_numbers() {
        assert_eq!(sub(1000000000, 999999999), 1);
        assert_eq!(sub(-1000000000, -999999999), -1);
    }

    #[test]
    fn test_sub_same_number() {
        assert_eq!(sub(42, 42), 0);
        assert_eq!(sub(-42, -42), 0);
    }
}
