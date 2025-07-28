#[cfg(test)]
mod tests {
    use former::Former;

    #[derive(Debug, PartialEq, Former)]
    pub struct Simple<'a> {
        name: &'a str,
    }

    #[test]
    fn test_compiles() {
        // Empty test to verify compilation
        println!("If this compiles, the macro works");
    }
}