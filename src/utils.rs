use num_traits::Num;
// use std::cmp::PartialOrd;

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
