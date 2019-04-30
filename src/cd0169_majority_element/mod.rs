//! Majority Element [leetcode: majority_element](https://leetcode.com/problems/majority-element/)
//!
//! Given an array of size n, find the majority element. The majority element is the element that appears **more than** `⌊ n/2 ⌋` times.
//!
//! You may assume that the array is non-empty and the majority element always exist in the array.
//!
//! ***Example1:***
//!
//! ```
//! Input: [3,2,3]
//! Output: 3
//!
//! ```
//!
//! ***Example2:***
//!
//! ```
//! Input: [2,2,1,1,1,2,2]
//! Output: 2
//! ```

use std::collections::HashMap;
/// # Solutions
///
/// # Approach 1: Hash Map
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(n)
///
/// * Runtime: 12 ms
/// * Memory: 2.9 MB
///
/// ```rust
/// use std::collections::HashMap;
///
/// impl Solution {
///     pub fn majority_element(nums: Vec<i32>) -> i32 {
///         let mut map = HashMap::new();
///         let len = nums.len();
///         for num in nums {
///            let new_count = match map.get(&num) {
///                None => 1,
///                Some(x) => x+1,
///             } ;
///             if new_count * 2 >= len {
///                 return num
///             }
///             map.insert(num, new_count);
///         }
///         0
///     }
/// }
/// ```
///
/// # Approach 2: Sort
///
/// * Time complexity: O(nlgn)
///
/// * Space complexity: O(lgn)
///
/// * Runtime: 0 ms
/// * Memory: 2.9 MB
///
/// ```rust
/// impl Solution {
///     pub fn majority_element(nums: Vec<i32>) -> i32 {
///         nums.sort();
///         nums[nums.len()/2]
///     }
/// }
///
pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    let len = nums.len();
    for num in nums {
       let new_count = match map.get(&num) {
           None => 1,
           Some(x) => x+1,
        } ;
        if new_count * 2 >= len { return num; }
        map.insert(num, new_count);
    }
    0
}
