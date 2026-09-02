#![allow(unused_variables, dead_code, unused_mut)]
// ============================================================================
// Vec<T> Exercises — 10 hands-on drills for mastering Rust's growable array.
//
// How to use this file:
//   1. Read the doc comment above each `exercise_N` function.
//   2. Replace the `todo!()` (or fill in the partial implementation) so the
//      function behaves as described.
//   3. Run `cargo test` — every exercise has at least one test that will
//      only pass once your implementation is correct.
//
// Exercises are ordered by difficulty: 1–3 cover the absolute basics
// (push/pop/indexing/insert/remove), 4–6 move into sorting/cleaning data,
// 7–9 cover more surgical operations (drain, extend/truncate, swap/windows),
// and 10 combines everything with a Vec<Vec<T>> (a common "2D grid" shape).
// ============================================================================

// ----------------------------------------------------------------------------
// Exercise 1: push / pop / len
//
// `Vec<T>` is a heap-allocated, growable array. `push` appends to the end
// and `pop` removes from the end — both are O(1) amortized, which is why
// Vec is the default "list" type in Rust instead of a linked list. This
// exercise builds the most fundamental mental model: a Vec grows and
// shrinks from its *tail*, and `len()` always reflects the current size.
//
// Real-world use: accumulating results in a loop (e.g. collecting parsed
// lines, batching outgoing network requests) almost always starts as
// `let mut buf = Vec::new();` followed by repeated `buf.push(...)`.
// ----------------------------------------------------------------------------
fn exercise_1_build_and_shrink(n: i32) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();

    // TODO: push the integers 1..=n onto `v` in order.
    // TODO: then pop the very last element off (discard the popped value).
    // Hint: `for i in 1..=n { ... }`, and `Option`-returning `v.pop()`.
    todo!()
}
// Takeaway: push/pop operate on the *end* of a Vec in O(1) amortized time — that's why Vec is Rust's default growable list.

// ----------------------------------------------------------------------------
// Exercise 2: indexing, get, and contains
//
// Rust gives you two ways to read an element by position: `v[i]` (panics
// on out-of-bounds) and `v.get(i)` (returns `Option<&T>`, never panics).
// `contains` does a linear scan to check membership without needing an
// index at all. Understanding when to use each is a core Rust habit:
// prefer `get` (or `iter().position()`) over raw indexing whenever the
// index might be invalid, because a panic will crash the whole program.
//
// Real-world use: validating user input against a list of allowed values,
// or safely reading an optional "nth" element from a parsed command line.
// ----------------------------------------------------------------------------
fn exercise_2_contains_and_index(v: &Vec<i32>, target: i32) -> (bool, Option<usize>) {
    // TODO: return a tuple of:
    //   .0 -> whether `target` is present in `v` (use `.contains(&target)`)
    //   .1 -> the index of `target` if present, else None
    //         (use `.iter().position(|&x| x == target)`)
    todo!()
}
// Takeaway: `contains` and `iter().position()` let you query a Vec safely without risking an out-of-bounds panic.

// ----------------------------------------------------------------------------
// Exercise 3: insert and remove at an arbitrary position
//
// Unlike `push`/`pop`, `insert(index, value)` and `remove(index)` operate
// anywhere in the Vec — but they're O(n) because every element after the
// index has to shift over by one to keep the Vec contiguous in memory.
// That contiguity is exactly what makes Vec cache-friendly and indexable
// in O(1); the shifting cost is the trade-off for that layout.
//
// Real-world use: maintaining a sorted or priority-ordered list where new
// items must land in the middle, or removing a specific item (e.g. a
// completed to-do) from a task list by its position.
// ----------------------------------------------------------------------------
fn exercise_3_insert_and_remove(
    v: &mut Vec<i32>,
    insert_at: usize,
    value: i32,
    remove_at: usize,
) -> i32 {
    // TODO: insert `value` at index `insert_at`, then remove the element
    // at index `remove_at` (evaluated *after* the insert) and return it.
    todo!()
}
// Takeaway: insert/remove keep a Vec contiguous by shifting elements, which costs O(n) — unlike the O(1) push/pop at the tail.

// ----------------------------------------------------------------------------
// Exercise 4: sorting in place
//
// `sort()` uses a stable comparison sort (Timsort-like), while
// `sort_unstable()` is typically faster but doesn't guarantee the relative
// order of equal elements is preserved. Both mutate the Vec in place
// rather than returning a new one — a common Rust convention for
// "self-mutating" methods (compare with `sorted()` in other languages,
// which doesn't exist in std Rust).
//
// Real-world use: sorting log entries by timestamp, leaderboard scores,
// or any dataset before running a binary search (`binary_search`) on it.
// ----------------------------------------------------------------------------
fn exercise_4_sort_ascending(v: &mut Vec<i32>) {
    // TODO: sort `v` in ascending order, in place.
    // Hint: `v.sort();` (stable) is the idiomatic default unless you have
    // a proven reason to reach for `sort_unstable()`.
    todo!()
}
// Takeaway: sort() and sort_unstable() mutate the Vec in place — Rust methods that end without returning a new collection usually mean "this modifies self."

// ----------------------------------------------------------------------------
// Exercise 5: dedup (and why it needs sorted input)
//
// `dedup()` only removes *consecutive* duplicate elements — it is NOT a
// general "unique-ify" operation. `[1, 2, 1]` stays `[1, 2, 1]` after
// dedup because the two `1`s aren't adjacent. This is a classic beginner
// trap: to remove *all* duplicates you must `sort()` first so equal
// values become neighbors, then `dedup()`.
//
// Real-world use: cleaning up a list of IDs collected from multiple
// sources before further processing, where true uniqueness matters more
// than preserving original order.
// ----------------------------------------------------------------------------
fn exercise_5_dedup_consecutive(v: &mut Vec<i32>) {
    // TODO: sort `v`, then remove duplicate values so every remaining
    // element is unique. Hint: `v.sort(); v.dedup();`
    todo!()
}
// Takeaway: dedup() only collapses *adjacent* duplicates — sort first if you want true set-like uniqueness.

// ----------------------------------------------------------------------------
// Exercise 6: retain (filtering in place)
//
// `retain(|x| predicate)` keeps only the elements for which the closure
// returns `true`, dropping the rest — all without allocating a second
// Vec. This is the idiomatic in-place alternative to
// `v.into_iter().filter(...).collect()` when you don't need to consume
// and rebuild the Vec, and it's the standard tool for "remove all items
// matching some condition."
//
// Real-world use: pruning expired cache entries, filtering out
// blocked/banned users from a list, or dropping malformed records
// during data cleaning — all without extra allocations.
// ----------------------------------------------------------------------------
fn exercise_6_retain_even(v: &mut Vec<i32>) {
    // TODO: mutate `v` in place so only even numbers remain.
    // Hint: `v.retain(|&x| x % 2 == 0);`
    todo!()
}
// Takeaway: retain() filters a Vec in place with a predicate — no second allocation needed, unlike collect()-based filtering.

// ----------------------------------------------------------------------------
// Exercise 7: drain (removing a range and getting the removed items back)
//
// `drain(range)` removes the specified range of elements from the Vec
// AND returns them as an iterator you can collect, all while leaving
// the rest of the Vec intact and re-compacted. It's the tool to reach
// for when you need to both *extract* and *shrink* in one pass, instead
// of `remove`-ing one element at a time (which would be O(n) per call).
//
// Real-world use: splitting a batch of work items off a shared queue to
// hand to a worker thread, or extracting a processed prefix of a buffer
// while keeping the unprocessed remainder for the next iteration.
// ----------------------------------------------------------------------------
fn exercise_7_drain_range(v: &mut Vec<i32>, start: usize, end: usize) -> Vec<i32> {
    // TODO: remove the elements in the half-open range [start, end) from
    // `v`, and return exactly those removed elements as a new Vec.
    // Hint: `v.drain(start..end).collect()`
    todo!()
}
// Takeaway: drain() removes AND returns a range in one O(n) pass — far cheaper than repeated single-element remove() calls.

// ----------------------------------------------------------------------------
// Exercise 8: extend and truncate
//
// `extend` appends every item from another iterable onto the end of a
// Vec (like calling `push` in a loop, but often more efficient because
// it can pre-reserve capacity). `truncate(n)` does the opposite: it
// shortens the Vec down to at most `n` elements, dropping anything past
// that length. Together they show how a Vec's length can be grown and
// clamped independently of how it was originally built.
//
// Real-world use: appending a newly-fetched page of API results onto an
// accumulator Vec, then truncating to a "top N" limit before displaying
// results to a user.
// ----------------------------------------------------------------------------
fn exercise_8_extend_and_truncate(v: &mut Vec<i32>, extra: &[i32], max_len: usize) {
    // TODO: append every element of `extra` onto `v`, then shrink `v`
    // down to at most `max_len` elements.
    // Hint: `v.extend(extra.iter().copied()); v.truncate(max_len);`
    todo!()
}
// Takeaway: extend() grows a Vec from another iterable and truncate() clamps its length — neither needs manual loops or reallocation logic.

// ----------------------------------------------------------------------------
// Exercise 9: swap and windows
//
// `swap(i, j)` exchanges two elements in place without needing a temp
// variable or fighting the borrow checker over two mutable references.
// `windows(n)` gives you a *view* of every overlapping slice of length
// `n` — useful whenever you need to compare or combine neighboring
// elements (e.g. "is this list sorted?" or "sum every adjacent pair").
// Note `windows` is defined on slices, so it borrows rather than clones.
//
// Real-world use: `swap` shows up in sorting algorithms and shuffles;
// `windows` is common in signal smoothing, detecting consecutive
// duplicates, or computing moving averages over a data series.
// ----------------------------------------------------------------------------
fn exercise_9_swap_and_window_sums(v: &mut Vec<i32>, i: usize, j: usize) -> Vec<i32> {
    // TODO: swap the elements at indices `i` and `j` in `v`, then return
    // a Vec containing the sum of every overlapping pair of adjacent
    // elements (i.e. use `v.windows(2)` and sum each window).
    // Hint: `v.swap(i, j);` then
    //       `v.windows(2).map(|w| w[0] + w[1]).collect()`
    todo!()
}
// Takeaway: swap() safely exchanges two elements in place, and windows(n) lets you inspect overlapping neighbor groups without manual index math.

// ----------------------------------------------------------------------------
// Exercise 10: Vec<Vec<T>> — a 2D grid
//
// A `Vec<Vec<T>>` is Rust's simplest representation of a 2D grid or
// matrix: an outer Vec of "rows," each of which is its own independent
// Vec. This exercise ties together everything above — you'll iterate the
// outer Vec, and for each inner Vec use `.iter().sum()` to reduce it to
// a single number, producing a new Vec of row sums.
//
// Real-world use: spreadsheet-like tabular data, image pixel grids, game
// boards (e.g. tic-tac-toe, Minesweeper), or any row/column dataset
// before it's handed off to a more specialized crate like `ndarray`.
// ----------------------------------------------------------------------------
fn exercise_10_matrix_row_sums(matrix: &Vec<Vec<i32>>) -> Vec<i32> {
    // TODO: return a new Vec where element `k` is the sum of row `k` of
    // `matrix`. Hint: `matrix.iter().map(|row| row.iter().sum()).collect()`
    todo!()
}
// Takeaway: Vec<Vec<T>> models a 2D grid as rows of independent Vecs — iterate the outer Vec and reduce each inner Vec with standard Iterator methods.

fn main() {
    // Just to silence the unused warnings — you can optionally experiment here.
}


// ============================================================================
// Tests — run `cargo test` to check your implementations.
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise_01_build_and_shrink() {
        // Push 1..=5 -> [1,2,3,4,5], then pop -> [1,2,3,4]
        assert_eq!(exercise_1_build_and_shrink(5), vec![1, 2, 3, 4]);
        assert_eq!(exercise_1_build_and_shrink(1), vec![]);
    }

    #[test]
    fn test_exercise_02_contains_and_index() {
        let v = vec![10, 20, 30, 40];
        assert_eq!(exercise_2_contains_and_index(&v, 30), (true, Some(2)));
        assert_eq!(exercise_2_contains_and_index(&v, 99), (false, None));
    }

    #[test]
    fn test_exercise_03_insert_and_remove() {
        let mut v = vec![1, 2, 3];
        // Insert 99 at index 1 -> [1, 99, 2, 3]; remove index 3 (the `3`) -> returns 3
        let removed = exercise_3_insert_and_remove(&mut v, 1, 99, 3);
        assert_eq!(removed, 3);
        assert_eq!(v, vec![1, 99, 2]);
    }

    #[test]
    fn test_exercise_04_sort_ascending() {
        let mut v = vec![5, 3, 1, 4, 2];
        exercise_4_sort_ascending(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_exercise_05_dedup_consecutive() {
        let mut v = vec![3, 1, 2, 1, 3, 2];
        exercise_5_dedup_consecutive(&mut v);
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn test_exercise_06_retain_even() {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        exercise_6_retain_even(&mut v);
        assert_eq!(v, vec![2, 4, 6]);
    }

    #[test]
    fn test_exercise_07_drain_range() {
        let mut v = vec![10, 20, 30, 40, 50];
        let drained = exercise_7_drain_range(&mut v, 1, 3);
        assert_eq!(drained, vec![20, 30]);
        assert_eq!(v, vec![10, 40, 50]);
    }

    #[test]
    fn test_exercise_08_extend_and_truncate() {
        let mut v = vec![1, 2, 3];
        exercise_8_extend_and_truncate(&mut v, &[4, 5, 6, 7], 5);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_exercise_09_swap_and_window_sums() {
        let mut v = vec![1, 2, 3, 4];
        // swap(0, 3) -> [4, 2, 3, 1]; window sums -> [6, 5, 4]
        let sums = exercise_9_swap_and_window_sums(&mut v, 0, 3);
        assert_eq!(v, vec![4, 2, 3, 1]);
        assert_eq!(sums, vec![6, 5, 4]);
    }

    #[test]
    fn test_exercise_10_matrix_row_sums() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(exercise_10_matrix_row_sums(&matrix), vec![6, 15, 24]);
    }
}