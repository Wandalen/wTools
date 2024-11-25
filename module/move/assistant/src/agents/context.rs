mod private
{
  use std::collections::HashMap;

  use crate::*;
  use agents::path::Path;

  /// Simplistic in-memory "filesystem". Represents the root of the filesystem.
  ///
  /// `T` is the type of terminal object.
  pub struct Context< T >
  {
    root : ContextDir< T >,
  }

  impl< T > Context< T >
  {
    /// Create an empty `Context`.
    pub fn new() -> Self
    {
      todo!()
    }

    /// Add new entry to the directory.
    ///
    /// Returns `true` if entry was successfully added.
    /// Returns `false` if there is already and entry with such name.
    /// Old entry will not be overriden.
    pub fn add( &mut self, name : impl Into< String >, entry : ContextEntry< T > ) -> bool
    {
      self.root.add( name, entry )
    }

    /// Get an entry by its name. Returns `None` is there is no such entry.
    ///
    /// This method is useful for quickly getting an entry only by its name.
    /// For complex paths, where your object is located in several consecutives directories,
    /// you can use `Path` type and use method `Context::get_by_path`.
    pub fn get( &self, name : impl AsRef< str > ) -> Option< &ContextEntry< T > >
    {
      todo!()
    }

    /// Get an entry by its path. Returns `None` is there is no such entry.
    ///
    /// This function can accept absolute `Path`s as `Context` represents the root of the
    /// filesystem.
    pub fn get_by_path( &self, path : Path ) -> Option< &ContextEntry< T > >
    {
      todo!()
    }
  }

  /// Represents a directory in `Context` with other directories and
  /// terminal objects.
  ///
  /// `T` is the type of terminal object.
  pub struct ContextDir< T >
  {
    /// Internal map of entry names and entries data (a directory or a terminal object).
    map : HashMap< String, ContextEntry< T > >,
  }

  impl< T > ContextDir< T >
  {
    /// Create an empty `ContextDir`.
    pub fn new() -> Self
    {
      todo!()
    }

    /// Add new entry to the directory.
    ///
    /// Returns `true` if entry was successfully added.
    /// Returns `false` if there is already and entry with such name.
    /// Old entry will not be overriden.
    pub fn add( &mut self, name : impl Into< String >, entry : ContextEntry< T > ) -> bool
    {
      todo!()
    }

    /// Get an entry by its name. Returns `None` is there is no such entry.
    ///
    /// This method is useful for quickly getting an entry only by its name.
    /// For complex paths, where your object is located in several consecutives directories,
    /// you can use `Path` type and use method `ContextDir::get_by_path`.
    pub fn get( &self, name : impl AsRef< str > ) -> Option< &ContextEntry< T > >
    {
      todo!()
    }

    /// Get an entry by its path. Returns `None` is there is no such entry.
    ///
    /// This function does not accept absolute `Path`, as `ContextDir` does not know
    /// whether it is root or not. For absolute `Path`s use `Context::get_by_path`.
    pub fn get_by_path( &self, path : Path ) -> Option< &ContextEntry< T > >
    {
      todo!()
    }
  }

  /// Entry in `Context`: either a directory or a terminal object `T`.
  ///
  /// Notice, this struct does not store the name of the entry.
  pub enum ContextEntry< T >
  {
    /// Directory in context.
    Dir( ContextDir< T > ),

    /// Terminal object.
    Terminal( T ),
  }

  impl< T > ContextEntry< T >
  {
    /// Make a terminal object `ContextEntry`.
    pub fn terminal( obj : T ) -> Self
    {
      Self::Terminal( obj )
    }
  }

  impl< T > Into< ContextEntry< T > > for ContextDir< T >
  {
    fn into( self ) -> ContextEntry< T >
    {
      ContextEntry::Dir( self )
    }
  }

  /*
  impl< T > Into< ContextEntry< T > > for T
  {
    fn into( self ) -> ContextEntry< T >
    {
      ContextEntry::Terminal( self )
    }
  }
  */
}

crate::mod_interface!
{

}