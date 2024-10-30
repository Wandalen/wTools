/// Internal namespace.

mod private
{
  use crate::*;

  use std::
  {
    // borrow::Cow,
    io,
    // path::{ Path, PathBuf },
  };

  /// A trait for joining path components into a `PathBuf`.
  pub trait PathJoined
  {
    /// Joins the path components into a single `PathBuf`.
    fn join_paths( self ) -> Result< PathBuf, io::Error >;
  }

  // Implementation for a tuple of length 1
  impl<'a, T1> PathJoined for ( T1, )
  where
    T1 : TryIntoCowPath<'a>,
  {
    fn join_paths( self ) -> Result< PathBuf, io::Error >
    {
      let ( p1, ) = self;
      let mut result = PathBuf::new();
      result.push( p1.try_into_cow_path()?.as_ref() );
      Ok( result )
    }
  }

  // Implementation for a tuple of length 2
  impl<'a, T1, T2> PathJoined for ( T1, T2 )
  where
    T1 : TryIntoCowPath<'a>,
    T2 : TryIntoCowPath<'a>,
  {
    fn join_paths( self ) -> Result< PathBuf, io::Error >
    {
      let ( p1, p2 ) = self;
      let mut result = PathBuf::new();
      result.push( p1.try_into_cow_path()?.as_ref() );
      result.push( p2.try_into_cow_path()?.as_ref() );
      Ok( result )
    }
  }

  // Implementation for a tuple of length 3
  impl<'a, T1, T2, T3> PathJoined for ( T1, T2, T3 )
  where
    T1 : TryIntoCowPath<'a>,
    T2 : TryIntoCowPath<'a>,
    T3 : TryIntoCowPath<'a>,
  {
    fn join_paths( self ) -> Result< PathBuf, io::Error >
    {
      let ( p1, p2, p3 ) = self;
      let mut result = PathBuf::new();
      result.push( p1.try_into_cow_path()?.as_ref() );
      result.push( p2.try_into_cow_path()?.as_ref() );
      result.push( p3.try_into_cow_path()?.as_ref() );
      Ok( result )
    }
  }

}

crate::mod_interface!
{
  exposed use PathJoined;
}