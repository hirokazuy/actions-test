
pub fn get_greeting() -> &'static str {
    return "Hello World!"
}

#[cfg(test)]
mod tests {
    use super::get_greeting;

    #[test]
    fn get_helloworld() {
        assert_eq!(get_greeting(), "Hello World!")
    }
}