//!
//! Nice print.
//!

/// Internal namespace.
pub( crate ) mod private
{

  use crate::*;
  // use std::
  // {
  //   borrow::Cow,
  //   collections::HashMap,
  //   collections::BTreeMap,
  // };
  // use core::
  // {
  //   fmt,
  //   borrow::Borrow,
  // };
  // use former::Former;

  // xxx : documentation and test
  pub fn size< S : AsRef< str > >( src : S ) -> [ usize; 2 ]
  {
    let text = src.as_ref();
    let mut height = 0;
    let mut width = 0;

    for line in text.lines()
    {
      height += 1;
      let line_length = line.len();
      if line_length > width
      {
        width = line_length;
      }
    }

    [ width, height ]
  }

}

#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
  #[ doc( inline ) ]
  pub use orphan::*;

  #[ doc( inline ) ]
  pub use private::
  {
    size,
  };

}

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  #[ doc( inline ) ]
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  pub use super::super::string;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
}
