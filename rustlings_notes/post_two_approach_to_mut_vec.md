# Postmortem: Two Approaches to Mutating a `Vec` in Rust

**Subject:** Comparing move-based vs. in-place (`&mut`) mutation
**Context:** Derived from `add_five` example — appending a value to a `Vec<i32>`

---

## Summary

Two functionally equivalent ways to "add five to a vec" were compared:

1. **In-place mutation** — borrow the vec mutably, mutate through the reference.
2. **Move-and-return** — take ownership of the vec, mutate it, return it.

Both compile, both work, both are idiomatic in the right context. They differ in ownership semantics, caller ergonomics, and what they compose with. This memo documents the tradeoffs so the choice isn't made by habit.

---

## Approach 1: In-place mutation via `&mut`

```rust
fn add_five(v: &mut Vec<i32>) {
    v.push(5);
}

fn main() {
    let mut x = vec![4];
    add_five(&mut x);
    dbg!(x);
}
```

### What happens
`x` is never moved. `add_five` borrows it exclusively for the duration of the call, mutates the underlying buffer, and returns control (and access) back to `main` immediately after.

### Pros
- **No ownership transfer.** `x` remains valid and usable in `main` before, during (conceptually), and after the call — nothing needs to come back "out" of the function.
- **Cheap.** No move of the `Vec`'s internals (pointer/len/capacity triple), no reallocation, no re-binding.
- **Composable with multiple mutations.** You can call `add_five(&mut x)` several times, interleaved with other operations on `x`, without re-threading ownership through return values each time.
- **Matches the mental model of "mutate this thing"** rather than "transform this thing into a new thing." Reads naturally at the call site: `add_five(&mut x)` clearly signals "this will change."

### Cons
- **Requires `x` to be declared `mut`.** Every mutable-borrow chain propagates the `mut` requirement back to the original binding.
- **Borrow checker friction.** While the `&mut` borrow is alive, no other reference (mutable or immutable) to `x` may exist. This is rarely an issue in a simple case like this, but becomes real friction in larger call graphs (e.g. can't also hold `&x` elsewhere, can't call two functions each wanting `&mut x` "at once" without sequencing them).
- **Silent mutation.** Nothing in `add_five`'s *return type* signals that mutation happened — you have to read the signature (`&mut`) to know. Slightly less self-documenting than a function that returns a new value.

### When to reach for this
Default choice when: the caller wants to keep using the same variable, mutation is conceptually "this collection changes over time" (e.g., accumulator patterns, builder-style loops, buffers), or performance matters and you want to avoid unnecessary moves/copies.

---

## Approach 2: Move-and-return

```rust
fn add_five(mut v: Vec<i32>) -> Vec<i32> {
    v.push(5);
    v
}

fn main() {
    let x = vec![4];
    let x = add_five(x); // shadowing
    dbg!(x);
}
```

### What happens
`x` is moved into `add_five`. The original `x` binding in `main` is no longer valid after the call — ownership fully transferred. Inside the function, `v` (now owning the data) is mutated and then moved out again via the return value. The caller rebinds the result, typically via shadowing (`let x = add_five(x)`).

### Pros
- **No `mut` required on the original binding** — `x` itself in `main` never needs `mut`, since it's never mutated in place, only replaced by shadowing.
- **No borrow checker entanglement.** Since ownership is transferred outright, there's no concern about other live references to `x` — the function has sole, unambiguous access to the data for as long as it holds it.
- **Explicit data flow.** The function signature `Vec<i32> -> Vec<i32>` documents "this transforms a vec into a (possibly different) vec," which reads well in a pipeline/functional style: `let x = step1(x); let x = step2(x);`
- **Fits functional-style chaining** (`.map()`, builder patterns, iterator-adapter-like APIs) where each step consumes and produces a value.

### Cons
- **Real cost for large data.** Although `Vec`'s move is just a pointer/len/cap copy (cheap, no deep clone), the *ceremony* of moving ownership back and forth on every call adds friction, and in more complex ownership graphs can force clones where borrowing wouldn't have.
- **Original variable is consumed.** If you forget to rebind (`let x = add_five(x)`) and instead just call `add_five(x);`, `x` is now gone — a use-after-move compile error awaits you the next time you touch it.
- **Awkward for repeated mutation.** Calling this pattern in a loop or many times in sequence means constant re-shadowing (`let x = f(x); let x = g(x); ...`), which is noisier than just calling `&mut` methods back to back.
- **Doesn't compose with "I want to keep the old value too."** Once moved, the caller has no path back to the pre-mutation value unless it was cloned beforehand.

### When to reach for this
Good fit when: you're writing in a transformation/pipeline style (each function is a pure-ish step), you want to avoid `mut` propagating through the caller's bindings, or the function's job is conceptually "consume this and produce something new" rather than "adjust this in place."

---

## Side-by-side

| | In-place (`&mut`) | Move-and-return |
|---|---|---|
| Caller needs `mut` on original binding | Yes | No (needs shadowing instead) |
| Original variable usable after call | Yes, immediately | No — must use returned value |
| Ownership transferred | No | Yes |
| Borrow checker constraints during call | Exclusive borrow required | None (full ownership) |
| Idiomatic for | accumulate/mutate-over-time patterns | transform/pipeline patterns |
| Call-site noise for repeated calls | Low (`f(&mut x); g(&mut x);`) | Higher (`let x = f(x); let x = g(x);`) |
| Signature self-documents mutation | Not directly (must read `&mut`) | Yes (input/output types) |

---

## Recommendation

For something like `add_five` — a small, single-purpose mutation that the caller clearly wants reflected on the *same* variable — **in-place `&mut` mutation is the better default.** It's cheaper, avoids the shadowing dance, and matches how `Vec::push` and friends are used throughout the standard library.

Reach for **move-and-return** when the function is genuinely more "transform" than "mutate" — e.g., part of a chain of steps, or when you deliberately want to prevent the caller from holding onto stale references while a transformation happens.

Rule of thumb: **if the function's job is "change this," borrow it. If the function's job is "turn this into that," move it.**