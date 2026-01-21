//! Manual corner case verification test
//!
//! This test manually verifies corner cases that may not be explicitly covered
//! in the automated test suite. Each test documents the expected behavior.

use pth::path;
use std::path::PathBuf;

#[ test ]
fn verify_is_glob_edge_cases()
{
  // Empty string should not be glob
  assert_eq!( path::is_glob( "" ), false, "Empty string should not be glob" );

  // Single glob character should be glob
  assert_eq!( path::is_glob( "*" ), true, "Single asterisk should be glob" );

  // Unclosed bracket - implementation specific behavior
  // Current implementation: entering bracket mode returns true on ]
  let result = path::is_glob( "file[0-9" );
  println!( "Unclosed bracket 'file[0-9' => {} (implementation specific)", result );

  // Unclosed brace - implementation specific behavior
  let result = path::is_glob( "file{{a,b" );
  println!( "Unclosed brace 'file{{{{a,b' => {} (implementation specific)", result );

  // Closing bracket without opening - should not be glob
  assert_eq!( path::is_glob( "file].txt" ), false, "Closing bracket without opening should not be glob" );

  // Closing brace without opening - should not be glob
  assert_eq!( path::is_glob( "file}}.txt" ), false, "Closing brace without opening should not be glob" );

  // Double backslash before asterisk
  // On Unix: first backslash escapes second, asterisk is unescaped
  // On Windows: both are path separators
  let result = path::is_glob( "\\\\*.txt" );
  println!( "Double backslash before asterisk '\\\\*.txt' => {} (platform specific)", result );

  // Mixed escaped/unescaped
  assert_eq!( path::is_glob( "\\*file*.txt" ), true, "Second asterisk should make it glob" );
}

#[ test ]
fn verify_normalize_edge_cases()
{
  // Empty string should normalize to "."
  let result = path::normalize( "" );
  assert!( result == PathBuf::from( "." ), "Empty string should normalize to '.', got {:?}", result );

  // Multiple trailing slashes
  let result = path::normalize( "/a/b///" );
  println!( "Multiple trailing slashes '/a/b///' => {:?}", result );

  // Multiple consecutive slashes should collapse
  let result = path::normalize( "/a//b///c" );
  // Expected: /a/b/c or possibly with some slashes preserved
  println!( "Multiple consecutive slashes '/a//b///c' => {:?}", result );

  // Only slashes should become root
  let result = path::normalize( "///" );
  assert!( result == PathBuf::from( "/" ), "Only slashes should become '/', got {:?}", result );

  // Root with multiple dotdots
  let result = path::normalize( "/../.." );
  println!( "Root with multiple dotdots '/../..' => {:?} (preserves dotdots)", result );
}

#[ test ]
fn verify_iter_join_edge_cases()
{
  // Empty iterator
  let paths: Vec< PathBuf > = vec![];
  let result = path::iter_join( paths.iter().map( | p | p.as_path() ) );
  println!( "Empty iterator => {:?}", result );
  // Should be empty path or "."

  // All empty strings
  let paths = vec![ PathBuf::from( "" ), PathBuf::from( "" ), PathBuf::from( "" ) ];
  let result = path::iter_join( paths.iter().map( | p | p.as_path() ) );
  println!( "All empty strings => {:?}", result );

  // Multiple absolute paths (last wins)
  let paths = vec![ PathBuf::from( "/a" ), PathBuf::from( "/b" ), PathBuf::from( "/c" ) ];
  let result = path::iter_join( paths.iter().map( | p | p.as_path() ) );
  assert_eq!( result, PathBuf::from( "/c" ), "Last absolute should win, got {:?}", result );

  // Windows-style backslash should be converted
  let paths = vec![ PathBuf::from( "a\\b" ), PathBuf::from( "c" ) ];
  let result = path::iter_join( paths.iter().map( | p | p.as_path() ) );
  println!( "Windows backslash ['a\\b', 'c'] => {:?} (should normalize to /)", result );
}

#[ test ]
fn verify_exts_edge_cases()
{
  // Hidden file without extension
  let result = path::exts( ".bashrc" );
  println!( "Hidden file '.bashrc' => {:?} (could be ['bashrc'] or [])", result );

  // Hidden file with extension
  let result = path::exts( ".file.txt" );
  println!( "Hidden file with ext '.file.txt' => {:?}", result );

  // Dot at end
  let result = path::exts( "file." );
  println!( "Dot at end 'file.' => {:?}", result );

  // Only dots
  let result = path::exts( "..." );
  println!( "Only dots '...' => {:?}", result );

  // Path with no filename
  let result = path::exts( "/path/to/" );
  assert_eq!( result, Vec::< String >::new(), "Path with no filename should return empty vec" );

  // Root only
  let result = path::exts( "/" );
  assert_eq!( result, Vec::< String >::new(), "Root only should return empty vec" );
}

#[ test ]
fn verify_without_ext_edge_cases()
{
  // No extension
  let result = path::without_ext( "file" );
  println!( "No extension 'file' => {:?}", result );

  // Multiple extensions (removes last only)
  let result = path::without_ext( "file.tar.gz" );
  println!( "Multiple extensions 'file.tar.gz' => {:?} (should be Some('file.tar'))", result );

  // Hidden file
  let result = path::without_ext( ".bashrc" );
  println!( "Hidden file '.bashrc' => {:?}", result );

  // Dot at end
  let result = path::without_ext( "file." );
  println!( "Dot at end 'file.' => {:?}", result );

  // Path with no filename
  let result = path::without_ext( "/path/to/" );
  println!( "Path with no filename '/path/to/' => {:?} (expected None)", result );
}

#[ test ]
fn verify_change_ext_edge_cases()
{
  // No existing extension
  let result = path::change_ext( "file", "txt" );
  println!( "No existing extension 'file' + 'txt' => {:?}", result );

  // Empty extension removes extension (returns Some, not None)
  // Note: Documentation was fixed to reflect this behavior
  let result = path::change_ext( "file.txt", "" );
  assert_eq!( result, Some( PathBuf::from( "file" ) ), "Empty extension should remove extension" );

  // Extension with leading dot
  let result = path::change_ext( "file.txt", ".json" );
  println!( "Extension with dot 'file.txt' + '.json' => {:?}", result );

  // Multiple extensions (replaces last only)
  let result = path::change_ext( "file.tar.gz", "zip" );
  println!( "Multiple extensions 'file.tar.gz' + 'zip' => {:?}", result );
}

#[ test ]
fn verify_path_common_edge_cases()
{
  // Empty iterator
  let paths: Vec< &str > = vec![];
  let result = path::path_common( paths.into_iter() );
  println!( "Empty iterator => {:?} (expected None)", result );

  // Single path
  let paths = vec![ "/a/b/c" ];
  let result = path::path_common( paths.into_iter() );
  println!( "Single path ['/a/b/c'] => {:?}", result );

  // Identical paths
  let paths = vec![ "/a/b", "/a/b" ];
  let result = path::path_common( paths.into_iter() );
  println!( "Identical paths ['/a/b', '/a/b'] => {:?}", result );

  // No common path (different roots)
  let paths = vec![ "/a/b", "/c/d" ];
  let result = path::path_common( paths.into_iter() );
  println!( "No common path ['/a/b', '/c/d'] => {:?}", result );
}

#[ test ]
fn verify_rebase_edge_cases()
{
  // Root as file path
  let result = path::rebase( "/", "/new", None );
  println!( "Root as file path rebase('/', '/new', None) => {:?}", result );

  // Root as old path (strips everything)
  let result = path::rebase( "/home/user/file.txt", "/new", Some( "/" ) );
  println!( "Root as old path rebase('/home/user/file.txt', '/new', Some('/')) => {:?}", result );
}

#[ test ]
fn verify_path_relative_edge_cases()
{
  // Same path
  let result = path::path_relative( "/a/b", "/a/b" );
  println!( "Same path /a/b → /a/b => {:?} (expected '' or '.')", result );

  // Direct child
  let result = path::path_relative( "/a/b", "/a/b/c" );
  println!( "Direct child /a/b → /a/b/c => {:?} (expected 'c')", result );

  // Direct parent
  let result = path::path_relative( "/a/b/c", "/a/b" );
  println!( "Direct parent /a/b/c → /a/b => {:?} (expected '..')", result );

  // Root to child
  let result = path::path_relative( "/", "/a/b" );
  println!( "Root to child / → /a/b => {:?} (expected 'a/b')", result );

  // Child to root
  let result = path::path_relative( "/a/b", "/" );
  println!( "Child to root /a/b → / => {:?} (expected '../..')", result );

  // No common ancestor
  let result = path::path_relative( "/a/b", "/c/d" );
  println!( "No common ancestor /a/b → /c/d => {:?} (expected '../../c/d')", result );
}

#[ test ]
fn verify_ext_edge_cases()
{
  // No extension
  let result = path::ext( "file" );
  assert_eq!( result, "", "File without extension should return empty string" );

  // Multiple extensions (last only)
  let result = path::ext( "file.tar.gz" );
  assert_eq!( result, "gz", "Multiple extensions should return last only" );

  // Hidden file
  let result = path::ext( ".bashrc" );
  println!( "Hidden file '.bashrc' => '{}'", result );

  // Dot at end
  let result = path::ext( "file." );
  println!( "Dot at end 'file.' => '{}'", result );

  // Path with no filename
  let result = path::ext( "/path/to/" );
  assert_eq!( result, "", "Path with no filename should return empty string" );

  // Root only
  let result = path::ext( "/" );
  assert_eq!( result, "", "Root should return empty string" );
}

#[ test ]
fn verify_unique_folder_name_uniqueness()
{
  // Generate multiple names and ensure uniqueness
  let mut names = std::collections::HashSet::new();
  for _ in 0..100
  {
    let name = path::unique_folder_name().expect( "Should generate unique name" );
    assert!( names.insert( name ), "Names should be unique" );
 }

  println!( "✓ Generated 100 unique folder names" );
}
