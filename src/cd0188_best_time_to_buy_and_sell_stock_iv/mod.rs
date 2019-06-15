//! Best Time to Buy and Sell Stock IV [leetcode: best_time_to_buy_and_sell_stock_IV](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iv/)
//!
//! Say you have an array for which the ith element is the price of a given stock on day *i*.
//!
//! Design an algorithm to find the maximum profit. You may complete at most *k* transactions.
//!
//! **Note**: You may not engage in multiple transactions at the same time (i.e., you must sell the stock before you buy again).
//!
//! ***Example1:***
//!
//! ```
//! Input: [2,4,1], k = 2
//! Output: 2
//! Explanation: Buy on day 1 (price = 2) and sell on day 2 (price = 4), profit = 4-2 = 2.
//! ```
//!
//! ***Example2:***
//!
//! ```
//! Input: [3,2,6,5,0,3], k = 2
//! Output: 7
//! Explanation: Buy on day 2 (price = 2) and sell on day 3 (price = 6), profit = 6-2 = 4.
//!              Then buy on day 5 (price = 0) and sell on day 6 (price = 3), profit = 3-0 = 3.
//! ```

use std::cmp::max;
/// # Solutions
///
/// # Approach 1: Dynamic Programming
///
/// * Time complexity: O(n*k)
///
/// * Space complexity: O(n)
///
/// * Runtime: 240 ms
/// * Memory: 223.8 MB
///
/// ```rust
/// use std::cmp;
///
/// impl Solution {
///     pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
///         if prices.len() < 2 { return 0; }
///
///         let k = cmp::min(k as usize, prices.len() / 2 + 1);
///         let mut profits = vec![vec![0; prices.len()]; (k+1)];
///         let mut max_profit = 0;
///         for kk in 1..=k {
///             let mut tmp_max = profits[kk-1][0] - prices[0];
///             for i in 1..prices.len() {
///                 profits[kk][i] = cmp::max(profits[kk][i-1], prices[i] + tmp_max);
///                 tmp_max       = cmp::max(tmp_max, profits[kk-1][i] - prices[i]);
///                 max_profit    = cmp::max(profits[kk][i], max_profit);
///             }
///         }
///
///         max_profit
///
///     }
/// }
/// ```
///
/// # Approach 2: Dynamic Programming
///
/// * Time complexity: O(n*k)
///
/// * Space complexity: O(n)
///
/// * Runtime: 0 ms
/// * Memory: 2.9 MB
///
/// ```rust
/// use std::cmp::max;
///
/// impl Solution {
///     pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
///         if prices.len() < 2 { return 0; }
///
///         let k = k as usize;
///         let mut max_profit = 0;
///         if k >= prices.len() / 2 {
///             for i in 1..prices.len() {
///                 max_profit += max(0, prices[i] - prices[i - 1]);
///             }
///
///             return max_profit;
///         }
///
///         let mut profits = vec![vec![0; prices.len()]; k+1];
///         for kk in 1..=k {
///             let mut tmp_max = profits[kk-1][0] - prices[0];
///             for i in 1..prices.len() {
///                 profits[kk][i] = max(profits[kk][i-1], prices[i] + tmp_max);
///                 tmp_max       = max(tmp_max, profits[kk-1][i] - prices[i]);
///                 max_profit    = max(profits[kk][i], max_profit);
///             }
///         }
///
///         max_profit
///
///     }
/// }
/// ```
///
pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
    if prices.len() < 2 { return 0; }

    let k = k as usize;
    let mut max_profit = 0;
    // if k >= prices.len() / 2 as many transactions
    if k >= prices.len() / 2 {
        for i in 1..prices.len() {
            max_profit += max(0, prices[i] - prices[i - 1]);
        }

        return max_profit;
    }

    let mut profits = vec![vec![0; prices.len()]; k+1];
    for kk in 1..=k {
        let mut tmp_max = profits[kk-1][0] - prices[0];
        for i in 1..prices.len() {
            profits[kk][i] = max(profits[kk][i-1], prices[i] + tmp_max);
            tmp_max       = max(tmp_max, profits[kk-1][i] - prices[i]);
            max_profit    = max(profits[kk][i], max_profit);
        }
    }

    max_profit

}
