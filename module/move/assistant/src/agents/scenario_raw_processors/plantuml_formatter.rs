//!
//! Format scenario in PlantUML diagram.
//!

mod private
{
  use std::io;

  /// Format scenario in PlantUML diagram.
  pub fn plantuml_formatter
  (
    scenario : &ScenarioRaw,
    writer : impl io::Write,
  ) -> Result< (), io::Error >
  {
    todo!()
  }
}

crate::mod_interface!
{
  own use plantuml_formatter;
}