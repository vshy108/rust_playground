#![allow(dead_code)]

// Goal: Ownership thinking
//
// API:
//   cache.put(key, value, ttl_secs)   insert or update; evicts LRU entry when at capacity
//   cache.get(key)                    returns Option<i32>; promotes the entry to MRU on hit
//
// Learn:
//
// - Option<T> — null-free way to represent a value that may or may not exist; `.map()` transforms without unwrapping
// - std::time::Instant — monotonic clock snapshot; never jumps back; `Instant::now() + Duration::from_secs(n)` is a deadline
// - HashMap — O(1) average lookup; here maps each cache key → Vec index of its node
// - doubly-linked list — keeps entries in recency order; head = MRU, tail = LRU; splice on every get/put
// - Vec<Node> + usize indices — arena allocator pattern; avoids Rust's two-owner problem for linked structures
// - &mut self on get — get promotes a node to MRU, so reads also mutate; &mut enforces exclusive access
//
// Notes:
// 1. Data model: `Vec<Node>` + `usize` indices avoids Rust's two-owner problem for
//    doubly-linked structures. `HEAD` and `TAIL` are sentinel indices, so insert/remove
//    never need empty-list special cases. `map: HashMap<i32, usize>` gives O(1) average
//    lookup from cache key to node slot.
// 2. Core list operations: `unlink(idx)` bridges a node's neighbours so the node becomes
//    unreachable from the list; `insert_after_head(idx)` rewires four edges to make the
//    node the new MRU entry. Both helpers read the old neighbour indices before writes,
//    because later rewiring would otherwise overwrite the information they still need.
// 3. Core LRU behavior: `put` handles update, evict-and-reuse, and allocate paths.
//    Existing keys are moved to MRU after updating; full-cache inserts evict the node just
//    before `TAIL` (= current LRU); new inserts either reuse a free slot or append.
//    `get` returns `Option<i32>` and promotes hits to MRU, so reads also mutate recency.
// 4. TTL support: `Node` gained `expires_at: Option<Instant>`. `put(..., ttl_secs)` maps
//    `Some(secs)` to `Some(Instant::now() + Duration::from_secs(secs))` and stores `None`
//    when no TTL is provided. `get` treats expired entries as misses and removes them from
//    both the HashMap and the recency list before returning `None`.
// 5. TTL eviction fix: on a full cache, `put` now walks backward from LRU toward MRU to
//    reclaim any expired entry before evicting a still-live one. This makes the full-cache
//    TTL path O(n), but preserves the intended rule that expired entries should not keep
//    consuming capacity while live entries are being evicted.
// 6. Verification: focused tests cover empty reads, round-trip, LRU eviction, promotion,
//    update-without-growth, expired miss cleanup, non-expired reads, and reclaiming expired
//    entries before live eviction. `cargo test --bin lru_cache` is passing.

use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

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
    expires_at: Option<std::time::Instant>,
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
            expires_at: None,
        });
        // Sentinel TAIL: prev points to HEAD initially
        nodes.push(Node {
            key: 0,
            value: 0,
            prev: HEAD,
            next: 0,
            expires_at: None,
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
    fn put(&mut self, key: i32, value: i32, ttl_secs: Option<u64>) {
        let expires_at = ttl_secs.map(|secs| Instant::now() + Duration::from_secs(secs));
        // Existing key: keep the same slot, just refresh its value and recency.
        if let Some(idx) = self.map.get(&key).copied() {
            // FIXED ORDER: unlink before re-inserting at the head, otherwise the
            // old neighbours would still point at idx and the list would be corrupted.
            self.unlink(idx);
            self.nodes[idx].value = value;
            self.nodes[idx].expires_at = expires_at;
            self.insert_after_head(idx);
            return;
        }

        // Reclaim expired entries before deciding the cache is truly full.
        while self.map.len() == self.capacity {
            let mut cursor = self.nodes[TAIL].prev;
            let mut reclaimed = false;
            // Walk backward from LRU toward MRU and stop at the HEAD sentinel.
            // Tradeoff: this makes full-cache put() O(n), but it lets TTL reclaim
            // any expired entry before evicting a still-live one.
            while cursor != HEAD {
                let prev = self.nodes[cursor].prev;
                if let Some(expires_at) = self.nodes[cursor].expires_at
                    && expires_at <= Instant::now() {
                        let old_key = self.nodes[cursor].key;
                        // FIX: checking only TAIL.prev misses expired entries that are
                        // newer than the current LRU, which can wrongly evict a live
                        // entry while a dead one still consumes capacity. Scan the
                        // whole live list and reclaim any expired node before falling
                        // back to normal LRU eviction.
                        self.map.remove(&old_key);
                        self.unlink(cursor);
                        self.free.push(cursor);
                        reclaimed = true;
                        break;
                    }
                cursor = prev;
            }
            if reclaimed {
                continue;
            }
            break;
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
            self.nodes[victim].expires_at = expires_at;
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
                expires_at,
            };
            free_idx
        } else {
            let idx = self.nodes.len();
            self.nodes.push(Node {
                key,
                value,
                prev: HEAD,
                next: TAIL,
                expires_at,
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

        if let Some(expires_at) = self.nodes[idx].expires_at
            && expires_at <= Instant::now() {
                // FIX: treating an expired entry as a plain miss by only returning
                // None would leave stale state behind in both the HashMap and the
                // recency list, so the expired entry would still consume capacity.
                // Remove it from both structures and recycle its slot before
                // reporting the miss.
                self.map.remove(&key);
                self.unlink(idx);
                self.free.push(idx);
                return None;
            }

        // Read after the expiry check so we only copy a live value.
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

        cache.put(1, 10, None);

        assert_eq!(cache.get(1), Some(10));
    }

    #[test]
    fn lru_eviction_removes_oldest_untouched_key() {
        let mut cache = LruCache::new(2);

        cache.put(1, 10, None);
        cache.put(2, 20, None);
        cache.put(3, 30, None);

        assert_eq!(cache.get(1), None);
        assert_eq!(cache.get(2), Some(20));
        assert_eq!(cache.get(3), Some(30));
    }

    #[test]
    fn get_promotes_key_so_different_key_is_evicted() {
        let mut cache = LruCache::new(2);

        cache.put(1, 10, None);
        cache.put(2, 20, None);
        assert_eq!(cache.get(1), Some(10));
        cache.put(3, 30, None);

        assert_eq!(cache.get(1), Some(10));
        assert_eq!(cache.get(2), None);
        assert_eq!(cache.get(3), Some(30));
    }

    #[test]
    fn updating_existing_key_does_not_grow_beyond_capacity() {
        let mut cache = LruCache::new(2);

        cache.put(1, 10, None);
        cache.put(2, 20, None);
        cache.put(1, 15, None);
        cache.put(3, 30, None);

        assert_eq!(cache.get(1), Some(15));
        assert_eq!(cache.get(2), None);
        assert_eq!(cache.get(3), Some(30));
    }

    #[test]
    fn expired_key_returns_none_and_frees_capacity() {
        let mut cache = LruCache::new(2);

        cache.put(1, 10, Some(0));
        assert_eq!(cache.get(1), None);

        cache.put(2, 20, None);
        cache.put(3, 30, None);

        assert_eq!(cache.get(2), Some(20));
        assert_eq!(cache.get(3), Some(30));
    }

    #[test]
    fn non_expired_key_still_returns_value() {
        let mut cache = LruCache::new(2);

        cache.put(1, 10, Some(60));

        assert_eq!(cache.get(1), Some(10));
    }

    #[test]
    fn put_reclaims_expired_entry_before_evicting_live_one() {
        let mut cache = LruCache::new(2);

        cache.put(1, 10, Some(0));
        cache.put(2, 20, None);
        cache.put(3, 30, None);

        assert_eq!(cache.get(1), None);
        assert_eq!(cache.get(2), Some(20));
        assert_eq!(cache.get(3), Some(30));
    }

    #[test]
    fn put_reclaims_expired_non_lru_entry_before_evicting_live_one() {
        let mut cache = LruCache::new(2);

        cache.put(1, 10, None);
        cache.put(2, 20, Some(0));
        assert_eq!(cache.get(1), Some(10));
        cache.put(3, 30, None);

        assert_eq!(cache.get(1), Some(10));
        assert_eq!(cache.get(2), None);
        assert_eq!(cache.get(3), Some(30));
    }
}
