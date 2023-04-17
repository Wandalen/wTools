/// Internal namespace.
pub( crate ) mod private
{
  use crate::commands;

  ///
  /// Form CA commands grammar.
  ///

  pub fn grammar_form() -> Vec< wca::Command >
  {
    vec!
    [
      commands::each::each_command(),
      commands::package::info::info_command(),
      commands::package::publish::publish_command(),
      commands::end::end_command(),
    ]
  }

  ///
  /// Form CA commands executor.
  ///

  pub fn executor_form() -> std::collections::HashMap< String, wca::Routine >
  {
    std::collections::HashMap::from(
    [
      ( "each".to_owned(), wca::Routine::new_with_ctx( commands::each::each ) ),
      ( "crate.info".to_owned(), wca::Routine::new_with_ctx( commands::package::info::info ) ),
      ( "crate.publish".to_owned(), wca::Routine::new_with_ctx( commands::package::publish::publish ) ),
      ( "end".to_owned(), wca::Routine::new_with_ctx( commands::end::end ) ),
    ])
  }
}

//

crate::mod_interface!
{
  prelude use grammar_form;
  prelude use executor_form;
}
