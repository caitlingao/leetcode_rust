//! Two Sum [leetcode: two_sum](https://leetcode.com/problems/two-sum/)
//!
//! Given an array of integers, return indices of the two numbers such that they add up to a specific target.
//!
//! You may assume that each input would have exactly one solution, and you may not use the same element twice.
//!
//! ***Example:***
//!
//! ```
//! Given `nums = [2, 7, 11, 15], target = 9`,
//!
//! Because `nums[0] + nums[1] = 2 + 7 = 9`,
//!
//! return `[0, 1]`.
//! ```

use std::collections::HashMap;
/// # Solutions
///
/// # Approach 1: One-pass Hash Table
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(n)
///
/// ```rust
/// use std::collections::HashMap;
///
/// impl Solution {
///     pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
///         if nums.len() < 2 {
///             return vec![];
///         }
///
///         let mut positon = HashMap::new();
///         for(i, num) in nums.iter().enumerate() {
///             if positon.contains_key(num) {
///                 return vec![positon[num] as i32, i as i32];
///             } else {
///                 positon.insert(target - num, i);
///             }
///         }
///
///         return vec![];
///     }
/// }
/// ```
///
/// # Approach 2: Brute Force
///
/// * Time complexity: O(n^2)
///
/// * Space complexity: O(1)
///
/// ```rust
/// impl Solution {
///     pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
///         if nums.len() < 2 {
///             return vec![];
///         }
///
///         let mut v: Vec<i32> = vec![];
///         for i in 0..nums.len() {
///             let n = target - nums[i];
///             for j in (i+1)..nums.len() {
///                 if n == nums[j] {
///                     v.push(i as i32);
///                     v.push(j as i32);
///                     return v;
///                 }
///             }
///         }
///
///         v
///     }
/// }
///
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.len() < 2 {
        return vec![];
    }

    let mut positon = HashMap::new();
    for(i, num) in nums.iter().enumerate() {
        if positon.contains_key(num) {
            return vec![positon[num] as i32, i as i32];
        } else {
            positon.insert(target - num, i);
        }
    }

    return vec![];
}
