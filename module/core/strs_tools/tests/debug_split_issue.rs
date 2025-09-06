//! For debugging split issues.
// This file is for debugging purposes only and will be removed after the issue is resolved.

#[ cfg( all( feature = "string_split", feature = "std" ) ) ]
#[ test ]
fn debug_split_issue() {
  use strs_tools::string::split::{SplitOptionsFormer}; // Removed SplitType

  let input = r#"cmd name::"a\\\\b\\\"c\\\'d\\ne\\tf""#;
  let splitter = SplitOptionsFormer::new(vec!["::", " "])
    .src(input)
    .quoting(true)
    .quoting_prefixes(vec![r#"""#, r"'"])
    .quoting_postfixes(vec![r#"""#, r"'"])
    .perform();

  println!("Input: {input:?}");
  for item in splitter {
    println!("Split item: {item:?}");
  }
}
