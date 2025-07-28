#[cfg(test)]
mod tests {
    use former::Former;

    #[derive(Debug, PartialEq, Former)]
    #[debug]
    pub struct Simple<'a> {
        name: &'a str,
    }

    #[test]
    fn test_simple_lifetime() {
        // Just see if this compiles
        assert_eq!(1, 1);
    }
}