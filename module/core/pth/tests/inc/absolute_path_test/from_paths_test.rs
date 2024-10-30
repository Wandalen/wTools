use super::*;

// xxx : make it working

#[ test ]
fn test_from_paths_single_absolute_segment()
{
  use the_module::AbsolutePath;
  use std::convert::TryFrom;

  let segments = vec![ "/single" ];
  let got = AbsolutePath::from_iter( segments.iter().map( |s| *s ) ).unwrap();
  let exp = AbsolutePath::try_from( "/single" ).unwrap();

  assert_eq!( got, exp );
}

#[ test ]
fn test_from_paths_multiple_segments()
{
  use the_module::AbsolutePath;
  use std::convert::TryFrom;

  let segments = vec![ "/path", "to", "file" ];
  let got = AbsolutePath::from_iter( segments.iter().map( |s| *s ) ).unwrap();
  let exp = AbsolutePath::try_from( "/path/to/file" ).unwrap();

  assert_eq!( got, exp );
}

#[ test ]
fn test_from_paths_empty_segments()
{
  use the_module::AbsolutePath;

  let segments : Vec< &str > = vec![];
  let result = AbsolutePath::from_iter( segments.iter().map( | s | *s ) );

  assert!( result.is_err(), "Expected an error for empty segments" );
}

#[ test ]
fn test_from_paths_with_dot_segments()
{
  use the_module::AbsolutePath;
  use std::convert::TryFrom;

  let segments = vec![ "/path", ".", "to", "file" ];
  let got = AbsolutePath::from_iter( segments.iter().map( |s| *s ) ).unwrap();
  let exp = AbsolutePath::try_from( "/path/to/file" ).unwrap();

  assert_eq!( got, exp );
}

#[ test ]
fn test_from_paths_with_dotdot_segments()
{
  use the_module::AbsolutePath;
  use std::convert::TryFrom;

  let segments = vec![ "/path", "to", "..", "file" ];
  let got = AbsolutePath::from_iter( segments.iter().map( |s| *s ) ).unwrap();
  let exp = AbsolutePath::try_from( "/path/file" ).unwrap();

  assert_eq!( got, exp );
}

#[ test ]
fn test_from_paths_with_trailing_slash()
{
  use the_module::AbsolutePath;
  use std::convert::TryFrom;

  let segments = vec![ "/path", "to", "file/" ];
  let got = AbsolutePath::from_iter( segments.iter().map( |s| *s ) ).unwrap();
  let exp = AbsolutePath::try_from( "/path/to/file/" ).unwrap();

  assert_eq!( got, exp );
}

#[ test ]
fn test_from_paths_with_mixed_slashes()
{
  use the_module::AbsolutePath;
  use std::convert::TryFrom;

  let segments = vec![ "/path\\to", "file" ];
  let got = AbsolutePath::from_iter( segments.iter().map( |s| *s ) ).unwrap();
  let exp = AbsolutePath::try_from( "/path/to/file" ).unwrap();

  assert_eq!( got, exp );
}
