//! ZigZag Conversion [leetcode: zigzag_conversion](https://leetcode.com/problems/zigzag-conversion/)
//!
//! The string `"PAYPALISHIRING"` is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)
//!
//! ```
//! P   A   H   N
//! A P L S I I G
//! Y   I   R
//! ```
//!
//! And then read line by line: `"PAHNAPLSIIGYIR"`
//!
//! Write the code that will take a string and make this conversion given a number of rows:
//!
//! `string convert(string s, int numRows);`
//!
//! **Example1:**
//!
//! ```
//! Input: s = "PAYPALISHIRING", numRows = 3
//! Output: "PAHNAPLSIIGYIR"
//! ```
//!
//! **Example2:**
//!
//! ```
//! Input: s = "PAYPALISHIRING", numRows = 4
//! Output: "PINALSIGYAHRPI"
//! Explanation:
//!
//! P     I    N
//! A   L S  I G
//! Y A   H R
//! P     I
//! ```
//!
/// # Solutions
///
/// # Approach 1: Visit by Row
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(n)
///
/// * Runtime: 4 ms
/// * Memory: 2.4 MB
///
/// ```rust
/// impl Solution {
///     pub fn convert(s: String, num_rows: i32) -> String {
///         let n = s.len() as i32;
///         if n == 0 || num_rows <= 1 { return s; }
///
///         let s_bytes = s.as_bytes();
///         let mut bytes_vec = vec![];
///         let circle_length = 2 * num_rows - 2;
///
///         for i in 0..num_rows {
///             let mut j = i;
///             while j < n {
///                 bytes_vec.push(s_bytes[j as usize]);
///                 if i != 0 && i != (num_rows - 1) && (j - i) + (circle_length - i) < n {
///                     bytes_vec.push(s_bytes[((j - i)+(circle_length -i)) as usize]);
///                 }
///                 j += circle_length;
///             }
///         }
///         String::from_utf8(bytes_vec).unwrap()
///     }
/// }
/// ```
///
pub fn convert(s: String, num_rows: i32) -> String {
    let n = s.len() as i32;
    if n == 0 || num_rows <= 1 { return s; }

    let s_bytes = s.as_bytes();
    let mut bytes_vec = vec![];
    let circle_length = 2 * num_rows - 2;

    for i in 0..num_rows {
        let mut j = i;
        while j < n {
            bytes_vec.push(s_bytes[j as usize]);
            if i != 0 && i != (num_rows - 1) && (j - i) + (circle_length - i) < n {
                bytes_vec.push(s_bytes[((j - i)+(circle_length -i)) as usize]);
            }
            j += circle_length;
        }
    }
    String::from_utf8(bytes_vec).unwrap()
}
