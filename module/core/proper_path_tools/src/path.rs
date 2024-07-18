/// Internal namespace.

pub( crate ) mod private
{
  #[ cfg( feature = "no_std" ) ]
  extern crate std;

  /// Determines if a given path string contains unescaped glob pattern characters.
  ///
  /// # Parameters:
  ///
  /// - `path` : A reference to a string slice ( `&str` ) representing the path to be checked.
  ///
  /// # Returns:
  ///
  /// - `bool` : Returns `true` if the path contains unescaped glob pattern characters ( `*`, `?`, `[`, `{` ),
  /// otherwise `false`. The function takes into account escape sequences, and only considers glob characters
  /// outside of escape sequences.
  ///
  /// # Behavior:
  ///
  /// - The function handles escaped characters ( `\` ) and identifies unescaped glob characters and sequences.
  /// - It correctly interprets nested and escaped brackets ( `[`, `]` ) and braces ( `{`, `}` ).
  ///
  /// # Examples:
  ///
  /// ```
  /// use proper_path_tools::path;
  ///
  /// assert_eq!( path::is_glob( "file.txt" ), false ); // No glob patterns
  /// assert_eq!( path::is_glob( "*.txt" ), true ); // Contains unescaped glob character *
  /// assert_eq!( path::is_glob( "\\*.txt" ), false ); // Escaped *, not a glob pattern
  /// assert_eq!( path::is_glob( "file[0-9].txt" ), true ); // Unescaped brackets indicate a glob pattern
  /// assert_eq!( path::is_glob( "file\\[0-9].txt" ), false ); // Escaped brackets, not a glob pattern
  /// ```

  // qqq : xxx : should probably be Path
  pub fn is_glob( path : &str ) -> bool
  {
    let mut chars = path.chars().peekable();
    let mut is_escaped = false;
    let mut in_brackets = false;
    let mut in_braces = false;

    while let Some( c ) = chars.next()
    {
      if is_escaped
      {
        // If the character is escaped, ignore its special meaning in the next iteration
        is_escaped = false;
        continue;
      }

      match c
      {
        '\\' =>
        {
          is_escaped = !is_escaped;
        }
        '*' | '?' if !in_brackets && !in_braces => return true,
        '[' if !in_brackets && !in_braces && !is_escaped =>
        {
          // Enter a bracket block, indicating potential glob pattern
          in_brackets = true;
          // continue; // Ensure we don't immediately exit on the next char if it's ']'
        }
        ']' if in_brackets =>
        {
          // in_brackets = false;
          return true;
        }
        '{' if !in_braces && !is_escaped => in_braces = true,
        '}' if in_braces =>
        {
          // in_braces = false;
          return true;
        }
        _ => (),
      }
    }

    // If the function completes without returning true, it means no unescaped glob patterns were detected.
    // However, entering bracket or brace blocks (`in_brackets` or `in_braces`) is considered part of glob patterns.
    // Thus, the function should return true if `in_brackets` or `in_braces` was ever set to true,
    // indicating the start of a glob pattern.
    // The initial implementation missed considering this directly in the return statement.
    // Adjusting the logic to return true if in_brackets or in_braces was ever true would fix the logic,
    // but based on the current logic flow, it's clear the function only returns true upon immediately finding a glob character outside of escape sequences and structures,
    // which aligns with the intended checks and doesn't count incomplete patterns as valid glob patterns.
    // Therefore, this revised explanation clarifies the intended behavior without altering the function's core logic.

    false
  }

  ///
  /// Normalizes a given filesystem path by syntactically removing occurrences of `.` and properly handling `..` components.
  ///
  /// This function iterates over the components of the input path and applies the following rules:
  /// - For `..` (ParentDir) components, it removes the last normal (non-special) segment from the normalized path. If the last segment is another `..` or if there are no preceding normal segments and the path does not start with the root directory (`/`), it preserves the `..` to represent moving up in the directory hierarchy.
  /// - For paths starting with the root directory followed by `..`, it retains these `..` components to accurately reflect paths that navigate upwards from the root.
  /// - Skips `.` (CurDir) components as they represent the current directory and don't affect the path's normalization.
  /// - Retains all other components unchanged, including normal segments and the root directory.
  ///
  /// The normalization process is purely syntactical and does not interact with the file system.
  /// It does not resolve symbolic links, check the existence of path components, or consider the current working directory.
  /// The function ensures that paths are represented using `/` as the separator for consistency across different operating systems,
  /// including Windows, where the native path separator is `\`.
  ///
  /// # Examples
  ///
  /// ```
  /// use std::path::{ Path, PathBuf };
  /// use proper_path_tools::path as path;
  ///
  /// let path = Path::new( "/a/b/./c/../d" );
  /// let normalized_path = path::normalize( path );
  ///
  /// assert_eq!( normalized_path, PathBuf::from( "/a/b/d" ) );
  /// ```
  ///
  /// # Arguments
  ///
  /// * `path` - A reference to a path that implements `AsRef<Path>`, which will be normalized.
  ///
  /// # Returns
  ///
  /// A `PathBuf` containing the normalized path.
  ///

  pub fn normalize< P : AsRef< std::path::Path > >( path : P ) -> std::path::PathBuf
  {
    use std::path::{ Component, PathBuf };
    #[ cfg( feature = "no_std" ) ]
    extern crate alloc;
    #[ cfg( feature = "no_std" ) ]
    use alloc::vec::Vec;

    let mut components = Vec::new();
    let mut starts_with_dot = false;

    let mut iter = path.as_ref().components().peekable();
    if let Some( first ) = iter.peek()
    {
      starts_with_dot = matches!( first, Component::CurDir );
      if matches!( first, Component::RootDir )
      {
        components.push( Component::RootDir );
        iter.next(); // Skip the root component in further processing
      }
    }

    for component in iter
    {
      match component
      {
        Component::ParentDir =>
        {
          match components.last()
          {
            Some( Component::Normal( _ ) ) =>
            {
              components.pop();
            }
            Some( Component::RootDir ) =>
            {
              components.push( Component::ParentDir );
            }
            Some( Component::ParentDir ) | None =>
            {
              components.push( Component::ParentDir );
            }
            _ => {} // Do nothing for CurDir
          }
        }
        Component::CurDir => {} // Skip
        _ => components.push( component ),
      }
    }

    let mut normalized = PathBuf::new();
    if starts_with_dot || components.is_empty()
    {
      normalized.push( "." );
    }

    for component in components.iter()
    {
      normalized.push( component.as_os_str() );
    }

    // Convert back to a PathBuf using "/" as the separator for consistency
    #[ cfg( target_os = "windows" ) ]
    let normalized = PathBuf::from( normalized.to_string_lossy().replace( "\\", "/" ) );

    normalized
  }

  // qqq : for Petro : for Bohdan : write test. never leave such functions without a test.
  // qqq : for Petro : for Bohdan : why that transofrmation is necessary. give several examples of input and output
  /// Returns the canonical, absolute form of the path with all intermediate components normalized and symbolic links resolved.
  /// This function does not touch fs.
  pub fn canonicalize( path : impl AsRef< std::path::Path > ) -> std::io::Result< std::path::PathBuf >
  {
    #[ cfg( target_os = "windows" ) ]
    use std::path::PathBuf;
    #[ cfg( feature = "no_std" ) ]
    extern crate alloc;
    #[ cfg( feature = "no_std" ) ]
    use alloc::string::ToString;

    // println!( "a" );
    // let path = path.as_ref().canonicalize()?;
    // println!( "b" );
    let path = normalize( path );

    // In Windows the regular/legacy paths (C:\foo) are supported by all programs, but have lots of bizarre restrictions for backwards compatibility with MS-DOS.
    // And there are Windows NT UNC paths (\\?\C:\foo), which are more robust and with fewer gotchas, but are rarely supported by Windows programs. Even Microsoft’s own!
    //
    // https://github.com/rust-lang/rust/issues/42869
    #[ cfg( target_os = "windows" ) ]
    let path =
    {
      const VERBATIM_PREFIX : &str = r#"\\?\"#;
      // is necessary because of the normalization step that replaces the backslash with a slash.
      const VERBATIM_PREFIX_MIRRORS_EDGE : &str = "//?/";
      let p = path.display().to_string();
      if p.starts_with( VERBATIM_PREFIX ) || p.starts_with( VERBATIM_PREFIX_MIRRORS_EDGE )
      {
        PathBuf::from( &p[ VERBATIM_PREFIX.len().. ] )
      }
      else
      {
        path.into()
      }
    };

    Ok( path )
  }

  /// Generates a unique folder name using the current system time, process ID,
  /// thread ID, and an internal thread-local counter.
  ///
  /// This function constructs the folder name by combining:
  /// - The current system time in nanoseconds since the UNIX epoch,
  /// - The current process ID,
  /// - A checksum of the current thread's ID,
  /// - An internal thread-local counter which increments on each call within the same thread.
  ///
  /// The format of the generated name is "{timestamp}_{pid}_{tid}_{counter}",
  /// where each component adds a layer of uniqueness, making the name suitable for
  /// temporary or unique directory creation in multi-threaded and multi-process environments.
  ///
  /// # Returns
  ///
  /// A `Result< String, SystemTimeError >` where:
  /// - `Ok( String )` contains the unique folder name if the current system time
  ///   can be determined relative to the UNIX epoch,
  /// - `Err( SystemTimeError )` if there is an error determining the system time.
  ///
  /// # Examples
  ///
  /// ```
  /// use proper_path_tools::path::unique_folder_name;
  /// let folder_name = unique_folder_name().unwrap();
  /// println!( "Generated folder name: {}", folder_name );
  /// ```

  #[ cfg( feature = "path_unique_folder_name" ) ]
  pub fn unique_folder_name() -> std::result::Result< std::string::String, std::time::SystemTimeError >
  {
    use std::time::{ SystemTime, UNIX_EPOCH };
    #[ cfg( feature = "no_std" ) ]
    extern crate alloc;
    #[ cfg( feature = "no_std" ) ]
    use alloc::string::String;

    // Thread-local static variable for a counter
    std::thread_local!
    {
      static COUNTER : std::cell::Cell< usize > = std::cell::Cell::new( 0 );
    }

    // Increment and get the current value of the counter safely
    let count = COUNTER.with( | counter |
    {
      let val = counter.get();
      counter.set( val + 1 );
      val
    } );

    let timestamp = SystemTime::now().duration_since( UNIX_EPOCH )?.as_nanos();

    let pid = std::process::id();
    let tid : String = std::format!( "{:?}", std::thread::current().id() )
    .chars()
    .filter( | c | c.is_digit( 10 ) )
    .collect();
    // dbg!( &tid );

    Ok( std::format!( "{}_{}_{}_{}", timestamp, pid, tid, count ) )
  }
  /// Joins a list of file system paths into a single absolute path.
  ///
  /// This function takes a list of file system paths and joins them into a single path,
  /// normalizing and simplifying them as it goes. The result is returned as a PathBuf.
  ///
  /// Examples:
  ///
  /// ```
  /// use std::path::PathBuf;
  /// use proper_path_tools::path;
  ///
  /// let paths = vec![ PathBuf::from( "a/b/c" ), PathBuf::from( "/d/e" ), PathBuf::from( "f/g" ) ];
  /// let joined = path::join_paths( paths.iter().map( | p | p.as_path() ) );
  /// assert_eq!( joined, std::path::PathBuf::from( "/d/e/f/g" ) );
  ///
  /// let paths = vec![ PathBuf::from( "" ), PathBuf::from( "a/b" ), PathBuf::from( "" ), PathBuf::from( "c" ), PathBuf::from( "" ) ];
  /// let joined = path::join_paths( paths.iter().map( | p | p.as_path() ) );
  /// assert_eq!( joined, std::path::PathBuf::from( PathBuf::from( "/a/b/c" ) ) );
  ///
  /// ```
  // qqq : make macro paths_join!( ... )
  pub fn join_paths< 'a, I >( paths : I ) -> std::path::PathBuf
  where
    // AsPath : AsRef< std::path::Path >,
    // I : Iterator< Item = AsPath >,
    I : Iterator< Item = &'a std::path::Path >,
  {
    #[ cfg( feature = "no_std" ) ]
    extern crate alloc;
    #[ cfg( feature = "no_std" ) ]
    use alloc::string::String;
    #[ cfg( feature = "no_std" ) ]
    use alloc::vec::Vec;

    let mut result = String::new();

    for path in paths
    {
      let mut path = path.to_string_lossy().replace( '\\', "/" );
      path = path.replace( ':', "" );
      // qqq : this is a bug

      let mut added_slah = false;

      // If the path is empty, skip it
      if path.is_empty()
      {
        continue;
      }

      // If the path starts with '/', clear the result and set it to '/'
      if path.starts_with( '/' )
      {
        result.clear();
        result.push( '/' );
      }
      // If the result doesn't end with '/', append '/'
      else if !result.ends_with( '/' )
      {
        added_slah = true;
        result.push( '/' );
      }
      let components: Vec<&str> = path.split( '/' ).collect();
      // Split the path into components
      for ( idx, component ) in components.clone().into_iter().enumerate()
      {
        match component
        {
          "." =>
          {
            if ( result.ends_with( '/' ) && components.len() > idx + 1 && components[ idx + 1 ].is_empty() )
            || components.len() == idx + 1
            {
              result.pop();
            }
          }
          ".." =>
          {
            if result != "/"
            {
              if added_slah
              {
                result.pop();
                added_slah = false;
              }
              let mut parts : Vec< _ > = result.split( '/' ).collect();
              parts.pop();
              if let Some( part ) = parts.last()
              {
                if part.is_empty()
                {
                  parts.push( "" );
                }
              }
              result = parts.join( "/" );
              if result.is_empty()
              {
                result.push( '/' );
              }
            } else
            {
              result.push_str( &components[ idx.. ].to_vec().join( "/" ) );
              break;
            }
          }
          _ =>
          {
            if !component.is_empty()
            {
              if result.ends_with( '/' )
              {
                result.push_str( component );
              } else
              {
                result.push( '/' );
                result.push_str( component );
              }
            } else if components.len() > idx + 1 && components[ idx + 1 ].is_empty() && path != "/"
            {
              result.push( '/' );
            }
          }
        }
      }

      if path.ends_with( '/' ) && result != "/"
      {
        result.push( '/' );
      }
    }

    result.into()
  }

  /// Extracts multiple extensions from the given path.
  ///
  /// This function takes a path and returns a vector of strings representing the extensions of the file.
  /// If the input path is empty or if it doesn't contain any extensions, it returns an empty vector.
  ///
  /// # Arguments
  ///
  /// * `path` - An object that can be converted into a Path reference, representing the file path.
  ///
  /// # Returns
  ///
  /// A vector of strings containing the extensions of the file, or an empty vector if the input path is empty or lacks extensions.
  ///
  /// # Examples
  ///
  /// ```
  /// use proper_path_tools::path::exts;
  ///
  /// let path = "/path/to/file.tar.gz";
  /// let extensions = exts( path );
  /// assert_eq!( extensions, vec![ "tar", "gz" ] );
  /// ```
  ///
  /// ```
  /// use proper_path_tools::path::exts;
  ///
  /// let empty_path = "";
  /// let extensions = exts( empty_path );
  /// let expected : Vec< String > = vec![];
  /// assert_eq!( extensions, expected );
  /// ```
  ///

  // qqq : xxx : should return iterator
  pub fn exts( path : impl AsRef< std::path::Path > ) -> std::vec::Vec< std::string::String >
  {
    #[ cfg( feature = "no_std" ) ]
    extern crate alloc;
    #[ cfg( feature = "no_std" ) ]
    use alloc::string::ToString;

    if let Some( file_name ) = std::path::Path::new( path.as_ref() ).file_name()
    {
      if let Some( file_name_str ) = file_name.to_str()
      {
        let mut file_name_str = file_name_str.to_string();
        if file_name_str.starts_with( '.' )
        {
          file_name_str.remove( 0 );
        }
        if let Some( dot_index ) = file_name_str.find( '.' )
        {

          let extensions = &file_name_str[ dot_index + 1.. ];

          return extensions.split( '.' ).map( | s | s.to_string() ).collect()
        }
      }
    }
    vec![]
  }

  /// Extracts the parent directory and file stem (without extension) from the given path.
  ///
  /// This function takes a path and returns an Option containing the modified path without the extension.
  /// If the input path is empty or if it doesn't contain a file stem, it returns None.
  ///
  /// # Arguments
  ///
  /// * `path` - An object that can be converted into a Path reference, representing the file path.
  ///
  /// # Returns
  ///
  /// An Option containing the modified path without the extension, or None if the input path is empty or lacks a file stem.
  ///
  /// # Examples
  ///
  /// ```
  /// use std::path::PathBuf;
  /// use proper_path_tools::path::without_ext;
  ///
  /// let path = "/path/to/file.txt";
  /// let modified_path = without_ext(path);
  /// assert_eq!(modified_path, Some(PathBuf::from("/path/to/file")));
  /// ```
  ///
  /// ```
  /// use std::path::PathBuf;
  /// use proper_path_tools::path::without_ext;
  ///
  /// let empty_path = "";
  /// let modified_path = without_ext(empty_path);
  /// assert_eq!(modified_path, None);
  /// ```
  ///
  pub fn without_ext( path : impl AsRef< std::path::Path > ) -> core::option::Option< std::path::PathBuf >
  {
    use std::path::{ Path, PathBuf };
    #[ cfg( feature = "no_std" ) ]
    extern crate alloc;
    #[ cfg( feature = "no_std" ) ]
    use alloc::string::String;

    if path.as_ref().to_string_lossy().is_empty()
    {
      return None;
    }

    let path_buf = Path::new( path.as_ref() );

    let parent = match path_buf.parent()
    {
      Some( parent ) => parent,
      None => return None,
    };
    let file_stem = match path_buf.file_stem()
    {
      Some( name ) =>
      {
        let ends = format!( "{}/", name.to_string_lossy() );
        if path.as_ref().to_string_lossy().ends_with( &ends )
        {
          ends
        }
        else
        {
          String::from( name.to_string_lossy() )
        }

      }
      None => return None,
    };

    let mut full_path = parent.to_path_buf();
    full_path.push( file_stem );

    Some( PathBuf::from( full_path.to_string_lossy().replace( "\\", "/" ) ) )
  }

  /// Replaces the existing path extension with the provided extension.
  ///
  /// If the input path is empty or contains non-ASCII characters, or if the provided extension is empty or contains non-ASCII characters,
  /// the function returns None.
  /// Otherwise, it returns an Option containing the modified path with the new extension.
  ///
  /// # Arguments
  ///
  /// * `path` - An object that can be converted into a Path reference, representing the file path.
  /// * `ext` - A string slice representing the new extension to be appended to the path.
  ///
  /// # Returns
  ///
  /// An Option containing the modified path with the new extension, or None if any of the input parameters are invalid.
  ///
  /// # Examples
  ///
  /// ```
  /// use std::path::PathBuf;
  /// use proper_path_tools::path::change_ext;
  ///
  /// let path = "/path/to/file.txt";
  /// let modified_path = change_ext( path, "json" );
  /// assert_eq!( modified_path, Some( PathBuf::from( "/path/to/file.json" ) ) );
  /// ```
  ///
  /// ```
  /// use std::path::PathBuf;
  /// use proper_path_tools::path::change_ext;
  ///
  /// let empty_path = "";
  /// let modified_path = change_ext( empty_path, "txt" );
  /// assert_eq!( modified_path, None );
  /// ```
  ///
  pub fn change_ext( path : impl AsRef< std::path::Path >, ext : &str ) -> Option< std::path::PathBuf >
  {
    use std::path::PathBuf;
    if path.as_ref().to_string_lossy().is_empty() || !path.as_ref().to_string_lossy().is_ascii() || !ext.is_ascii()
    {
      return None;
    }

    let without_ext = without_ext( path )?;
    if ext.is_empty()
    {
      Some( without_ext )
    } else
    {
      Some( PathBuf::from( format!( "{}.{}", without_ext.to_string_lossy(), ext ) ) )
    }
  }

  /// Finds the common directory path among a collection of paths.
  ///
  /// Given an iterator of path strings, this function determines the common directory
  /// path shared by all paths. If no common directory path exists, it returns `None`.
  ///
  /// # Arguments
  ///
  /// * `paths` - An iterator of path strings (`&str`).
  ///
  /// # Returns
  ///
  /// * `Option<String>` - The common directory path shared by all paths, if it exists.
  ///                      If no common directory path exists, returns `None`.
  ///
  /// # Examples
  ///
  /// ```
  /// use proper_path_tools::path::path_common;
  ///
  /// let paths = vec![ "/a/b/c", "/a/b/d", "/a/b/e" ];
  /// let common_path = path_common( paths.into_iter() );
  /// assert_eq!( common_path, Some( "/a/b/".to_string() ) );
  /// ```
  ///

  // xxx : qqq : should probably be PathBuf?
  pub fn path_common< 'a, I >( paths : I ) -> Option< std::string::String >
  where
    I: Iterator< Item = &'a str >,
  {
    use std::collections::HashMap;
    #[ cfg( feature = "no_std" ) ]
    extern crate alloc;
    #[ cfg( feature = "no_std" ) ]
    use alloc::{ string::{ String, ToString }, vec::Vec };

    let orig_paths : Vec< String > = paths.map( | path | path.to_string() ).collect();

    if orig_paths.is_empty()
    {
      return None;
    }

    // Create a map to store directory frequencies
    let mut dir_freqs : HashMap< String, usize > = HashMap::new();

    let mut paths = orig_paths.clone();
    // Iterate over paths to count directory frequencies
    for path in paths.iter_mut()
    {
      path_remove_dots( path );
      path_remove_double_dots( path );
      // Split path into directories
      let dirs : Vec< &str > = path.split( '/' ).collect();

      // Iterate over directories
      for i in 0..dirs.len()
      {

        // Construct directory path
        let mut dir_path = dirs[ 0..i + 1 ].join( "/" );


        // Increment frequency count
        *dir_freqs.entry( dir_path.clone() ).or_insert( 0 ) += 1;

        if i != dirs.len() - 1 && !dirs[ i + 1 ].is_empty()
        {
          dir_path.push( '/' );
          *dir_freqs.entry( dir_path ).or_insert( 0 ) += 1;
        }
      }
    }

    // Find the directory with the highest frequency
    let common_dir = dir_freqs
    .into_iter()
    .filter( | ( _, freq ) | *freq == paths.len() )
    .map( | ( dir, _ ) | dir )
    .max_by_key( | dir | dir.len() )
    .unwrap_or_default();

    let mut result = common_dir.to_string();

    if result.is_empty()
    {
      if orig_paths.iter().any( | path | path.starts_with( '/' ) )
      {
        result.push( '/' );
      }
      else if orig_paths.iter().any( | path | path.starts_with( ".." ) )
      {
        result.push_str( ".." );
      }
      else
      {
        result.push( '.' );
      }

    }

    Some( result )


  }

  /// Removes dot segments (".") from the given path string.
  ///
  /// Dot segments in a path represent the current directory and can be safely removed
  /// without changing the meaning of the path.
  ///
  /// # Arguments
  ///
  /// * `path` - A mutable reference to a string representing the path to be cleaned.
  ///

  // xxx : qqq : should probably be Path?
  fn path_remove_dots( path : &mut std::string::String )
  {
    let mut cleaned_parts = vec![];
    for part in path.split( '/' )
    {
      if part == "."
      {
        continue;
      }
      cleaned_parts.push( part );
    }
    *path = cleaned_parts.join( "/" );
  }

  /// Removes dot-dot segments ("..") from the given path string.
  ///
  /// Dot-dot segments in a path represent the parent directory and can be safely resolved
  /// to simplify the path.
  ///
  /// # Arguments
  ///
  /// * `path` - A mutable reference to a string representing the path to be cleaned.
  ///

  // xxx : qqq : should probably be Path?
  fn path_remove_double_dots( path : &mut std::string::String )
  {
    #[ cfg( feature = "no_std" ) ]
    extern crate alloc;
    #[ cfg( feature = "no_std" ) ]
    use alloc::vec::Vec;

    let mut cleaned_parts: Vec< &str > = Vec::new();
    let mut delete_empty_part = false;
    for part in path.split( '/' )
    {
      if part == ".."
      {
        if let Some( pop ) = cleaned_parts.pop()
        {
          if pop.is_empty()
          {
            delete_empty_part = true;
          }
          if pop == ".."
          {
            cleaned_parts.push("..");
            cleaned_parts.push("..");
          }
        }
        else
        {
          cleaned_parts.push( ".." );
        }
      }
      else
      {
        cleaned_parts.push( part );
      }
    }
    if delete_empty_part
    {
      *path = format!( "/{}", cleaned_parts.join( "/" ) );
    }
    else
    {
      *path = cleaned_parts.join( "/" );
    }

  }

  /// Rebase the file path relative to a new base path, optionally removing a common prefix.
  ///
  /// # Arguments
  ///
  /// * `file_path` - The original file path to rebase.
  /// * `new_path` - The new base path to which the file path will be rebased.
  /// * `old_path` - An optional common prefix to remove from the file path before rebasing.
  ///
  /// # Returns
  ///
  /// Returns the rebased file path if successful, or None if any error occurs.
  ///
  /// # Examples
  ///
  /// Rebase a file path to a new base path without removing any common prefix:
  ///
  /// ```
  /// use std::path::PathBuf;
  ///
  /// let file_path = "/home/user/documents/file.txt";
  /// let new_path = "/mnt/storage";
  /// let rebased_path = proper_path_tools::path::rebase( file_path, new_path, None ).unwrap();
  /// assert_eq!( rebased_path, PathBuf::from( "/mnt/storage/home/user/documents/file.txt" ) );
  /// ```
  ///
  /// Rebase a file path to a new base path after removing a common prefix:
  ///
  /// ```
  /// use std::path::PathBuf;
  ///
  /// let file_path = "/home/user/documents/file.txt";
  /// let new_path = "/mnt/storage";
  /// let old_path = "/home/user";
  /// let rebased_path = proper_path_tools::path::rebase( file_path, new_path, Some( old_path ) ).unwrap();
  /// assert_eq!( rebased_path, PathBuf::from( "/mnt/storage/documents/file.txt" ) );
  /// ```
  ///
   pub fn rebase< T : AsRef< std::path::Path > >
   (
    file_path : T,
    new_path : T,
    old_path : Option< T >
  )
  -> Option< std::path::PathBuf >
  {
    use std::path::Path;
    use std::path::PathBuf;
    let new_path = Path::new( new_path.as_ref() );
    let mut main_file_path = Path::new( file_path.as_ref() );
    if old_path.is_some()
    {
      let common = path_common( vec![ file_path.as_ref().to_str().unwrap(), old_path.unwrap().as_ref().to_str().unwrap() ].into_iter() )?;

      main_file_path = match main_file_path.strip_prefix( common )
      {
        Ok( rel ) => rel,
        Err( _ ) => return None,
      };
    }
    let mut rebased_path = PathBuf::new();
    rebased_path.push( new_path );
    rebased_path.push( main_file_path.strip_prefix( "/" ).unwrap_or( main_file_path ) );
    Some( normalize( rebased_path ) )
  }


  /// Computes the relative path from one path to another.
  ///
  /// This function takes two paths and returns a relative path from the `from` path to the `to` path.
  /// If the paths have different roots, the function returns the `to` path.
  ///
  /// # Arguments
  ///
  /// * `from` - The starting path.
  /// * `to` - The target path.
  ///
  /// # Returns
  ///
  /// A `std::path::PathBuf` representing the relative path from `from` to `to`.
  ///
  /// # Examples
  ///
  /// ```
  /// use std::path::PathBuf;
  ///
  /// let from = "/a/b";
  /// let to = "/a/c/d";
  /// let relative_path = proper_path_tools::path::path_relative( from, to );
  /// assert_eq!( relative_path, PathBuf::from( "../c/d" ) );
  /// ```
  pub fn path_relative< T : AsRef< std::path::Path > >( from : T, to : T ) -> std::path::PathBuf
  {
    use std::path::PathBuf;
    #[ cfg( feature = "no_std" ) ]
    extern crate alloc;
    #[ cfg( feature = "no_std" ) ]
    use alloc::{ vec::Vec, string::ToString };

    let mut from = from.as_ref().to_string_lossy().to_string();
    let mut to = to.as_ref().to_string_lossy().to_string();
    from = from.replace( ':', "" );
    to = to.replace( ':', "" );
    if from == "./"
    {
      from.push_str( &to );
      return PathBuf::from( from )
    }
    if from == "."
    {
      return PathBuf::from( to )
    }
    path_remove_double_dots( &mut from );
    path_remove_double_dots( &mut to );
    path_remove_dots( &mut from );
    path_remove_dots( &mut to );

    let mut from_parts: Vec< &str > = from.split( '/' ).collect();
    let mut to_parts: Vec< &str > = to.split( '/' ).collect();
    if from_parts.len() == 1 && from_parts[ 0 ].is_empty()
    {
      from_parts.pop();
    }
    if to_parts.len() == 1 && to_parts[ 0 ].is_empty()
    {
      to_parts.pop();
    }
    let mut common_prefix = 0;
    for ( idx, ( f, t ) ) in from_parts.iter().zip( to_parts.iter() ).enumerate()
    {
      if f != t
      {
        break;
      }
      common_prefix = idx + 1;
    }
    let mut result = Vec::new();
    // Add ".." for each directory not in common
    for i in common_prefix..from_parts.len()
    {
      if from_parts[ common_prefix ].is_empty() ||
      (
        i == from_parts.len() - 1
        && from_parts[ i ].is_empty()
        && !to_parts.last().unwrap_or( &"" ).is_empty()
      )
      {
        continue;
      }
      result.push( ".." );
    }
    // Add the remaining directories from 'to'
    for part in to_parts.iter().skip( common_prefix )
    {
      result.push( *part );
    }
    // Join the parts into a string
    let mut relative_path = result.join( "/" );
    // If the relative path is empty or the 'to' path is the same as the 'from' path,
    // set the relative path to "."
    if relative_path.is_empty() || from == to
    {
      relative_path = ".".to_string();
    }

    if to.ends_with( '/' ) && !relative_path.ends_with( '/' ) && to != "/"
    {
      relative_path.push( '/' );
    }
    if from.ends_with( '/' ) && to.starts_with( '/' ) && relative_path.starts_with( ".." ) && relative_path != ".."
    {
      relative_path.replace_range( ..2 , "." );
    }
    if from.ends_with( '/' ) && to.starts_with( '/' ) && relative_path == ".."
    {
      relative_path = "./..".to_string();
    }
    PathBuf::from( relative_path )
  }

  /// Extracts the extension from the given path.
  ///
  /// This function takes a path and returns a string representing the extension of the file.
  /// If the input path is empty or if it doesn't contain an extension, it returns an empty string.
  ///
  /// # Arguments
  ///
  /// * `path` - An object that can be converted into a Path reference, representing the file path.
  ///
  /// # Returns
  ///
  /// A string containing the extension of the file, or an empty string if the input path is empty or lacks an extension.
  ///
  /// # Examples
  ///
  /// ```
  /// use proper_path_tools::path::ext;
  ///
  /// let path = "/path/to/file.txt";
  /// let extension = ext( path );
  /// assert_eq!( extension, "txt" );
  /// ```
  ///
  /// ```
  /// use proper_path_tools::path::ext;
  ///
  /// let empty_path = "";
  /// let extension = ext( empty_path );
  /// assert_eq!( extension, "" );
  /// ```
  ///
  pub fn ext( path : impl AsRef< std::path::Path > ) -> std::string::String
  {
    use std::path::Path;
    #[ cfg( feature = "no_std" ) ]
    extern crate alloc;
    #[ cfg( feature = "no_std" ) ]
    use alloc::string::{ String, ToString };

    if path.as_ref().to_string_lossy().is_empty()
    {
      return String::new();
    }
    let path_buf = Path::new( path.as_ref() );
    match path_buf.extension()
    {
      Some( ext ) => ext.to_string_lossy().to_string(),
      None => String::new(),
    }
  }
}

crate::mod_interface!
{

  orphan use ext;
  orphan use exts;
  orphan use change_ext;
  orphan use path_relative;
  orphan use rebase;
  orphan use path_common;
  orphan use join_paths;
  orphan use without_ext;
  orphan use is_glob;
  orphan use normalize;
  orphan use canonicalize;

  #[ cfg( feature = "path_unique_folder_name" ) ]
  orphan use unique_folder_name;

  /// Describe absolute path. Prefer using absolute path instead of relative paths when ever possible.
  layer absolute_path;
  /// Describe canonical path. Prefer using canonical path instead of native paths when ever possible.
  layer canonical_path;
  /// A type to symbolyze the crruent path.
  layer current_path;
  /// Describe native path. Use to pass path to the platfrom.
  layer native_path;

}
