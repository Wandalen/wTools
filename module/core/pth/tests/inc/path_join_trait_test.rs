use super::*;
use std::
{
  borrow::Cow,
  io,
  path::{ Path, PathBuf },
};

#[ test ]
fn basic() -> Result< (), io::Error >
{
  use the_module::PathJoined;
  use std::path::PathBuf;

  let path1 : &str = "/some";
  let path2 : String = "path".into();
  let path3 : PathBuf = "to/file".into();
  let path4 : &str = "extra";
  let path5 : String = "components".into();

  // Test with a tuple of length 1
  let joined1 : PathBuf = ( path1, ).iter_join()?;
  println!( "Joined PathBuf (1): {:?}", joined1 );

  // Test with a tuple of length 2
  let joined2 : PathBuf = ( path1, path2.clone() ).iter_join()?;
  println!( "Joined PathBuf (2): {:?}", joined2 );

  // Test with a tuple of length 3
  let joined3 : PathBuf = ( path1, path2.clone(), path3.clone() ).iter_join()?;
  println!( "Joined PathBuf (3): {:?}", joined3 );

  // Test with a tuple of length 4
  let joined4 : PathBuf = ( path1, path2.clone(), path3.clone(), path4 ).iter_join()?;
  println!( "Joined PathBuf (4): {:?}", joined4 );

  // Test with a tuple of length 5
  let joined5 : PathBuf = ( path1, path2, path3, path4, path5 ).iter_join()?;
  println!( "Joined PathBuf (5): {:?}", joined5 );

  Ok( () )
}

#[ test ]
fn array_join_paths_test() -> Result< (), io::Error >
{
  use the_module::{ PathJoined, TryIntoCowPath };
  use std::path::PathBuf;

  // Define a slice of path components
  let path_components : [ &str; 3 ] = [ "/some", "path", "to/file" ];
  // Join the path components into a PathBuf
  let joined : PathBuf = path_components.iter_join()?;
  println!( "Joined PathBuf from slice: {:?}", joined );
  let expected = PathBuf::from( "/some/path/to/file" );
  assert_eq!( joined, expected );

  Ok( () )
}

#[ test ]
fn slice_join_paths_test() -> Result< (), io::Error >
{
  use the_module::{ PathJoined, TryIntoCowPath };
  use std::path::PathBuf;

  // Define a slice of path components
  let path_components : [ &str; 3 ] = [ "/some", "path", "to/file" ];
  let slice : &[ &str ] = &path_components[ .. ];
  // Join the path components into a PathBuf
  let joined : PathBuf = slice.iter_join()?;
  println!( "Joined PathBuf from slice: {:?}", joined );
  let expected = PathBuf::from( "/some/path/to/file" );
  assert_eq!( joined, expected );

  Ok( () )
}

#[ test ]
fn all_types() -> Result< (), io::Error >
{
  use std::path::Path;
  use the_module::{ AbsolutePath, CanonicalPath, NativePath, CurrentPath };
  use the_module::{ PathJoined, AsPath, TryIntoPath };

  // AbsolutePath and CurrentPath
  {
    let absolute_path = AbsolutePath::try_from( "/absolute/path" ).unwrap();
    let current_path = CurrentPath;
    let joined = ( absolute_path.clone(), current_path ).iter_join()?;
    let expected = current_path.try_into_path()?;
    println!( "Joined PathBuf: {:?}", joined );
    assert_eq!( joined, expected );
  }

  // // CurrentPath and AbsolutePath
  // {
  //   let absolute_path = AbsolutePath::try_from( "/absolute/path" ).unwrap();
  //   let current_path = CurrentPath;
  //   let joined = ( current_path, absolute_path.clone() ).iter_join()?;
  //   let expected = absolute_path.as_path().to_path_buf();
  //   println!( "Joined PathBuf: {:?}", joined );
  //   assert_eq!( joined, expected );
  // }
  // // qqq : qqq2 : for Denys : bad

  // AbsolutePath and Component
  {
    let absolute_path = AbsolutePath::try_from( "/absolute/path" ).unwrap();
    let component = Path::new( "/component/path" ).components().next().unwrap();
    println!( "component : {component:?}" );
    let joined = ( absolute_path, component ).iter_join()?;
    let expected = component.as_path();
    println!( "Joined PathBuf: {:?}", joined );
    assert_eq!( joined, expected );
  }

  // AbsolutePath and &str
  {
    let absolute_path = AbsolutePath::try_from( "/absolute/path" ).unwrap();
    let path_str : &str = "additional/str";
    let joined = ( absolute_path, path_str ).iter_join()?;
    let expected = PathBuf::from( "/absolute/path/additional/str" );
    println!( "Joined PathBuf: {:?}", joined );
    assert_eq!( joined, expected );
  }

  // AbsolutePath and NativePath
  {
    let absolute_path = AbsolutePath::try_from( "/absolute/path" ).unwrap();
    let native_path = NativePath::try_from( PathBuf::from( "/native/path" ) ).unwrap();
    let joined = ( absolute_path, native_path ).iter_join()?;
    let expected = PathBuf::from( "/native/path" );
    println!( "Joined PathBuf: {:?}", joined );
    assert_eq!( joined, expected );
  }

  // AbsolutePath and CanonicalPath
  {
    let absolute_path = AbsolutePath::try_from( "/absolute/path" ).unwrap();
    let canonical_path = CanonicalPath::try_from( "/canonical/path" ).unwrap();
    let joined = ( absolute_path, canonical_path ).iter_join()?;
    let expected = PathBuf::from( "/canonical/path" );
    println!( "Joined PathBuf: {:?}", joined );
    assert_eq!( joined, expected );
  }

  // NativePath and CurrentPath
  {
    let native_path = NativePath::try_from( PathBuf::from( "/native/path" ) ).unwrap();
    let current_path = CurrentPath;
    let joined = ( native_path, current_path ).iter_join()?;
    let expected = current_path.try_into_path()?;
    println!( "Joined PathBuf: {:?}", joined );
    assert_eq!( joined, expected );
  }

  // CanonicalPath and Component
  {
    let canonical_path = CanonicalPath::try_from( "/canonical/path" ).unwrap();
    let component = Path::new( "/component/path" ).components().next().unwrap();
    println!( "component : {component:?}" );
    let joined = ( canonical_path, component ).iter_join()?;
    let expected = component.as_path();
    // let expected = PathBuf::from( "/canonical/component" );
    println!( "Joined PathBuf: {:?}", joined );
    assert_eq!( joined, expected );
  }

  Ok( () )
}

#[ test ]
fn join_function_test() -> Result< (), io::Error >
{
  use the_module::path;
  use std::path::PathBuf;

  // Test joining a tuple of path components
  let path1 : &str = "/some";
  let path2 : String = "path".into();
  let path3 : PathBuf = "to/file".into();

  // Use the join function to join the path components
  let joined : PathBuf = path::join( ( path1, path2.clone(), path3.clone() ) )?;
  println!( "Joined PathBuf: {:?}", joined );
  // Verify the expected outcome
  let expected = PathBuf::from( "/some/path/to/file" );
  assert_eq!( joined, expected );

  // Test joining a tuple of length 2
  let joined : PathBuf = path::join( ( path1, path2.clone() ) )?;
  println!( "Joined PathBuf (2 components): {:?}", joined );
  // Verify the expected outcome
  let expected = PathBuf::from( "/some/path" );
  assert_eq!( joined, expected );

  // Test joining a tuple of length 1
  let joined : PathBuf = path::join( ( path1, ) )?;
  println!( "Joined PathBuf (1 component): {:?}", joined );
  // Verify the expected outcome
  let expected = PathBuf::from( "/some" );
  assert_eq!( joined, expected );

  Ok( () )
}