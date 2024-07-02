/// Internal namespace.
#[ allow( unused_imports ) ]
pub( crate ) mod private
{
  use crate::tool::*;
  use ::error_tools::protected::*;

}

crate::mod_interface!
{
  // #![ debug ]

  use ::error_tools;
  protected use ::error_tools::protected::*;

  // exposed use ErrWith;
  // exposed use ResultWithReport;
  // exposed use ::error_tools::Result;

}
