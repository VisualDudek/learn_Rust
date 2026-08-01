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