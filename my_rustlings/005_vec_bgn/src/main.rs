//! # `Vec<T>` Practice: 10 Beginner Exercises
//!
//! Complete each `// TODO` in the numbered functions below. Every exercise
//! is paired with at least one test in the `tests` module at the bottom —
//! run `cargo test` (or `rustc --test vec_exercises.rs && ./vec_exercises`)
//! to check your work.
//!
//! The exercises are ordered from simplest to most involved, and each one
//! is designed to teach a single, distinct idea about how `Vec<T>` behaves
//! and why it's shaped the way it is.

// ===========================================================================
// Exercise 1: Creating a vector and pushing elements
// ===========================================================================

/// **Concept: `Vec::new()` and `push`**
///
/// A `Vec<T>` is a growable, heap-allocated list. `Vec::new()` creates an
/// empty vector with no allocation yet — Rust doesn't allocate memory until
/// you actually put something in it. Calling `.push(value)` appends an
/// element to the end, growing the vector's capacity (and reallocating its
/// backing buffer) automatically as needed.
///
/// The takeaway: unlike a fixed-size array `[T; N]`, a `Vec<T>`'s length is
/// not known at compile time — it's tracked at runtime as three pieces of
/// state under the hood: a pointer, a length, and a capacity.
///
/// Task: build and return a vector containing the integers 1, 2, 3, 4, 5,
/// using `Vec::new()` and repeated calls to `.push()`.
fn ex1_create_and_push() -> Vec<i32> {
    // TODO: create an empty Vec<i32> with Vec::new(), push 1..=5 onto it,
    // and return it.
    Vec::new()
}

// ===========================================================================
// Exercise 2: Vec::with_capacity and the vec! macro
// ===========================================================================

/// **Concept: pre-allocating capacity vs. the `vec!` macro**
///
/// `Vec::with_capacity(n)` reserves room for `n` elements up front, which
/// avoids repeated reallocations when you already know roughly how many
/// items you'll store — a small but real performance idiom. The `vec!`
/// macro is the idiomatic shorthand for building a vector with known
/// initial contents, e.g. `vec![1, 2, 3]`.
///
/// The takeaway: capacity (how much memory is reserved) and length (how
/// many elements are actually stored) are different things. Reserving
/// capacity does not add elements — `len()` stays 0 until you push.
///
/// Task: create a vector with capacity for at least 10 `i32`s using
/// `Vec::with_capacity`, confirm its length is still 0, then return it.
fn ex2_with_capacity() -> Vec<i32> {
    // TODO: use Vec::with_capacity(10) to create the vector and return it
    // without pushing anything.
    Vec::new()
}

// ===========================================================================
// Exercise 3: Popping elements
// ===========================================================================

/// **Concept: `pop()` returns `Option<T>`**
///
/// `.pop()` removes and returns the *last* element of the vector, wrapped
/// in `Some(value)` — or `None` if the vector was already empty. This is a
/// classic example of Rust modeling "this operation might not have an
/// answer" directly in the type system instead of returning a sentinel
/// value (like `-1` or `null`) or panicking.
///
/// The takeaway: because the possibility of "no last element" is baked
/// into the return type, the compiler forces you to handle the empty case
/// before you can use the value — there's no way to accidentally read
/// garbage from an empty vector.
///
/// Task: given a mutable vector, pop its last element and return it as an
/// `Option<i32>` (don't unwrap — pass the `Option` straight through).
fn ex3_pop_last(numbers: &mut Vec<i32>) -> Option<i32> {
    // TODO: remove and return the last element using .pop().
    None
}

// ===========================================================================
// Exercise 4: Safe indexing with get()
// ===========================================================================

/// **Concept: `[]` indexing panics, `.get()` doesn't**
///
/// Indexing a vector with `numbers[i]` panics if `i` is out of bounds —
/// fine when you're certain the index is valid, dangerous when it's user
/// input or otherwise untrusted. `.get(i)` instead returns `Option<&T>`,
/// letting you handle an invalid index gracefully instead of crashing the
/// whole program.
///
/// The takeaway: prefer `.get()` over `[]` whenever the index isn't
/// guaranteed to be in bounds. This is the same "put the failure case in
/// the type" idea as `.pop()` in Exercise 3, applied to lookups.
///
/// Task: return a copy of the element at `index` if it exists, or `None`
/// if `index` is out of bounds.
fn ex4_safe_get(numbers: &[i32], index: usize) -> Option<i32> {
    // TODO: use numbers.get(index) and copy the value out (i32 is Copy).
    None
}

// ===========================================================================
// Exercise 5: Iterating and summing
// ===========================================================================

/// **Concept: `for` loops borrow by reference**
///
/// Writing `for n in &numbers` iterates over *references* to each element
/// (`&i32`), leaving the original vector untouched and still usable
/// afterward. Writing `for n in numbers` instead would *move* the vector
/// into the loop, consuming it — after that loop, `numbers` no longer
/// exists in the caller's scope.
///
/// The takeaway: borrowing in a loop is the default choice when you only
/// need to read the elements; it's what lets you iterate over data you
/// still need later, without cloning it.
///
/// Task: sum every element in the slice and return the total.
fn ex5_sum_all(numbers: &[i32]) -> i32 {
    let mut total = 0;
    // TODO: loop over `numbers` by reference, adding each element to `total`.
    total
}

// ===========================================================================
// Exercise 6: Iterator adapters — map and collect
// ===========================================================================

/// **Concept: `iter().map(...).collect()`**
///
/// Rust's iterator adapters let you describe a data transformation
/// declaratively instead of manually managing loop indices and an output
/// buffer. `.iter()` produces `&i32` references, `.map(|n| n * 2)`
/// transforms each one, and `.collect()` gathers the results into a new
/// `Vec<i32>` — inferred from the function's return type.
///
/// The takeaway: iterator chains compile down to code just as fast as a
/// hand-written loop (Rust calls this "zero-cost abstraction") while being
/// far more readable and composable — you can chain `.filter()`, `.map()`,
/// `.take()`, etc. in sequence.
///
/// Task: return a new vector containing every element of `numbers`
/// doubled, without mutating the input.
fn ex6_double_all(numbers: &[i32]) -> Vec<i32> {
    // TODO: use numbers.iter().map(...).collect() to build the result.
    Vec::new()
}

// ===========================================================================
// Exercise 7: Slicing
// ===========================================================================

/// **Concept: `&[T]` slices are a view, not a copy**
///
/// A slice like `&numbers[1..3]` borrows a contiguous range of the
/// underlying vector without copying any elements — it's just a pointer
/// plus a length pointing into the original buffer. This is why functions
/// that only need to *read* a sequence should generally take `&[T]`
/// parameters instead of `&Vec<T>`: a slice works for arrays, vectors, and
/// sub-ranges alike.
///
/// The takeaway: slicing is cheap (no allocation, no copying) and is the
/// idiomatic way to refer to "some contiguous part of a sequence."
///
/// Task: return a slice containing every element except the first and
/// last (assume the input has at least 2 elements).
fn ex7_middle_slice(numbers: &[i32]) -> &[i32] {
    // TODO: return &numbers[1..numbers.len() - 1].
    &[]
}

// ===========================================================================
// Exercise 8: A Vec of Strings
// ===========================================================================

/// **Concept: `Vec<String>` and ownership of owned data**
///
/// So far every vector has held `Copy` types like `i32`, where cloning is
/// implicit and cheap. `String` is different: it owns a heap allocation,
/// so pushing one into a vector *moves* it in, and if you want a copy you
/// must call `.clone()` explicitly. This exercise is the first place the
/// ownership rules from earlier in your Rust learning show up inside a
/// collection.
///
/// The takeaway: a `Vec<String>` owns both the vector's buffer *and* every
/// string inside it — dropping the vector drops all of them. Building one
/// with `.to_string()` (or `String::from`) turns a borrowed `&str` literal
/// into an owned value the vector can hold onto independently.
///
/// Task: build and return a `Vec<String>` containing the words "rust",
/// "is", and "fun", in that order, as owned `String`s.
fn ex8_build_string_vec() -> Vec<String> {
    // TODO: push "rust".to_string(), "is".to_string(), "fun".to_string()
    // (or use the vec! macro) and return the vector.
    Vec::new()
}

// ===========================================================================
// Exercise 9: A Vec of structs, filtered
// ===========================================================================

/// A simple record type used by Exercise 9 and Exercise 10.
#[derive(Debug, Clone, PartialEq)]
struct Player {
    name: String,
    score: u32,
}

/// **Concept: filtering a `Vec<T>` of custom structs with `iter().filter()`**
///
/// Vectors aren't limited to primitive types — a `Vec<Player>` behaves
/// exactly the same way, just with a bigger `T`. `.iter().filter(|p| ...)`
/// walks the vector by reference and keeps only the elements matching a
/// predicate, producing a new iterator (and, after `.cloned().collect()`,
/// a new owned `Vec<Player>`) without touching the original.
///
/// The takeaway: `filter` is *non-destructive* — it builds a new
/// collection rather than modifying the source in place. That makes it the
/// right tool when you still need the original data afterward, in
/// contrast with `retain` in Exercise 10, which mutates in place.
///
/// Task: given a slice of `Player`s, return a new `Vec<Player>` containing
/// only the players whose `score` is greater than or equal to
/// `min_score`.
fn ex9_filter_high_scorers(players: &[Player], min_score: u32) -> Vec<Player> {
    // TODO: use players.iter().filter(|p| p.score >= min_score).cloned().collect().
    Vec::new()
}

// ===========================================================================
// Exercise 10: Sorting and deduplicating in place with retain
// ===========================================================================

/// **Concept: `sort_by_key`, `dedup`, and `retain` — in-place mutation**
///
/// This exercise combines three common in-place `Vec` operations:
/// - `.sort_by_key(|p| ...)` sorts the vector in place by a derived key
///   (here, descending score), unlike `filter`/`map`, which build new
///   collections.
/// - `.dedup_by_key(|p| ...)` removes *consecutive* duplicate keys — which
///   is exactly why sorting first matters: `dedup` only catches
///   neighbors, not duplicates scattered throughout the vector.
/// - `.retain(|p| ...)` keeps only the elements matching a predicate,
///   removing the rest in place, with no new allocation for the result.
///
/// The takeaway: `sort`, `dedup`, and `retain` all mutate the vector
/// directly through `&mut self` rather than returning a new one — that's
/// your cue, when reading their signatures, that they're in-place
/// operations, and it's why the parameter here is `&mut Vec<Player>`
/// rather than `&[Player]`.
///
/// Task: given a mutable vector of players, sort it by score in
/// descending order, remove any players with a score of 0 using
/// `.retain()`, and return nothing — mutate `players` in place.
fn ex10_sort_desc_and_remove_zero_scores(players: &mut Vec<Player>) {
    // TODO:
    // 1. players.sort_by_key(|p| std::cmp::Reverse(p.score));
    // 2. players.retain(|p| p.score > 0);
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1_create_and_push() {
        let result = ex1_create_and_push();
        // Checks that all five elements were pushed, in order.
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_ex2_with_capacity() {
        let result = ex2_with_capacity();
        // Capacity should be reserved, but length must still be zero —
        // capacity and length are independent properties.
        assert_eq!(result.len(), 0);
        assert!(result.capacity() >= 10);
    }

    #[test]
    fn test_ex3_pop_last() {
        let mut numbers = vec![10, 20, 30];
        let popped = ex3_pop_last(&mut numbers);
        // The last element should be returned...
        assert_eq!(popped, Some(30));
        // ...and removed from the original vector.
        assert_eq!(numbers, vec![10, 20]);
    }

    #[test]
    fn test_ex3_pop_last_empty() {
        let mut empty: Vec<i32> = Vec::new();
        // Popping an empty vector must return None, not panic.
        assert_eq!(ex3_pop_last(&mut empty), None);
    }

    #[test]
    fn test_ex4_safe_get() {
        let numbers = vec![1, 2, 3];
        // In-bounds index returns Some(value).
        assert_eq!(ex4_safe_get(&numbers, 1), Some(2));
        // Out-of-bounds index returns None instead of panicking.
        assert_eq!(ex4_safe_get(&numbers, 10), None);
    }

    #[test]
    fn test_ex5_sum_all() {
        let numbers = vec![1, 2, 3, 4];
        // 1 + 2 + 3 + 4 = 10.
        assert_eq!(ex5_sum_all(&numbers), 10);
        // Summing an empty slice should yield the additive identity, 0.
        assert_eq!(ex5_sum_all(&[]), 0);
    }

    #[test]
    fn test_ex6_double_all() {
        let numbers = vec![1, 2, 3];
        // Every element should be doubled, in the original order.
        assert_eq!(ex6_double_all(&numbers), vec![2, 4, 6]);
        // The original input must be unaffected — map does not mutate.
        assert_eq!(numbers, vec![1, 2, 3]);
    }

    #[test]
    fn test_ex7_middle_slice() {
        let numbers = vec![1, 2, 3, 4, 5];
        // First and last elements should be excluded.
        assert_eq!(ex7_middle_slice(&numbers), &[2, 3, 4]);
    }

    #[test]
    fn test_ex8_build_string_vec() {
        let words = ex8_build_string_vec();
        // Checks contents and order of the owned String vector.
        assert_eq!(words, vec!["rust".to_string(), "is".to_string(), "fun".to_string()]);
    }

    #[test]
    fn test_ex9_filter_high_scorers() {
        let players = vec![
            Player { name: "Ada".to_string(), score: 90 },
            Player { name: "Grace".to_string(), score: 40 },
            Player { name: "Alan".to_string(), score: 75 },
        ];
        let high_scorers = ex9_filter_high_scorers(&players, 50);
        // Only players with score >= 50 should remain, in original order.
        assert_eq!(
            high_scorers,
            vec![
                Player { name: "Ada".to_string(), score: 90 },
                Player { name: "Alan".to_string(), score: 75 },
            ]
        );
        // The original vector must be untouched by the filter.
        assert_eq!(players.len(), 3);
    }

    #[test]
    fn test_ex10_sort_desc_and_remove_zero_scores() {
        let mut players = vec![
            Player { name: "Ada".to_string(), score: 0 },
            Player { name: "Grace".to_string(), score: 40 },
            Player { name: "Alan".to_string(), score: 75 },
            Player { name: "Linus".to_string(), score: 0 },
        ];
        ex10_sort_desc_and_remove_zero_scores(&mut players);
        // Zero-score players are removed, and the rest are sorted by
        // descending score.
        assert_eq!(
            players,
            vec![
                Player { name: "Alan".to_string(), score: 75 },
                Player { name: "Grace".to_string(), score: 40 },
            ]
        );
    }
}