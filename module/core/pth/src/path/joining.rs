mod private
{
  #[ allow( clippy::wildcard_imports ) ]
  use crate::*;
  use std::{ io, path::PathBuf };

  /// Joins path components into a `PathBuf`.
  ///
  /// This function leverages the `PathJoined` trait to join multiple path components into a single `PathBuf`.
  ///
  /// # Arguments
  ///
  /// * `paths` - A tuple of path components implementing the `PathJoined` trait.
  ///
  /// # Returns
  ///
  /// * `Ok(PathBuf)` - The joined path as a `PathBuf`.
  /// * `Err(io::Error)` - An error if any component fails to convert.
  /// # Errors
  /// qqq: doc
  pub fn join< Paths : PathJoined >( paths : Paths ) -> Result< PathBuf, io::Error >
  {
    paths.iter_join()
  }

  /// A trait for joining path components into a `PathBuf`.
  ///
  /// This trait provides a method to join multiple path components into a single `PathBuf`.
  /// It is implemented for tuples of varying lengths, allowing for flexible combinations of path components.
  /// Each component must implement the `TryIntoCowPath` trait, enabling conversion into a `Cow<Path>`.
  pub trait PathJoined
  {
    /// Joins the path components into a single `PathBuf`.
    ///
    /// # Returns
    ///
    /// * `Ok(PathBuf)` - The joined path as a `PathBuf`.
    /// * `Err(io::Error)` - An error if any component fails to convert.
    /// # Errors
    /// qqq: doc
    fn iter_join( self ) -> Result< PathBuf, io::Error >;
  }

  // // Implementation for an Iterator over items implementing TryIntoCowPath
  // impl< 'a, I, T > PathJoined for I
  // where
  //   I : Iterator< Item = T >,
  //   T : TryIntoCowPath< 'a >,
  // {
  //   fn iter_join( self ) -> Result< PathBuf, io::Error >
  //   {
  //     let mut result = PathBuf::new();
  //     for item in self
  //     {
  //       result.push( item.try_into_cow_path()?.as_ref() );
  //     }
  //     Ok( result )
  //   }
  // }

  // Implementation for a tuple of length 1
  impl< 'a, T1 > PathJoined for ( T1, )
  where
    T1 : TryIntoCowPath< 'a >,
  {
    #[ inline ]
    fn iter_join( self ) -> Result< PathBuf, io::Error >
    {
      let ( p1, ) = self;
      let mut result = PathBuf::new();
      result.push( p1.try_into_cow_path()?.as_ref() );
      Ok( result )
    }
  }

  // Implementation for a tuple of length 2
  impl< 'a, T1, T2 > PathJoined for ( T1, T2 )
  where
    T1 : TryIntoCowPath< 'a >,
    T2 : TryIntoCowPath< 'a >,
  {
    #[ inline ]
    fn iter_join( self ) -> Result< PathBuf, io::Error >
    {
      let ( p1, p2 ) = self;
      let mut result = PathBuf::new();
      result.push( p1.try_into_cow_path()?.as_ref() );
      result.push( p2.try_into_cow_path()?.as_ref() );
      Ok( result )
    }
  }

  // Implementation for a tuple of length 3
  impl< 'a, T1, T2, T3 > PathJoined for ( T1, T2, T3 )
  where
    T1 : TryIntoCowPath< 'a >,
    T2 : TryIntoCowPath< 'a >,
    T3 : TryIntoCowPath< 'a >,
  {
    #[ inline ]
    fn iter_join( self ) -> Result< PathBuf, io::Error >
    {
      let ( p1, p2, p3 ) = self;
      let mut result = PathBuf::new();
      result.push( p1.try_into_cow_path()?.as_ref() );
      result.push( p2.try_into_cow_path()?.as_ref() );
      result.push( p3.try_into_cow_path()?.as_ref() );
      Ok( result )
    }
  }

  // Implementation for a tuple of length 4
  impl< 'a, T1, T2, T3, T4 > PathJoined for ( T1, T2, T3, T4 )
  where
    T1 : TryIntoCowPath< 'a >,
    T2 : TryIntoCowPath< 'a >,
    T3 : TryIntoCowPath< 'a >,
    T4 : TryIntoCowPath< 'a >,
  {
    #[ inline ]
    fn iter_join( self ) -> Result< PathBuf, io::Error >
    {
      let ( p1, p2, p3, p4 ) = self;
      let mut result = PathBuf::new();
      result.push( p1.try_into_cow_path()?.as_ref() );
      result.push( p2.try_into_cow_path()?.as_ref() );
      result.push( p3.try_into_cow_path()?.as_ref() );
      result.push( p4.try_into_cow_path()?.as_ref() );
      Ok( result )
    }
  }

  // Implementation for a tuple of length 5
  impl< 'a, T1, T2, T3, T4, T5 > PathJoined for ( T1, T2, T3, T4, T5 )
  where
    T1 : TryIntoCowPath< 'a >,
    T2 : TryIntoCowPath< 'a >,
    T3 : TryIntoCowPath< 'a >,
    T4 : TryIntoCowPath< 'a >,
    T5 : TryIntoCowPath< 'a >,
  {
    #[ inline ]
    fn iter_join( self ) -> Result< PathBuf, io::Error >
    {
      let ( p1, p2, p3, p4, p5 ) = self;
      let mut result = PathBuf::new();
      result.push( p1.try_into_cow_path()?.as_ref() );
      result.push( p2.try_into_cow_path()?.as_ref() );
      result.push( p3.try_into_cow_path()?.as_ref() );
      result.push( p4.try_into_cow_path()?.as_ref() );
      result.push( p5.try_into_cow_path()?.as_ref() );
      Ok( result )
    }
  }

  // Implementation for slices
  impl< 'a, T > PathJoined for &'a [ T ]
  where
    T : TryIntoCowPath< 'a > + Clone,
  {
    #[ inline ]
    fn iter_join( self ) -> Result< PathBuf, io::Error >
    {
      let mut result = PathBuf::new();
      for item in self
      {
        result.push( item.clone().try_into_cow_path()?.as_ref() );
      }
      Ok( result )
    }
  }

  // Implementation for arrays
  impl< 'a, T, const N : usize > PathJoined for [ T; N ]
  where
    T : TryIntoCowPath< 'a > + Clone,
  {
    #[ inline ]
    fn iter_join( self ) -> Result< PathBuf, io::Error >
    {
      let mut result = PathBuf::new();
      for item in &self
      {
        result.push( item.clone().try_into_cow_path()?.as_ref() );
      }
      Ok( result )
    }
  }

}

crate::mod_interface!
{
  orphan use join;
  exposed use PathJoined;
}
