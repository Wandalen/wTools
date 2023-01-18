use std::{ io, fs };
use std::path::Path;

pub use std::path::PathBuf;

#[ derive( Debug ) ]
pub struct Asset
{
  remove_after_use : bool,
  path : PathBuf,
}

impl From< PathBuf > for Asset
{
  fn from( path : PathBuf ) -> Self
  {
    Self{ remove_after_use : false, path }
  }
}

impl Asset
{
  pub fn path_buf( &self ) -> &PathBuf
  {
    &self.path
  }

  pub fn copied( mut self ) -> Self
  {
    let tmp_dir = tempfile::tempdir().unwrap();

    Self::copy_dir_all( &self.path, &tmp_dir ).unwrap();
    self.path = tmp_dir.into_path();
    self.remove_after_use = true;

    self
  }
}

impl Asset
{
  fn copy_dir_all( src : impl AsRef< Path >, dst : impl AsRef< Path > ) -> io::Result< () >
  {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)?
    {
      let entry = entry?;
      let ty = entry.file_type()?;
      if ty.is_dir()
      {
        Asset::copy_dir_all( entry.path(), dst.as_ref().join( entry.file_name() ) )?;
      } else
      {
        fs::copy( entry.path(), dst.as_ref().join( entry.file_name() ) )?;
      }
    }
    Ok( () )
  }
}

impl Drop for Asset
{
  fn drop( &mut self )
  {
    if self.remove_after_use
    {
      fs::remove_dir_all( &self.path )
      .expect( &format!( "Can not delete \"{}\"", &self.path.display() ) )
    }
  }
}
