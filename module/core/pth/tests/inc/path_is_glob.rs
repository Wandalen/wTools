#[allow(unused_imports)]
use super::*;

#[test]
fn path_with_no_glob_patterns() {
  assert!(!the_module::path::is_glob("file.txt"));
}

#[test]
fn path_with_unescaped_glob_star() {
  assert!(the_module::path::is_glob("*.txt"));
}

#[test]
fn path_with_escaped_glob_star() {
  assert!(!the_module::path::is_glob("\\*.txt"));
}

#[test]
fn path_with_unescaped_brackets() {
  assert!(the_module::path::is_glob("file[0-9].txt"));
}

#[test]
fn path_with_escaped_brackets() {
  assert!(!the_module::path::is_glob("file\\[0-9].txt"));
}

#[test]
fn path_with_unescaped_question_mark() {
  assert!(the_module::path::is_glob("file?.txt"));
}

#[test]
fn path_with_escaped_question_mark() {
  assert!(!the_module::path::is_glob("file\\?.txt"));
}

#[test]
fn path_with_unescaped_braces() {
  assert!(the_module::path::is_glob("file{a,b}.txt"));
}

#[test]
fn path_with_escaped_braces() {
  assert!(!the_module::path::is_glob("file\\{a,b}.txt"));
}

#[test]
fn path_with_mixed_escaped_and_unescaped_glob_characters() {
  assert!(!the_module::path::is_glob("file\\*.txt"));
  assert!(the_module::path::is_glob("file[0-9]\\*.txt"));
}

#[test]
fn path_with_nested_brackets() {
  assert!(the_module::path::is_glob("file[[0-9]].txt"));
}

#[test]
fn path_with_nested_escaped_brackets() {
  assert!(!the_module::path::is_glob("file\\[\\[0-9\\]\\].txt"));
}

#[test]
fn path_with_escaped_backslash_before_glob_characters() {
  assert!(!the_module::path::is_glob("file\\*.txt"));
}

#[test]
fn path_with_escaped_double_backslashes_before_glob_characters() {
  assert!(the_module::path::is_glob("file\\\\*.txt"));
}

#[test]
fn path_with_complex_mix_of_escaped_and_unescaped_glob_characters() {
  assert!(the_module::path::is_glob("file\\[0-9]*?.txt"));
}
