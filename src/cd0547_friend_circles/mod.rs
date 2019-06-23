//! Friend Circles [leetcode: friend_circles](https://leetcode.com/problems/friend-circles/)
//!
//! There are **N** students in a class. Some of them are friends, while some are not. Their friendship is transitive in nature. For example, if A is a **direct** friend of B, and B is a **direct** friend of C, then A is an **indirect** friend of C. And we defined a friend circle is a group of students who are direct or indirect friends.
//!
//! Given a N*N matrix **M** representing the friend relationship between students in the class. If M[i][j] = 1, then the ith and jth students are **direct** friends with each other, otherwise not. And you have to output the total number of friend circles among all the students.
//!
//! ***Example1:***
//!
//! ```
//! Input:
//! [[1,1,0],
//!  [1,1,0],
//!  [0,0,1]]
//! Output: 2
//! Explanation:The 0th and 1st students are direct friends, so they are in a friend circle.
//! The 2nd student himself is in a friend circle. So return 2.
//! ```
//!
//! ***Example2:***
//!
//! ```
//! Input:
//! [[1,1,0],
//!  [1,1,1],
//!  [0,1,1]]
//! Output: 1
//! Explanation:The 0th and 1st students are direct friends, the 1st and 2nd students are direct friends,
//! so the 0th and 2nd students are indirect friends. All of them are in the same friend circle, so return 1.
//! ```
//!
//! **Note:**
//!
//! 1. N is in range [1,200].
//! 2. M[i][i] = 1 for all students.
//! 3. If M[i][j] = 1, then M[j][i] = 1.


/// # Solutions
///
/// # Approach 1: Union Find
///
/// * Time complexity: O(m*n)
///
/// * Space complexity: O(m*n)
///
/// * Runtime: 0 ms
/// * Memory: 2.7 MB
///
/// ```rust
/// struct UnionFind {
///     parents: Vec<i32>,
///     count: i32,
/// }
///
/// impl UnionFind {
///     fn new(grid: &Vec<Vec<i32>>) -> Self {
///         let m = grid.len();
///         let count = m as i32;
///         let mut parents = vec![];
///         for i in 0..m { parents.push(i as i32); }
///
///         UnionFind { parents: parents, count: count }
///     }
///
///     fn find(&mut self, i: i32) -> i32 {
///         if self.parents[i as usize] != i { self.parents[i as usize] = self.find(self.parents[i as usize]) };
///         self.parents[i as usize]
///     }
///
///     fn union(&mut self, x: i32, y: i32) -> i32 {
///         let x_parent = self.find(x);
///         let y_parent = self.find(y);
///
///         if x_parent == y_parent { return self.count; }
///
///         self.parents[x_parent as usize] = y_parent;
///         self.count -= 1;
///
///         self.count
///     }
/// }
///
/// impl Solution {
///     pub fn find_circle_num(m: Vec<Vec<i32>>) -> i32 {
///         if m.is_empty() { return 0; }
///         let n = m.len();
///         let mut uf = UnionFind::new(&m);
///
///         for i in 0..n {
///             for j in 0..n {
///                 if m[i][j] == 1 { uf.union(i as i32, j as i32); }
///             }
///         }
///         uf.count
///     }
/// }
/// ```
///

pub fn find_circle_num(m: Vec<Vec<i32>>) -> i32 {
    if m.is_empty() { return 0; }
    let n = m.len();
    let mut uf = UnionFind::new(&m);

    for i in 0..n {
        for j in 0..n {
            if m[i][j] == 1 { uf.union(i as i32, j as i32); }
        }
    }
    uf.count
}

#[derive(Debug, Default)]
struct UnionFind {
    parents: Vec<i32>,
    count: i32,
}

impl UnionFind {
    fn new(grid: &Vec<Vec<i32>>) -> Self {
        let m = grid.len();
        let count = m as i32;
        let mut parents = vec![];
        for i in 0..m { parents.push(i as i32); }

        UnionFind { parents: parents, count: count }
    }

    fn find(&mut self, i: i32) -> i32 {
        if self.parents[i as usize] != i { self.parents[i as usize] = self.find(self.parents[i as usize]) };
        self.parents[i as usize]
    }

    fn union(&mut self, x: i32, y: i32) -> i32 {
        let x_parent = self.find(x);
        let y_parent = self.find(y);

        if x_parent == y_parent { return self.count; }

        self.parents[x_parent as usize] = y_parent;
        self.count -= 1;

        self.count
    }
}
