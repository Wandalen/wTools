//! Tests for coordinate and named argument parsing edge cases

use unilang_parser::*;

#[test]
fn test_coordinate_with_comma_parsing() {
    let parser = Parser::new(UnilangParserOptions::default());
    let input = ".region.buy_castle coord::1,1";
    let result = parser.parse_single_instruction(input);

    assert!(result.is_ok(), "Failed to parse coordinate input: {:?}", result.err());
    let instruction = result.unwrap();

    assert_eq!(instruction.command_path_slices, vec!["region", "buy_castle"]);
    assert_eq!(instruction.named_arguments.len(), 1);
    assert!(instruction.named_arguments.contains_key("coord"));
    assert_eq!(instruction.named_arguments["coord"][0].value, "1,1");
}

#[test]
fn test_named_arg_with_and_without_spaces() {
    let parser = Parser::new(UnilangParserOptions::default());

    // Test with spaces around operator
    let input_with_spaces = "cmd coord :: value";
    let result = parser.parse_single_instruction(input_with_spaces);
    assert!(result.is_ok(), "Failed to parse spaced named arg: {:?}", result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd"]);
    assert_eq!(instruction.named_arguments["coord"][0].value, "value");

    // Test without spaces around operator
    let input_no_spaces = "cmd a::b";
    let result = parser.parse_single_instruction(input_no_spaces);
    assert!(result.is_ok(), "Failed to parse unspaced named arg: {:?}", result.err());
    let instruction = result.unwrap();
    assert_eq!(instruction.command_path_slices, vec!["cmd"]);
    assert_eq!(instruction.named_arguments["a"][0].value, "b");
}