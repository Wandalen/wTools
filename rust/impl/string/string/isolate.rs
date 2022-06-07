
pub( crate ) mod private
{
  use former::Former;

  ///
  /// Options for isolate.
  ///

  #[ allow( dead_code ) ]
  #[ derive( Debug ) ]
  #[ derive( Former ) ]
  #[ perform( fn isolate( &self ) -> ( &'a str, Option<&'a str>, &'a str ) ) ]
  pub struct IsolateOptions<'a>
  {
    #[ default( "" ) ]
    src : &'a str,
    #[ default( " " ) ]
    delimeter : &'a str,
    #[ default( true ) ]
    quote : bool,
    #[ default( true ) ]
    left : bool,
    #[ default( 1 ) ]
    times : u8, /* qqq : former do not form u16, u32, u64, usize */
    #[ default( true ) ]
    none : bool,
  }

  ///
  /// Adapter for IsolateOptions.
  ///

  pub trait IsolateOptionsAdapter< 'a >
  {
    /// Do isolate.
    fn isolate( &self ) -> ( &'a str, Option<&'a str>, &'a str )
    where
      Self : Sized,
    {
      ( "", None, "" )
    }
  }

  impl< 'a > IsolateOptionsAdapter< 'a > for IsolateOptions< 'a >
  {
    fn isolate( &self ) -> ( &'a str, Option<&'a str>, &'a str )
    {
      let result;
      if self.left
      {
        let parts : Vec<&str> = self.src.trim().splitn( self.times.into(), self.delimeter ).collect();
        if parts.len() == 1
        {
          result = ( parts[ 0 ], None, "" );
        }
        else
        {
          result = ( parts[ 0 ], Some( self.delimeter ), parts[ parts.len() - 1 ] )
        }
      }
      else
      {
        let parts : Vec<&str> = self.src.trim().rsplitn( self.times.into(), self.delimeter ).collect();
        if parts.len() == 1
        {
          result = ( "", None, parts[ 0 ] );
        }
        else
        {
          result = ( parts[ parts.len() - 1 ], Some( self.delimeter ), parts[ 0 ] )
        }
      }

      result
    }
  }

  ///
  /// Function to parse a string with command request.
  ///
  /// It produces former. To convert former into options and run algorithm of splitting call `perform()`.
  ///

  pub fn isolate_left<'a>() -> IsolateOptionsFormer<'a>
  {
    IsolateOptions::former()
  }
}

/// Owned namespace of the module.
pub mod protected
{
  use super::private as i;

  pub use i::IsolateOptions;
  pub use i::IsolateOptionsAdapter;
  pub use i::isolate_left;
}

pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  use super::private as i;

  pub use i::IsolateOptionsAdapter;
  pub use i::isolate_left;
}

/// Namespace of the module to include with `use module::*`.
pub mod prelude
{
  use super::private as i;

  pub use i::IsolateOptionsAdapter;
}
