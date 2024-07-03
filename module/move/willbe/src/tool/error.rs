/// Internal namespace.
#[ allow( unused_imports ) ]
pub( crate ) mod private
{
  use crate::tool::*;
  use ::error_tools::own::*;

}

crate::mod_interface!
{
  // #![ debug ]

  use ::error_tools;
  own use ::error_tools::own::*;

  // exposed use ErrWith;
  // exposed use ResultWithReport;
  // exposed use ::error_tools::Result;

}
