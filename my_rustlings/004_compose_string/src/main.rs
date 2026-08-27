fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There are multiple ways to do this.

    // 1.
    // format!("{} world!", input)

    // 2.
    // input.to_string()  + " world!"

    // 3.
    // input.to_owned() + " world!"

    // 4.
    // String::from(input) + " world!"

    // 5.
    // input.to_string() + String::from(" world!")
    // dlaczego nie działa z tym drugim String::from(" world!")?
    // ^^^ Why not String + String? -> See: Obsidian/journal

    // 6.
    // zakręcone ale działa 
    input.to_string() + &String::from(" world!")

}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

}