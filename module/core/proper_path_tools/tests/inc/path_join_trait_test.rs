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

  let path1 : &str = "/some";
  let path2 : String = "path".into();
  let path3 : PathBuf = "to/file".into();

  let joined1 : PathBuf = ( path1, ).join_paths()?;
  println!( "Joined PathBuf (1): {:?}", joined1 );

  let joined2 : PathBuf = ( path1, path2.clone() ).join_paths()?;
  println!( "Joined PathBuf (2): {:?}", joined2 );

  let joined3 : PathBuf = ( path1, path2, path3 ).join_paths()?;
  println!( "Joined PathBuf (3): {:?}", joined3 );

  Ok( () )
}

// xxx2
// #[ test ]
// fn all_types() -> Result< (), io::Error >
// {
//   use std::path::Path;
//
//   let absolute_path = AbsolutePath::try_from( "/absolute/path" ).unwrap();
//   let current_path = CurrentPath;
//   let component = Path::new( "/component/path" ).components().next().unwrap();
//   let native_path = NativePath::try_from( PathBuf::from( "/native/path" ) ).unwrap();
//   let canonical_path = CanonicalPath::try_from( "/canonical/path" ).unwrap();
//   let path_str : &str = "additional/str";
//
//   let joined1 : PathBuf = ( absolute_path.clone(), current_path ).join_paths()?;
//   println!( "Joined PathBuf (1): {:?}", joined1 );
//
//   let joined2 : PathBuf = ( absolute_path.clone(), component ).join_paths()?;
//   println!( "Joined PathBuf (2): {:?}", joined2 );
//
//   let joined3 : PathBuf = ( absolute_path.clone(), path_str ).join_paths()?;
//   println!( "Joined PathBuf (3): {:?}", joined3 );
//
//   let joined4 : PathBuf = ( absolute_path.clone(), native_path ).join_paths()?;
//   println!( "Joined PathBuf (4): {:?}", joined4 );
//
//   let joined5 : PathBuf = ( absolute_path.clone(), canonical_path ).join_paths()?;
//   println!( "Joined PathBuf (5): {:?}", joined5 );
//
//   let joined6 : PathBuf = ( native_path, current_path ).join_paths()?;
//   println!( "Joined PathBuf (6): {:?}", joined6 );
//
//   let joined7 : PathBuf = ( canonical_path, component ).join_paths()?;
//   println!( "Joined PathBuf (7): {:?}", joined7 );
//
//   Ok( () )
// }