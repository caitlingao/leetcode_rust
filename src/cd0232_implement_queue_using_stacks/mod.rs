//! Implement Queue using Stacks [leetcode: implement_queue_using_stacks](https://leetcode.com/problems/implement-queue-using-stacks/)
//!
//! Implement the following operations of a queue using stacks.
//! * push(x) -- Push element x to the back of queue.
//! * pop() -- Removes the element from in front of queue.
//! * peek() -- Get the front element.
//! * empty() -- Return whether the queue is empty.
//!
//! ***Example:***
//!
//! ```
//! MyQueue queue = new MyQueue();
//!
//! queue.push(1);
//! queue.push(2);
//! queue.peek();  // returns 1
//! queue.pop();   // returns 1
//! queue.empty(); // returns false
//! ```
//!
//! ***Notes***
//!
//! * You must use only standard operations of a stack -- which means only push to top, peek/pop from top, size, and is empty operations are valid.
//! * Depending on your language, stack may not be supported natively. You may simulate a stack by using a list or deque (double-ended queue), as long as you use only standard operations of a stack.
//! * You may assume that all operations are valid (for example, no pop or peek operations will be called on an empty queue).

/// # Solutions
///
/// # Approach 1: Linkedlist
///
/// * Time complexity: O(1)
///
/// * Space complexity: O(1)
///
/// ```rust
/// use std::collections::LinkedList;
///
/// struct MyQueue {
///     queue: LinkedList<i32>,
/// }
///
///
/// /**
///  * `&self` means the method takes an immutable reference.
///  * If you need a mutable reference, change it to `&mut self` instead.
///  */
/// impl MyQueue {
///
///     /** Initialize your data structure here. */
///     fn new() -> Self {
///         return MyQueue { queue: LinkedList::new(), };
///
///     }
///
///     /** Push element x to the back of queue. */
///     fn push(&mut self, x: i32) {
///         self.queue.push_back(x);
///
///     }
///
///     /** Removes the element from in front of queue and returns that element. */
///     fn pop(&mut self) -> i32 {
///         return self.queue.pop_front().unwrap();
///     }
///
///     /** Get the front element. */
///     fn peek(&mut self) -> i32 {
///         let n = self.queue.pop_front().unwrap();
///         self.queue.push_front(n);
///         return n;
///     }
///
///     /** Returns whether the queue is empty. */
///     fn empty(&self) -> bool {
///         return self.queue.is_empty();
///     }
/// }
/// /**
///  * Your MyQueue object will be instantiated and called as such:
///  * let obj = MyQueue::new();
///  * obj.push(x);
///  * let ret_2: i32 = obj.pop();
///  * let ret_3: i32 = obj.peek();
///  * let ret_4: bool = obj.empty();
///  */
/// ```
pub struct MyQueue;
