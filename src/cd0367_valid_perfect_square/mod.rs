//! Valid Perfect Square [leetcode: valid_perfect_square](https://leetcode.com/problems/valid-perfect-square/)
//!
//! Given a positive integer *num*, write a function which returns True if *num* is a perfect square else False.
//!
//! **Note: Do not** use any built-in library function such as `sqrt`.
//!
//! **Example 1:**
//! ```
//! Input: 16
//! Output: true
//! ```
//!
//! **Example 2:**
//! ```
//! Input: 14
//! Output: false
//! ```
//!
/// # Solutions
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
///     pub fn is_perfect_square(num: i32) -> bool {
///         if num == 0 { return false; }
///
///         let num = num as usize;
///         let mut left = 1 as usize;
///         let mut right = num;
///
///         while left <= right {
///             let mid = (right - left) / 2 as usize + left;
///             if mid * mid == num { return true; }
///             if mid * mid > num { right = mid - 1 as usize; } else { left = mid + 1 as usize; }
///         }
///
///         false
///     }
/// }
/// ```
///
pub fn is_perfect_square(num: i32) -> bool {
    if num == 0 { return false; }

    let num = num as usize;
    let mut left = 1 as usize;
    let mut right = num;

    while left <= right {
        let mid = (right - left) / 2 as usize + left;
        if mid * mid == num { return true; }
        if mid * mid > num { right = mid - 1 as usize; } else { left = mid + 1 as usize; }
    }

    false
}
