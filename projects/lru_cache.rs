// Goal: Ownership thinking
//
// API:
//   cache.put(key, value)   insert or update; evicts LRU entry when at capacity
//   cache.get(key)          returns Option<i32>; promotes the entry to MRU on hit
//
// Learn:
//
// - HashMap
//   - HashMap<K, V> maps keys to values with O(1) average lookup.
//   - Here the HashMap maps each key → the Vec index of its node, for O(1) lookup
//     followed by O(1) list splice.
//
// - Linked structures
//   - A doubly-linked list keeps entries in recency order: head = MRU, tail = LRU.
//   - On every get/put, the touched node is unlinked and re-inserted at the head.
//   - On eviction, the tail node is removed and its key used to clean the HashMap.
//
// - Mutability
//   - Both put and get mutate the list (get promotes a node).
//   - &mut self on both methods makes the borrow checker enforce exclusive access.
//
// Data layout — Vec<Node> + usize indices ("arena allocator" pattern):
//   All nodes live in a Vec owned by LruCache.
//   prev/next are usize indices into that Vec, not pointers.
//   Two nodes "pointing" to the same node just hold the same integer — no ownership conflict.
//   Sentinel head/tail nodes (indices 0 and 1) simplify edge cases: the real list
//   lives between them, so insert/remove never need to special-case empty lists.
//
//   Memory layout:
//     nodes[0]  = sentinel HEAD  (key/value unused)
//     nodes[1]  = sentinel TAIL  (key/value unused)
//     nodes[2+] = real entries
//
//   List invariant:  HEAD <-> [MRU] <-> ... <-> [LRU] <-> TAIL

use std::collections::HashMap;

// HEAD and TAIL are the fixed Vec indices of the two sentinel nodes.
// They are constants so that `self.nodes[HEAD]` reads clearly instead of
// `self.nodes[0]`, and the compiler inlines the value — no runtime cost.
// Using indices into the Vec means sentinels are just normal nodes;
// no special pointer type or Option wrapping is needed.
const HEAD: usize = 0;
const TAIL: usize = 1;

// Node is one slot in the arena (the Vec).
// key   — stored here so that when the tail node is evicted, we know which
//          HashMap entry to remove without scanning the whole map.
// value — the cached data.
// prev/next — indices of the neighbouring nodes, not pointers.
//   Storing integers instead of Box<Node> sidesteps the two-owner problem:
//   integers carry no ownership, so multiple nodes can "point" to the same
//   slot by holding the same usize.
struct Node {
    key: i32,
    value: i32,
    prev: usize, // index of the node closer to HEAD
    next: usize, // index of the node closer to TAIL
}

// LruCache is the single owner of all data.
// capacity — max number of real entries (sentinels don't count).
// nodes    — the arena: all nodes (sentinels + real) live here.
//            The Vec is the one true owner; prev/next are just indices into it.
// map      — HashMap<key, index> for O(1) lookup: given a key, find its
//            node in the Vec instantly without walking the list.
// free     — when a node is evicted its Vec slot becomes a hole; the index
//            is pushed here so put() can reuse it instead of growing the Vec.
//            Without this, the Vec would grow forever even though capacity is bounded.
struct LruCache {
    capacity: usize,
    nodes: Vec<Node>,
    map: HashMap<i32, usize>, // key → node index in `nodes`
    free: Vec<usize>,         // recycled indices from evicted nodes
}

// impl block groups all behaviour on LruCache in one place.
// Rust has no classes; data (struct) and behaviour (impl) are separate.
// All methods take &mut self because both put and get mutate the list.
impl LruCache {
    fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must be > 0");
        let mut nodes = Vec::with_capacity(capacity + 2);
        // Sentinel HEAD: next points to TAIL initially (empty list)
        nodes.push(Node { key: 0, value: 0, prev: 0, next: TAIL });
        // Sentinel TAIL: prev points to HEAD initially
        nodes.push(Node { key: 0, value: 0, prev: HEAD, next: 0 });
        LruCache {
            capacity,
            nodes,
            map: HashMap::new(),
            free: Vec::new(),
        }
    }

    // TODO slice 2: implement put
    fn put(&mut self, _key: i32, _value: i32) {
        todo!()
    }

    // TODO slice 3: implement get
    fn get(&mut self, _key: i32) -> Option<i32> {
        todo!()
    }

    // Unlink the node at `idx` from wherever it currently sits in the list.
    // Does NOT remove it from `nodes` or `map` — callers do that.
    fn unlink(&mut self, _idx: usize) {
        todo!()
    }

    // Insert the node at `idx` immediately after the HEAD sentinel (= MRU position).
    fn insert_after_head(&mut self, _idx: usize) {
        todo!()
    }
}

fn main() {
    // Not needed for tests — binary entry point placeholder.
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO slice 4: write tests
    #[test]
    fn get_on_empty_cache_returns_none() {
        todo!()
    }

    #[test]
    fn put_and_get_round_trip() {
        todo!()
    }

    #[test]
    fn lru_eviction_removes_oldest_untouched_key() {
        todo!()
    }

    #[test]
    fn get_promotes_key_so_different_key_is_evicted() {
        todo!()
    }

    #[test]
    fn updating_existing_key_does_not_grow_beyond_capacity() {
        todo!()
    }
}
