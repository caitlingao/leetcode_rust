//! Number of Recent Calls[leetcode: number_of_recent_calls](https://leetcode.com/problems/number-of-recent-calls/)
//!
//! Write a class `RecentCounter` to count recent requests.
//!
//! It has only one method: `ping(int t)`, where t represents some time in milliseconds.
//!
//! Return the number of `pings` that have been made from 3000 milliseconds ago until now.
//!
//! Any ping with time in `[t - 3000, t]` will count, including the current ping.
//!
//! It is guaranteed that every call to `ping` uses a strictly larger value of `t` than before.
//!
//! **Example1:**
//!
//! ```
//! Input: inputs = ["RecentCounter","ping","ping","ping","ping"], inputs = [[],[1],[100],[3001],[3002]]
//! Output: [null,1,2,3,3]
//! ```
//!
//! **Note**
//!
//! 1. Each test case will have at most `10000` calls to `ping`.
//! 2. Each test case will call `ping` with strictly increasing values of `t`.
//! 3. Each call to ping will have `1 <= t <= 10^9`.

/// # Solutions
///
/// # Approach 1: Queue
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(1)
///
/// * Runtime: 40 ms
/// * Memory: 5.4 MB
///
/// ```rust
/// use std::collections::VecDeque;
///
/// #[derive(Default)]
/// struct RecentCounter {
///     pings: VecDeque<i32>,
/// }
///
/// /**
///  * `&self` means the method takes an immutable reference.
///  * If you need a mutable reference, change it to `&mut self` instead.
///  */
/// impl RecentCounter {
///
///     fn new() -> Self {
///        Default::default()
///     }
///
///     fn ping(&mut self, t: i32) -> i32 {
///        self.pings.push_back(t);
///         while t - *self.pings.front().unwrap() > 3000 {
///             self.pings.pop_front();
///         }
///
///         self.pings.len() as i32
///     }
/// }
///
/// /**
///  * Your RecentCounter object will be instantiated and called as such:
///  * let obj = RecentCounter::new();
///  * let ret_1: i32 = obj.ping(t);
///  */
/// ```
///
#[allow(dead_code)]
pub struct RecentCounter {
}
