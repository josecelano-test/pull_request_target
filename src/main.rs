// minor change 11

fn main() {
    println!("{}", greetings());
}

fn greetings() -> String {
    "Hello, world!".to_string()
}

#[cfg(test)]
mod tests {

    use super::greetings;
    use std::env;

    #[test]
    fn it_should_greet() {
        // Print all environment variables
        for (key, value) in env::vars() {
            println!("{key}: {value}");
        }

        assert_eq!(greetings(), "Hello, world!".to_string());
    }
}
