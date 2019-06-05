//! Power of Two [power_of_two](https://leetcode.com/problems/power-of-two/)
//!
//! Given an integer, write a function to determine if it is a power of two.
//!
//! ***Example1:***
//!
//! ```
//! Input: 1
//! Output: true
//! Explanation: 20 = 1
//! ```
//!
//! ***Example2:***
//!
//! ```
//! Input: 16
//! Output: true
//! Explanation: 24 = 16
//! ```
//!
//! ***Example3:***
//!
//! ```
//! Input: 218
//! Output: false
//! ```

/// # Solutions
///
/// # Approach 1: Bit Manipulation Trick
///
/// * Time complexity: O(1)
///
/// * Space complexity: O(1)
///
/// * Runtime: 0 ms
/// * Memory: 2.4 MB
///
/// ```rust
/// impl Solution {
///     pub fn is_power_of_two(n: i32) -> bool {
///         if n > 0 && (n & (n - 1) == 0) { true } else { false }
///     }
/// }
/// ```
///
pub fn is_power_of_two(n: i32) -> bool {
    if n > 0 && (n & (n - 1) == 0) { true } else { false }
}
