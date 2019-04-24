//! Sliding Window Maximum [leetcode: sliding_window_maximum](https://leetcode.com/problems/sliding-window-maximum/)
//!
//! Given an array nums, there is a sliding window of size k which is moving from the very left of the array to the very right. You can only see the k numbers in the window. Each time the sliding window moves right by one position. Return the max sliding window.
//!
//! ***Example:***
//!
//!```
//! Input: nums = [1,3,-1,-3,5,3,6,7], and k = 3
//! Output: [3,3,5,5,6,7]
//! Explanation:
//!
//! Window position                Max
//! ---------------               -----
//! [1  3  -1] -3  5  3  6  7       3
//!  1 [3  -1  -3] 5  3  6  7       3
//!  1  3 [-1  -3  5] 3  6  7       5
//!  1  3  -1 [-3  5  3] 6  7       5
//!  1  3  -1  -3 [5  3  6] 7       6
//!  1  3  -1  -3  5 [3  6  7]      7
//!```
//! ***Note:***
//! You may assume k is always valid, 1 ≤ k ≤ input array's size for non-empty array.
//!
//! ***Follow up:***
//! Could you solve it in linear time?

use std::collections::VecDeque;
/// # Solutions
///
/// # Approach 1: Deque
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(k)
///
/// ```rust
/// use std::collections::VecDeque;
///
/// pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
///     let mut deque = VecDeque::new();
///     let mut result = vec![];
///     let mut start = 0;
///     let k = k as usize;
///
///     for index in 0..nums.len() as usize {
///         while !deque.get(start).is_none() && nums[*deque.back().unwrap()] < nums[index] {
///             deque.pop_back();
///         }
///
///         deque.push_back(index);
///
///         if index >= k && *deque.get(start).unwrap() == (index - k) {
///             start += 1;
///         }
///
///         if index >= k - 1 {
///             result.push(nums[*deque.get(start).unwrap()]);
///         }
///     }
///
///     result
/// }
/// ```
///
/// # Approach 2: Deque
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(k)
///
/// ```rust
/// use std::collections::VecDeque;
///
/// pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
///     let mut window = VecDeque::new();
///     let mut result = vec![];
///     let k = k as usize;
///
///     for i in 0..nums.len() {
///         if i > k && window[0] <= i - k {
///             window.pop_front();
///         }
///
///         while !window.is_empty() && nums[*window.back().unwrap()] <= nums[i] {
///             window.pop_back();
///         }
///
///         window.push_back(i);
///
///         if i >= k - 1 {
///             result.push(nums[*window.front().unwrap()]);
///         }
///     }
///     result
/// }
/// ```
///
pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut deque = VecDeque::new();
    let mut result = vec![];
    let mut start = 0;
    let k = k as usize;

    for index in 0..nums.len() as usize {
        while !deque.get(start).is_none() && nums[*deque.back().unwrap()] < nums[index] {
            deque.pop_back();
        }

        deque.push_back(index);

        if index >= k && *deque.get(start).unwrap() == (index - k) {
            start += 1;
        }

        if index >= k - 1 {
            result.push(nums[*deque.get(start).unwrap()]);
        }
    }

    result
}
