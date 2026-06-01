# TODO: lru_cache

## Usage

```bash
cargo test --bin lru_cache
```

## 1. Data model

- [x] Define a `Node` struct with `key: i32`, `value: i32`, and `prev`/`next` pointers.
      - `key` is needed so that when evicting the tail, the HashMap entry can be removed without a search.
      - `prev`/`next` make it a doubly-linked list node: any node can splice itself out in O(1)
        by rewiring its two neighbours, without walking from the head.
  - `Node` is the final implemented name; `CacheEntry` was only earlier planning vocabulary.
- [x] Define an `LruCache` struct with `capacity: usize`, a `HashMap` for O(1) lookup,
      and a doubly-linked list (or ordered structure) for O(1) LRU eviction.
- [x] Decide how to represent the linked list in safe Rust.
      The core problem: `Box<T>` means unique ownership. A doubly-linked node has
      two neighbours pointing at it (`prev` from one side, `next` from the other),
      so two `Box` values would claim ownership of the same node — Rust rejects that.
      Three ways to sidestep it:
      1. **`Vec<Node>` + `usize` indices** — the `Vec` owns all nodes; `prev`/`next`
         are just integers (indices into the Vec), not ownership claims. No two-owner
         problem because integers don't own anything.
         + Pure safe Rust; all nodes in one contiguous allocation (cache-friendly); easiest to debug.
         - Evicted slots become holes — need a free-list to reuse them.
         - Stale indices are not caught at compile time (no dangling-pointer safety).
         ★ Best choice for this learning project.
         ★ Also the production-practical pattern — known as an "arena allocator":
           one big slab of memory, integer handles instead of pointers. Used in
           database buffer pools, HTTP caches, and the `lru` crate itself.
      2. **`Rc<RefCell<Node>>`** — `Rc` gives shared ownership (reference-counted);
         multiple `Rc` clones can point to the same node without violating the single-
         owner rule. `RefCell` defers the borrow check to runtime so you can mutate
         through a shared reference.
         + Closest to a "real" pointer-based list; good `Rc`/`RefCell` practice.
         - `prev` back-pointers create reference cycles — nodes never free unless you
           use `Weak` for one direction.
         - Verbose (`.borrow_mut()` everywhere); runtime panics instead of compile errors.
      3. **`std::collections::LinkedList`** — std's built-in doubly-linked list, written
         with `unsafe` internally so you don't have to. Exposes a safe API, but gives
         you less control (e.g. hard to move an arbitrary node to the front efficiently).
         + Zero boilerplate; correct and well-tested.
         - Cursor API for splicing arbitrary nodes is nightly-only — can't efficiently
           move a node to the front on stable Rust. Practically unusable for LRU.

Acceptance check: structs compile, fields are private, `new(capacity)` returns an empty cache.

## 2. `put(key, value)`

- [x] If the key already exists, update the value and move it to the front (most-recently-used).
- [x] If the key is new and the cache is at capacity, evict the least-recently-used entry first.
- [x] Insert the new key at the front.

Acceptance check:

Implementation is in place; final verification still depends on the tests in section 4.

```
put(1, 10), put(2, 20), put(3, 30) with capacity 2:
after put(3): key 1 must have been evicted.
```

## 3. `get(key) -> Option<i32>`

- [x] Look up the key in the HashMap.
- [x] If found, move the entry to the front (counts as a use) and return `Some(value)`.
- [x] If not found, return `None`.

Acceptance check:

Implementation is in place; final verification still depends on the tests in section 4.

```
put(1, 10), put(2, 20) — capacity 2.
get(1)  → Some(10), and key 1 is now MRU.
put(3, 30) → key 2 (LRU) is evicted, not key 1.
get(2)  → None.
```

## 4. Tests

- [x] `get` on empty cache returns `None`.
- [x] `put` + `get` round-trip returns the right value.
- [x] LRU eviction: put past capacity, confirm oldest untouched key is gone.
- [x] `get` promotes a key: access it, then overflow, confirm a different key is evicted.
- [x] Updating an existing key does not grow the cache beyond capacity.

## Extra: TTL

- [ ] Add `expires_at: Option<std::time::Instant>` to `Node`.
- [ ] `put` accepts an optional `ttl_secs: Option<u64>`.
- [ ] `get` treats an expired entry as a cache miss (return `None`, evict the entry).
- [ ] Test: expired key returns `None`; non-expired key still returns the value.
