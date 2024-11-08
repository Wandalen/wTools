use super::*;

#[ test ]
fn try_into_cow_path_test()
{
  use std::
  {
    borrow::Cow,
    path::{ Component, Path, PathBuf },
  };
  #[ cfg( feature = "path_utf8" ) ]
  use the_module::{ Utf8Path, Utf8PathBuf };
  use the_module::
  {
    TryIntoCowPath, AbsolutePath, CanonicalPath, NativePath, CurrentPath,
  };

  // Test with &str
  let path_str : &str = "/some/path";
  let cow_path : Cow< '_ , Path > = TryIntoCowPath::try_into_cow_path( path_str ).unwrap();
  println!( "Cow<Path> from &str: {:?}", cow_path );

  // Test with &String
  let string_path : String = String::from( "/another/path" );
  let cow_path : Cow< '_ , Path > = TryIntoCowPath::try_into_cow_path( &string_path ).unwrap();
  println!( "Cow<Path> from &String: {:?}", cow_path );

  // Test with String
  let cow_path : Cow< '_ , Path > = TryIntoCowPath::try_into_cow_path( string_path.clone() ).unwrap();
  println!( "Cow<Path> from String: {:?}", cow_path );

  // Test with &Path
  let path : &Path = Path::new( "/yet/another/path" );
  let cow_path : Cow< '_ , Path > = TryIntoCowPath::try_into_cow_path( path ).unwrap();
  println!( "Cow<Path> from &Path: {:?}", cow_path );

  // Test with &PathBuf
  let path_buf : PathBuf = PathBuf::from( "/yet/another/path" );
  let cow_path : Cow< '_ , Path > = TryIntoCowPath::try_into_cow_path( &path_buf ).unwrap();
  println!( "Cow<Path> from &PathBuf: {:?}", cow_path );

  // Test with PathBuf
  let cow_path : Cow< '_ , Path > = TryIntoCowPath::try_into_cow_path( path_buf.clone() ).unwrap();
  println!( "Cow<Path> from PathBuf: {:?}", cow_path );

  // Test with &AbsolutePath
  let absolute_path : AbsolutePath = AbsolutePath::try_from( "/absolute/path" ).unwrap();
  let cow_path : Cow< '_ , Path > = TryIntoCowPath::try_into_cow_path( &absolute_path ).unwrap();
  println!( "Cow<Path> from &AbsolutePath: {:?}", cow_path );

  // Test with AbsolutePath
  let cow_path : Cow< '_ , Path > = TryIntoCowPath::try_into_cow_path( absolute_path.clone() ).unwrap();
  println!( "Cow<Path> from AbsolutePath: {:?}", cow_path );

  // Test with &CanonicalPath
  let canonical_path = CanonicalPath::try_from( "/canonical/path" ).unwrap();
  let cow_path : Cow< '_ , Path > = TryIntoCowPath::try_into_cow_path( &canonical_path ).unwrap();
  println!( "Cow<Path> from &CanonicalPath: {:?}", cow_path );

  // Test with CanonicalPath
  let cow_path : Cow< '_ , Path > = TryIntoCowPath::try_into_cow_path( canonical_path.clone() ).unwrap();
  println!( "Cow<Path> from CanonicalPath: {:?}", cow_path );

  // Test with &NativePath
  let native_path = NativePath::try_from( PathBuf::from( "/native/path" ) ).unwrap();
  let cow_path : Cow< '_ , Path > = TryIntoCowPath::try_into_cow_path( &native_path ).unwrap();
  println!( "Cow<Path> from &NativePath: {:?}", cow_path );

  // Test with NativePath
  let cow_path : Cow< '_ , Path > = TryIntoCowPath::try_into_cow_path( native_path.clone() ).unwrap();
  println!( "Cow<Path> from NativePath: {:?}", cow_path );

  // Test with &CurrentPath
  let current_path = CurrentPath;
  let cow_path : Cow< '_ , Path > = TryIntoCowPath::try_into_cow_path( &current_path ).unwrap();
  println!( "Cow<Path> from &CurrentPath: {:?}", cow_path );
  assert!( cow_path.to_string_lossy().len() > 1 );

  // Test with CurrentPath
  let cow_path : Cow< '_ , Path > = TryIntoCowPath::try_into_cow_path( current_path ).unwrap();
  println!( "Cow<Path> from CurrentPath: {:?}", cow_path );
  assert!( cow_path.to_string_lossy().len() > 1 );

  // Test with &Component
  let root_component : Component< '_ > = Component::RootDir;
  let cow_path : Cow< '_ , Path > = TryIntoCowPath::try_into_cow_path( &root_component ).unwrap();
  println!( "Cow<Path> from &Component: {:?}", cow_path );
  assert!( cow_path.to_string_lossy().len() >= 1 );

  // Test with Component
  let cow_path : Cow< '_ , Path > = TryIntoCowPath::try_into_cow_path( root_component ).unwrap();
  println!( "Cow<Path> from Component: {:?}", cow_path );
  assert!( cow_path.to_string_lossy().len() >= 1 );

  // Test with Component
  let path = Path::new( "/component/path" );
  for component in path.components()
  {
    let cow_path : Cow< '_ , Path > = TryIntoCowPath::try_into_cow_path( component ).unwrap();
    println!( "Cow<Path> from Component: {:?}", cow_path );
    assert!( cow_path.to_string_lossy().len() >= 1 );
  }

  #[ cfg( feature = "path_utf8" ) ]
  {
    // Test with &Utf8Path
    let utf8_path = Utf8Path::new( "/utf8/path" );
    let cow_path : Cow< '_ , Path > = TryIntoCowPath::try_into_cow_path( &utf8_path ).unwrap();
    println!( "Cow<Path> from &Utf8Path: {:?}", cow_path );

    // Test with Utf8Path
    let cow_path : Cow< '_ , Path > = TryIntoCowPath::try_into_cow_path( utf8_path ).unwrap();
    println!( "Cow<Path> from Utf8Path: {:?}", cow_path );

    // Test with &Utf8PathBuf
    let utf8_path_buf = Utf8PathBuf::from( "/utf8/pathbuf" );
    let cow_path : Cow< '_ , Path > = TryIntoCowPath::try_into_cow_path( &utf8_path_buf ).unwrap();
    println!( "Cow<Path> from &Utf8PathBuf: {:?}", cow_path );

    // Test with Utf8PathBuf
    let cow_path : Cow< '_ , Path > = TryIntoCowPath::try_into_cow_path( utf8_path_buf.clone() ).unwrap();
    println!( "Cow<Path> from Utf8PathBuf: {:?}", cow_path );
  }
}
