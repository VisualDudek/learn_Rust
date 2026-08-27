// --- SOLUTION ---
#![allow(dead_code)]
// TODO: Fix the compiler error in this function.
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}

fn fill_vec_better(mut vec: Vec<i32>) -> Vec<i32> {
    // you do not need following line
    // let vec = vec;

    vec.push(88);

    vec
}

fn vec_from_slice(v: &[i32]) -> Vec<i32> {
    //               ^^^^^^ tutaj się dzieją ciekawe rzeczy przy przekazywaniu &Vec<i32>
    Vec::from(v)
}

fn vec_form_vec_ref(v: &Vec<i32>) -> Vec<i32> {
    v.to_vec()
    // OR
    // v.clone()

    // Vec::from(v) <-- dlaczego to nie działa ?
    /*
    1. nie ma impl<T> From<&Vec<T>> for Vec<T>
    i teraz elaborat dlaczego to jest świadoma decyzja projektowa w Rust, a nie przypadek

The key insight: &Vec<T> -> Vec<T> is Vec<T>'s own Clone, not a conversion

From/Into are meant to represent conversions between different representations — often different types, or at least a meaningfully different "shape" of data (&[T] → Vec<T> is "borrowed sequence of unknown length → owned growable buffer"; [T; N] → Vec<T> is "fixed-size stack array → heap buffer").

But &Vec<T> -> Vec<T> isn't converting between representations — it's asking for another owned copy of the exact same type. That's not a conversion, that's a duplication. And Rust already has a trait whose entire job is "duplicate this owned value": Clone.

If From<&Vec<T>> for Vec<T> existed, it would be functionally 100% identical to Clone::clone, just spelled differently. Adding it would be pure duplication of an existing, more semantically precise trait — bad API design, not a missing feature.

But wait — isn't &[T] -> Vec<T> the same kind of "borrowed -> owned" case?

Good instinct to question this, because it looks inconsistent at first. The difference is that &[T] and Vec<T> are genuinely different types (a slice is an unsized, non-owning view; Vec<T> is an owning, growable, heap-allocated buffer with capacity tracking). Going from one to the other really is a conversion — it involves allocating a new buffer and copying elements in. That's squarely From's territory.

&Vec<T> -> Vec<T>, by contrast, involves no type change at all — you strip a reference and get literally the same type back. Same job as Clone, every time, no exceptions.
     */

    // 2. v.clone()  // <-- this is the same as v.to_vec() ??? i tak i nie SEE: Obsidian/journal#FAQ

    // 3. v.clone() <-- jak tutaj działa autoderef?
    /*
Vec<T>: Clone is defined directly on Vec<T> itself (impl<T: Clone> Clone for Vec<T>), and .clone() is a method call — so ordinary autoref/autoderef kicks in: v: &Vec<i32>, the compiler tries (*v).clone() (i.e., Vec<i32>::clone), finds it, done. No trait-impl-matching-on-the-argument-type is involved at all; the receiver-lookup mechanism (method resolution) is doing the deref work, not From. 
     */
}

// brakuje mut in-place przez ref
fn vec_mut_inplace(v: &mut Vec<i32>) {
    v.push(88);
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }

    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec_better(vec0);
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }

    #[test]
    fn test_vec_from_slice() {
        let vec0 = vec![22, 44, 66];
        let vec1 = vec_from_slice(&vec0);
        assert_eq!(vec1, vec![22, 44, 66]);
    }

    #[test]
    fn test_vec_from_vec_ref() {
        let vec0 = vec![22, 44, 66];
        let vec1 = vec_form_vec_ref(&vec0);
        assert_eq!(vec1, vec![22, 44, 66]);
    }

    #[test]
    fn test_vec_mut_inplace() {
        let mut vec0 = vec![22, 44, 66];
        vec_mut_inplace(&mut vec0);
        assert_eq!(vec0, vec![22, 44, 66, 88]);
    }

}
