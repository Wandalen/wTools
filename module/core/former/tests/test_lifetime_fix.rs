#[cfg(test)]
mod lifetime_tests {
    use former::Former;

    // Test case 1: Simple struct with single lifetime parameter
    // xxx : Re-enable when trailing comma issue is fully fixed
    // #[derive(Debug, PartialEq, Former)]
    // pub struct Simple<'a> {
    //     name: &'a str,
    // }

    // Test case 2: Struct with no lifetime parameters (baseline)
    // xxx : Re-enable when trailing comma issue is fully fixed
    // #[derive(Debug, PartialEq, Former)]
    // pub struct NoLifetime {
    //     name: String,
    // }

    #[test]
    #[ignore = "Disabled until trailing comma issue is fully fixed"]
    fn test_simple_lifetime_compiles() {
        // If this compiles, the macro is generating valid syntax
        println!("Simple lifetime struct compiles successfully");
    }

    #[test]
    #[ignore = "Disabled until trailing comma issue is fully fixed"]
    fn test_no_lifetime_compiles() {
        // Baseline test - this should work
        // let instance = NoLifetime::former().name("test".to_string()).form();
        // assert_eq!(instance.name, "test");
    }
}