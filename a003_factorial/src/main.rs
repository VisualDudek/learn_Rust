//! Factorial exercise
//!
//! Complete the `factorial` function below. Run `cargo test` to check your work.

/// Computes the factorial of `n`, defined as:
///
/// ```text
/// n! = n × (n - 1) × (n - 2) × ... × 1,   with 0! = 1 by convention
/// ```
///
/// # Expected inputs
/// - `n` is a non-negative integer (`u64`, so negative values aren't representable).
/// - `0` and `1` both map to `1`.
///
/// # Overflow
/// `u64` can represent factorials up to `20!` (`2_432_902_008_176_640_000`).
/// `21!` exceeds `u64::MAX` and will panic in debug builds (overflow check)
/// or silently wrap in release builds — see the `factorial_overflows_at_21`
/// test below for how to detect and handle this instead of ignoring it.
///
/// # Examples
/// ```
/// # use factorial_exercise::factorial;
/// assert_eq!(factorial(0), 1);
/// assert_eq!(factorial(5), 120);
/// ```
pub fn factorial(n: u64) -> u64 {
    todo!("implement factorial — try an iterative fold, or recursion if you're feeling bold")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_factorial_is_one() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn one_factorial_is_one() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn small_cases() {
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(4), 24);
        assert_eq!(factorial(5), 120);
    }

    #[test]
    fn larger_case() {
        assert_eq!(factorial(10), 3_628_800);
    }

    #[test]
    fn largest_value_that_fits_in_u64() {
        // 20! is the largest factorial that fits in a u64 without overflowing.
        assert_eq!(factorial(20), 2_432_902_008_176_640_000);
    }

    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn factorial_overflows_at_21() {
        // 21! = 51_090_942_171_709_440_000, which exceeds u64::MAX.
        // In a debug build, Rust's runtime overflow checks turn this into a
        // panic rather than silently wrapping — that's a deliberate safety
        // net, not a bug. If you want *checked* behavior in production code
        // (no panics), reach for `checked_mul` and return an `Option<u64>`
        // or `Result<u64, _>` instead of a bare `u64`.
        let _ = factorial(21);
    }
}