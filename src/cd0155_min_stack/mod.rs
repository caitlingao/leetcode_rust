//! Min Stack [leetcode: min_stack](https://leetcode.com/problems/min-stack/)
//!
//! Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.
//!
//! * push(x) -- Push element x onto stack.
//! * pop() -- Removes the element on top of the stack.
//! * top() -- Get the top element.
//! * getMin() -- Retrieve the minimum element in the stack.
//!
//! ***Example1:***
//!
//! ```
//! MinStack minStack = new MinStack();
//! minStack.push(-2);
//! minStack.push(0);
//! minStack.push(-3);
//! minStack.getMin();   --> Returns -3.
//! minStack.pop();
//! minStack.top();      --> Returns 0.
//! minStack.getMin();   --> Returns -2.
//!
//! ```

/// # Solutions
///
/// # Approach 1:
///
/// * Time complexity: O(1)
///
/// * Space complexity: O(n)
///
/// * Runtime: 0 ms
/// * Memory: 5.2 MB
///
/// ```rust
/// struct MinStack {
///     stack: Vec<i32>,
///     min: i32,
/// }
///
/// /**
///  * `&self` means the method takes an immutable reference.
///  * If you need a mutable reference, change it to `&mut self` instead.
///  */
/// impl MinStack {
///
///     /** initialize your data structure here. */
///     fn new() -> Self {
///         MinStack {
///             stack: vec![],
///             min: i32::max_value(),
///         }
///     }
///
///     fn push(&mut self, x: i32) {
///         if self.min >= x {
///             self.stack.push(self.min);
///             self.min = x;
///         }
///         self.stack.push(x);
///     }
///
///     fn pop(&mut self) {
///         if let Some(i) = self.stack.pop() {
///             if i == self.min { self.min = self.stack.pop().unwrap(); }
///         }
///     }
///
///     fn top(&self) -> i32 {
///         *self.stack.last().unwrap()
///     }
///
///     fn get_min(&self) -> i32 {
///         self.min
///     }
/// }
///
/// /**
///  * Your MinStack object will be instantiated and called as such:
///  * let obj = MinStack::new();
///  * obj.push(x);
///  * obj.pop();
///  * let ret_3: i32 = obj.top();
///  * let ret_4: i32 = obj.get_min();
///  */
/// ```
///
#[allow(dead_code)]
pub struct MinStack {
    stack: Vec<i32>,
    min: i32,
}
