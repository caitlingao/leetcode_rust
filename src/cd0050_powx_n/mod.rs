//! Pow(x, n) [leetcode: powx_n](https://leetcode.com/problems/powx-n/)
//!
//! Implement `pow(x, n)`, which calculates x raised to the power n (xn).
//!
//! ***Example1:***
//!
//! ```
//! Input: 2.00000, 10
//! Output: 1024.00000
//! ```
//!
//! ***Example2:***
//!
//! ```
//! Input: 2.10000, 3
//! Output: 9.26100
//! ```
//!
//! ***Example3:***
//!
//! ```
//! Input: 2.00000, -2
//! Output: 0.25000
//! Explanation: 2-2 = 1/22 = 1/4 = 0.25
//! ```
//!
//! ***Note:***
//!
//! * -100.0 < x < 100.0
//! * n is a 32-bit signed integer, within the range [−231, 231 − 1]

/// # Solutions
///
/// # Approach 1: Iteration
///
/// * Time complexity: O(logn)
///
/// * Space complexity: O(1)
///
/// * Runtime: 0 ms
/// * Memory: 2.4 MB
///
/// ```rust
/// impl Solution {
///     pub fn my_pow(mut x: f64, n: i32) -> f64 {
///         let mut ln = n as i64;
///         let mut pow = 1f64;
///         let mut current = x;
///
///         if ln < 0 {
///             current = 1f64 / current;
///             ln = -ln;
///         }
///
///         while ln > 0 {
///             if ln & 1 == 1 { pow *= current; }
///             current *= current;
///             ln >>= 1;
///         }
///         pow
///     }
/// }
/// ```
///
/// # Approach 2: Recursion
///
/// * Time complexity: O(logn)
///
/// * Space complexity: O(1)
///
/// ```rust
/// impl Solution {
///     pub fn my_pow(mut x: f64, mut n: i64) -> f64 {
///         if n < 0 {
///             x = 1.0 / x;
///             n = -n;
///         }
///         if n == 0 { return 1.0; }
///         let half = my_pow(x, n/2);
///         if n & 1 == 0 {
///             return half * half;
///         } else {
///             return half * half * x;
///         }
///     }
/// }
///
pub fn my_pow(mut x: f64, mut n: i64) -> f64 {
    if n < 0 {
        x = 1.0 / x;
        n = -n;
    }
    if n == 0 { return 1.0; }
    let half = my_pow(x, n/2);
    if n & 1 == 0 {
        return half * half;
    } else {
        return half * half * x;
    }
}
