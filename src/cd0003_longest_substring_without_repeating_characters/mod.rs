//! Longest Substring Without Repeating Characters [leetcode: longest_substring_without_repeating_characters](https://leetcode.com/problems/longest-substring-without-repeating-characters/)
//!
//! Given a string, find the length of the **longest substring** without repeating characters.
//!
//! **Example1:**
//!
//! ```
//! Input: "abcabcbb"
//! Output: 3
//! Explanation: The answer is "abc", with the length of 3.
//! ```
//!
//! **Example2:**
//!
//! ```
//! Input: "bbbbb"
//! Output: 1
//! Explanation: The answer is "b", with the length of 1.
//! ```
//!
//! **Example3:**
//!
//! ```
//! Input: "pwwkew"
//! Output: 3
//! Explanation: The answer is "wke", with the length of 3.
//!              Note that the answer must be a substring, "pwke" is a subsequence and not a substring.
//! ```
//!
/// # Solutions
///
/// # Approach 1: Sliding Window
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(n)
///
/// * Runtime: 4 ms
/// * Memory: 2.5 MB
///
/// ```rust
/// use std::collections::HashMap;
///
/// impl Solution {
///     pub fn length_of_longest_substring(s: String) -> i32 {
///         let mut window_hash = HashMap::new();
///         let mut start = 0;
///         let mut res = 0;
///
///         // 检查字母在 hash 中是否存在，如果存在，窗口开始位置为在 hash 中出现的字母的最后位置
///         for (end, ch) in s.chars().enumerate() {
///             if let Some(pos) = window_hash.get(&ch) {
///                 start = start.max(*pos);
///             }
///             res = res.max(end + 1 - start);
///             window_hash.insert(ch, end + 1);
///         }
///         res as i32
///     }
/// }
/// ```
///
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut window_hash = HashMap::new();
    let mut start = 0;
    let mut res = 0;

    // 检查字母在 hash 中是否存在，如果存在，窗口开始位置为在 hash 中出现的字母的最后位置
    for (end, ch) in s.chars().enumerate() {
        if let Some(pos) = window_hash.get(&ch) {
            start = start.max(*pos);
        }
        res = res.max(end + 1 - start);
        window_hash.insert(ch, end + 1);
    }
    res as i32
}

use std::collections::HashMap;
