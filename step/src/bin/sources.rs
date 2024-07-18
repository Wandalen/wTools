//! List all sources

use willbe::exposed::*;
use willbe::{ Entries, Sources, CodeItems};
use std::
{
  fs,
  fs::File,
  io::Write,
};

fn main() -> Result< () >
{

  let workspace = Workspace::try_from( CurrentPath )?;

  let package = workspace
  .packages_which()
  .crate_dir( CrateDir::transitive_try_from::< AbsolutePath >( CurrentPath )? )
  .find()
  .expect( "No workspace at current path" )
  ;

  println!( " = package - {}", package.crate_dir().unwrap() );

//   let ins = r#"
// pub mod exposed
// {
// "#;
//
//   let sub = r#"
// pub mod exposed
// {
//   #[ allow( unused_imports ) ]
//   use super::*;
// "#;

  package.sources().for_each( | source |
  {
    println!( "   - {source}" );

    // let code = source.as_code().unwrap();
    // let code2 = code.replace( ins, sub );

    // source
    // .items()
    // .for_each( | item |
    // {
    //   println!( "     - {}", std::any::type_name_of_val( &item ) );
    //   // println!( "     - item : {item:?}" );
    // });

  });

  // println!( "{}", package.as_code().unwrap() );

  // let dst_path = format!( "{}.rs", package.name() );
  // let _ = fs::remove_file( &dst_path );
  // let code = package.as_code().unwrap();
  // let mut file = File::create( dst_path )?;
  // file.write_all( code.as_bytes() )?;

  dbg!( &workspace.crate_dir );

  return Ok( () );
}
