fn static_str() -> &'static str {
    "I have a static lifetime."
}

fn only_slice_str() -> &str {
    "I have a static lifetime."
}

fn static_str_error() -> &'static str {
    let s = String::from("I have a static lifetime.");
    &s // ERROR will not compile because `s` is dropped at the end of this function, so the reference will be dangling.
}

fn static_str_error_002() -> &str {
    let s = String::from("I have a static lifetime.");
    &s
}

fn outlive_function_by_borrowing(s: &str) -> &str {
    s
}

fn main() {
    println!("static_str: {}", static_str());
    // println!("static_str_error: {}", static_str_error());
    // println!("static_str_error_002: {}", static_str_error_002());
    println!("outlive_function_by_borrowing: {}", outlive_function_by_borrowing("I have a lifetime."));
}
