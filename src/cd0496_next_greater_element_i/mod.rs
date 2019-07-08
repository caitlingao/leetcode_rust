//! Next Greater Element I [leetcode: next_greater_element_i](https://leetcode.com/problems/next-greater-element-i/)
//!
//! You are given two arrays **(without duplicates)** `nums1` and `nums2` where `nums1`’s elements are subset of `nums2`. Find all the next greater numbers for nums1's elements in the corresponding places of nums2.
//!
//! The Next Greater Number of a number x in `nums1` is the first greater number to its right in `nums2`. If it does not exist, output -1 for this number.
//!
//! **Example1:**
//!
//! ```
//! Input: nums1 = [4,1,2], nums2 = [1,3,4,2].
//! Output: [-1,3,-1]
//! Explanation:
//!     For number 4 in the first array, you cannot find the next greater number for it in the second array, so output -1.
//!     For number 1 in the first array, the next greater number for it in the second array is 3.
//!     For number 2 in the first array, there is no next greater number for it in the second array, so output -1.
//! ```
//!
//! **Example2:**
//!
//! ```
//! Input: nums1 = [2,4], nums2 = [1,2,3,4].
//! Output: [3,-1]
//! Explanation:
//!     For number 2 in the first array, the next greater number for it in the second array is 3.
//!     For number 4 in the first array, there is no next greater number for it in the second array, so output -1.
//! ```
//!
//! **Note**
//!
//! 1. All elements in `nums1` and `nums2` are unique.
//! 2. The length of both `nums1` and `nums2` would not exceed 1000.
//!
use std::collections::HashMap;
/// # Solutions
///
/// # Approach 1: HashMap
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
/// use std::collections::HashMap;
///
/// impl Solution {
///     pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
///         let mut stack = vec![-1; nums1.len()];
///         let mut nums_map: HashMap<&i32, usize> = HashMap::new();
///
///         for (i, num) in nums2.iter().enumerate() {
///             nums_map.insert(num, i);
///         }
///
///         for (i, num) in nums1.iter().enumerate() {
///             for j in *nums_map.get(num).unwrap()..nums2.len() {
///                 if nums2[j] > *num {
///                     stack[i] = nums2[j];
///                     break;
///                 }
///             }
///         }
///         stack
///     }
/// }
/// ```
///
///
/// # Approach 2: Stack
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(n)
///
/// * Runtime: 0ms
///
/// Memory: 2.6MB
///
/// ```rust
/// use std::collections::HashMap;
///
/// impl Solution {
///     pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
///         let mut nums_map: HashMap<&i32, usize> = HashMap::new();
///         let mut stack = vec![];
///         let mut result = vec![-1; nums1.len()];
///
///         for (i, num) in nums1.iter().enumerate() {
///             nums_map.insert(num, i);
///         }
///
///         for num in nums2.into_iter() {
///             while !stack.is_empty() && *stack.last().unwrap() < num {
///                 if let Some(val) = nums_map.get(&stack.pop().unwrap()) {
///                     result[*val] = num;
///                 }
///             }
///             stack.push(num);
///         }
///         result
///     }
/// }
/// ```
///
pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut nums_map: HashMap<&i32, usize> = HashMap::new();
    let mut stack = vec![];
    let mut result = vec![-1; nums1.len()];

    for (i, num) in nums1.iter().enumerate() {
        nums_map.insert(num, i);
    }

    for num in nums2.into_iter() {
        while !stack.is_empty() && *stack.last().unwrap() < num {
            if let Some(val) = nums_map.get(&stack.pop().unwrap()) {
                result[*val] = num;
            }
        }
        stack.push(num);
    }
    result
}
