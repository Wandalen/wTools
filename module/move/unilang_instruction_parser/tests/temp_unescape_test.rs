//! Temporary test for unescaping behavior of strs_tools.
use unilang_instruction_parser::*;
use strs_tools::string::split;

#[test]
fn temp_strs_tools_unescaping()
{
    let input = r#""a\\b\"c\'d\ne\tf""#; // Raw string literal to avoid Rust's unescaping
    let delimiters = vec![ " " ]; // Simple delimiter, not relevant for quoted string
    let split_iterator = split::SplitOptionsFormer::new(delimiters)
    .src( input )
    .preserving_delimeters( true )
    .quoting( true )
    .perform();

    let mut splits = split_iterator.collect::< Vec< _ > >();
    assert_eq!(splits.len(), 1);
    let s = &splits[0];
    assert_eq!(s.string, "a\\b\"c'd\ne\tf"); // Expected unescaped by strs_tools
}