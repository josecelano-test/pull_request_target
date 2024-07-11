fn main() {
    println!("{}", greetings());
}

fn greetings() -> String {
    // minor change
    "Hello, world!".to_string()
}

#[cfg(test)]
mod tests {

    use super::greetings;

    #[test]
    fn it_should_greet() {
        assert_eq!(greetings(), "Hello, world!".to_string())
    }
}
