//!
//! Format scenario in YAML format (pretty-printing).
//!

mod private
{
  use std::io;

  use crate::*;
  use agents::scenario_raw::ScenarioRaw;

  /// Pretty-print `ScenarioRaw` in YAML format.
  pub fn yaml_formatter
  (
    scenario : &ScenarioRaw,
    writer : &mut impl io::Write,
  ) -> Result< (), serde_yaml::Error >
  {
    serde_yaml::to_writer( writer, scenario )
  }
}

crate::mod_interface!
{
  own use yaml_formatter;
}