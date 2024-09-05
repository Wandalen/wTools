#[ allow( unused_imports ) ]
use super::*;

#[ cfg( not( feature="no_std" ) ) ]
use the_module::
{
  AbsolutePath,
  // Path,
  PathBuf,
};

#[ cfg( feature = "path_utf8" ) ]
use the_module::Utf8PathBuf;

#[ test ]
#[ cfg( not( feature="no_std" ) ) ]
fn basic()
{

  let cd = the_module::CurrentPath;
  let cd_path : PathBuf = cd.try_into().unwrap();
  println!( "cd_path : {cd_path:?}" );

  let cd = the_module::CurrentPath;
  let absolute_path : AbsolutePath = cd.try_into().unwrap();
  println!( "absolute_path : {absolute_path:?}" );

  #[ cfg( feature = "path_utf8" ) ]
  #[ cfg( not( feature="no_std" ) ) ]
  {
    let cd = the_module::CurrentPath;
    let utf8_path : Utf8PathBuf = cd.try_into().unwrap();
    println!( "utf8_path : {utf8_path:?}" );
  }

}
