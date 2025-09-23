//! Test for debugging `coord::1,1` parsing issue

use unilang_parser::*;

#[test]
fn test_coord_comma_parsing() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = ".region.buy_castle coord::1,1";
    let result = parser.parse_single_instruction(input);

    match result {
        Ok(instruction) => {
            println!("Parsed successfully!");
            println!("Command path: {:?}", instruction.command_path_slices);
            println!("Named arguments: {:?}", instruction.named_arguments);
            println!("Positional arguments: {:?}", instruction.positional_arguments);
        }
        Err(e) => {
            println!("Parse error: {e:?}");
            // Don't panic in this test - let it show the error for now
        }
    }
}

#[test]
fn test_simple_named_arg() {
    let parser = Parser::new(UnilangParserOptions::default());

    // Test with spaces (should work per existing tests)
    let input_with_spaces = "cmd coord :: value";
    let result = parser.parse_single_instruction(input_with_spaces);

    match result {
        Ok(instruction) => {
            println!("Simple named arg with spaces parsed successfully!");
            println!("Command path: {:?}", instruction.command_path_slices);
            println!("Named arguments: {:?}", instruction.named_arguments);
        }
        Err(e) => {
            println!("Parse error for simple named arg with spaces: {e:?}");
        }
    }

    // Test without spaces (what we're trying to fix)
    let input_no_spaces = "cmd a::b";
    let result = parser.parse_single_instruction(input_no_spaces);

    match result {
        Ok(instruction) => {
            println!("Simple named arg without spaces parsed successfully!");
            println!("Command path: {:?}", instruction.command_path_slices);
            println!("Named arguments: {:?}", instruction.named_arguments);
        }
        Err(e) => {
            println!("Parse error for simple named arg without spaces: {e:?}");
        }
    }
}