use mod_interface::mod_interface;
use error_tools::thiserror;

mod private
{
}

pub mod ser
{
  pub use serde::
  {
    Serialize,
    Deserialize,
  };
  pub use serde_json::
  {
    value::{ Value as JsonValue, Number as JsonNumber },
    error::Error,
    self
  };
  pub use serde_with::*;
}

crate::mod_interface!
{

  layer client;
  layer debug;
  layer commands;
  layer actions;
  layer secret;
  layer util;

  exposed use ::reflect_tools::
  {
    Fields,
    _IteratorTrait,
    IteratorTrait,
  };

}