//! Median of Two Sorted Arrays [leetcode: median_of_two_sorted_arrays](https://leetcode.com/problems/median-of-two-sorted-arrays/)
//!
//! There are two sorted arrays **nums1** and **nums2** of size m and n respectively.
//!
//! Find the median of the two sorted arrays. The overall run time complexity should be O(log (m+n)).
//!
//! You may assume **nums1** and **nums2** cannot be both empty.
//!
//! **Example1:**
//!
//! ```
//! nums1 = [1, 3]
//! nums2 = [2]
//!
//! The median is 2.0
//! ```
//!
//! **Example2:**
//!
//! ```
//! nums1 = [1, 2]
//! nums2 = [3, 4]
//!
//! The median is (2 + 3)/2 = 2.5
//! ```
//!
/// # Solutions
///
/// # Approach 1: Recursive Approach
///
/// * Time complexity: O(log(min(m,n)))
///
/// * Space complexity: O(1)
///
/// * Runtime: 0 ms
/// * Memory: 2.5 MB
///
/// ```rust
/// // 思路
/// // 将 nums1 与 nums2 使用二分法分别将其拆分为两半, nums1_left, nums1_right, nums2_left, nums2_right；
/// // 将 nums1_left 与 nums2_left 合并 part_left；nums1_right 与 nums2_right 合并 part_right
/// // 当 part_left == part_right 且 max(part_left)≤min(part_right) 时，中间数即为 (max(left_part)+min(right_part)) / 2
/// pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
///     let mut first = &nums1;
///     let mut second = &nums2;
///
///     if first.len() > second.len() {
///         first = &nums2;
///         second = &nums1
///     }
///
///     let m = first.len();
///     let n = second.len();
///     let mut low = 0;
///     let mut high = m;
///     let half_len = ((n - m + 1) >> 1) + m;
///
///     while low <= high {
///        let mid_first = ((high - low) >> 1) + low;
///        let mid_second = half_len - mid_first;
///
///        let first_left = if mid_first == 0 { std::i32::MIN } else { first[mid_first - 1] };
///        let first_right = if mid_first == m { std::i32::MAX } else { first[mid_first] };
///        let second_left = if mid_second == 0 { std::i32::MIN } else { second[mid_second - 1] };
///        let second_right = if mid_second == n { std::i32::MAX } else { second[mid_second] };
///
///        if first_left <= second_right && second_left <= first_right {
///            if (n + m) & 1 == 0 {
///                // 避免溢出，不使用 (right_min + left_max) as f64 / 2.0 的方式
///                let mut left_max = first_left.max(second_left);
///                let mut right_min = first_right.min(second_right);
///                if left_max > right_min {
///                    let tmp = left_max;
///                    left_max = right_min;
///                    right_min = tmp;
///                }
///                return (right_min - left_max) as f64 / 2.0 + (left_max as f64);
///            } else {
///                return first_left.max(second_left) as f64
///            }
///        } else if first_left > second_right {
///            high = mid_first - 1;
///        } else {
///            low = mid_first + 1;
///        }
///     }
///     0.0
/// }
/// ```
///
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut first = &nums1;
    let mut second = &nums2;

    if first.len() > second.len() {
        first = &nums2;
        second = &nums1
    }

    let m = first.len();
    let n = second.len();
    let mut low = 0;
    let mut high = m;
    let half_len = ((n - m + 1) >> 1) + m;

    while low <= high {
       let mid_first = ((high - low) >> 1) + low;
       let mid_second = half_len - mid_first;

       let first_left = if mid_first == 0 { std::i32::MIN } else { first[mid_first - 1] };
       let first_right = if mid_first == m { std::i32::MAX } else { first[mid_first] };
       let second_left = if mid_second == 0 { std::i32::MIN } else { second[mid_second - 1] };
       let second_right = if mid_second == n { std::i32::MAX } else { second[mid_second] };

       if first_left <= second_right && second_left <= first_right {
           if (n + m) & 1 == 0 {
               // 避免溢出，不使用 (right_min + left_max) as f64 / 2.0 的方式
               let mut left_max = first_left.max(second_left);
               let mut right_min = first_right.min(second_right);
               if left_max > right_min {
                   let tmp = left_max;
                   left_max = right_min;
                   right_min = tmp;
               }
               return (right_min - left_max) as f64 / 2.0 + (left_max as f64);
           } else {
               return first_left.max(second_left) as f64
           }
       } else if first_left > second_right {
           high = mid_first - 1;
       } else {
           low = mid_first + 1;
       }
    }
    0.0
}
