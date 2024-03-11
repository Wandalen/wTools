mod private
{
  use std::collections::BTreeMap;
  use std::fs;
  use std::io::Write;
  use error_tools::for_app::Context;
  use error_tools::Result;
  use former::Former;
  use wca::Props;
  use std::path::Path;
  use std::path::PathBuf;
  use wca::Value;
  use std::collections::HashMap;

  /// Trait for creating a template for a file structure.
  pub trait Template< F > : Sized
  where
    F : TemplateFiles + Default
  {
    /// Creates all files in the template.
    ///
    /// Path is the base path for the template to be created in.
    fn create_all( self, path : &Path ) -> Result< () >;

    /// Returns all parameters used by the template.
    fn parameters( &self ) -> &TemplateParameters;

    /// Sets values for provided parameters.
    fn set_values( &mut self, values : TemplateValues );
  }

  /// Files stored in a template.
  ///
  /// Can be iterated over, consuming the owner of the files.
  pub trait TemplateFiles : IntoIterator< Item = TemplateFileDescriptor > + Sized
  {
    /// Creates all files in provided path with values for required parameters.
    ///
    /// Consumes owner of the files.
    fn create_all( self, path : &Path, values : &TemplateValues ) -> Result< () >
    {
      let fsw = FileSystem;
      for file in self.into_iter()
      {
        let full_path = path.join( &file.path );
        let dir = full_path.parent().context( "Invalid file path provided" )?;

        if !dir.exists()
        {
          fs::create_dir_all( dir )?;
        }
        if !full_path.exists()
        {
          file.create_file( &fsw, path, values )?;
        }
      }
      Ok( () )
    }
  }

  /// Parameters required for the template.
  #[ derive( Debug, Default ) ]
  pub struct TemplateParameters( Vec< String > );

  impl TemplateParameters
  {
    /// Creates new template parameters from a list of strings.
    ///
    /// Type of the parameter will be automatically converted from value
    /// that was provided during template creation.
    pub fn new( parameters : &[ &str ] ) -> Self
    {
      Self( parameters.into_iter().map( | parameter | parameter.to_string() ).collect() )
    }

    /// Extracts template values from props for parameters required for this template.
    pub fn values_from_props( &self, props : &Props ) -> TemplateValues
    {
      let values = self.0.iter().map( | param | ( param.clone(), props.get( param ).map( Value::clone ) ) ).collect();
      TemplateValues( values )
    }
  }

  /// Holds a map of parameters and their values.
  #[ derive( Debug, Default ) ]
  pub struct TemplateValues( HashMap< String, Option< Value > > );

  impl TemplateValues
  {
    /// Converts values to a serializable object.
    ///
    /// Currently only `String`, `Number`, and `Bool` are supported.
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
                  Value::String( val ) => val.to_string(),
                  Value::Number( val ) => val.to_string(),
                  Value::Path( _ ) => "unsupported".to_string(),
                  Value::Bool( val ) => val.to_string(),
                  Value::List( _ ) => "unsupported".to_string(),
                }
              }
            )
            .unwrap_or( "___UNSPECIFIED___".to_string() );
          ( key.to_owned(), value )
        }
      )
      .collect()
    }
  }

  /// File descriptor for the template.
  ///
  /// Holds raw template data, relative path for the file, and a flag that
  /// specifies whether the raw data should be treated as a template.
  #[ derive( Debug, Former ) ]
  pub struct TemplateFileDescriptor
  {
    path : PathBuf,
    data : &'static str,
    is_template : bool,
  }

  impl TemplateFileDescriptor
  {
    fn contents( &self, values : &TemplateValues ) -> Result< String >
    {
      if self.is_template
      {
        self.build_template( values )
      }
      else
      {
        Ok( self.data.to_owned() )
      }
    }

    fn build_template( &self, values : &TemplateValues ) -> Result< String >
    {
      let mut handlebars = handlebars::Handlebars::new();
      handlebars.register_escape_fn( handlebars::no_escape );
      handlebars.register_template_string( "templated_file", self.data )?;
      handlebars.render( "templated_file", &values.to_serializable() ).context( "Failed creating a templated file" )
    }

    fn create_file< W : FileSystemWriter >( &self, writer : &W, path : &Path, values : &TemplateValues ) -> Result< () >
    {
      let data = self.contents( values )?.as_bytes().to_vec();
      let instruction = FileWriteInstruction { path : path.join( &self.path ), data };
      writer.write( &instruction )?;
      Ok( () )
    }
  }

  /// Helper builder for full template file list.
  #[ derive( Debug, Former ) ]
  pub struct TemplateFilesBuilder
  {
    /// Stores all file descriptors for current template.
    #[ setter( false ) ]
    pub files : Vec< TemplateFileDescriptor >,
  }

  impl< Context, End > TemplateFilesBuilderFormer< Context, End >
  where
    End : former::ToSuperFormer< TemplateFilesBuilder, Context >,
  {
    #[ inline( always ) ]
    pub fn file( self ) -> TemplateFileDescriptorFormer< Self, impl former::ToSuperFormer< TemplateFileDescriptor, Self > >
    {
      let on_end = | descriptor : TemplateFileDescriptor, super_former : core::option::Option< Self > | -> Self
      {
        let mut super_former = super_former.unwrap();
        if let Some( ref mut files ) = super_former.container.files
        {
          files.push( descriptor );
        }
        else
        {
          super_former.container.files = Some( vec![ descriptor ] );
        }
        super_former
      };
      TemplateFileDescriptorFormer::begin( Some( self ), on_end )
    }
  }

  /// Instruction for writing a file.
  #[ derive( Debug ) ]
  pub struct FileWriteInstruction
  {
    path : PathBuf,
    data : Vec<u8>,
  }

  /// Describes how template file creation should be handled.
  pub trait FileSystemWriter
  {
    /// Writing to file implementation.
    fn write( &self, instruction : &FileWriteInstruction ) -> Result< () >;
  }

  struct FileSystem;
  impl FileSystemWriter for FileSystem
  {
    fn write( &self, instruction : &FileWriteInstruction ) -> Result< () >
    {
      let FileWriteInstruction { path, data } = instruction;
      let mut file = fs::File::create( path ).context( "Failed creating file" )?;
      file.write_all( data ).context( "Failed writing to file" )
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
  orphan use TemplateFilesBuilder;
  orphan use FileSystemWriter;
  orphan use FileWriteInstruction;
}
