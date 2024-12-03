//!
//! Main module for agents framework.
//!

mod private {}

crate::mod_interface!
{

  layer path;
  layer context;
  layer scenario_raw;
  layer scenario_raw_processors;
  layer scenario_processed;

}