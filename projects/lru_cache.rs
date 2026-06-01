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
    // map.get(&key) returns Option<&usize>, not Option<usize>.
    // HashMap::get always returns a reference to the stored value, not a copy.
    // Since usize is Copy, use .copied() to convert Option<&usize> → Option<usize>:
    //   let idx = self.map.get(&key).copied()?;
    // Alternatively: let idx = *self.map.get(&key)?;  (manual deref)
    // .copied() is idiomatic for Copy types.
    free: Vec<usize>, // recycled indices from evicted nodes
}

// impl block groups all behaviour on LruCache in one place.
// Rust has no classes; data (struct) and behaviour (impl) are separate.
// All methods take &mut self because both put and get mutate the list.
impl LruCache {
    fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must be > 0");
        let mut nodes = Vec::with_capacity(capacity + 2);
        // Sentinel HEAD: next points to TAIL initially (empty list)
        nodes.push(Node {
            key: 0,
            value: 0,
            prev: 0,
            next: TAIL,
        });
        // Sentinel TAIL: prev points to HEAD initially
        nodes.push(Node {
            key: 0,
            value: 0,
            prev: HEAD,
            next: 0,
        });
        LruCache {
            capacity,
            nodes,
            map: HashMap::new(),
            free: Vec::new(),
        }
    }

    // put has 3 cases:
    // 1) key exists       → update value, move node to MRU position
    // 2) key is new, full → evict LRU node, reuse its slot, insert as MRU
    // 3) key is new, room → allocate/reuse a free slot, insert as MRU
    fn put(&mut self, key: i32, value: i32) {
        // Existing key: keep the same slot, just refresh its value and recency.
        if let Some(idx) = self.map.get(&key).copied() {
            // FIXED ORDER: unlink before re-inserting at the head, otherwise the
            // old neighbours would still point at idx and the list would be corrupted.
            self.unlink(idx);
            self.nodes[idx].value = value;
            self.insert_after_head(idx);
            return;
        }

        // Full cache: evict the real node just before TAIL (= current LRU).
        if self.map.len() == self.capacity {
            let victim = self.nodes[TAIL].prev;
            // Read the old key BEFORE overwriting the slot, or we would lose the
            // HashMap entry that still points at this node index.
            let old_key = self.nodes[victim].key;
            self.map.remove(&old_key);
            self.unlink(victim);

            // Reuse the evicted node's Vec slot instead of growing the arena.
            self.nodes[victim].key = key;
            self.nodes[victim].value = value;
            self.map.insert(key, victim);
            self.insert_after_head(victim);
            return;
        }

        // Cache has room: use a recycled slot if one exists, otherwise append.
        let idx = if let Some(free_idx) = self.free.pop() {
            self.nodes[free_idx] = Node {
                key,
                value,
                prev: HEAD,
                next: TAIL,
            };
            free_idx
        } else {
            let idx = self.nodes.len();
            self.nodes.push(Node {
                key,
                value,
                prev: HEAD,
                next: TAIL,
            });
            idx
        };

        self.map.insert(key, idx);
        self.insert_after_head(idx);
    }

    // get is a read semantically, but an LRU hit also mutates recency order:
    // the accessed node becomes the new MRU node right after HEAD.
    fn get(&mut self, key: i32) -> Option<i32> {
        let idx = self.map.get(&key).copied()?;
        // Copy the value out before rewiring the list so the later mutable
        // operations stay simple and we can return the cached value at the end.
        let value = self.nodes[idx].value;
        self.unlink(idx);
        self.insert_after_head(idx);
        Some(value)
    }

    // Unlink the node at `idx` from wherever it currently sits in the list.
    // Does NOT remove it from `nodes` or `map` — callers do that.
    fn unlink(&mut self, idx: usize) {
        // Read prev/next BEFORE any writes — insert_after_head overwrites these
        // fields, so reading them after would give wrong neighbours.
        let prev = self.nodes[idx].prev;
        let next = self.nodes[idx].next;
        // Bridge the gap: tell each neighbour to skip over idx.
        // self.nodes[prev].next = next  →  prev now points forward to next
        // self.nodes[next].prev = prev  →  next now points backward to prev
        // idx is now unreachable from the list; its own prev/next are stale but unused.
        self.nodes[prev].next = next;
        self.nodes[next].prev = prev;
    }

    // Insert the node at `idx` immediately after the HEAD sentinel (= MRU position).
    // Before: HEAD <-> old_first <-> ...
    // After:  HEAD <-> idx <-> old_first <-> ...
    fn insert_after_head(&mut self, idx: usize) {
        // Save old_first BEFORE any writes — we need it for two later assignments.
        let old_first = self.nodes[HEAD].next;
        self.nodes[HEAD].next = idx; // HEAD  →(next)→  idx
        self.nodes[idx].prev = HEAD; // HEAD  ←(prev)←  idx
        self.nodes[idx].next = old_first; // idx   →(next)→  old_first
        self.nodes[old_first].prev = idx; // idx   ←(prev)←  old_first
    }
}

fn main() {
    // Not needed for tests — binary entry point placeholder.
}

#[cfg(test)]
mod tests {
    use super::*;

    // Focused behavior checks for the core LRU semantics.
    #[test]
    fn get_on_empty_cache_returns_none() {
        // &mut self on get hence need mut
        let mut cache = LruCache::new(2);
        assert_eq!(cache.get(123), None);
    }

    #[test]
    fn put_and_get_round_trip() {
        let mut cache = LruCache::new(2);

        cache.put(1, 10);

        assert_eq!(cache.get(1), Some(10));
    }

    #[test]
    fn lru_eviction_removes_oldest_untouched_key() {
        let mut cache = LruCache::new(2);

        cache.put(1, 10);
        cache.put(2, 20);
        cache.put(3, 30);

        assert_eq!(cache.get(1), None);
        assert_eq!(cache.get(2), Some(20));
        assert_eq!(cache.get(3), Some(30));
    }

    #[test]
    fn get_promotes_key_so_different_key_is_evicted() {
        let mut cache = LruCache::new(2);

        cache.put(1, 10);
        cache.put(2, 20);
        assert_eq!(cache.get(1), Some(10));
        cache.put(3, 30);

        assert_eq!(cache.get(1), Some(10));
        assert_eq!(cache.get(2), None);
        assert_eq!(cache.get(3), Some(30));
    }

    #[test]
    fn updating_existing_key_does_not_grow_beyond_capacity() {
        let mut cache = LruCache::new(2);

        cache.put(1, 10);
        cache.put(2, 20);
        cache.put(1, 15);
        cache.put(3, 30);

        assert_eq!(cache.get(1), Some(15));
        assert_eq!(cache.get(2), None);
        assert_eq!(cache.get(3), Some(30));
    }
}
