pub fn times(a: i32, b: i32) -> i32 {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_times_positive() {
        assert_eq!(times(2, 3), 6);
        assert_eq!(times(4, 5), 20);
    }

    #[test]
    fn test_times_negative() {
        assert_eq!(times(-2, 3), -6);
        assert_eq!(times(2, -3), -6);
        assert_eq!(times(-2, -3), 6);
    }

    #[test]
    fn test_times_zero() {
        assert_eq!(times(0, 5), 0);
        assert_eq!(times(5, 0), 0);
        assert_eq!(times(0, 0), 0);
    }

    #[test]
    fn test_times_one() {
        assert_eq!(times(1, 5), 5);
        assert_eq!(times(5, 1), 5);
        assert_eq!(times(1, 1), 1);
    }

    #[test]
    fn test_times_max_min() {
        assert_eq!(times(i32::MAX, 1), i32::MAX);
        assert_eq!(times(i32::MIN, 1), i32::MIN);
    }
}
