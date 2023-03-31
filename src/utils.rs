use num_traits::Num;

pub fn clamp<N: Num + PartialOrd>(n: N, min: N, max: N) -> N {
    debug_assert!(min < max);
    if n < min {
        return min;
    }
    if n > max {
        return max;
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clamp_within_range() {
        let n = 5;
        let min = 1;
        let max = 10;
        assert_eq!(clamp(n, min, max), 5);
    }

    #[test]
    fn test_clamp_below_range() {
        let n = -5;
        let min = 1;
        let max = 10;
        assert_eq!(clamp(n, min, max), 1);
    }

    #[test]
    fn test_clamp_above_range() {
        let n = 15;
        let min = 1;
        let max = 10;
        assert_eq!(clamp(n, min, max), 10);
    }

    #[test]
    #[should_panic(expected = "assertion failed: min < max")]
    fn test_clamp_min_equals_max() {
        let n = 5;
        let min = 10;
        let max = 10;
        clamp(n, min, max);
    }
}
