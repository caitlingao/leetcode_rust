//! Basic Calculator [leetcode: basic_calculator](https://leetcode.com/problems/basic-calculator/)
//!
//! Implement a basic calculator to evaluate a simple expression string.
//!
//! The expression string may contain open `(` and closing parentheses `)`, the plus `+` or minus sign `-`, **non-negative** integers and empty spaces .
//!
//! **Example1:**
//!
//! ```
//! Input: "1 + 1"
//! Output: 2
//! ```
//!
//! **Example2:**
//!
//! ```
//! Input: " 2-1 + 2 "
//! Output: 3
//! ```
//!
//! **Example3:**
//!
//! ```
//! Input: "(1+(4+5+2)-3)+(6+8)"
//! Output: 23
//! ```
//!
//! **Note**
//!
//! * You may assume that the given expression is always valid.
//!
//! * **Do not** use the `eval` built-in library function.
//!
/// # Solutions
///
/// # Approach 1: Stack
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(n)
///
/// * Runtime: 0ms
///
/// Memory: 2.9MB
///
/// ```rust
/// impl Solution {
///     pub fn calculate(s: String) -> i32 {
///         let mut stack = vec![];
///         let mut operand = 0;
///         let mut result = 0;
///         let mut sign = 1;
///
///         stack.push(sign);
///         for ch in s.chars() {
///             match ch {
///                 '0'...'9' => {
///                     operand = 10 * operand + (ch as u8 - '0' as u8) as i32;
///                 },
///                 '(' => {
///                     stack.push(sign);
///                 },
///                 ')' => {
///                     stack.pop();
///                 },
///                 '+' | '-' => {
///                     result += sign * operand;
///                     operand = 0;
///                     sign = stack.last().unwrap() * if ch == '+' { 1 } else { -1 };
///                 },
///                 _ => {},
///             }
///         }
///         result + (sign * operand)
///     }
/// }
/// ```
///
pub fn calculate(s: String) -> i32 {
    let mut stack = vec![];
    let mut operand = 0;
    let mut result = 0;
    let mut sign = 1;

    stack.push(sign);
    for ch in s.chars() {
        match ch {
            '0'..='8' => {
                operand = 10 * operand + (ch as u8 - '0' as u8) as i32;
            },
            '(' => {
                stack.push(sign);
            },
            ')' => {
                stack.pop();
            },
            '+' | '-' => {
                result += sign * operand;
                operand = 0;
                sign = stack.last().unwrap() * if ch == '+' { 1 } else { -1 };
            },
            _ => {},
        }
    }
    result + (sign * operand)
}
