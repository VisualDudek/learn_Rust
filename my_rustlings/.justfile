# Default recipe to display help
default:
    @just --list

# run tests with cargo watch
[no-cd]
@w:
    @cargo watch -c -x "test -- --nocapture"
    