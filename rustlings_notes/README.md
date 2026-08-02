# Rustlings notes

- variables shadowing
```Rust

    let number = "T-H-R-E-E"; 

    let number: i32 = 3; // variable shadowing, use `let` to create a new variable with the same name but different type
    
    number = 3; // this will not work because number is immutable and different type
```

- constants are always immutable and must have a type annotation. `const N: u8 = 3;` 

---

- `let-if` v. useful, below analogy to Python

**ternary** operator
```python
x = 5
result = "even" if x % 2 == 0 else "odd"
```

**chaining**, but readability suffers
```python
result = "A" if score >= 90 else "B" if score >= 80 else "F"
```

---

single vs. double quote:
- `'x'` -> `char` and 
- `"x"` -> `&str`

---
TIP: Read `std` reference:
- [primitive type array](https://doc.rust-lang.org/stable/std/primitive.array.html)

---
How to not create a slice
```Rust
let a = [1, 2, 3, 4, 5];

let b = a[1..4]; // !!! see comment below
```
this is effectively saying:
- "I want to store this slice as a value"
- but a slice is not a concrete owned value like `i32` or `String`
- therefore this is not something Rust can place is a variable by itself

By contrast
```Rust
let b = &[1..4];
```
means: 
- take that slice
- create a reference to it
- store that reference in `b`

---
tuple elements can be accessed by index, e.g. `tuple.0`, `tuple.1`, etc. BUT do not use this, because it is not readable. Instead, destructure the tuple into named variables, e.g. `let (x, y) = tuple;` or use structs with named fields.

---
Different ways of creating Vec

```Rust
let a = [1, 2, 3];

let v = vec![1, 2, 3];

let v = Vec::from(a);
```

---
**TIP:** do not use `for` loop, use functional programming -> iterators

iterator multiline syntax
```Rust
// input: &[i32]
input
    .iter()
    .map(|element| {
        *element * 2
    })
    .collect()
```

---
- `println!()`
- `print!()` - print without newline

---
`for` loop consume 

```Rust
let v = vec![1,2,3];

for item in v {
    println!("{}", item); 
}
// does v still exist ?
println!("{:?}", v);  // NO
```

By contrast
```Rust
let v = vec![1,2,3];

for item in &v {
    //     ^^^ by reference
    println!("{}", item); 
}
// does v still exist ?
println!("{:?}", v);  // YES
```

Explain diff:
- `for i in v {}`
- `for i in &v {}`
- `for &i in v {}`

---
**Idiomatic Rust**: `&Vec<T>` Rust applies a coercion to `&[T]` automatically, so you can use `&Vec<T>` as a slice. But it is more idiomatic to use `&[T]` directly.

**It is the idiomatic Rust API style: take the least specific type that still does the job.**

for example, with `&[i32]` all of these can works:
```Rust
foo(&vec);
foo(&array);
foo(&slice);

// BUT in contrast:
fn bar(vec: &Vec<i32>) {
    // ...
}
// only works with Vec, not array or slice
```

Takeaway: instead of `&Vec<T>`, use `&[T]` in function signatures.

---
`Vec<T>` and `String` are owned types, and they are heap allocated. Their borrowed "views" are slices:
- `&[T]` for `Vec<T>`
- `&str` for `String`

---
Explain the diff:
- `fn foo(v : Vec<i32>)` - takes ownership of the vector, and the caller can no longer use it after calling `foo`
- `fn foo(v : &Vec<i32>)` - takes a reference to the vector
- `fn foo(v : &[i32])` - takes a slice of the vector, which is more general and idiomatic
- `fn foo(mut v : Vec<i32>)` - takes ownership of the vector, and the caller can no longer use it after calling `foo`, but `foo` can modify the vector
- `fn foo(mut v : &mut Vec<i32>)` - takes a mutable reference to the vector, and the caller can still use it after calling `foo`, but `foo` can modify the vector

IMPORTANT: Rust treats function parameters as immutable by default, so if you want to modify the parameter, you need to use `mut` keyword.
```Rust
fn foo(mut v : Vec<i32>) {
    // v is moved into foo, and foo can modify it
    v.push(4);
} // v is dropped here, and the memory is freed
```
**`mut` does not make the caller's variable mutable**

---
next move-semantics step: `mut vec: Vec<i32>` vs `vec: &mut Vec<i32>`
- `fn foo(mut vec: Vec<i32>) -> Vec<i32> {}`
    - ownership moves into the function
    - the function may modify it because the local binding is mutable
    - the caller loses access unless the value is returned

- `fn foo(vec: &mut Vec<i32>) {}`
    - ownership stays with the caller
    - the function may modify it through a mutable reference
    - the caller gets the same vector back automatically after the borrow ends
    - call by `foo(&mut vec)` instead of `foo(vec)`, The `&mut` at the call site means: “do not move ownership; instead, create a mutable reference and pass that reference into the function.”
    - the function do not need to return the vector, because the caller still owns it, and the function has modified it in place.

---
**cannot borrow var as mutable more than once at a time**
```Rust
let mut x = Vec::new();
let y = &mut x; // first mutable borrow
let z = &mut x; // second mutable borrow, this is not allowed
```

---
Here is the compact table.

| Function parameter | Meaning | Call syntax | Can function modify data? | Does caller keep ownership? |
|---|---|---|---|---|
| `v: Vec<i32>` | move ownership into function | `foo(vec)` | yes, if binding is `mut` | no |
| `v: &Vec<i32>` | borrow immutably | `foo(&vec)` | no | yes |
| `v: &[i32]` | borrow immutable slice | `foo(&vec)` or `foo(&array)` | no | yes |
| `v: &mut Vec<i32>` | borrow mutably | `foo(&mut vec)` | yes | yes |
| `mut v: Vec<i32>` | move ownership, mutable local binding | `foo(vec)` | yes | no |
| `mut v: &mut Vec<i32>` | mutable reference binding | `foo(&mut vec)` | yes, but `mut` is usually unnecessary | yes |

The most important distinction is this:

- `mut v: Vec<i32>`:
  - `mut` applies to the local variable binding
  - the function owns the vector

- `v: &mut Vec<i32>`:
  - `&mut` is part of the type
  - the function borrows the caller’s vector mutably

Example set:

```rust
fn a(v: Vec<i32>) {
    // owns v, but cannot push unless declared mut
}

fn b(mut v: Vec<i32>) {
    v.push(4); // OK
}

fn c(v: &Vec<i32>) {
    println!("{:?}", v); // read only
}

fn d(v: &[i32]) {
    println!("{:?}", v); // read only, more idiomatic
}

fn e(v: &mut Vec<i32>) {
    v.push(4); // OK
}
```

Calls:

```rust
let mut vec = vec![1, 2, 3];
let array = [1, 2, 3];

a(vec);          // moves ownership
b(vec![1, 2, 3]); // also moves ownership
c(&vec);         // immutable borrow
d(&vec);         // slice borrow from Vec
d(&array);       // slice borrow from array
e(&mut vec);     // mutable borrow
```

One subtle correction to your note in README.md: this form

```rust
fn foo(mut v: &mut Vec<i32>)
```

usually does not need `mut`. `v.push(4)` works fine with just:

```rust
fn foo(v: &mut Vec<i32>) {
    v.push(4);
}
```

because the vector data is mutable through the reference. Writing `mut v` only means you want to reassign the reference variable itself, for example:

```rust
fn foo(mut v: &mut Vec<i32>) {
    v = &mut Vec::new(); // reassign binding
}
```

That is rare.

A cleaner version of that line in your notes would be:

- `fn foo(v: &mut Vec<i32>)` - takes a mutable reference to the vector, caller keeps ownership, and `foo` can modify the vector in place

If you want, I can also explain the `for i in v` vs `for i in &v` vs `for &i in &v` part from your notes, because that connects directly to moves and references.
