use super::*;
use the_module::{format_ident, ident};

#[test]
fn ident_maybe_raw_non_keyword() {
  let input = format_ident!("my_variable");
  let expected = format_ident!("my_variable");
  let got = ident::ident_maybe_raw(&input);
  assert_eq!(got, expected);
  assert_eq!(got.to_string(), "my_variable");
}

#[test]
fn ident_maybe_raw_keyword_fn() {
  let input = format_ident!("fn");
  let expected = format_ident!("r#fn");
  let got = ident::ident_maybe_raw(&input);
  assert_eq!(got, expected);
  assert_eq!(got.to_string(), "r#fn");
}

#[test]
fn ident_maybe_raw_keyword_struct() {
  let input = format_ident!("struct");
  let expected = format_ident!("r#struct");
  let got = ident::ident_maybe_raw(&input);
  assert_eq!(got, expected);
  assert_eq!(got.to_string(), "r#struct");
}

#[test]
fn ident_maybe_raw_keyword_break() {
  let input = format_ident!("break");
  let expected = format_ident!("r#break");
  let got = ident::ident_maybe_raw(&input);
  assert_eq!(got, expected);
  assert_eq!(got.to_string(), "r#break");
}

#[test]
fn ident_maybe_raw_non_keyword_but_looks_like() {
  // Ensure it only checks the exact string, not variations
  let input = format_ident!("break_point");
  let expected = format_ident!("break_point");
  let got = ident::ident_maybe_raw(&input);
  assert_eq!(got, expected);
  assert_eq!(got.to_string(), "break_point");
}
