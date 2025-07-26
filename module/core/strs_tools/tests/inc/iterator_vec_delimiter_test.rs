use strs_tools::string::split::{Split};

#[test]
fn test_split_with_vec_delimiter_iterator() {
  let input = "test string";
  let delimiters = vec![" "];
  let splits: Vec<Split<'_>> = strs_tools::split()
    .src(input)
    .delimeter(delimiters)
    .preserving_delimeters(false)
    .form()
    .into_iter()
    .collect();

  assert_eq!(splits.len(), 2);
  assert_eq!(splits[0].string, "test");
  assert_eq!(splits[1].string, "string");
}
