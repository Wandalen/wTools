mod private
{
  use std::collections::BTreeMap;
  use std::fs;
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

    /// Loads provided parameters from previous run.
    fn load_existing_params( &mut self ) -> Option< () >
    {
      let data = fs::read_to_string( ".template_params.toml" ).ok()?;
      let document = data.parse::< toml_edit::Document >().ok()?;
      let parameters = self.parameters().descriptors.iter().map( | d | &d.parameter ).cloned().collect::< Vec< _ > >();
      for parameter in parameters
      {
        let value = document.get( &parameter )
        .and_then
        (
          | item |
          match item
          {
            toml_edit::Item::Value( toml_edit::Value::String( val ) ) => Some( val.value() ),
            _ => None
          }
        );
        if let Some( value ) = value
        {
          self.get_values_mut().insert_if_empty( &parameter, Value::String( value.into() ) );
        }
      }
      Some( () )
    }

    /// Get all template values.
    fn get_values( &self ) -> &TemplateValues;

    /// Get all template values as a mutable reference.
    fn get_values_mut( &mut self ) -> &mut TemplateValues;

    /// Saves parameter values after current run.
    fn save_param_values( &self ) -> Result< () >
    {
      let data = fs::read_to_string( ".template_params.toml" ).unwrap_or_default();
      let mut document = data.parse::< toml_edit::Document >()?;
      for ( parameter, value ) in self.get_values().to_serializable()
      {
        let value = toml_edit::Item::Value( toml_edit::Value::String( toml_edit::Formatted::new( value ) ) );
        match document.get_mut( &parameter )
        {
          Some( item ) =>
          {
            *item = value;
          },
          None => document[ &parameter ] = value,
        }
      }
      fs::write( ".template_params.toml", document.to_string() )?;

      Ok( () )
    }

    /// Fetches mandatory parameters that are not set yet.
    fn get_missing_mandatory( &self ) -> Vec< &str >
    {
      let values = self.get_values();
      self
      .parameters()
      .get_mandatory()
      .into_iter()
      .filter( | key | values.0.get( *key ).map( | val | val.as_ref() ).flatten().is_none() )
      .collect()
    }
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
        file.create_file( &fsw, path, values )?;
      }
      Ok( () )
    }
  }

  /// Parameters required for the template.
  #[ derive( Debug, Default, Former ) ]
  pub struct TemplateParameters
  {
    descriptors : Vec< TemplateParameterDescriptor >
  }

  impl TemplateParameters
  {
    /// Extracts template values from props for parameters required for this template.
    pub fn values_from_props( &self, props : &Props ) -> TemplateValues
    {
      let values = self.descriptors
      .iter()
      .map( | d | &d.parameter )
      .map( | param | ( param.clone(), props.get( param ).map( Value::clone ) ) )
      .collect();
      TemplateValues( values )
    }

    /// Get a list of all mandatory parameters.
    pub fn get_mandatory( &self ) -> Vec< &str >
    {
      self.descriptors.iter().filter( | d | d.is_mandatory ).map( | d | d.parameter.as_str() ).collect()
    }
  }
  
  /// Parameter description.
  #[ derive( Debug, Default, Former ) ]
  pub struct TemplateParameterDescriptor
  {
    parameter : String,
    is_mandatory : bool
  }

  impl< Context, End > TemplateParametersFormer< Context, End >
  where
    End : former::ToSuperFormer< TemplateParameters, Context >,
  {
    #[ inline( always ) ]
    pub fn parameter( self, name : &str ) -> TemplateParameterDescriptorFormer< Self, impl former::ToSuperFormer< TemplateParameterDescriptor, Self > >
    {
      let on_end = | descriptor : TemplateParameterDescriptor, super_former : core::option::Option< Self > | -> Self
      {
        let mut super_former = super_former.unwrap();
        if let Some( ref mut descriptors ) = super_former.container.descriptors
        {
          descriptors.push( descriptor );
        }
        else
        {
          super_former.container.descriptors = Some( vec![ descriptor ] );
        }
        super_former
      };
      TemplateParameterDescriptorFormer::begin( Some( self ), on_end ).parameter( name )
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

    /// Inserts new value if parameter wasn't initialized before.
    pub fn insert_if_empty( &mut self, key : &str, value : Value )
    {
      if let None = self.0.get( key ).and_then( | v | v.as_ref() )
      {
        self.0.insert( key.into() , Some( value ) );
      }
    }

    /// Interactively asks user to provide value for a parameter.
    pub fn interactive_if_empty( &mut self, key : &str )
    {
      if let None = self.0.get( key ).and_then( | v | v.as_ref() )
      {
        println! ("Parameter `{key}` is not set" );
        let answer = wca::ask( "Enter value" );
        self.0.insert( key.into(), Some( Value::String( answer ) ) );
      }
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
      let dir = path.parent().context( "Invalid file path provided" )?;
      if !dir.exists()
      {
        fs::create_dir_all( dir )?;
      }
      fs::write( path, data ).context( "Failed creating and writing to file" )
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
  orphan use TemplateParameterDescriptor;
  orphan use TemplateValues;
  orphan use TemplateFilesBuilder;
  orphan use FileSystemWriter;
  orphan use FileWriteInstruction;
}
