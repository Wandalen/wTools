mod private
{
  ::meta_tools::mod_interface!
  {
    own use ::rgb::*;
    exposed use ::rgb::Rgba;
    // own use super::abs::*;

  }
}
pub use private::Rgba;
