//! Minimal Reproducible Examples for Task 026: Empty Value Tokenization Failures
//!
//! Based on Task 026 specification, these tests should expose critical tokenization issues

use unilang_parser::*;

#[cfg(test)]
mod task_026_empty_value_mre {

    use super::*;

    #[test]
    fn mre_empty_quoted_value() {
        let parser = Parser::new(UnilangParserOptions::default());

        // T4.1: Empty Quoted Command Values - should parse but task claims it fails
        let result = parser.parse_single_instruction(r#".test content::"""#);

        assert!(result.is_ok(), "Empty quoted values should parse successfully: {result:?}");

        if let Ok(instruction) = result {
            assert_eq!(instruction.command_path_slices, vec!["test"]);
            assert_eq!(instruction.named_arguments.len(), 1);
            assert!(instruction.named_arguments.contains_key("content"));
            assert_eq!(instruction.named_arguments["content"][0].value, "");
        }
    }

    #[test]
    fn mre_whitespace_only_value() {
        let parser = Parser::new(UnilangParserOptions::default());

        // T4.2: Whitespace-Only Values - should preserve whitespace
        let result = parser.parse_single_instruction(r#".test content::"   ""#);

        assert!(result.is_ok(), "Whitespace-only values should be preserved: {result:?}");

        if let Ok(instruction) = result {
            assert_eq!(instruction.command_path_slices, vec!["test"]);
            assert_eq!(instruction.named_arguments.len(), 1);
            assert!(instruction.named_arguments.contains_key("content"));
            assert_eq!(instruction.named_arguments["content"][0].value, "   ");
        }
    }

    #[test]
    fn mre_escaped_quotes() {
        let parser = Parser::new(UnilangParserOptions::default());

        // T4.3: Escape Sequences - should handle escaped quotes
        let result = parser.parse_single_instruction(r#".test content::"test with \"quotes\"""#);

        assert!(result.is_ok(), "Escape sequences should be processed correctly: {result:?}");

        if let Ok(instruction) = result {
            assert_eq!(instruction.command_path_slices, vec!["test"]);
            assert_eq!(instruction.named_arguments.len(), 1);
            assert!(instruction.named_arguments.contains_key("content"));
            assert_eq!(instruction.named_arguments["content"][0].value, r#"test with "quotes""#);
        }
    }

    #[test]
    fn mre_multiple_empty_values() {
        let parser = Parser::new(UnilangParserOptions::default());

        // T4.4: Multiple Empty Values - should handle sequence correctly
        let result = parser.parse_multiple_instructions(r#".test1 content::"" ;; .test2 content::"""#);

        assert!(result.is_ok(), "Multiple empty values in sequence should work: {result:?}");

        if let Ok(instructions) = result {
            assert_eq!(instructions.len(), 2);

            // First instruction
            assert_eq!(instructions[0].command_path_slices, vec!["test1"]);
            assert_eq!(instructions[0].named_arguments["content"][0].value, "");

            // Second instruction
            assert_eq!(instructions[1].command_path_slices, vec!["test2"]);
            assert_eq!(instructions[1].named_arguments["content"][0].value, "");
        }
    }

    #[test]
    fn mre_mixed_empty_and_content() {
        let parser = Parser::new(UnilangParserOptions::default());

        // Test mixing empty and non-empty values
        let result = parser.parse_single_instruction(r#".test empty::"" full::"content" whitespace::"   ""#);

        assert!(result.is_ok(), "Mixed empty and non-empty values should work: {result:?}");

        if let Ok(instruction) = result {
            assert_eq!(instruction.command_path_slices, vec!["test"]);
            assert_eq!(instruction.named_arguments.len(), 3);

            assert_eq!(instruction.named_arguments["empty"][0].value, "");
            assert_eq!(instruction.named_arguments["full"][0].value, "content");
            assert_eq!(instruction.named_arguments["whitespace"][0].value, "   ");
        }
    }

    #[test]
    fn mre_complex_escape_sequences() {
        let parser = Parser::new(UnilangParserOptions::default());

        // Test various escape sequences
        let result = parser.parse_single_instruction(r#".test newline::"line1\nline2" tab::"col1\tcol2" quote::"say \"hello\"""#);

        assert!(result.is_ok(), "Complex escape sequences should work: {result:?}");

        if let Ok(instruction) = result {
            assert_eq!(instruction.command_path_slices, vec!["test"]);
            assert_eq!(instruction.named_arguments.len(), 3);

            assert_eq!(instruction.named_arguments["newline"][0].value, "line1\nline2");
            assert_eq!(instruction.named_arguments["tab"][0].value, "col1\tcol2");
            assert_eq!(instruction.named_arguments["quote"][0].value, r#"say "hello""#);
        }
    }

    #[test]
    fn mre_unicode_with_empty_values() {
        let parser = Parser::new(UnilangParserOptions::default());

        // Test Unicode content mixed with empty values
        let result = parser.parse_single_instruction(r#".test unicode::"🚀 test" empty::"" ascii::"hello""#);

        assert!(result.is_ok(), "Unicode with empty values should work: {result:?}");

        if let Ok(instruction) = result {
            assert_eq!(instruction.command_path_slices, vec!["test"]);
            assert_eq!(instruction.named_arguments.len(), 3);

            assert_eq!(instruction.named_arguments["unicode"][0].value, "🚀 test");
            assert_eq!(instruction.named_arguments["empty"][0].value, "");
            assert_eq!(instruction.named_arguments["ascii"][0].value, "hello");
        }
    }

    #[test]
    fn mre_edge_case_quoted_operators() {
        let parser = Parser::new(UnilangParserOptions::default());

        // Test quoted strings containing operators
        let result = parser.parse_single_instruction(r#".test operators::"key::value" empty::"" special::"a::b::c""#);

        assert!(result.is_ok(), "Quoted operators should be preserved: {result:?}");

        if let Ok(instruction) = result {
            assert_eq!(instruction.command_path_slices, vec!["test"]);
            assert_eq!(instruction.named_arguments.len(), 3);

            assert_eq!(instruction.named_arguments["operators"][0].value, "key::value");
            assert_eq!(instruction.named_arguments["empty"][0].value, "");
            assert_eq!(instruction.named_arguments["special"][0].value, "a::b::c");
        }
    }

}