//!
//! Print data as table.
//!

/// Internal namespace.
pub( crate ) mod private
{

  // use crate::*;

  /// Filter columns of a table to print it only partially.
  pub trait FilterCol
  {
    /// Filter columns of a table to print it only partially.
    fn filter_col( &self, key : &str ) -> bool;
  }

  /// Filter passing all elements.
  #[ derive( Debug, Default, PartialEq, Clone, Copy ) ]
  pub struct All;

  impl All
  {
    /// Returns a reference to a static instance of `Ordinary`.
    ///
    /// This method provides access to a single shared instance of `Ordinary`,
    /// ensuring efficient reuse of the classic table output format.
    pub fn instance() -> & 'static dyn FilterCol
    {
      static INSTANCE : All = All;
      &INSTANCE
    }
  }

  impl Default for &'static dyn FilterCol
  {
    #[ inline( always ) ]
    fn default() -> Self
    {
      All::instance()
    }
  }

  impl FilterCol for All
  {
    #[ inline( always ) ]
    fn filter_col( &self, _key : &str ) -> bool
    {
      true
    }
  }

  /// Filter skipping all elements.
  #[ derive( Debug, Default, PartialEq, Clone, Copy ) ]
  pub struct None;
  impl FilterCol for None
  {
    #[ inline( always ) ]
    fn filter_col( &self, _key : &str ) -> bool
    {
      false
    }
  }

  impl None
  {
    /// Returns a reference to a static instance of `Ordinary`.
    ///
    /// This method provides access to a single shared instance of `Ordinary`,
    /// ensuring efficient reuse of the classic table output format.
    pub fn instance() -> & 'static dyn FilterCol
    {
      static INSTANCE : All = All;
      &INSTANCE
    }
  }

  impl< F : Fn( &str ) -> bool > FilterCol for F
  {
    #[ inline( always ) ]
    fn filter_col( &self, key : &str ) -> bool
    // fn filter_col( &self, key : &str ) -> bool
    {
      self( key )
    }
  }

  //

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
    All,
    None,
  };

}

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  #[ doc( inline ) ]
  pub use exposed::*;

  #[ doc( inline ) ]
  pub use private::
  {
    FilterCol,
  };

}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  pub use super::super::filter;

  #[ doc( inline ) ]
  pub use private::
  {
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
}
