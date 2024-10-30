use super::*;

#[ test ]
fn try_into_path_test()
{
  use std::path::{ Component, Path, PathBuf };
  #[ cfg( feature = "path_utf8" ) ]
  use the_module::{ Utf8Path, Utf8PathBuf };
  use the_module::{ TryIntoPath, AbsolutePath, CanonicalPath, NativePath, CurrentPath };

  // Test with &str
  let path_str : &str = "/some/path";
  let path_buf : PathBuf = TryIntoPath::try_into_path( path_str ).unwrap();
  println!( "PathBuf from &str: {:?}", path_buf );

  // Test with &String
  let string_path : String = String::from( "/another/path" );
  let path_buf : PathBuf = TryIntoPath::try_into_path( &string_path ).unwrap();
  println!( "PathBuf from &String: {:?}", path_buf );

  // Test with String
  let path_buf : PathBuf = TryIntoPath::try_into_path( string_path.clone() ).unwrap();
  println!( "PathBuf from String: {:?}", path_buf );

  // Test with &Path
  let path : &Path = Path::new( "/yet/another/path" );
  let path_buf : PathBuf = TryIntoPath::try_into_path( path ).unwrap();
  println!( "PathBuf from &Path: {:?}", path_buf );

  // Test with &PathBuf
  let path_buf_instance : PathBuf = PathBuf::from( "/yet/another/path" );
  let path_buf : PathBuf = TryIntoPath::try_into_path( &path_buf_instance ).unwrap();
  println!( "PathBuf from &PathBuf: {:?}", path_buf );

  // Test with PathBuf
  let path_buf : PathBuf = TryIntoPath::try_into_path( path_buf_instance.clone() ).unwrap();
  println!( "PathBuf from PathBuf: {:?}", path_buf );

  // Test with &AbsolutePath
  let absolute_path : AbsolutePath = AbsolutePath::try_from( "/absolute/path" ).unwrap();
  let path_buf : PathBuf = TryIntoPath::try_into_path( &absolute_path ).unwrap();
  println!( "PathBuf from &AbsolutePath: {:?}", path_buf );

  // Test with AbsolutePath
  let path_buf : PathBuf = TryIntoPath::try_into_path( absolute_path.clone() ).unwrap();
  println!( "PathBuf from AbsolutePath: {:?}", path_buf );

  // Test with &CanonicalPath
  let canonical_path = CanonicalPath::try_from( "/canonical/path" ).unwrap();
  let path_buf : PathBuf = TryIntoPath::try_into_path( &canonical_path ).unwrap();
  println!( "PathBuf from &CanonicalPath: {:?}", path_buf );

  // Test with CanonicalPath
  let path_buf : PathBuf = TryIntoPath::try_into_path( canonical_path.clone() ).unwrap();
  println!( "PathBuf from CanonicalPath: {:?}", path_buf );

  // Test with &NativePath
  let native_path = NativePath::try_from( PathBuf::from( "/native/path" ) ).unwrap();
  let path_buf : PathBuf = TryIntoPath::try_into_path( &native_path ).unwrap();
  println!( "PathBuf from &NativePath: {:?}", path_buf );

  // Test with NativePath
  let path_buf : PathBuf = TryIntoPath::try_into_path( native_path.clone() ).unwrap();
  println!( "PathBuf from NativePath: {:?}", path_buf );

  // Test with &CurrentPath
  let current_path = CurrentPath;
  let path_buf : PathBuf = TryIntoPath::try_into_path( &current_path ).unwrap();
  println!( "PathBuf from &CurrentPath: {:?}", path_buf );
  assert!( path_buf.to_string_lossy().len() > 1 );

  // Test with CurrentPath
  let path_buf : PathBuf = TryIntoPath::try_into_path( current_path ).unwrap();
  println!( "PathBuf from CurrentPath: {:?}", path_buf );
  assert!( path_buf.to_string_lossy().len() > 1 );

  // Test with &Component
  let root_component : Component< '_ > = Component::RootDir;
  let path_buf : PathBuf = TryIntoPath::try_into_path( &root_component ).unwrap();
  println!( "PathBuf from &Component: {:?}", path_buf );
  assert!( path_buf.to_string_lossy().len() >= 1 );

  // Test with Component
  let path_buf : PathBuf = TryIntoPath::try_into_path( root_component ).unwrap();
  println!( "PathBuf from Component: {:?}", path_buf );
  assert!( path_buf.to_string_lossy().len() >= 1 );

  // Test with Component
  let path = Path::new( "/component/path" );
  for component in path.components()
  {
    let path_buf : PathBuf = TryIntoPath::try_into_path( component ).unwrap();
    println!( "PathBuf from Component: {:?}", path_buf );
    assert!( path_buf.to_string_lossy().len() >= 1 );
  }

  #[ cfg( feature = "path_utf8" ) ]
  {
    // Test with &Utf8Path
    let utf8_path = Utf8Path::new( "/utf8/path" );
    let path_buf : PathBuf = TryIntoPath::try_into_path( &utf8_path ).unwrap();
    println!( "PathBuf from &Utf8Path: {:?}", path_buf );

    // Test with Utf8Path
    let path_buf : PathBuf = TryIntoPath::try_into_path( utf8_path ).unwrap();
    println!( "PathBuf from Utf8Path: {:?}", path_buf );

    // Test with &Utf8PathBuf
    let utf8_path_buf = Utf8PathBuf::from( "/utf8/pathbuf" );
    let path_buf : PathBuf = TryIntoPath::try_into_path( &utf8_path_buf ).unwrap();
    println!( "PathBuf from &Utf8PathBuf: {:?}", path_buf );

    // Test with Utf8PathBuf
    let path_buf : PathBuf = TryIntoPath::try_into_path( utf8_path_buf.clone() ).unwrap();
    println!( "PathBuf from Utf8PathBuf: {:?}", path_buf );
  }
}
