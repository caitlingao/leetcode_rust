//! Best Time to Buy and Sell Stock III [leetcode: best_time_to_buy_and_sell_stock_III](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/)
//!
//! Say you have an array for which the ith element is the price of a given stock on day *i*.
//!
//! Design an algorithm to find the maximum profit. You may complete at most *two* transactions.
//!
//! **Note**: You may not engage in multiple transactions at the same time (i.e., you must sell the stock before you buy again).
//!
//! ***Example1:***
//!
//! ```
//! Input: [3,3,5,0,0,3,1,4]
//! Output: 6
//! Explanation: Buy on day 4 (price = 0) and sell on day 6 (price = 3), profit = 3-0 = 3.
//!              Then buy on day 7 (price = 1) and sell on day 8 (price = 4), profit = 4-1 = 3.
//! ```
//!
//! ***Example2:***
//!
//! ```
//! Input: [1,2,3,4,5]
//! Output: 4
//! Explanation: Buy on day 1 (price = 1) and sell on day 5 (price = 5), profit = 5-1 = 4.
//!              Note that you cannot buy on day 1, buy on day 2 and sell them later, as you are
//!              engaging multiple transactions at the same time. You must sell before buying again.
//! ```
//!
//! ***Example3:***
//!
//! ```
//! Input: [7,6,4,3,1]
//! Output: 0
//! Explanation: In this case, no transaction is done, i.e. max profit = 0.
//! ```

/// # Solutions
///
/// # Approach 1: Dynamic Programming
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(n)
///
/// * Runtime: 16 ms
/// * Memory: 5.7 MB
///
/// ```rust
/// use std::cmp;
///
/// impl Solution {
///     pub fn max_profit(prices: Vec<i32>) -> i32 {
///         if prices.len() < 2 { return 0; }
///
///         let mut result = 0;
///         let n = prices.len();
///         let mut profits = vec![vec![vec![0; 2]; 3]; n];
///         profits[0][0][1] = -prices[0];
///         profits[0][1][1] = -prices[0];
///
///         for i in 1..n {
///             profits[i][0][0] = profits[i - 1][0][0];
///             profits[i][0][1] = cmp::max(profits[i - 1][0][1], profits[i - 1][0][0] - prices[i]);
///
///             profits[i][1][0] = cmp::max(profits[i - 1][1][0], profits[i - 1][0][1] + prices[i]);
///             profits[i][1][1] = cmp::max(profits[i - 1][1][1], profits[i - 1][1][0] - prices[i]);
///
///             profits[i][2][0] = cmp::max(profits[i - 1][2][0], profits[i - 1][1][1] + prices[i]);
///         }
///
///         result = cmp::max(profits[n-1][0][0], cmp::max(profits[n-1][1][0], profits[n-1][2][0]));
///         result
///     }
/// }
/// ```
///
/// # Approach 2: Dynamic Programming
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(1)
///
/// * Runtime: 0 ms
/// * Memory: 2.6 MB
///
/// ```rust
/// use std::cmp::max;
///
/// impl Solution {
///     pub fn max_profit(prices: Vec<i32>) -> i32 {
///         if prices.len() < 2 { return 0; }
///
///         let kk = 2;
///         let mut profits = vec![vec![0; prices.len()]; kk+1];
///         let mut max_profit = 0;
///         for k in 1..=kk {
///             let mut tmp_max = profits[k-1][0] - prices[0];
///             for i in 1..prices.len() {
///                 profits[k][i] = max(profits[k][i-1], prices[i] + tmp_max);
///                 tmp_max       = max(tmp_max, profits[k-1][i] - prices[i]);
///                 max_profit    = max(profits[k][i], max_profit);
///             }
///         }
///
///         max_profit
///     }
/// }
/// ```
/// reference:
/// * [A clean DP solution which generalizes to k
/// transactions](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/discuss/39608/a-clean-dp-solution-which-generalizes-to-k-transactions)
/// * [Rust, generic K transactions solution with O(N) time and O(N) space,
/// 中文注释](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/discuss/238936/Rust-generic-K-transactions-solution-with-O(N)-time-and-O(N)-space)
///
pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() < 2 { return 0; }

    let kk = 2;
    let mut profits = vec![vec![0; prices.len()]; kk+1];
    let mut max_profit = 0;
    for k in 1..=kk {
        let mut tmp_max = profits[k-1][0] - prices[0];
        for i in 1..prices.len() {
            profits[k][i] = i32::max(profits[k][i-1], prices[i] + tmp_max);
            tmp_max       = i32::max(tmp_max, profits[k-1][i] - prices[i]);
            max_profit    = i32::max(profits[k][i], max_profit);
        }
    }

    max_profit
}
