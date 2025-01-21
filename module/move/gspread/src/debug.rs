mod private
{
}

use format_tools::
{
  Fields,
  TableWithFields,
};
use std::borrow::Cow;

pub mod row_wrapper;

crate::mod_interface!
{
  exposed use
  {
    row_wrapper::
    {
      RowWrapper
    }
  };
}
