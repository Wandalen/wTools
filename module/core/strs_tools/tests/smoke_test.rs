//! Smoke testing of the package.

#[test]
fn local_smoke_test() {
  ::test_tools::smoke_test_for_local_run();
}

#[test]
fn published_smoke_test() {
  ::test_tools::smoke_test_for_published_run();
}

#[test]
fn debug_strs_tools_semicolon_only() {
  let input = ";;";
  let splits: Vec<_> = strs_tools::string::split()
    .src(input)
    .delimeter(vec![";;"])
    .preserving_delimeters(true)
    .preserving_empty(false)
    .stripping(true)
    .form()
    .split()
    .collect();

  println!("DEBUG: Splits for ';;': {splits:?}");

  use strs_tools::string::split::{Split, SplitType};
  use std::borrow::Cow;

  let expected = vec![Split {
    string: Cow::Borrowed(";;"),
    typ: SplitType::Delimiter,
    start: 0,
    end: 2,
    was_quoted: false,
  }];
  assert_eq!(splits, expected);
}

#[test]
fn debug_strs_tools_trailing_semicolon_space() {
  let input = "cmd1 ;; ";
  let splits: Vec<_> = strs_tools::string::split()
    .src(input)
    .delimeter(vec![";;"])
    .preserving_delimeters(true)
    .preserving_empty(false)
    .stripping(true)
    .form()
    .split()
    .collect();

  println!("DEBUG: Splits for 'cmd1 ;; ': {splits:?}");

  use strs_tools::string::split::{Split, SplitType};
  use std::borrow::Cow;

  let expected = vec![
    Split {
      string: Cow::Borrowed("cmd1"),
      typ: SplitType::Delimeted,
      start: 0,
      end: 4,
      was_quoted: false,
    },
    Split {
      string: Cow::Borrowed(";;"),
      typ: SplitType::Delimiter,
      start: 5,
      end: 7,
      was_quoted: false,
    },
  ];
  assert_eq!(splits, expected);
}

#[test]
fn debug_strs_tools_only_semicolon() {
  let input = ";;";
  let splits: Vec<_> = strs_tools::string::split()
    .src(input)
    .delimeter(vec![";;"])
    .preserving_delimeters(true)
    .preserving_empty(false)
    .stripping(true)
    .form()
    .split()
    .collect();

  println!("DEBUG: Splits for ';;': {splits:?}");

  use strs_tools::string::split::{Split, SplitType};
  use std::borrow::Cow;

  let expected = vec![Split {
    string: Cow::Borrowed(";;"),
    typ: SplitType::Delimiter,
    start: 0,
    end: 2,
    was_quoted: false,
  }];
  assert_eq!(splits, expected);
}
