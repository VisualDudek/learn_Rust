//! Exercises: `String`, `&str`, and slices
//!
//! Drop this in as `src/lib.rs` in a fresh `cargo new --lib string_exercises`
//! and run `cargo test`. Each function has a `todo!()` body — replace it.
//! Tests are intentionally strict about edge cases (empty strings, non-ASCII
//! input) since that's where most `&str` bugs live.
//!
//! Difficulty roughly increases from 1 -> 10.

// ---------------------------------------------------------------------
// 1. Warm-up: borrowing vs. owning
// ---------------------------------------------------------------------
// Return the first whitespace-delimited word of `s` WITHOUT allocating.
// Think about why the signature takes and returns `&str`, not `String`.
pub fn f001_first_word(s: &str) -> &str {
    todo!()
}

// ---------------------------------------------------------------------
// 2. Iterating by `char`, not by byte
// ---------------------------------------------------------------------
// Count the vowels (a, e, i, o, u — case-insensitive, ASCII only for now).
pub fn f002_count_vowels(s: &str) -> usize {
    todo!()
}

// ---------------------------------------------------------------------
// 3. Building a new `String` correctly for non-ASCII input
// ---------------------------------------------------------------------
// Capitalize the first character and leave the rest unchanged.
// "straße" -> "Straße", "élan" -> "Élan", "" -> "".
// Byte-indexing s[0..1] will panic or corrupt data on multi-byte first
// chars — figure out why, and what API sidesteps it.
pub fn f003_capitalize_first(s: &str) -> String {
    todo!()
}

// ---------------------------------------------------------------------
// 4. Two-pointer / iterator reasoning over `char`s
// ---------------------------------------------------------------------
// Case-insensitive palindrome check, ignoring nothing else (no punctuation
// stripping needed for these tests).
pub fn f004_is_palindrome(s: &str) -> bool {
    todo!()
}

// ---------------------------------------------------------------------
// 5. Lifetime elision in a real signature
// ---------------------------------------------------------------------
// Return the longest whitespace-delimited word. On ties, return the first
// one encountered. Empty input -> "".
// Write out the full signature with an explicit lifetime, then figure out
// whether elision rules would let you drop it.
pub fn f005_longest_word<'a>(s: &'a str) -> &'a str {
    todo!()
}

// ---------------------------------------------------------------------
// 6. UTF-8 boundaries and panics
// ---------------------------------------------------------------------
// Slice `s[start..end]` (byte indices) but return `None` instead of
// panicking if the indices are out of bounds OR don't fall on a char
// boundary. Check `str::is_char_boundary`.
pub fn f006_safe_slice(s: &str, start: usize, end: usize) -> Option<&str> {
    todo!()
}

// ---------------------------------------------------------------------
// 7. Splitting, transforming, rejoining
// ---------------------------------------------------------------------
// Reverse the order of whitespace-delimited words, single-space-joined.
// "the quick  fox" -> "fox quick the" (collapse extra whitespace).
pub fn f007_reverse_words(s: &str) -> String {
    todo!()
}

// ---------------------------------------------------------------------
// 8. `len()` vs `chars().count()`
// ---------------------------------------------------------------------
// Return (byte_length, char_count). These differ for any non-ASCII input —
// know exactly why `len()` is O(1) and `chars().count()` is O(n).
pub fn f008_byte_vs_char_len(s: &str) -> (usize, usize) {
    todo!()
}

// ---------------------------------------------------------------------
// 9. Mutating a `String` in place
// ---------------------------------------------------------------------
// Append "!" to `s` in place (no new `String` returned). Think about why
// this needs `&mut String` and specifically cannot take `&mut str`.
pub fn f009_append_exclamation(s: &mut String) {
    todo!()
}

// ---------------------------------------------------------------------
// 10. Byte index vs. char index — the classic footgun
// ---------------------------------------------------------------------
// Find `target` in `s` and return (byte_index, char_index) of its first
// occurrence — i.e. its position measured in bytes from the start of the
// string, AND its position measured in chars from the start of the string.
// These diverge once any multi-byte char appears before the target.
pub fn f010_find_char_index(s: &str, target: char) -> Option<(usize, usize)> {
    todo!()
}

// =======================================================================
// Tests
// =======================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_word() {
        assert_eq!(f001_first_word("hello world"), "hello");
        assert_eq!(f001_first_word("hello"), "hello");
        assert_eq!(f001_first_word(""), "");
        assert_eq!(f001_first_word("  leading spaces"), "");
        assert_eq!(f001_first_word("trailing   "), "trailing");
    }

    #[test]
    fn test_count_vowels() {
        assert_eq!(f002_count_vowels("hello world"), 3);
        assert_eq!(f002_count_vowels("HELLO"), 2);
        assert_eq!(f002_count_vowels(""), 0);
        assert_eq!(f002_count_vowels("xyz"), 0);
        assert_eq!(f002_count_vowels("AEIOUaeiou"), 10);
    }

    #[test]
    fn test_capitalize_first() {
        assert_eq!(f003_capitalize_first("hello"), "Hello");
        assert_eq!(f003_capitalize_first(""), "");
        assert_eq!(f003_capitalize_first("straße"), "Straße");
        assert_eq!(f003_capitalize_first("élan"), "Élan");
        assert_eq!(f003_capitalize_first("日本語"), "日本語"); // no uppercase form, unchanged
        assert_eq!(f003_capitalize_first("already Capitalized"), "Already Capitalized");
    }

    #[test]
    fn test_is_palindrome() {
        assert!(f004_is_palindrome("Racecar"));
        assert!(f004_is_palindrome(""));
        assert!(f004_is_palindrome("a"));
        assert!(!f004_is_palindrome("hello"));
        assert!(f004_is_palindrome("NoonNOON"));
    }

    #[test]
    fn test_longest_word() {
        assert_eq!(f005_longest_word("the quick brown fox"), "quick");
        assert_eq!(f005_longest_word(""), "");
        assert_eq!(f005_longest_word("a bb ccc dd"), "ccc");
        assert_eq!(f005_longest_word("tie ties"), "ties"); // first-max on ties: "tie"(3) vs "ties"(4)
        assert_eq!(f005_longest_word("abcd wxyz"), "abcd"); // true tie, length 4 both -> first one
    }

    #[test]
    fn test_safe_slice() {
        let s = "hello";
        assert_eq!(f006_safe_slice(s, 0, 5), Some("hello"));
        assert_eq!(f006_safe_slice(s, 1, 3), Some("el"));
        assert_eq!(f006_safe_slice(s, 0, 10), None); // out of bounds
        assert_eq!(f006_safe_slice(s, 3, 1), None); // start > end

        let unicode = "héllo"; // 'é' is 2 bytes, so byte indices shift after it
        assert_eq!(f006_safe_slice(unicode, 0, 1), Some("h"));
        assert_eq!(f006_safe_slice(unicode, 1, 2), None); // lands mid-char-boundary of 'é'
        assert_eq!(f006_safe_slice(unicode, 1, 3), Some("é"));
    }

    #[test]
    fn test_reverse_words() {
        assert_eq!(f007_reverse_words("the quick brown fox"), "fox brown quick the");
        assert_eq!(f007_reverse_words(""), "");
        assert_eq!(f007_reverse_words("single"), "single");
        assert_eq!(f007_reverse_words("the  quick   fox"), "fox quick the");
        assert_eq!(f007_reverse_words("  padded  "), "padded");
    }

    #[test]
    fn test_byte_vs_char_len() {
        assert_eq!(f008_byte_vs_char_len("hello"), (5, 5));
        assert_eq!(f008_byte_vs_char_len(""), (0, 0));
        assert_eq!(f008_byte_vs_char_len("héllo"), (6, 5)); // 'é' = 2 bytes, 1 char
        assert_eq!(f008_byte_vs_char_len("日本語"), (9, 3)); // each char = 3 bytes
    }

    #[test]
    fn test_append_exclamation() {
        let mut s = String::from("hello");
        f009_append_exclamation(&mut s);
        assert_eq!(s, "hello!");

        let mut empty = String::new();
        f009_append_exclamation(&mut empty);
        assert_eq!(empty, "!");
    }

    #[test]
    fn test_find_char_index() {
        assert_eq!(f010_find_char_index("hello", 'l'), Some((2, 2)));
        assert_eq!(f010_find_char_index("hello", 'z'), None);
        assert_eq!(f010_find_char_index("", 'a'), None);

        // 'é' (2 bytes) precedes the target 'x', so byte_index != char_index
        assert_eq!(f010_find_char_index("héxlo", 'x'), Some((3, 2)));
        // target itself is multi-byte
        assert_eq!(f010_find_char_index("ab日", '日'), Some((2, 2)));
    }
}