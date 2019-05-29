//! Implement Trie (Prefix Tree) [leetcode: implement_trie_prefix_tree](https://leetcode.com/problems/implement-trie-prefix-tree/)
//!
//! Implement a trie with `insert`, `search`, and `startsWith` methods.
//!
//! **Example:**
//!
//! ```
//! Trie trie = new Trie();
//!
//! trie.insert("apple");
//! trie.search("apple");   // returns true
//! trie.search("app");     // returns false
//! trie.startsWith("app"); // returns true
//! trie.insert("app");
//! trie.search("app");     // returns true
//! ```
//!
//! **Note:**
//!
//! You may assume that all inputs are consist of lowercase letters `a-z`.
//! All inputs are guaranteed to be non-empty strings.

/// # Solutions
///
/// # Approach 1: Iterative
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(1)
///
/// ```rust
/// #[derive(Default)]
/// struct Trie {
///     is_end: bool,
///     nodes: [Option<Box<Trie>>; 26],
/// }
///
///
/// /**
///  * `&self` means the method takes an immutable reference.
///  * If you need a mutable reference, change it to `&mut self` instead.
///  */
/// impl Trie {
///
///     /** Initialize your data structure here. */
///     fn new() -> Self {
///         Default::default()
///     }
///
///     /** Inserts a word into the trie. */
///     fn insert(&mut self, word: String) {
///        let mut curr = self;
///         for i in word.chars().map(|char| (char as u8 - 'a' as u8) as usize) {
///             curr = curr.nodes[i].get_or_insert_with(|| Box::new(Trie::new()));
///         }
///         curr.is_end = true;
///     }
///
///     /** Returns if the word is in the trie. */
///     fn search(&self, word: String) -> bool {
///        let mut curr = self;
///         for i in word.chars().map(|char| (char as u8 - 'a' as u8) as usize) {
///             match curr.nodes[i].as_ref() {
///                 Some(node) => { curr = node; },
///                 None => { return false; },
///             }
///         }
///         curr.is_end
///     }
///
///     /** Returns if there is any word in the trie that starts with the given prefix. */
///     fn starts_with(&self, prefix: String) -> bool {
///        let mut curr = self;
///         for i in prefix.chars().map(|char| (char as u8 - 'a' as u8) as usize) {
///             match curr.nodes[i].as_ref() {
///                 Some(node) => { curr = node; },
///                 None => { return false; },
///             }
///         }
///         true
///     }
/// }
///
/// /**
///  * Your Trie object will be instantiated and called as such:
///  * let obj = Trie::new();
///  * obj.insert(word);
///  * let ret_2: bool = obj.search(word);
///  * let ret_3: bool = obj.starts_with(prefix);
///  */
/// ```
///
/// # Approach 2: Iterative
///
/// * Time complexity: O(n)
///
/// * Space complexity: O(1)
///
/// ```rust
/// #[derive(Default)]
/// struct Trie {
///     is_end: bool,
///     nodes: [Option<Box<Trie>>; 26],
/// }
///
///
/// /**
///  * `&self` means the method takes an immutable reference.
///  * If you need a mutable reference, change it to `&mut self` instead.
///  */
/// impl Trie {
///
///     /** Initialize your data structure here. */
///     fn new() -> Self {
///         Default::default()
///     }
///
///
///     /** Returns if the word is in the trie. */
///     fn search(&self, word: String) -> bool {
///         self.find(word).map_or(false, |t| t.is_end)
///     }
///
///     /** Returns if there is any word in the trie that starts with the given prefix. */
///     fn starts_with(&self, prefix: String) -> bool {
///         self.find(prefix).is_some()
///     }
///
///     fn find(&self, word: String) -> Option<&Trie> {
///         let mut curr = self;
///         for i in word.chars().map(|ch| (ch as u8 - 'a' as u8) as usize) {
///             curr = curr.nodes[i].as_ref()?;
///         }
///         Some(curr)
///     }
///
/// }
///
/// /**
///  * Your Trie object will be instantiated and called as such:
///  * let obj = Trie::new();
///  * obj.insert(word);
///  * let ret_2: bool = obj.search(word);
///  * let ret_3: bool = obj.starts_with(prefix);
///  */
/// ```
#[derive(Default)]
pub struct Trie {
    is_end: bool,
    nodes: [Option<Box<Trie>>; 26],
}
impl Trie {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Default::default()
    }

    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
       let mut curr = self;
        for i in word.chars().map(|char| (char as u8 - 'a' as u8) as usize) {
            curr = curr.nodes[i].get_or_insert_with(|| Box::new(Trie::new()));
        }
        curr.is_end = true;
    }

    /** Returns if the word is in the trie. */
    fn search(&self, word: String) -> bool {
       let mut curr = self;
        for i in word.chars().map(|char| (char as u8 - 'a' as u8) as usize) {
            match curr.nodes[i].as_ref() {
                Some(node) => { curr = node; },
                None => { return false; },
            }
        }
        curr.is_end
    }

    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&self, prefix: String) -> bool {
       let mut curr = self;
        for i in prefix.chars().map(|char| (char as u8 - 'a' as u8) as usize) {
            match curr.nodes[i].as_ref() {
                Some(node) => { curr = node; },
                None => { return false; },
            }
        }
        true
    }
}
