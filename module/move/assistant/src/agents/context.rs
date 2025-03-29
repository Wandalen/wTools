//!
//! Context representation. Can be used as compile-time context and as a runtime-context.
//!
//! Represents a simplistic "filesystem" with directories and terminal objects.
//!

mod private
{
  use std::collections::HashMap;

  use crate::*;
  use agents::path::
  {
    Path,
    PATH_SEPARATOR,
  };

  /// Represents a directory in a simplistic in-memory "filesystem"
  /// with other directories and terminal objects.
  ///
  /// `T` is the type of terminal object.
  #[ derive( Debug, PartialEq, Clone, Default ) ]
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
      Self
      {
        map : HashMap::new()
      }
    }

    /// Add new entry to the directory.
    ///
    /// Returns `true` if entry was successfully added.
    /// Returns `false` if there is already and entry with such name.
    /// Old entry will not be overriden.
    pub fn add( &mut self, name : impl Into< String >, entry : ContextEntry< T > ) -> bool
    {
      let name = name.into();

      if self.map.contains_key( name.as_str() )
      {
        false
      }
      else
      {
        self.map.insert( name, entry );
        true
      }
    }

    /// Get an entry by its name. Returns `None` is there is no such entry.
    ///
    /// `name` must be a valid path item. Refer to `path::PATH_ITEM_REGEX_STR` for syntax.
    ///
    /// This method is useful for quickly getting an entry only by its name.
    /// For complex paths, where your object is located in several consecutives directories,
    /// you can use `Path` type and use method `ContextDir::get_by_path`.
    pub fn get( &self, name : impl AsRef< str > ) -> Option< &ContextEntry< T > >
    {
      self.map.get( name.as_ref() )
    }

    /// Get an entry by its path. Returns `None` is there is no such entry.
    ///
    /// This function accepts both relative and absolute paths and it will
    /// treat itself as the root.
    pub fn get_by_path( &self, path : &Path ) -> Option< &ContextEntry< T > >
    {
      let mut cur : Option< &ContextEntry< T > > = None;

      for component in path.components()
      {
        if component == PATH_SEPARATOR 
        {
          continue;
        }

        match cur
        {
          None =>
          {
            cur = self.get( component );
          },

          Some( entry ) =>
          {
            match entry
            {
              ContextEntry::Terminal( _ ) =>
              {
                return None;
              },

              ContextEntry::Dir( dir ) => 
              {
                cur = dir.get( component );
              }
            }
          }
        }

        if cur.is_none()
        {
          return None;
        }
      }

      cur
    }
  }

  /// Entry in a simplistic in-memory "filesystem": either a directory or a terminal object `T`.
  ///
  /// Notice, this struct does not store the name of the entry.
  #[ derive( Debug, PartialEq, Clone ) ]
  pub enum ContextEntry< T >
  {
    /// Directory in context.
    Dir( ContextDir< T > ),

    /// Terminal object.
    Terminal( T ),
  }

  impl< T > Into< ContextEntry< T > > for ContextDir< T >
  {
    fn into( self ) -> ContextEntry< T >
    {
      ContextEntry::Dir( self )
    }
  }
}

crate::mod_interface!
{
  own use
  {
    ContextDir,
    ContextEntry,
  };
}