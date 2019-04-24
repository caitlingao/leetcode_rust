//! Valid Anagram [leetcode: valid_anagram](https://leetcode.com/problems/valid-anagram/)
//!
//! Given two strings *s* and *t* , write a function to determine if *t* is an anagram of *s*.
//!
//! ***Example1:***
//!
//!```
//! Input: s = "anagram", t = "nagaram"
//! Output: true
//!```
//!
//! ***Example2:***
//!
//!```
//! Input: s = "rat", t = "car"
//! Output: false
//!```
//!
//! ***Note:***
//!
//! You may assume the string contains only lowercase alphabets.
//!
//! ***Follow up:***
//!
//! What if the inputs contain unicode characters? How would you adapt your solution to such case?

use std::collections::HashMap;
/// # Solutions
///
/// # Approach 1: HashMap
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(1)
///
/// ```rust
/// use std::collections::HashMap;
///
/// pub fn is_anagram(s: String, t: String) -> bool {
///     if s.len() != t.len() { return false; }
///
///     let mut s_hash = HashMap::new();
///     let mut t_hash = HashMap::new();
///
///     for c in s.chars() {
///         *s_hash.entry(c).or_insert(0) += 1;
///     }
///
///     for c in t.chars() {
///         *t_hash.entry(c).or_insert(0) += 1;
///     }
///
///     for (key, val) in s_hash.iter() {
///         if t_hash.get(key) != Some(val) { return false; }
///     }
///
///     true
/// }
/// ```
///
/// # Approach 2: Vec Sort
///
/// * Time complexity: O(nlogn)
///
/// * Space complexity: O(1)
///
/// ```rust
///
/// pub fn is_anagram(s: String, t: String) -> bool {
///     if s.len() != t.len() { return false; }
///
///     let mut s_chars: Vec<char> = s.chars().collect();
///     let mut t_chars: Vec<char> = t.chars().collect();
///
///     s_chars.sort_by(|a, b| a.cmp(b));
///     t_chars.sort_by(|a, b| a.cmp(b));
///
///     s_chars == t_chars
/// }
/// ```
///
pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() { return false; }

    let mut s_hash = HashMap::new();
    let mut t_hash = HashMap::new();

    for c in s.chars() {
        *s_hash.entry(c).or_insert(0) += 1;
    }

    for c in t.chars() {
        *t_hash.entry(c).or_insert(0) += 1;
    }

    for (key, val) in s_hash.iter() {
        if t_hash.get(key) != Some(val) { return false; }
    }

    true
}
