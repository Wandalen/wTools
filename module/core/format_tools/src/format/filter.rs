//!
//! Print data as table.
//!

/// Define a private namespace for all its items.
mod private
{

  // use crate::*;

  use std::
  {
    borrow::Cow,
  };

  /// Represents a line type in a table, either a header or a regular row.
  ///
  /// `LineType` is used to distinguish between different types of lines
  /// in a table structure, aiding in formatting and processing.
  ///
  #[ derive( Debug, Default, PartialEq, Eq, Copy, Clone ) ]
  pub enum LineType
  {
    /// Represents a regular row of data in the table.
    #[ default ]
    Regular,
    /// Represents a header line in the table.
    Header,
  }

  // = filters

  /// Filter passing all elements.
  #[ derive( Debug, Default, PartialEq, Clone, Copy ) ]
  pub struct All;

  /// Filter skipping all elements.
  #[ derive( Debug, Default, PartialEq, Clone, Copy ) ]
  pub struct None;

  // = FilterCol

  /// Filter columns of a table to print it only partially.
  pub trait FilterCol
  {
    /// Filter columns of a table to print it only partially.
    fn filter_col( &self, key : &str ) -> bool;
    /// Determine is arguments needed for the filter or it can give answer even without arguments. Useful for optimization.
    fn need_args( &self ) -> bool
    {
      true
    }
  }

  impl Default for &'static dyn FilterCol
  {
    #[ inline( always ) ]
    fn default() -> Self
    {
      All::col()
    }
  }

  impl All
  {
    /// Returns a reference to a static instance.
    pub fn col() -> & 'static dyn FilterCol
    {
      static INSTANCE : All = All;
      &INSTANCE
    }
  }

  impl FilterCol for All
  {
    #[ inline( always ) ]
    fn filter_col( &self, _key : &str ) -> bool
    {
      true
    }
    #[ inline( always ) ]
    fn need_args( &self ) -> bool
    {
      false
    }
  }

  impl None
  {
    /// Returns a reference to a static instance.
    pub fn col() -> & 'static dyn FilterCol
    {
      static INSTANCE : All = All;
      &INSTANCE
    }
  }

  impl FilterCol for None
  {
    #[ inline( always ) ]
    fn filter_col( &self, _key : &str ) -> bool
    {
      false
    }
    #[ inline( always ) ]
    fn need_args( &self ) -> bool
    {
      false
    }
  }

  impl< F : Fn( &str ) -> bool > FilterCol for F
  {
    #[ inline( always ) ]
    fn filter_col( &self, key : &str ) -> bool
    {
      self( key )
    }
  }

  // = FilterRow

  /// Filter columns of a table to print it only partially.
  pub trait FilterRow
  {
    /// Filter rows of a table to print it only partially.
    fn filter_row( &self, typ : LineType, irow : usize, row : &[ ( Cow< '_, str >, [ usize ; 2 ] ) ] ) -> bool;
    /// Determine is arguments needed for the filter or it can give answer even without arguments. Useful for optimization.
    fn need_args( &self ) -> bool
    {
      true
    }
  }

  impl Default for &'static dyn FilterRow
  {
    #[ inline( always ) ]
    fn default() -> Self
    {
      All::row()
    }
  }

  impl FilterRow for All
  {
    #[ inline( always ) ]
    fn filter_row( &self, _typ : LineType, _irow : usize, _row : &[ ( Cow< '_, str >, [ usize ; 2 ] ) ] ) -> bool
    {
      true
    }
    #[ inline( always ) ]
    fn need_args( &self ) -> bool
    {
      false
    }
  }

  impl All
  {
    /// Returns a reference to a static instance.
    pub fn row() -> & 'static dyn FilterRow
    {
      static INSTANCE : All = All;
      &INSTANCE
    }
  }

  impl FilterRow for None
  {
    #[ inline( always ) ]
    fn filter_row( &self, _typ : LineType, _irow : usize, _row : &[ ( Cow< '_, str >, [ usize ; 2 ] ) ] ) -> bool
    {
      false
    }
    #[ inline( always ) ]
    fn need_args( &self ) -> bool
    {
      false
    }
  }

  impl None
  {
    /// Returns a reference to a static instance.
    pub fn row() -> & 'static dyn FilterRow
    {
      static INSTANCE : None = None;
      &INSTANCE
    }
  }

  impl< F : Fn( LineType, usize, &[ ( Cow< '_, str >, [ usize ; 2 ] ) ] ) -> bool > FilterRow for F
  {
    #[ inline( always ) ]
    fn filter_row( &self, typ : LineType, irow : usize, row : &[ ( Cow< '_, str >, [ usize ; 2 ] ) ] ) -> bool
    {
      self( typ, irow, row )
    }
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
    LineType,
    FilterCol,
    FilterRow,
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
