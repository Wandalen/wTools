mod private
{
  use std::collections::BTreeMap;
  use std::fs;
  use std::io::Write;
  use error_tools::Result;
  use std::path::Path;
  use std::path::PathBuf;
  use wca::Value;
  use std::collections::HashMap;

  /// todo
  pub trait Template< F, D > : Sized
  where
    F : TemplateFiles< D > + Default,
    D : TemplateFileDescriptor
  {
    /// todo
    fn create_all( self, path : &Path ) -> Result< () >;

    /// todo
    fn parameters( &self ) -> &TemplateParameters;

    /// todo
    fn set_values( &mut self, values : TemplateValues );
  }

  /// todo
  pub trait TemplateFiles< D : TemplateFileDescriptor > : IntoIterator< Item = D > + Sized
  {
    /// todo
    fn create_all( self, path : &Path, values: &TemplateValues ) -> Result< () >
    {
      for file in self.into_iter()
      {
        if !path.join( file.path() ).exists()
        {
          fs::create_dir( path.join( file.path() ) )?;
        }
        if !path.join( file.path() ).join( file.filename() ).exists()
        {
          file.create_file( path, values )?;
        }
      }
      Ok( () )
    }
  }

  /// todo
  pub trait TemplateFileDescriptor
  {
    /// todo
    fn builder( filename : &str ) -> FileDescriptorBuilder
    {
      FileDescriptorBuilder::new( filename )
    }
    /// todo
    fn new
    (
      path : PathBuf,
      filename : String,
      data : &'static str,
      templated : bool,
    ) -> Self;
    /// todo
    fn path( &self ) -> &Path;
    /// todo
    fn filename( &self ) -> &str;
    /// todo
    fn data( &self ) -> &'static str;
    /// todo
    fn templated( &self ) -> bool;
    /// todo
    fn contents( &self, values : &TemplateValues ) -> Result< String >
    {
      if self.templated() {
        Self::build_template( self.data(), values )
      } else {
        Ok( self.data().to_owned() )
      }
    }
    /// todo
    fn build_template( data : &'static str, values : &TemplateValues ) -> Result< String >;
    /// todo
    fn create_file( &self, path : &Path, values: &TemplateValues ) -> Result< () >
    {
      let mut file = fs::File::create( path.join( self.path() ).join( self.filename() ) )?;
      file.write_all( self.contents( values )?.as_bytes() )?;
      Ok( () )
    }
  }

  /// todo
  #[ derive( Debug, Default ) ]
  pub struct TemplateParameters( Vec< String > );

  impl TemplateParameters
  {
    /// todo
    pub fn new( parameters : &[ &str ] ) -> Self
    {
      Self( parameters.into_iter().map( | parameter | parameter.to_string() ).collect() )
    }
  }

  /// todo
  #[ derive( Debug, Default ) ]
  pub struct TemplateValues( HashMap< String, Option< Value > > );

  impl TemplateValues
  {
    /// todo
    pub fn to_serializable( &self ) -> BTreeMap< String, String >
    {
      self.0.iter().map
      (
        | ( key, value ) |
        {
          let value = value.as_ref().map
            (
              | value |
              {
                match value
                {
                  Value::String(val) => val.to_string(),
                  Value::Number(val) => val.to_string(),
                  Value::Path(_) => "unsupported".to_string(),
                  Value::Bool(val) => val.to_string(),
                  Value::List(_) => "unsupported".to_string(),
                }
              }
            )
            .unwrap_or_default();
          ( key.to_owned(), value)
        }
      )
      .collect()
    }
  }

  /// todo
  #[ derive( Debug ) ]
  pub struct FileDescriptorBuilder
  {
    path: Option<PathBuf>,
    filename: String,
    data: &'static str,
    templated: bool,
  }

  impl FileDescriptorBuilder
  {
    /// todo
    fn new( filename : &str) -> Self
    {
      Self
      {
        path : None,
        filename : filename.into(),
        data : "",
        templated : false,
      }
    }

    /// todo
    pub fn build< D : TemplateFileDescriptor >( self ) -> D
    {
      let Self { path, filename, data, templated } = self;
      D::new( path.unwrap_or( ".".into() ), filename, data, templated )
    }

    /// todo
    pub fn data( mut self, data : &'static str) -> Self
    {
      self.data = data;
      self
    }

    pub fn templated( mut self, templated: bool ) -> Self
    {
      self.templated = templated;
      self
    }

    pub fn path( mut self, path: &str ) -> Self
    {
      self.path = Some( path.into() );
      self
    }
  }
}

//

crate::mod_interface!
{
  orphan use Template;
  orphan use TemplateFiles;
  orphan use TemplateFileDescriptor;
  orphan use TemplateParameters;
  orphan use TemplateValues;
}
