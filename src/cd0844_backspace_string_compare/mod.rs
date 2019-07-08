//! Backspace String Compare [leetcode: backspace_string_compare](https://leetcode.com/problems/backspace-string-compare/)
//!
//! Given two strings `S` and `T`, return if they are equal when both are typed into empty text editors. `#` means a backspace character.
//!
//! **Example1:**
//!
//! ```
//! Input: S = "ab#c", T = "ad#c"
//! Output: true
//! Explanation: Both S and T become "ac".
//! ```
//!
//! **Example2:**
//!
//! ```
//! Input: S = "ab##", T = "c#d#"
//! Output: true
//! Explanation: Both S and T become "".
//! ```
//!
//! **Example3:**
//!
//! ```
//! Input: S = "a##c", T = "#a#c"
//! Output: true
//! Explanation: Both S and T become "c".
//! ```
//!
//! **Example4:**
//!
//! ```
//! Input: S = "a#c", T = "b"
//! Output: false
//! Explanation: S becomes "c" while T becomes "b".
//! ```
//!
//! **Note**
//!
//! 1. `1 <= S.length <= 200`
//! 2. `1 <= T.length <= 200`
//! 3. `S` and `T` only contain lowercase letters and `'#'` characters.
//!
//! **Follow up:**
//!
//! * Can you solve it in `O(N)` time and `O(1)` space?
//!
/// # Solutions
///
/// # Approach 1: Build String
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(n)
///
/// * Runtime: 0ms
///
/// Memory: 2.4MB
///
/// ```rust
/// impl Solution {
///     pub fn backspace_compare(s: String, t: String) -> bool {
///         Self::build(s) == Self::build(t)
///     }
///
///     pub fn build(s: String) -> Vec<char> {
///         let mut stack = vec![];
///         for ch in s.chars() {
///             if ch != '#' {
///                 stack.push(ch);
///             } else {
///                 stack.pop();
///             }
///         }
///         stack
///     }
/// }
/// ```
///
///
/// # Approach 2: Two Pointer
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(1)
///
/// * Runtime: 0ms
///
/// Memory: 2.3MB
///
/// ```rust
/// impl Solution {
///     pub fn backspace_compare(s: String, t: String) -> bool {
///         let s_bytes = s.as_bytes();
///         let t_bytes = t.as_bytes();
///         let mut i = s_bytes.len();
///         let mut j = t_bytes.len();
///         let mut skip = 0;
///
///         loop {
///             while i > 0 {
///                 if s_bytes[i-1] == b'#' {
///                     skip += 1;
///                 } else if skip > 0 {
///                     skip -= 1;
///                 } else {
///                     break;
///                 }
///                 i -= 1;
///             }
///
///             while j > 0 {
///                 if t_bytes[j-1] == b'#' {
///                     skip += 1;
///                 } else if skip > 0 {
///                     skip -= 1;
///                 } else {
///                     break;
///                 }
///                 j -= 1;
///             }
///
///             if i > 0 && j > 0 && s_bytes[i-1] == t_bytes[j-1] {
///                 i -= 1;
///                 j -= 1;
///             } else {
///                 return i == 0 && j == 0 // only success when both totally consumed!
///             }
///         }
///     }
/// }
/// ```
pub fn backspace_compare(s: String, t: String) -> bool {
    let s_bytes = s.as_bytes();
    let t_bytes = t.as_bytes();
    let mut i = s_bytes.len();
    let mut j = t_bytes.len();
    let mut skip = 0;

    loop {
        while i > 0 {
            if s_bytes[i-1] == b'#' {
                skip += 1;
            } else if skip > 0 {
                skip -= 1;
            } else {
                break;
            }
            i -= 1;
        }

        while j > 0 {
            if t_bytes[j-1] == b'#' {
                skip += 1;
            } else if skip > 0 {
                skip -= 1;
            } else {
                break;
            }
            j -= 1;
        }

        if i > 0 && j > 0 && s_bytes[i-1] == t_bytes[j-1] {
            i -= 1;
            j -= 1;
        } else {
            return i == 0 && j == 0
        }
    }
}
