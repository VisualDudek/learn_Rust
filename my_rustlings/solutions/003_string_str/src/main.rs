//! Solutions: `String`, `&str`, and slices
//!
//! Companion to `string_str_exercises.rs`. Same tests, `todo!()` bodies
//! filled in. Comments explain the *why*, not just the *what* — read them
//! even for the ones you got right, since several have a non-obvious
//! reason the idiomatic version looks the way it does.

// ---------------------------------------------------------------------
// 1. Warm-up: borrowing vs. owning
// ---------------------------------------------------------------------
// `&str` in, `&str` out: the returned slice borrows from `s`, so no
// allocation happens at all. This is only possible because the lifetime
// of the output is tied to the lifetime of the input by elision rule #1
// (one input lifetime -> it's assigned to all elided output lifetimes).
pub fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}

// ---------------------------------------------------------------------
// 2. Iterating by `char`, not by byte
// ---------------------------------------------------------------------
// `matches(char::is_ascii_vowel)`-style byte iteration would be wrong for
// non-ASCII text; `chars()` yields whole Unicode scalar values regardless
// of how many bytes each one occupies.
pub fn count_vowels(s: &str) -> usize {
    s.chars()
        .filter(|c| matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u'))
        .count()
}

// ---------------------------------------------------------------------
// 3. Building a new `String` correctly for non-ASCII input
// ---------------------------------------------------------------------
// `s[0..1]` assumes the first char is exactly 1 byte — false for 'é'
// (2 bytes) or '日' (3 bytes), and would either panic (landing mid-char)
// or silently corrupt data. `chars().next()` gives you the first *char*
// regardless of its byte width, and `char::to_uppercase()` returns an
// iterator because some chars uppercase to multiple chars (e.g. German
// 'ß' -> "SS" in some contexts) — collecting handles that generally.
pub fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

// NOTATKI: Dlaczego wewnątrz funkcji możemy zbudować String i nie jest on 
//dealokowany po wyściu z funkcji
/*
1. String jest alokowany na stercie (heap), a nie na stosie (stack). 
Kiedy tworzysz String wewnątrz funkcji, jego dane są przechowywane w pamięci 
sterty, która jest zarządzana dynamicznie.
2.
Why it's not dropped — this is the important part

This is not about heap vs. stack. It's about ownership.

Drop::drop runs when a value's owner goes out of scope and that value hasn't been moved away. In your function:

rust
first.to_uppercase().collect::<String>() + chars.as_str()

This whole expression is the tail expression of the match, which is the tail expression of the function body (no semicolon). In Rust, a tail expression's value becomes the function's return value — ownership of that String is moved out to the caller as part of the return. There's no local variable binding still "owning" it when the function frame is torn down, so there's nothing left for Drop to run on.

*/


// ---------------------------------------------------------------------
// 4. Two-pointer / iterator reasoning over `char`s
// ---------------------------------------------------------------------
// Comparing `chars()` against `chars().rev()` element-by-element sidesteps
// manual index math entirely. `to_lowercase()` is used (not
// `to_ascii_lowercase`) to be consistent with the rest of the exercise
// set, though for these ASCII-only test cases either works.
pub fn is_palindrome(s: &str) -> bool {
    let lower: String = s.chars().flat_map(|c| c.to_lowercase()).collect();
    lower.chars().eq(lower.chars().rev())
}

// ---------------------------------------------------------------------
// 5. Lifetime elision in a real signature
// ---------------------------------------------------------------------
// The explicit `<'a>` here is what elision rule #1 would generate for you
// automatically: single input lifetime -> assumed to be the output
// lifetime. You could legally write `fn longest_word(s: &str) -> &str`
// and the compiler would desugar it identically. `max_by_key` with
// `.len()` as the key naturally keeps the *first* max on ties, because
// `max_by_key` returns the last element that compares greatest-or-equal
// — actually it's the reverse: `max_by_key` returns the LAST maximal
// element, so we need `.rev()` or a manual fold to keep the first. Below
// uses a fold to make "keep first on tie" explicit rather than relying on
// iterator-internals trivia.
pub fn longest_word<'a>(s: &'a str) -> &'a str {
    s.split_whitespace()
        .fold("", |longest, word| {
            if word.len() > longest.len() {
                word
            } else {
                longest
            }
        })
}

// ---------------------------------------------------------------------
// 6. UTF-8 boundaries and panics
// ---------------------------------------------------------------------
// `get()` is the panic-free sibling of indexing: it returns `None` for
// out-of-range indices AND for indices that don't land on a char
// boundary, which is exactly the two failure modes we need to guard.
pub fn safe_slice(s: &str, start: usize, end: usize) -> Option<&str> {
    s.get(start..end)
}

// ---------------------------------------------------------------------
// 7. Splitting, transforming, rejoining
// ---------------------------------------------------------------------
// `split_whitespace()` (unlike `split(' ')`) treats runs of whitespace as
// a single delimiter and ignores leading/trailing whitespace, which is
// why "the  quick   fox" and "  padded  " behave correctly here without
// extra filtering.
pub fn reverse_words(s: &str) -> String {
    s.split_whitespace()
        .rev()
        .collect::<Vec<_>>()
        .join(" ")
}

// ---------------------------------------------------------------------
// 8. `len()` vs `chars().count()`
// ---------------------------------------------------------------------
// `str::len()` returns the byte length stored directly in the slice's
// metadata (a fat pointer: {ptr, len}) — O(1), no scanning. `chars()`
// has to walk the buffer decoding UTF-8 byte sequences one at a time to
// know where each char boundary falls, so counting them is O(n).
pub fn byte_vs_char_len(s: &str) -> (usize, usize) {
    (s.len(), s.chars().count())
}

// ---------------------------------------------------------------------
// 9. Mutating a `String` in place
// ---------------------------------------------------------------------
// `str` is a dynamically-sized type (DST) with no capacity headroom —
// it's just {ptr, len} pointing at exactly as many bytes as it has.
// There is nowhere to grow into. `String` owns a heap buffer with a
// `capacity` that can exceed `len`, so `push_str` can (re)allocate as
// needed. `&mut str` lets you mutate bytes in place but never resize.
pub fn append_exclamation(s: &mut String) {
    s.push('!');
}

// ---------------------------------------------------------------------
// 10. Byte index vs. char index — the classic footgun
// ---------------------------------------------------------------------
// `char_indices()` yields (byte_offset, char) pairs directly — the byte
// offset is exactly what you'd need for slicing with `s[..i]`. The char
// index has no built-in iterator, so it's tracked manually via
// `.enumerate()` over the same sequence.
pub fn find_char_index(s: &str, target: char) -> Option<(usize, usize)> {
    s.char_indices()
        .enumerate()
        .find(|(_, (_, c))| *c == target)
        .map(|(char_idx, (byte_idx, _))| (byte_idx, char_idx))
}

// =======================================================================
// Tests (identical to the exercise file — solutions must pass unchanged)
// =======================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("hello"), "hello");
        assert_eq!(first_word(""), "");
        assert_eq!(first_word("  leading spaces"), "");
        assert_eq!(first_word("trailing   "), "trailing");
    }

    #[test]
    fn test_count_vowels() {
        assert_eq!(count_vowels("hello world"), 3);
        assert_eq!(count_vowels("HELLO"), 2);
        assert_eq!(count_vowels(""), 0);
        assert_eq!(count_vowels("xyz"), 0);
        assert_eq!(count_vowels("AEIOUaeiou"), 10);
    }

    #[test]
    fn test_capitalize_first() {
        assert_eq!(capitalize_first("hello"), "Hello");
        assert_eq!(capitalize_first(""), "");
        assert_eq!(capitalize_first("straße"), "Straße");
        assert_eq!(capitalize_first("élan"), "Élan");
        assert_eq!(capitalize_first("日本語"), "日本語");
        assert_eq!(capitalize_first("already Capitalized"), "Already Capitalized");
    }

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome("Racecar"));
        assert!(is_palindrome(""));
        assert!(is_palindrome("a"));
        assert!(!is_palindrome("hello"));
        assert!(is_palindrome("NoonNOON"));
    }

    #[test]
    fn test_longest_word() {
        assert_eq!(longest_word("the quick brown fox"), "quick");
        assert_eq!(longest_word(""), "");
        assert_eq!(longest_word("a bb ccc dd"), "ccc");
        assert_eq!(longest_word("tie ties"), "ties");
        assert_eq!(longest_word("abcd wxyz"), "abcd");
    }

    #[test]
    fn test_safe_slice() {
        let s = "hello";
        assert_eq!(safe_slice(s, 0, 5), Some("hello"));
        assert_eq!(safe_slice(s, 1, 3), Some("el"));
        assert_eq!(safe_slice(s, 0, 10), None);
        assert_eq!(safe_slice(s, 3, 1), None);

        let unicode = "héllo";
        assert_eq!(safe_slice(unicode, 0, 1), Some("h"));
        assert_eq!(safe_slice(unicode, 1, 2), None);
        assert_eq!(safe_slice(unicode, 1, 3), Some("é"));
    }

    #[test]
    fn test_reverse_words() {
        assert_eq!(reverse_words("the quick brown fox"), "fox brown quick the");
        assert_eq!(reverse_words(""), "");
        assert_eq!(reverse_words("single"), "single");
        assert_eq!(reverse_words("the  quick   fox"), "fox quick the");
        assert_eq!(reverse_words("  padded  "), "padded");
    }

    #[test]
    fn test_byte_vs_char_len() {
        assert_eq!(byte_vs_char_len("hello"), (5, 5));
        assert_eq!(byte_vs_char_len(""), (0, 0));
        assert_eq!(byte_vs_char_len("héllo"), (6, 5));
        assert_eq!(byte_vs_char_len("日本語"), (9, 3));
    }

    #[test]
    fn test_append_exclamation() {
        let mut s = String::from("hello");
        append_exclamation(&mut s);
        assert_eq!(s, "hello!");

        let mut empty = String::new();
        append_exclamation(&mut empty);
        assert_eq!(empty, "!");
    }

    #[test]
    fn test_find_char_index() {
        assert_eq!(find_char_index("hello", 'l'), Some((2, 2)));
        assert_eq!(find_char_index("hello", 'z'), None);
        assert_eq!(find_char_index("", 'a'), None);
        assert_eq!(find_char_index("héxlo", 'x'), Some((3, 2)));
        assert_eq!(find_char_index("ab日", '日'), Some((2, 2)));
    }
}