//! Number of 1 Bits [leetcode: number_of_1_bits](https://leetcode.com/problems/number-of-1-bits/)
//!
//! Write a function that takes an unsigned integer and return the number of '1' bits it has (also known as the [Hamming weight](https://en.wikipedia.org/wiki/Hamming_weight)).
//!
//! ***Example1:***
//!
//! ```
//! Input: 00000000000000000000000000001011
//! Output: 3
//! Explanation: The input binary string 00000000000000000000000000001011 has a total of three '1' bits.
//!
//! ```
//!
//! ***Example2:***
//!
//! ```
//! Input: 00000000000000000000000010000000
//! Output: 1
//! Explanation: The input binary string 00000000000000000000000010000000 has a total of one '1' bit.
//!
//! ```
//!
//! ***Example3:***
//!
//! ```
//! Input: 11111111111111111111111111111101
//! Output: 31
//! Explanation: The input binary string 11111111111111111111111111111101 has a total of thirty one '1' bits.
//!
//! ```
//!
//! **Note:**
//! * Note that in some languages such as Java, there is no unsigned integer type. In this case, the input will be given as signed integer type and should not affect your implementation, as the internal binary representation of the integer is the same whether it is signed or unsigned.
//! * In Java, the compiler represents the signed integers using [2's complement notation](https://en.wikipedia.org/wiki/Two%27s_complement). Therefore, in Example 3 above the input represents the signed integer `-3`.
//!
//! **Follow up:**
//! If this function is called many times, how would you optimize it?

/// # Solutions
///
/// # Approach 1: Loop and Flip
///
/// * Time complexity: O(1)
///
/// * Space complexity: O(1)
///
/// * Runtime:
/// * Memory:
///
/// ```rust
/// impl Solution {
///     pub fn hamming_weight(mut n: u32) -> i32 {
///         let mut count = 0;
///         let mut mask = 1;
///         for i in 0..32 {
///             if n & mask != 0 { count += 1; }
///             mask <<= 1
///         }
///         count
///     }
/// }
/// ```
///
/// # Approach 2: Loop and Flip
///
/// * Time complexity: O(1)
///
/// * Space complexity: O(1)
///
/// * Runtime:
/// * Memory:
///
/// ```rust
/// impl Solution {
///     pub fn hamming_weight(mut n: u32) -> i32 {
///         let mut count = 0;
///         for i in 0..32 {
///             if n & 1 == 1 { count += 1; }
///             n >>= 1
///         }
///         count
///     }
/// }
/// ```
///
/// # Approach 3: Bit Manipulation Trick
///
/// * Time complexity: O(1)
///
/// * Space complexity: O(1)
///
/// * Runtime:
/// * Memory:
///
/// ```rust
/// impl Solution {
///     pub fn hamming_weight(mut n: u32) -> i32 {
///         let mut count = 0;
///         while n != 0 {
///             count += 1;
///             n &= (n - 1);
///         }
///         count
///     }
/// }
/// ```
///
pub fn hamming_weight(mut n: u32) -> i32 {
    let mut count = 0;
    while n != 0 {
        count += 1;
        n = n & (n - 1);
    }
    count
}
