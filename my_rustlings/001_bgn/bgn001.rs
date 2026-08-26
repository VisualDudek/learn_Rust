// Exercise: Copy semantics of `[i32; N]`
//
// GOAL: prove to yourself — via the compiler, not just reading — that an
// array of `i32` is `Copy`, so passing it "by value" into a function does
// NOT move it. The original binding stays valid and usable afterward.
//
// Fill in the TODOs. `cargo test` (or `rustc --test copy_semantics.rs && ./copy_semantics`)
// should pass once everything is correct.

/// Takes an array BY VALUE, doubles every element, and returns the new array.
/// Note the signature: `arr: [i32; 4]`, not `&[i32; 4]`. No borrowing here.
fn double_all(arr: [i32; 4]) -> [i32; 4] {
    // TODO: return a new array where every element of `arr` is doubled.
    // (Don't mutate `arr` in place — array indexing assignment would also
    // work, but a fresh `.map()` keeps this purely functional.)
    todo!()
}

/// TODO: Uncomment this function once `double_all` is implemented, and fill
/// in the two blanks. The KEY TAKEAWAY this function is meant to prove:
/// `a` is still valid to use AFTER `double_all(a)` was called, because
/// `[i32; 4]` implements `Copy` — the array was copied into `double_all`,
/// not moved out of `a`.
fn array_and_doubled() -> ([i32; 4], [i32; 4]) {
    let a = [10, 20, 30, 40];

    let doubled = double_all(a); // <- `a` copied in, not moved

    // TODO: return the tuple `(a, doubled)`.
    // If `a` had been moved above, this line would fail to compile with
    // "use of moved value: `a`". It won't fail here — that's the point.
    todo!()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double_all_computes_correctly() {
        let input = [1, 2, 3, 4];
        assert_eq!(double_all(input), [2, 4, 6, 8]);
    }

    #[test]
    fn original_array_still_usable_after_move_into_function() {
        // This test only compiles — let alone passes — if `a` was NOT
        // actually consumed by `double_all(a)` inside `array_and_doubled`.
        let (original, doubled) = array_and_doubled();
        assert_eq!(original, [10, 20, 30, 40]);
        assert_eq!(doubled, [20, 40, 60, 80]);
    }

    #[test]
    fn array_of_i32_implements_copy() {
        // A purely type-level assertion: this only compiles if [i32; 4]: Copy.
        fn assert_copy<T: Copy>() {}
        assert_copy::<[i32; 4]>();
    }

    #[test]
    fn copying_an_array_yields_independent_memory() {
        // Bonus proof: a "copy" isn't just "still readable", it's a fully
        // independent value — mutating one does not affect the other.
        let a = [1, 2, 3, 4];
        let mut b = a; // copy, not move — `a` remains valid below
        b[0] = 999;

        assert_eq!(a[0], 1); // untouched
        assert_eq!(b[0], 999);
    }
}

// ---------------------------------------------------------------------
// STRETCH GOAL (optional, do this after the above passes):
//
// Uncomment the block below. It mirrors `array_and_doubled` exactly, but
// with `String` (which is NOT `Copy`) instead of `i32`. It will fail to
// compile with E0382 "use of moved value: `a`" — because `String` owns a
// heap allocation, so the compiler can't silently duplicate it; a real
// move happens instead. This is the contrast case that proves the `i32`
// version above is special because of `Copy`, not because of anything
// about arrays or function-call syntax in general.
// ---------------------------------------------------------------------

// fn strings_and_shouted() -> ([String; 2], [String; 2]) {
//     fn shout_all(arr: [String; 2]) -> [String; 2] {
//         arr.map(|s| s.to_uppercase())
//     }
//
//     let a = [String::from("hi"), String::from("bye")];
//     let shouted = shout_all(a); // <- `a` is genuinely MOVED here
//     (a, shouted) // <- compile error: use of moved value: `a`
// }