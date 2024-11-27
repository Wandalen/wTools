//!
//! Format scenario in YAML format (pretty-printing).
//!

mod private
{
  use std::io;

  /// Pretty-print `ScenarioRaw` in YAML format.
  pub fn yaml_formatter
  (
    scenario : &ScenarioRaw,
    writer : impl io::Write,
  ) -> Result< (), serde_yaml::Error >
  {
    todo!()
  }
}

crate::mod_interface!
{
  own use yaml_formatter;
}