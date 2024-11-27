//!
//! `ScenarioRaw` processors: functions that work with `ScenarioRaw`.
//!
//! Currently only formatters are implemented.
//!

mod private {}

crate::mod_interface!
{
  layer yaml_formatter;
  layer plantuml_formatter;
}