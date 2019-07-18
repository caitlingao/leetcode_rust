//! Search in Rotated Sorted Array [leetcode: search_in_rotated_sorted_array](https://leetcode.com/problems/search-in-rotated-sorted-array/)
//!
//! Suppose an array sorted in ascending order is rotated at some pivot unknown to you beforehand.
//!
//! (i.e., `[0,1,2,4,5,6,7]` might become `[4,5,6,7,0,1,2]`).
//!
//! You are given a target value to search. If found in the array return its index, otherwise return `-1`.
//!
//! You may assume no duplicate exists in the array.
//!
//! Your algorithm's runtime complexity must be in the order of O(log n).
//!
//! **Example1:**
//!
//! ```
//! Input: nums = [4,5,6,7,0,1,2], target = 0
//! Output: 4
//! ```
//!
//! **Example2:**
//!
//! ```
//! Input: nums = [4,5,6,7,0,1,2], target = 3
//! Output: -1
//! ```
//!
/// # Solutions
///
/// # Approach 1: Binary Search
///
/// * Time complexity: O(logn)
///
/// * Space complexity: O(1)
///
/// * Runtime: 0 ms
/// * Memory: 2.5 MB
///
/// ```rust
/// impl Solution {
///     pub fn search(nums: Vec<i32>, target: i32) -> i32 {
///         if nums.is_empty() { return -1; }
///
///         let mut low = 0;
///         let mut high = nums.len() - 1;
///
///         while low <= high {
///             let mid = low + ((high - low) >> 1);
///             if nums[mid] == target { return mid as i32; }
///
///             // left half is sorted
///             if nums[low] <= nums[mid] {
///                 // target is in the left half array
///                if nums[low] <= target && target <= nums[mid] {
///                    high = mid - 1;
///                 }  else {
///                     low = mid + 1;
///                 }
///             } else { // right half is sorted
///                 // target is in the right half array
///                 if nums[mid] <= target && target <= nums[high] {
///                     low = mid + 1;
///                 } else {
///                     high = mid - 1;
///                 }
///             }
///         }
///         -1
///     }
/// }
/// ```
///
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    if nums.is_empty() { return -1; }

    let mut low = 0;
    let mut high = nums.len() - 1;

    while low <= high {
        let mid = low + ((high - low) >> 1);
        if nums[mid] == target { return mid as i32; }

        // left half is sorted
        if nums[low] <= nums[mid] {
            // target is in the left half array
           if nums[low] <= target && target <= nums[mid] {
               high = mid - 1;
            }  else {
                low = mid + 1;
            }
        } else { // right half is sorted
            // target is in the right half array
            if nums[mid] <= target && target <= nums[high] {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
    }
    -1
}
