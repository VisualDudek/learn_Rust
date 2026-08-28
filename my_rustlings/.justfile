# Default recipe to display help
default:
    @just --list

# run tests with cargo watch
[no-cd]
@w:
    @cargo watch -c -x "test -- --nocapture"

# Explain a Rust compiler error by number
e code:
    @rustc --explain E{{code}}
    