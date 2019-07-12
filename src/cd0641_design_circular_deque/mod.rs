//! Design Circular Deque [leetcode: design_circular_deque](https://leetcode.com/problems/design-circular-deque/)
//!
//! Design your implementation of the circular double-ended queue (deque).
//!
//! Your implementation should support following operations:
//!
//! * `MyCircularDeque(k)`: Constructor, set the size of the deque to be k.
//! * `insertFront()`: Adds an item at the front of Deque. Return true if the operation is successful.
//! * `insertLast()`: Adds an item at the rear of Deque. Return true if the operation is successful.
//! * `deleteFront()`: Deletes an item from the front of Deque. Return true if the operation is successful.
//! * `deleteLast()`: Deletes an item from the rear of Deque. Return true if the operation is successful.
//! * `getFront()`: Gets the front item from the Deque. If the deque is empty, return -1.
//! * `getRear()`: Gets the last item from Deque. If the deque is empty, return -1.
//! * `isEmpty()`: Checks whether Deque is empty or not.
//! * `isFull()`: Checks whether Deque is full or not.
//!
//! ***Example:***
//!
//! ```
//! MyCircularDeque circularDeque = new MycircularDeque(3); // set the size to be 3
//! circularDeque.insertLast(1);			// return true
//! circularDeque.insertLast(2);			// return true
//! circularDeque.insertFront(3);			// return true
//! circularDeque.insertFront(4);			// return false, the queue is full
//! circularDeque.getRear();  			// return 2
//! circularDeque.isFull();				// return true
//! circularDeque.deleteLast();			// return true
//! circularDeque.insertFront(4);			// return true
//! circularDeque.getFront();			// return 4
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
/// * Runtime: 8 ms
/// * Memory: 2.8 MB
///
/// ```rust
/// struct MyCircularDeque {
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
/// impl MyCircularDeque {
///
///     /** Initialize your data structure here. Set the size of the deque to be k. */
///     fn new(k: i32) -> Self {
///         MyCircularDeque {
///             items: vec![None; k as usize],
///             head: 0,
///             tail: -1,
///             capacity: k,
///             size: 0,
///         }
///
///     }
///
///     /** Adds an item at the front of Deque. Return true if the operation is successful. */
///     fn insert_front(&mut self, value: i32) -> bool {
///         if self.is_full() { return false; }
///
///         self.head = self.get_curr_position(self.head);
///         self.items[self.head as usize] = Some(value);
///         self.size += 1;
///
///         if self.size == 1 { self.tail = self.head; }
///
///         true
///     }
///
///     /** Adds an item at the rear of Deque. Return true if the operation is successful. */
///     fn insert_last(&mut self, value: i32) -> bool {
///         if self.is_full() { return false; }
///
///         self.tail = (self.tail + 1) % self.capacity;
///         self.items[self.tail as usize] = Some(value);
///         self.size += 1;
///
///         true
///     }
///
///     /** Deletes an item from the front of Deque. Return true if the operation is successful. */
///     fn delete_front(&mut self) -> bool {
///         if self.is_empty() { return false; }
///
///         self.items[self.head as usize] = None;
///         self.head = (self.head + 1) % self.capacity;
///         self.size -= 1;
///
///         true
///     }
///
///     /** Deletes an item from the rear of Deque. Return true if the operation is successful. */
///     fn delete_last(&mut self) -> bool {
///         if self.is_empty() { return false; }
///
///         self.items[self.tail as usize] = None;
///         self.tail = self.get_curr_position(self.tail);
///         self.size -= 1;
///
///         true
///     }
///
///     /** Get the front item from the deque. */
///     fn get_front(&self) -> i32 {
///         self.items[self.head as usize].unwrap_or(-1)
///     }
///
///     /** Get the last item from the deque. */
///     fn get_rear(&self) -> i32 {
///         self.items[self.tail as usize].unwrap_or(-1)
///     }
///
///     /** Checks whether the circular deque is empty or not. */
///     fn is_empty(&self) -> bool {
///         self.size == 0
///     }
///
///     /** Checks whether the circular deque is full or not. */
///     fn is_full(&self) -> bool {
///         self.size == self.capacity
///     }
///
///     fn get_curr_position(&self, p: i32) -> i32 {
///         if p == 0 { self.capacity - 1 } else { (p - 1) % self.capacity }
///     }
/// }
/// /**
///  * Your MyCircularDeque object will be instantiated and called as such:
///  * let obj = MyCircularDeque::new(k);
///  * let ret_1: bool = obj.insert_front(value);
///  * let ret_2: bool = obj.insert_last(value);
///  * let ret_3: bool = obj.delete_front();
///  * let ret_4: bool = obj.delete_last();
///  * let ret_5: i32 = obj.get_front();
///  * let ret_6: i32 = obj.get_rear();
///  * let ret_7: bool = obj.is_empty();
///  * let ret_8: bool = obj.is_full();
///  */
/// ```
///
#[allow(dead_code)]
pub struct MyCircularDeque {
    items: Vec<Option<i32>>,
    head: i32,
    tail: i32,
    capacity: i32,
    size: i32,
}
