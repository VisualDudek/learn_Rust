//! ============================================================================
//! HashMap Fundamentals — A Progressive Exercise Set (SOLUTIONS)
//! ============================================================================
//!
//! This is the solution file for the companion exercise file. Layout, comments,
//! and tests are identical — only the `todo!()` bodies have been replaced with
//! idiomatic implementations. Read the explanatory comments even if you already
//! got a test passing; they call out the idiom you should walk away with.

use std::collections::HashMap;

// ============================================================================
// Exercise 1: Creating a HashMap and Inserting a Single Entry
// ============================================================================
//
// CONCEPT: `HashMap<K, V>` is Rust's key-value store, roughly analogous to a
// Python dict or a JS object used as a map. Unlike `Vec`, it has no `[]` literal
// syntax — you must construct it explicitly, most commonly with `HashMap::new()`.
//
// WHY IT MATTERS: HashMaps are how you model "lookup by identity" relationships:
// usernames to user IDs, config keys to values, word to frequency, etc. Learning
// to construct and populate one is the entry point to almost every other HashMap
// operation.
//
// MENTAL MODEL: Think of `HashMap::new()` as creating an empty, unordered box of
// slots. `.insert(key, value)` either fills an empty slot (if the key is new) or
// overwrites the value already there (if the key exists) — insertion is *always*
// safe to call, never a "does this exist yet?" question you need to answer first.
//
// TAKEAWAY: `HashMap::new()` + `.insert(k, v)` is the "hello world" of building
// a map — insertion never panics and silently overwrites duplicate keys.
pub fn ex1_create_and_insert() -> HashMap<String, i32> {
    let mut map = HashMap::new();
    map.insert("apples".to_string(), 5);
    map
}

// ============================================================================
// Exercise 2: Inserting Multiple Entries from Separate Values
// ============================================================================
//
// CONCEPT: Real programs usually populate a map from several pieces of data at
// once, not just one. This exercise reinforces that `.insert()` is called once
// per key-value pair — there's no special "bulk insert" syntax you need yet.
//
// WHY IT MATTERS: You'll constantly build maps from function parameters, loop
// iterations, or parsed input. Being fluent with repeated `.insert()` calls (or
// eventually iterator-based construction) is foundational before you reach for
// shortcuts like `HashMap::from([...])` or `.collect()`.
//
// MENTAL MODEL: Treat the map as a running total you build up one fact at a
// time. Order of insertion does NOT determine order of iteration later — a
// HashMap makes no promises about the order its entries come back in.
//
// TAKEAWAY: Building a map from scratch is just insert, insert, insert — and
// insertion order is not preserved or meaningful for a `HashMap`.
pub fn ex2_insert_three_fruits(
    fruit_a: &str,
    count_a: i32,
    fruit_b: &str,
    count_b: i32,
    fruit_c: &str,
    count_c: i32,
) -> HashMap<String, i32> {
    let mut map = HashMap::new();
    map.insert(fruit_a.to_string(), count_a);
    map.insert(fruit_b.to_string(), count_b);
    map.insert(fruit_c.to_string(), count_c);
    map
}

// ============================================================================
// Exercise 3: Reading a Value Safely with `.get()`
// ============================================================================
//
// CONCEPT: `.get(&key)` returns `Option<&V>`, NOT the value itself and NOT a
// panic if the key is missing. This is Rust forcing you to confront "what if
// this key doesn't exist?" at compile time instead of at runtime (unlike, say,
// Python's `dict[key]`, which raises `KeyError`).
//
// WHY IT MATTERS: Missing keys are one of the most common real-world bugs.
// Rust's `Option` return type means the compiler won't let you accidentally
// treat "key not found" as if it were valid data — you must explicitly handle
// both cases.
//
// MENTAL MODEL: Every time you see `.get()`, immediately ask "what are my two
// branches?" — `Some(value)` and `None`. Reach for `match`, `if let`, or
// `Option` combinators (`.map()`, `.unwrap_or()`, etc.) rather than assuming
// success.
//
// TAKEAWAY: `.get()` returns `Option<&V>` — Rust makes "key might not exist"
// impossible to ignore, unlike indexing which can panic.
pub fn ex3_get_score(scores: &HashMap<String, i32>, player: &str) -> Option<i32> {
    // `.get()` gives us `Option<&i32>`. Since `i32` is `Copy`, we can turn that
    // into `Option<i32>` cheaply with `.copied()` rather than pattern-matching
    // by hand.
    scores.get(player).copied()
}

// ============================================================================
// Exercise 4: Checking Existence with `.contains_key()`
// ============================================================================
//
// CONCEPT: Sometimes you only care WHETHER a key exists, not its value.
// `.contains_key(&key)` returns a plain `bool`, which is clearer and slightly
// more efficient than calling `.get()` and pattern-matching just to check
// `Some`/`None`.
//
// WHY IT MATTERS: This shows up constantly in validation logic — "has this
// user already registered?", "have we already processed this ID?" — where you
// want a boolean answer, not the underlying value.
//
// MENTAL MODEL: Reach for `.contains_key()` when your next line of code is an
// `if`/`else` branching purely on presence, and you don't actually need the
// value in that branch. If you DO need the value too, prefer `.get()` (or the
// `entry()` API from Exercise 7) instead of checking twice.
//
// TAKEAWAY: Use `.contains_key()` for pure presence checks — it says exactly
// what you mean and avoids awkward `Option` handling when you don't need the
// value.
pub fn ex4_is_registered(users: &HashMap<String, u32>, username: &str) -> bool {
    users.contains_key(username)
}

// ============================================================================
// Exercise 5: Removing an Entry with `.remove()`
// ============================================================================
//
// CONCEPT: `.remove(&key)` deletes the entry (if present) and hands the value
// back to you as `Option<V>` — `Some(value)` if it existed, `None` if it
// didn't. Note this takes `&mut HashMap`, since removing mutates the map.
//
// WHY IT MATTERS: This is the same "might not exist" pattern as `.get()`, but
// combined with mutation. It teaches you that Rust's ownership model lets you
// take the *owned* value back out of the map (not just a reference to it) —
// once removed, you own that `V` again and can move it elsewhere.
//
// MENTAL MODEL: Removing is "pop by key" instead of "pop from the end" like
// `Vec::pop()`. Ask yourself: do I need the old value afterward? If yes, use
// the `Option<V>` that `.remove()` gives back rather than calling `.get()`
// first and `.remove()` second (which does redundant work).
//
// TAKEAWAY: `.remove()` both deletes AND returns the value as `Option<V>` in
// one step — no need to look before you remove.
pub fn ex5_remove_item(inventory: &mut HashMap<String, u32>, item: &str) -> Option<u32> {
    // Note: `.remove()` takes `&str` directly here (no `.to_string()` needed)
    // because `HashMap<String, _>::remove` accepts any `&Q` where `String:
    // Borrow<Q>` — and `str` is exactly such a `Q`.
    inventory.remove(item)
}

// ============================================================================
// Exercise 6: Iterating Over Key-Value Pairs
// ============================================================================
//
// CONCEPT: `HashMap` implements `IntoIterator`, so you can loop over it with
// `for (key, value) in &map`. Each iteration yields a `(&K, &V)` tuple. Since
// iteration order is unspecified, any logic you write inside the loop must not
// depend on which entry comes first.
//
// WHY IT MATTERS: Aggregating over a whole map — summing values, building a
// report, transforming into another collection — is one of the most common
// real-world HashMap operations.
//
// MENTAL MODEL: Think "for each pair, do something order-independent." If a
// task requires an order (like sorted output), you must collect keys/entries
// into a `Vec` and sort explicitly — the map itself will never guarantee one.
//
// TAKEAWAY: Iterating a `HashMap` gives `(&K, &V)` pairs in unspecified order —
// write aggregation logic (sums, counts, filters) that doesn't rely on order.
pub fn ex6_sum_all_values(inventory: &HashMap<String, u32>) -> u32 {
    // `.values()` gives us an iterator of `&u32` directly, since we don't need
    // the keys at all here — no reason to destructure `(k, v)` pairs.
    let mut total = 0;
    for quantity in inventory.values() {
        total += quantity;
    }
    total

    // Equally idiomatic one-liner, once you're comfortable with iterator
    // adapters:
    //   inventory.values().sum()
}

// ============================================================================
// Exercise 7: The `entry()` API — Insert-If-Absent
// ============================================================================
//
// CONCEPT: A very common pattern is "if this key exists, leave it; if not,
// insert a default." Doing this naively requires TWO map lookups (`.contains_key()`
// then `.insert()`), which is both slower and more verbose. The `entry()` API
// does it in ONE lookup: `map.entry(key).or_insert(default_value)`.
//
// WHY IT MATTERS: `entry()` is the idiomatic Rust way to avoid the classic
// "check-then-act" bug pattern (where the state could theoretically change
// between your check and your act — not a data race here since it's single-
// threaded, but still wasteful and non-idiomatic).
//
// MENTAL MODEL: Read `map.entry(key).or_insert(default)` as "get me a mutable
// handle to this slot, creating it with `default` if it's empty" — the
// expression evaluates to `&mut V`, ready for you to read or mutate further.
//
// TAKEAWAY: `entry(key).or_insert(default)` replaces the "check if exists,
// then insert or update" pattern with a single, efficient, idiomatic call.
pub fn ex7_ensure_default_score(scores: &mut HashMap<String, i32>, player: &str) {
    // `.entry()` takes an owned `String` (not `&str`), because it may need to
    // move that key into the map if the slot is currently empty. We only
    // allocate that `String` — we never touch the returned `&mut i32`, so
    // existing scores are left completely untouched.
    scores.entry(player.to_string()).or_insert(0);
}

// ============================================================================
// Exercise 8: The `entry()` API — Modify-Or-Insert
// ============================================================================
//
// CONCEPT: `entry()` composes with `.and_modify()` to express "if the key
// exists, run this closure on its value; either way, ensure a default is
// present." This is the natural next step after Exercise 7: instead of just
// inserting a default, you now conditionally UPDATE an existing value too.
//
// WHY IT MATTERS: This exact pattern — "add points if the player exists,
// otherwise create them with starting points" — appears everywhere: shopping
// cart quantities, running totals, leaderboard updates, config merging.
//
// MENTAL MODEL: Chain the methods left to right as a sentence: "get this
// entry; AND MODIFY it (if present) by adding points; OR (if absent) INSERT
// starting points instead." Only one of `.and_modify()`'s closure or
// `.or_insert()`'s value ever actually runs/applies for a given call.
//
// TAKEAWAY: `entry(key).and_modify(|v| ...).or_insert(default)` lets you
// update-if-present and insert-if-absent in a single, chainable expression.
pub fn ex8_add_points(scores: &mut HashMap<String, i32>, player: &str, points: i32) {
    scores
        .entry(player.to_string())
        // `.and_modify()` only runs this closure if the entry already exists;
        // it receives `&mut i32`, so we dereference to add in place.
        .and_modify(|score| *score += points)
        // `.or_insert()` only runs if the entry was absent — note `points`,
        // not `0`, since a brand-new player's starting score IS `points`.
        .or_insert(points);
}

// ============================================================================
// Exercise 9 (Capstone): Counting Occurrences — Word Frequency
// ============================================================================
//
// CONCEPT: This exercise combines everything above: iterating over input data,
// and using `entry().or_insert(0)` to build up counts as you go. Counting
// occurrences (word frequency, character frequency, vote tallying, histogram
// building) is probably the single most common real-world use of HashMap.
//
// WHY IT MATTERS: This is the pattern you will reach for constantly once you
// leave "learning exercises" and start writing real programs: log analysis,
// text processing, deduplication reports, and analytics all reduce to "count
// how many times each distinct thing appears."
//
// MENTAL MODEL: For each item in your input, ask "what's the current count for
// this key?" and answer it in one motion:
// `*counts.entry(item).or_insert(0) += 1;`
// Note the leading `*` — `entry(...).or_insert(0)` gives you a `&mut i32`, so
// you must dereference it before you can add to it in place.
//
// TAKEAWAY: `*map.entry(key).or_insert(0) += 1;` is the idiomatic, one-line
// Rust pattern for counting occurrences — memorize this line, you'll use it
// constantly.
pub fn ex9_word_frequency(text: &str) -> HashMap<String, u32> {
    let mut counts = HashMap::new();
    // `.split_whitespace()` splits on any run of whitespace and skips empty
    // fragments — exactly what you want for word-splitting (unlike `.split(' ')`,
    // which would produce empty strings for repeated spaces).
    for word in text.split_whitespace() {
        *counts.entry(word.to_string()).or_insert(0) += 1;
    }
    counts
}

// ============================================================================
// Tests
// ============================================================================
// These tests define the CORRECT behavior for each exercise. They pass against
// the implementations above and would fail against `todo!()` placeholders.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1_create_and_insert() {
        let map = ex1_create_and_insert();
        assert_eq!(map.len(), 1);
        assert_eq!(map.get("apples"), Some(&5));
    }

    #[test]
    fn test_ex2_insert_three_fruits() {
        let map = ex2_insert_three_fruits("apples", 5, "bananas", 3, "cherries", 12);
        assert_eq!(map.len(), 3);
        assert_eq!(map.get("apples"), Some(&5));
        assert_eq!(map.get("bananas"), Some(&3));
        assert_eq!(map.get("cherries"), Some(&12));
    }

    #[test]
    fn test_ex3_get_score() {
        let mut scores = HashMap::new();
        scores.insert("alice".to_string(), 42);

        assert_eq!(ex3_get_score(&scores, "alice"), Some(42));
        assert_eq!(ex3_get_score(&scores, "bob"), None);
    }

    #[test]
    fn test_ex4_is_registered() {
        let mut users = HashMap::new();
        users.insert("alice".to_string(), 1u32);

        assert!(ex4_is_registered(&users, "alice"));
        assert!(!ex4_is_registered(&users, "mallory"));
    }

    #[test]
    fn test_ex5_remove_item() {
        let mut inventory = HashMap::new();
        inventory.insert("widgets".to_string(), 10u32);
        inventory.insert("gadgets".to_string(), 4u32);

        let removed = ex5_remove_item(&mut inventory, "widgets");
        assert_eq!(removed, Some(10));
        assert!(!inventory.contains_key("widgets"));
        assert_eq!(inventory.len(), 1);

        let removed_missing = ex5_remove_item(&mut inventory, "sprockets");
        assert_eq!(removed_missing, None);
    }

    #[test]
    fn test_ex6_sum_all_values() {
        let mut inventory = HashMap::new();
        inventory.insert("widgets".to_string(), 10u32);
        inventory.insert("gadgets".to_string(), 4u32);
        inventory.insert("gizmos".to_string(), 6u32);

        assert_eq!(ex6_sum_all_values(&inventory), 20);
        assert_eq!(ex6_sum_all_values(&HashMap::new()), 0);
    }

    #[test]
    fn test_ex7_ensure_default_score() {
        let mut scores = HashMap::new();
        scores.insert("alice".to_string(), 99);

        // Existing player's score must be left untouched.
        ex7_ensure_default_score(&mut scores, "alice");
        assert_eq!(scores.get("alice"), Some(&99));

        // New player must be inserted with default 0.
        ex7_ensure_default_score(&mut scores, "bob");
        assert_eq!(scores.get("bob"), Some(&0));
    }

    #[test]
    fn test_ex8_add_points() {
        let mut scores = HashMap::new();
        scores.insert("alice".to_string(), 10);

        // Existing player: points should be added to current score.
        ex8_add_points(&mut scores, "alice", 5);
        assert_eq!(scores.get("alice"), Some(&15));

        // New player: points become their starting score.
        ex8_add_points(&mut scores, "carol", 7);
        assert_eq!(scores.get("carol"), Some(&7));
    }

    #[test]
    fn test_ex9_word_frequency() {
        let freq = ex9_word_frequency("the quick brown fox jumps over the lazy dog the fox runs");

        assert_eq!(freq.get("the"), Some(&3));
        assert_eq!(freq.get("fox"), Some(&2));
        assert_eq!(freq.get("quick"), Some(&1));
        assert_eq!(freq.get("dog"), Some(&1));
        assert_eq!(freq.get("cat"), None);
        assert_eq!(freq.len(), 9);
    }
}