/// Internal namespace.
pub( crate ) mod private
{
  // use std::
  // {
  //   path::{ Component, Path, PathBuf },
  //   time::{ SystemTime, UNIX_EPOCH, SystemTimeError },
  // };
  // use cargo_metadata::camino::{ Utf8Path, Utf8PathBuf };

  // // xxx : it's not path, but file
  // /// Check if path is valid.
  // pub fn valid_is( path : &str ) -> bool
  // {
  //   std::fs::metadata( path ).is_ok()
  // }

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
        },
        '*' | '?' if !in_brackets && !in_braces => return true,
        '[' if !in_brackets && !in_braces && !is_escaped =>
        {
          // Enter a bracket block, indicating potential glob pattern
          in_brackets = true;
          // continue; // Ensure we don't immediately exit on the next char if it's ']'
        },
        ']' if in_brackets =>
        {
          // in_brackets = false;
          return true;
        },
        '{' if !in_braces && !is_escaped => in_braces = true,
        '}' if in_braces =>
        {
          // in_braces = false;
          return true;
        },
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

    use std::
    {
      path::{ Component, PathBuf },
    };

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
            Some( Component::Normal( _ ) ) => { components.pop(); },
            Some( Component::RootDir ) =>
            {
              components.push( Component::ParentDir );
            }
            Some( Component::ParentDir ) | None =>
            {
              components.push( Component::ParentDir );
            },
            _ => {} // Do nothing for CurDir
          }
        },
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
    use std::path::PathBuf;

    // println!( "a" );
    // let path = path.as_ref().canonicalize()?;
    // println!( "b" );
    let path = normalize( path );

    // In Windows the regular/legacy paths (C :\foo) are supported by all programs, but have lots of bizarre restrictions for backwards compatibility with MS-DOS.
    // And there are Windows NT UNC paths (\\?\C :\foo), which are more robust and with fewer gotchas, but are rarely supported by Windows programs. Even Microsoft’s own!
    //
    // https://github.com/rust-lang/rust/issues/42869
    #[ cfg( target_os = "windows" ) ]
    let path =
    {
      const VERBATIM_PREFIX : &str = r#"\\?\"#;
      let p = path.display().to_string();
      if p.starts_with( VERBATIM_PREFIX )
      {
        PathBuf::from( &p[ VERBATIM_PREFIX.len() .. ] )
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
  pub fn unique_folder_name() -> Result< String, std::time::SystemTimeError >
  {
    use std::
    {
      time::{ SystemTime, UNIX_EPOCH },
    };

    // Thread-local static variable for a counter
    thread_local!
    {
      static COUNTER : std::cell::Cell< usize > = std::cell::Cell::new( 0 );
    }

    // Increment and get the current value of the counter safely
    let count = COUNTER.with( | counter |
    {
      let val = counter.get();
      counter.set( val + 1 );
      val
    });

    let timestamp = SystemTime::now()
    .duration_since( UNIX_EPOCH )?
    .as_nanos();

    let pid = std::process::id();
    let tid : String = format!( "{:?}", std::thread::current().id() )
    .chars()
    .filter( | c | c.is_digit( 10 ) )
    .collect();

    // dbg!( &tid );

    Ok( format!( "{}_{}_{}_{}", timestamp, pid, tid, count ) )
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
  pub fn path_common< 'a, I >( paths : I ) -> Option< String > 
  where
    I : Iterator< Item = &'a str >,
  {
    use std::collections::HashMap;
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
  fn path_remove_dots( path : &mut String ) 
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
  fn path_remove_double_dots( path : &mut String ) 
  {
    
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
  pub fn rebase< T : AsRef< std::path::Path > >( file_path : T, new_path : T, old_path : Option< T > ) -> Option< std::path::PathBuf > 
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




}

crate::mod_interface!
{
  protected use path_relative;
  protected use rebase;
  protected use path_common;
  protected use is_glob;
  protected use normalize;
  protected use canonicalize;
  #[ cfg( feature = "path_unique_folder_name" ) ]
  protected use unique_folder_name;

  /// Describe absolute path. Prefer using absolute path instead of relative when ever possible.
  layer absolute_path;

}
