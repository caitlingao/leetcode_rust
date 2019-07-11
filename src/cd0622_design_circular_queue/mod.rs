//! Design Circular Queue [leetcode: design_circular](https://leetcode.com/problems/design-circular-queue/)
//!
//! Design your implementation of the circular queue. The circular queue is a linear data structure in which the operations are performed based on FIFO (First In First Out) principle and the last position is connected back to the first position to make a circle. It is also called "Ring Buffer".
//!
//! One of the benefits of the circular queue is that we can make use of the spaces in front of the queue. In a normal queue, once the queue becomes full, we cannot insert the next element even if there is a space in front of the queue. But using the circular queue, we can use the space to store new values.
//!
//! Your implementation should support following operations:
//!
//! * `MyCircularQueue(k)`: Constructor, set the size of the queue to be k.
//! * `Front`: Get the front item from the queue. If the queue is empty, return -1.
//! * `Rear`: Get the last item from the queue. If the queue is empty, return -1.
//! * `enQueue(value)`: Insert an element into the circular queue. Return true if the operation is successful.
//! * `deQueue()`: Delete an element from the circular queue. Return true if the operation is successful.
//! * `isEmpty()`: Checks whether the circular queue is empty or not.
//! * `isFull()`: Checks whether the circular queue is full or not.
//!
//! ***Example:***
//!
//! ```
//! MyCircularQueue circularQueue = new MyCircularQueue(3); // set the size to be 3
//! circularQueue.enQueue(1);  // return true
//! circularQueue.enQueue(2);  // return true
//! circularQueue.enQueue(3);  // return true
//! circularQueue.enQueue(4);  // return false, the queue is full
//! circularQueue.Rear();  // return 3
//! circularQueue.isFull();  // return true
//! circularQueue.deQueue();  // return true
//! circularQueue.enQueue(4);  // return true
//! circularQueue.Rear();  // return 4
//! ```
//!
//! **Note:**
//!
//! All values will be in the range of [0, 1000].
//! The number of operations will be in the range of [1, 1000].
//! Please do not use the built-in Queue library.

/// # Solutions
///
/// # Approach 1:
///
/// * Time complexity: O(1)
///
/// * Space complexity: O(n)
///
/// * Runtime: 4 ms
/// * Memory: 2.8 MB
///
/// ```rust
/// struct MyCircularQueue {
///     items: Vec<Option<i32>>,
///     head: i32,
///     tail: i32,
///     capacity: i32,
///     size: i32,
/// }
///
///
/// /**
///  * `&self` means the method takes an immutable reference.
///  * If you need a mutable reference, change it to `&mut self` instead.
///  */
/// impl MyCircularQueue {
///
///     /** Initialize your data structure here. Set the size of the queue to be k. */
///     fn new(k: i32) -> Self {
///         MyCircularQueue {
///             items: vec![None; k as usize],
///             head: 0,
///             tail: 0,
///             capacity: k,
///             size: 0,
///         }
///     }
///
///     /** Insert an element into the circular queue. Return true if the operation is successful. */
///     fn en_queue(&mut self, value: i32) -> bool {
///         if self.is_full() { return false; }
///
///         self.items[self.tail as usize] = Some(value);
///         self.tail = (self.tail + 1) % self.capacity;
///         self.size += 1;
///
///         true
///     }
///
///     /** Delete an element from the circular queue. Return true if the operation is successful. */
///     fn de_queue(&mut self) -> bool {
///         if self.is_empty() { return false; }
///
///         self.items[self.head as usize] = None;
///         self.head = (self.head + 1) % self.capacity;
///         self.size -= 1;
///
///         true
///     }
///
///     /** Get the front item from the queue. */
///     fn front(&self) -> i32 {
///         self.items[self.head as usize].unwrap_or(-1)
///     }
///
///     /** Get the last item from the queue. */
///     fn rear(&self) -> i32 {
///         let tmp_tail = if self.tail == 0 { self.capacity - 1 } else { self.tail - 1 };
///         self.items[tmp_tail as usize].unwrap_or(-1)
///     }
///
///     /** Checks whether the circular queue is empty or not. */
///     fn is_empty(&self) -> bool {
///         self.size == 0
///     }
///
///     /** Checks whether the circular queue is full or not. */
///     fn is_full(&self) -> bool {
///         self.size == self.capacity
///     }
/// }
///
/// /**
///  * Your MyCircularQueue object will be instantiated and called as such:
///  * let obj = MyCircularQueue::new(k);
///  * let ret_1: bool = obj.en_queue(value);
///  * let ret_2: bool = obj.de_queue();
///  * let ret_3: i32 = obj.front();
///  * let ret_4: i32 = obj.rear();
///  * let ret_5: bool = obj.is_empty();
///  * let ret_6: bool = obj.is_full();
///  */
/// ```
///
#[allow(dead_code)]
pub struct MyCircularQueue {
    items: Vec<Option<i32>>,
    head: i32,
    tail: i32,
    capacity: i32,
    size: i32,
}
