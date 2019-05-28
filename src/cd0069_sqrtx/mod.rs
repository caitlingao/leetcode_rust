//! Sqrt(x) [leetcode: sqrt(x)](https://leetcode.com/problems/sqrtx/)
//!
//! Implement `int sqrt(int x)`.
//!
//! Compute and return the square root of *x*, where *x* is guaranteed to be a non-negative integer.
//!
//! Since the return type is an integer, the decimal digits are truncated and only the integer part of the result is returned.
//!
//! **Example 1:**
//! ```
//! Input: 4
//! Output: 2
//! ```
//!
//! **Example 2:**
//! ```
//! Input: 8
//! Output: 2
//! Explanation: The square root of 8 is 2.82842..., and since
//!              the decimal part is truncated, 2 is returned.
//! ```
//!
/// # Solutions for i32
///
/// # Approach 1: Binary Search
///
/// * Time complexity: log(n)
///
/// * Space complexity: log(n)
///
/// * Runtime: 0 ms
/// * Memory: 2.4 MB
///
/// ```rust
/// impl Solution {
///     pub fn my_sqrt(x: i32) -> i32 {
///         if x == 0 || x == 1 { return x; }
///
///         let mut left = 1;
///         let mut right = x;
///         let mut res = 1;
///
///         while left <= right {
///             let mid = (right - left) / 2 + left;
///             if x / mid == mid { return mid; };
///             if mid > x / mid {
///                 right = mid - 1;
///             } else {
///                 left = mid + 1;
///                 res = mid;
///             }
///         }
///         res
///     }
/// }
/// ```
///
/// # Approach 2: Newton's Method
///
/// * Time complexity: 𝑙𝑜𝑔2(𝑁)
///
/// * Space complexity:
///
/// * Runtime: 0 ms
/// * Memory: 2.4 MB
///
/// ```rust
/// impl Solution {
///     pub fn my_sqrt(x: i32) -> i32 {
///         if x == 0 { return x; }
///
///         let x = x as usize;
///         let mut r = x;
///         while r > x / r {
///             r = (r + x / r) / 2;
///         }
///         r as i32
///     }
/// }
/// ```
///
/// # Approach 3: Lomont's Paper
///
/// * Time complexity:
///
/// * Space complexity:
///
/// * Runtime: 0 ms
/// * Memory: 2.4 MB
///
/// ```rust
/// use::std::mem;
///
/// impl Solution {
///     pub fn my_sqrt(x: i32) -> i32 {
///         let mut n = x as f64;
///         let half = 0.5 * n;
///         let mut i = unsafe {
///             std::mem::transmute::<f64, i64>(n)
///         };
///         i = 0x5fe6ec85e7de30da - (i>>1);
///         n = unsafe{
///             std::mem::transmute::<i64, f64>(i)
///         };
///         for _i in 0..3 {
///             n = n * (1.5 - half * n * n);
///         }
///         (1.0 / n) as i32
///     }
/// }
/// ```
///
pub fn my_sqrt(x: i32) -> i32 {
    if x == 0 || x == 1 { return x; }

    let mut left = 1;
    let mut right = x;
    let mut res = 1;

    while left <= right {
        let mid = (right - left) / 2 + left;
        if x / mid == mid { return mid; };
        if mid > x / mid {
            right = mid - 1;
        } else {
            left = mid + 1;
            res = mid;
        }
    }
    res
}

/// # Solutions for f64
///
/// # Approach 1: Binary Search
///
/// * Time complexity: log(n)
///
/// * Space complexity: log(n)
///
/// * Runtime: 0 ms
/// * Memory: 2.4 MB
///
/// ```rust
/// impl Solution {
///     fn my_sqrt_f64(x: i32, precision: f64) -> f64 {
///         if x == 0 || x == 1 { return x as f64; }
///
///         let mut left = 0f64;
///         let mut right = x as f64;
///         let mut res = 0f64;
///
///         while left <= right {
///             let mid: f64 = (right - left) / 2f64 + left;
///             if (right - left).abs() < precision { return mid; }
///             if mid > x as f64 / mid {
///                 right = mid;
///             } else {
///                 left = mid;
///                 res = mid
///             }
///         }
///         res
///     }
/// }
/// ```
///
/// # Approach 2: Newton's Method
///
/// * Time complexity: 𝑙𝑜𝑔2(𝑁)
///
/// * Space complexity:
///
/// * Runtime: 0 ms
/// * Memory: 2.4 MB
///
/// ```rust
/// impl Solution {
///     pub fn my_sqrt_f64(x: i32) -> f64 {
///         if x == 0 { return x as f64; }
///
///         let x = x as f64;
///         let mut r = x;
///         while r > x / r {
///             r = (r + x / r) / 2.0;
///         }
///         r
///     }
/// }
/// ```
///
pub fn my_sqrt_f64(x: i32, precision: f64) -> f64 {
    if x == 0 || x == 1 { return x as f64; }

    let mut left = 0f64;
    let mut right = x as f64;
    let mut res = 0f64;

    while left <= right {
        let mid: f64 = (right - left) / 2.0 + left;
        if (right - left).abs() < precision { return mid; }
        if mid > x as f64 / mid {
            right = mid;
        } else {
            left = mid;
            res = mid
        }
    }
    res
}
