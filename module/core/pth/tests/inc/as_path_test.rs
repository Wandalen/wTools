use super::*;

#[ test ]
fn as_path_test()
{
  use std::path::{ Component, Path, PathBuf };
  #[ cfg( feature = "path_utf8" ) ]
  use the_module::{ Utf8Path, Utf8PathBuf };
  use the_module::{ AsPath, AbsolutePath, CanonicalPath, NativePath, CurrentPath };

  // Test with &str
  let path_str : &str = "/some/path";
  let path : &Path = AsPath::as_path( path_str );
  println!( "Path from &str: {:?}", path );

  // Test with &String
  let string_path : String = String::from( "/another/path" );
  let path : &Path = AsPath::as_path( &string_path );
  println!( "Path from &String: {:?}", path );

  // Test with String
  let path : &Path = AsPath::as_path( &string_path );
  println!( "Path from String: {:?}", path );

  // Test with &Path
  let path_ref : &Path = Path::new( "/yet/another/path" );
  let path : &Path = AsPath::as_path( path_ref );
  println!( "Path from &Path: {:?}", path );

  // Test with &PathBuf
  let path_buf : PathBuf = PathBuf::from( "/yet/another/path" );
  let path : &Path = AsPath::as_path( &path_buf );
  println!( "Path from &PathBuf: {:?}", path );

  // Test with PathBuf
  let path : &Path = AsPath::as_path( &path_buf );
  println!( "Path from PathBuf: {:?}", path );

  // Test with &AbsolutePath
  let absolute_path : AbsolutePath = AbsolutePath::try_from( "/absolute/path" ).unwrap();
  let path : &Path = AsPath::as_path( &absolute_path );
  println!( "Path from &AbsolutePath: {:?}", path );

  // Test with AbsolutePath
  let path : &Path = AsPath::as_path( &absolute_path );
  println!( "Path from AbsolutePath: {:?}", path );

  // Test with &CanonicalPath
  let canonical_path = CanonicalPath::try_from( "/canonical/path" ).unwrap();
  let path : &Path = AsPath::as_path( &canonical_path );
  println!( "Path from &CanonicalPath: {:?}", path );

  // Test with CanonicalPath
  let path : &Path = AsPath::as_path( &canonical_path );
  println!( "Path from CanonicalPath: {:?}", path );

  // Test with &NativePath
  let native_path = NativePath::try_from( PathBuf::from( "/native/path" ) ).unwrap();
  let path : &Path = AsPath::as_path( &native_path );
  println!( "Path from &NativePath: {:?}", path );

  // Test with NativePath
  let path : &Path = AsPath::as_path( &native_path );
  println!( "Path from NativePath: {:?}", path );

  // Test with &Component
  let root_component : Component< '_ > = Component::RootDir;
  let path : &Path = AsPath::as_path( &root_component );
  println!( "Path from &Component: {:?}", path );

  // Test with Component
  let path : &Path = AsPath::as_path( &root_component );
  println!( "Path from Component: {:?}", path );

  // Test with Component
  let path = Path::new( "/component/path" );
  for component in path.components()
  {
    let path : &Path = AsPath::as_path( &component );
    println!( "Path from Component: {:?}", path );
  }

  #[ cfg( feature = "path_utf8" ) ]
  {
    // Test with &Utf8Path
    let utf8_path = Utf8Path::new( "/utf8/path" );
    let path : &Path = AsPath::as_path( &utf8_path );
    println!( "Path from &Utf8Path: {:?}", path );

    // Test with Utf8Path
    let path : &Path = AsPath::as_path( &utf8_path );
    println!( "Path from Utf8Path: {:?}", path );

    // Test with &Utf8PathBuf
    let utf8_path_buf = Utf8PathBuf::from( "/utf8/pathbuf" );
    let path : &Path = AsPath::as_path( &utf8_path_buf );
    println!( "Path from &Utf8PathBuf: {:?}", path );

    // Test with Utf8PathBuf
    let path : &Path = AsPath::as_path( &utf8_path_buf );
    println!( "Path from Utf8PathBuf: {:?}", path );
  }
}
