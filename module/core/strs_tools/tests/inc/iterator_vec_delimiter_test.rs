#[cfg(all(feature = "string_split", feature = "std"))]
use strs_tools::string::split::{Split};

#[cfg(all(feature = "string_split", feature = "std"))]
#[ test ]
fn test_split_with_vec_delimiter_iterator() {
  let input = "test string";
  let delimiters = vec![" "];
  let splits: Vec<Split<'_>> = strs_tools::split()
    .src(input)
    .delimeters(&delimiters)
    .preserving_delimeters(false)
    .perform()
    .collect();

  assert_eq!(splits.len(), 2);
  assert_eq!(splits[0].string, "test");
  assert_eq!(splits[1].string, "string");
}
