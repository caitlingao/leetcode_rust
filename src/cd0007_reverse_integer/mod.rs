//! Reverse Integer [leetcode: reverse_integer](https://leetcode.com/problems/reverse-integer/)
//!
//! Given a 32-bit signed integer, reverse digits of an integer.
//!
//! **Example1:**
//!
//! ```
//! Input: 123
//! Output: 321
//! ```
//!
//! **Example2:**
//!
//! ```
//! Input: -123
//! Output: -321
//! ```
//!
//! **Example3:**
//!
//! ```
//! Input: 120
//! Output: 21
//! ```
//!
//! **Note:**
//!
//! Assume we are dealing with an environment which could only store integers within the 32-bit signed integer range: [−231,  231 − 1]. For the purpose of this problem, assume that your function returns 0 when the reversed integer overflows.
//!
/// # Solutions
///
/// # Approach 1: Pop and Push Digits & Check before Overflow
///
/// * Time complexity: O(log(x))
///
/// * Space complexity: O(1)
///
/// * Runtime: 0 ms
/// * Memory: 2.5 MB
///
/// ```rust
/// impl Solution {
///     pub fn reverse(x: i32) -> i32 {
///         let mut num = x;
///         let mut result = 0 as i64;
///
///         while num != 0 {
///             result = result * 10 + (num % 10) as i64;
///
///             if result > i64::from(std::i32::MAX) || result < i64::from(std::i32::MIN) { return 0; }
///
///             num = num / 10;
///         }
///
///         result as i32
///     }
/// }
/// ```
///
pub fn reverse(x: i32) -> i32 {
    let mut num = x;
    let mut result = 0 as i64;

    while num != 0 {
        result = result * 10 + (num % 10) as i64;

        if result > i64::from(std::i32::MAX) || result < i64::from(std::i32::MIN) { return 0; }

        num = num / 10;
    }

    result as i32
}
