mod private
{
  use std::fs::{ create_dir, File };
  use std::io::Write;
  use std::path::Path;
  use error_tools::for_app::*;
  
  /// todo
  #[ derive( Debug ) ]
  pub enum Content
  {
    ///todo
    Directory
    { 
      ///todo
      name: String, 
      ///todo
      content: Vec< Content > 
    },
    ///todo
    File
    { 
      ///todo
      name: String, 
      ///todo
      content: String 
    }, 
  }

  ///todo
  pub fn write_to_file< T : IntoIterator< Item = Content > >( base_path : &Path, content : T ) -> Result< () >
  {
    for content in content 
    {
      match content 
      {
        Content::Directory { name, content } => 
        {
          let path = base_path.join( name ); 
          create_dir( &path )?;
          write_to_file( &path, content.into_iter() )?;
        }
        Content::File { name, content } => 
        {
          let mut file = File::create( base_path.join( name ) )?;
          file.write_all( content.as_bytes() )?;
        }
      }
    }
    
    Ok( () )
  }


}

crate::mod_interface!
{
  /// todo!
  protected use write_to_file;
  /// todo!
  protected use Content;
}