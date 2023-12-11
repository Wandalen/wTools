use super::*;

///
/// Form CA commands grammar.
///

pub fn grammar_form() -> Vec< wca::Command >
{
  vec!
  [
    smoke::smoke_command(),
    smoke::smoke_with_subject_command(),
  ]
}

///
/// Form CA commands executor.
///

pub fn executor_form() -> std::collections::HashMap< String, wca::Routine >
{
  std::collections::HashMap::from_iter
  ([
    ( "smoke".to_string(), wca::Routine::new( smoke::smoke ) ),
  ])
}
