//! Generate Parentheses [leetcode: generate_parentheses](https://leetcode.com/problems/generate-parentheses/)
//!
//! Given *n* pairs of parentheses, write a function to generate all combinations of well-formed parentheses.
//!
//! For example, given `n = 3`, a solution set is:
//!
//! ***Example:***
//!
//! ```
//! [
//!   "((()))",
//!   "(()())",
//!   "(())()",
//!   "()(())",
//!   "()()()"
//! ]
//! ```
/// # Solutions
///
/// # Approach 1: Backtracking
///
/// * Time complexity:
///
/// * Space complexity:
///
/// ```rust
/// impl Solution {
///     pub fn generate_parenthesis(n: i32) -> Vec<String> {
///         let mut result: Vec<String> = vec![];
///         Self::_gen(&mut result, n, n, "".to_string());
///         result
///     }
///     pub fn _gen(result: &mut Vec<String>, left: i32, right: i32, sublist: String) {
///         if left == 0 && right == 0 {
///             result.push(sublist);
///             return;
///         }
///         if left > 0 {
///             Self::_gen(result, left - 1, right, sublist.clone() + "(");
///         }
///         if right > left {
///             Self::_gen(result, left, right - 1, sublist.clone() + ")");
///         }
///     }
/// }
/// ```
///
pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    _gen(&mut result, n, n, "".to_string());
    result
}
fn _gen(result: &mut Vec<String>, left: i32, right: i32, sublist: String) {
    if left == 0 && right == 0 {
        result.push(sublist);
        return;
    }
    if left > 0 {
        _gen(result, left - 1, right, sublist.clone() + "(");
    }
    if right > left {
        _gen(result, left, right - 1, sublist.clone() + ")");
    }
}
