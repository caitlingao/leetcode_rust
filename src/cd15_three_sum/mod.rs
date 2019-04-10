//! Three Sum [leetcode: three_sum](https://leetcode.com/problems/3sum/)
//!
//! Given an array `nums` of *n* integers, are there elements *a*, *b*, *c* in `nums` such that *a + b + c = 0*? Find all unique triplets in the array which gives the sum of zero.
//!
//! ***Note:***
//!
//! The solution set must not contain duplicate triplets.
//!
//! ***Example:***
//!
//! ```
//! Given array nums = [-1, 0, 1, 2, -1, -4],
//!
//! A solution set is:
//! [
//!   [-1, 0, 1],
//!   [-1, -1, 2]
//! ]
//! ```

/// # Solutions
///
/// # Approach 1: Iterator and use left and right item
///
/// * Time complexity: O(n^2)
///
/// * Space complexity: O(1)
///
/// * Runtime: 24 ms
/// * Memory: 4M
///
/// ```rust
/// use std::collections::HashMap;
///
/// impl Solution {
///     pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
///         if nums.len() < 3 { return vec![]; }
///
///         let mut result: Vec<Vec<i32>> = Vec::new();
///         let n_len = nums.len();
///         nums.sort();
///
///         for i in 0..n_len-2 {
///             if i >= 1 && nums[i-1] == nums[i] { continue }
///
///             let (mut left, mut right) = (i + 1, n_len - 1);
///
///             while left < right {
///                 if left > i + 1 && nums[left - 1] == nums[left] { left += 1; continue }
///                 if right < n_len - 1 && nums[right] == nums[right + 1] { right -= 1; continue }
///
///                 if nums[i] + nums[left] + nums[right] > 0 {
///                     right -= 1;
///                 } else if nums[i] + nums[left] + nums[right] < 0 {
///                     left += 1;
///                 } else {
///                     result.push(vec![nums[i], nums[left], nums[right]]);
///                     left += 1;
///                     right -= 1;
///                 }
///             }
///         }
///         result
///     }
/// }
/// ```
///
/// # Approach 2: HashSet
///
/// * Time complexity: O(n^2)
///
/// * Space complexity: O(n)
///
/// * Runtime: 456 ms
/// * Memory: 3.8M
///
/// ```rust
/// use std::collections::HashSet;
///
/// impl Solution {
///     pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
///         if nums.len() < 3 { return vec![]; }
///
///         let mut result = vec![];
///         nums.sort();
///
///         for i in 0..nums.len()-2 {
///             if i >= 1 && nums[i-1] == nums[i] { continue }
///             let mut h_set = HashSet::new();
///             for j in i+1..nums.len() {
///                 if h_set.contains(&nums[j]) {
///                     if result.last() == Some(&vec![nums[i], -nums[i]-nums[j], nums[j]]) { continue }
///                     result.push(vec![nums[i], -nums[i]-nums[j], nums[j]]);
///                 } else {
///                     h_set.insert(-nums[i]-nums[j]);
///                 }
///             }
///         }
///
///         result
///     }
/// }
///
pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.len() < 3 { return vec![]; }

    let mut result: Vec<Vec<i32>> = Vec::new();
    let n_len = nums.len();
    nums.sort();

    for i in 0..n_len-2 {
        if i >= 1 && nums[i-1] == nums[i] { continue }

        let (mut left, mut right) = (i + 1, n_len - 1);

        while left < right {
            if left > i + 1 && nums[left - 1] == nums[left] { left += 1; continue }
            if right < n_len - 1 && nums[right] == nums[right + 1] { right -= 1; continue }

            if nums[i] + nums[left] + nums[right] > 0 {
                right -= 1;
            } else if nums[i] + nums[left] + nums[right] < 0 {
                left += 1;
            } else {
                result.push(vec![nums[i], nums[left], nums[right]]);
                left += 1;
                right -= 1;
            }
        }
    }
    result
}
