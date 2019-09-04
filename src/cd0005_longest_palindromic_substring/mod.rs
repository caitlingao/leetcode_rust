//! Longest Palindromic Substring [leetcode: longest_palindromic_substring](https://leetcode.com/problems/longest-palindromic-substring/)
//!
//! Given a string s, find the longest palindromic substring in s. You may assume that the maximum length of s is 1000.
//!
//! **Example1:**
//!
//! ```
//! Input: "babad"
//! Output: "bab"
//! Note: "aba" is also a valid answer.
//! ```
//!
//! **Example2:**
//!
//! ```
//! Input: "cbbd"
//! Output: "bb"
//! ```
//!
/// # Solutions
///
/// # Approach 1: Expand Around Center
///
/// * Time complexity: O(n2)
///
/// * Space complexity: O(1)
///
/// * Runtime: 4 ms
/// * Memory: 2.4 MB
///
/// ```rust
/// impl Solution {
///     pub fn longest_palindrome(s: String) -> String {
///         let n = s.len() as i32;
///         if n == 0 { return s; }
///
///         let (mut start, mut end) = (0, 0);
///         let s_bytes = s.as_bytes();
///
///         for i in 0..n {
///             let (mut left, mut right) = (i as i32, i as i32);
///
///             while right < n - 1 && s_bytes[right as usize] == s_bytes[(right + 1) as usize] {
///                 right += 1;
///             }
///
///             while left >= 0 && right < n && s_bytes[left as usize] == s_bytes[right as usize] {
///                 if (right - left) as usize > end - start {
///                     start = left as usize;
///                     end = right as usize;
///                 }
///
///                 left -= 1;
///                 right += 1;
///             }
///         }
///         String::from_utf8(s_bytes[start..=end].to_vec()).unwrap()
///     }
/// }
/// ```
///
pub fn longest_palindrome(s: String) -> String {
    let n = s.len() as i32;
    if n == 0 { return s; }

    let (mut start, mut end) = (0, 0);
    let s_bytes = s.as_bytes();

    for i in 0..n {
        let (mut left, mut right) = (i as i32, i as i32);

        while right < n - 1 && s_bytes[right as usize] == s_bytes[(right + 1) as usize] {
            right += 1;
        }

        while left >= 0 && right < n && s_bytes[left as usize] == s_bytes[right as usize] {
            if (right - left) as usize > end - start {
                start = left as usize;
                end = right as usize;
            }

            left -= 1;
            right += 1;
        }
    }
    String::from_utf8(s_bytes[start..=end].to_vec()).unwrap()
}
