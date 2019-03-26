//! Valid Parentheses [leetcode: valid_parentheses](https://leetcode.com/problems/valid-parentheses/)
//!
//! Given a string containing just the characters `'('`, `')'`, `'{'`, `'}'`, `'['` and `']'`, determine if the input string is valid.
//!
//! An input string is valid if:
//!
//! 1. Open brackets must be closed by the same type of brackets.
//! 2. Open brackets must be closed in the correct order.
//!
//! Note that an empty string is also considered valid.
//!
//! ***Example1:***
//!
//! ```
//! Input: "()"
//!
//! Output: true
//! ```
//!
//! ***Example2:***
//!
//! ```
//! Input: "()[]{}"
//!
//! Output: true
//! ```
//!
//! ***Example3:***
//!
//! ```
//! Input: "(]"
//!
//! Output: false
//! ```
//!
//! ***Example4:***
//!
//! ```
//! Input: "([)]"
//!
//! Output: false
//! ```
//!
//! ***Example5:***
//!
//! ```
//! Input: "{[]}"
//!
//! Output: true
//! ```

use std::collections::HashMap;
/// # Solutions
///
/// # Approach 1: Stacks
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(n)
///
/// ```rust
/// use std::collections::HashMap;
///
/// impl Solution {
///     pub fn is_valid(s: String) -> bool {
///         let mut v = vec![];
///         let mut str_map: HashMap<char, char> = HashMap::new();
///         str_map.insert(')', '(');
///         str_map.insert('}', '{');
///         str_map.insert(']', '[');
///
///         for s1 in s.chars() {
///             if str_map.contains_key(&s1) {
///                 if v.pop() != Some(str_map[&s1]) {
///                     return false;
///                 }
///             } else {
///                 v.push(s1)
///             }
///         }
///
///         v.is_empty()
///     }
/// }
/// ```
///
pub fn is_valid(s: String) -> bool {
    let mut v = vec![];
    let mut str_map: HashMap<char, char> = HashMap::new();
    str_map.insert(')', '(');
    str_map.insert('}', '{');
    str_map.insert(']', '[');

    for s1 in s.chars() {
        if str_map.contains_key(&s1) {
            if v.pop() != Some(str_map[&s1]) {
                return false;
            }
        } else {
            v.push(s1)
        }
    }

    v.is_empty()
}

/// # Solutions
///
/// # Approach 1: Stacks
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(n)
///
/// ```rust
/// use std::collections::HashMap;
///
/// impl Solution {
///     pub fn is_valid2(s: String) -> bool {
///         let mut paren_stack = vec![];
///         let mut paren_map: HashMap<char, char> = HashMap::new();
///         paren_map.insert(')', '(');
///         paren_map.insert('}', '{');
///         paren_map.insert(']', '[');
///
///         for ch in s.chars() {
///             match ch {
///                 '(' | '{' | '[' => paren_stack.push(ch),
///                 _ => {
///                     let expected = *paren_map.get(&ch).unwrap();
///                     let actual = paren_stack.pop().unwrap();
///                     if expected != actual {
///                         return false;
///                     }
///                 }
///             }
///         }
///
///         paren_stack.is_empty()
///     }
/// }
/// ```
///
pub fn is_valid2(s: String) -> bool {
    let mut paren_stack = vec![];
    let mut paren_map: HashMap<char, char> = HashMap::new();
    paren_map.insert(')', '(');
    paren_map.insert('}', '{');
    paren_map.insert(']', '[');

    for ch in s.chars() {
        match ch {
            '(' | '{' | '[' => paren_stack.push(ch),
            _ => {
                let expected = *paren_map.get(&ch).unwrap();
                let actual = paren_stack.pop().unwrap();
                if expected != actual {
                    return false;
                }
            }
        }
    }

    paren_stack.is_empty()
}
