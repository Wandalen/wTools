// This file is for debugging purposes only and will be removed after the issue is resolved.

#[ test ]
fn debug_unescape_issue()
{
  use unilang_instruction_parser::item_adapter::unescape_string_with_errors;
  use unilang_instruction_parser::error::SourceLocation; // Removed ParseError as it's not used in success path

  let input = r#"a\\\\b\\\"c\\\'d\\ne\\tf"#;
  let expected = r#"a\\b\"c\'d\ne\tf"#;
  let location = SourceLocation::StrSpan { start: 0, end: input.len() };

  let result = unescape_string_with_errors( input, &location ).unwrap(); // Now unwrap directly to String

  assert_eq!( result, expected );
}
