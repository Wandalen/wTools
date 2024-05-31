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

  // qqq : for Viktor : is that trait really necessary?
  // Template - remove
  // DeployTemplate - move here
  // DeployTemplateFiles - remove

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

    /// Relative path for parameter values storage.
    fn parameter_storage( &self ) -> &Path;

    ///
    fn template_name( &self ) -> &'static str;

    /// Loads provided parameters from previous run.
    fn load_existing_params( &mut self, path : &Path ) -> Option< () >
    {
      let data = fs::read_to_string( path.join( self.parameter_storage() ) ).ok()?;
      let document = data.parse::< toml_edit::Document >().ok()?;
      let parameters = self.parameters().descriptors.iter().map( | d | &d.parameter ).cloned().collect::< Vec< _ > >();
      let template_table = document.get( self.template_name() )?;
      for parameter in parameters
      {
        let value = template_table.get( &parameter )
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

    /// Fetches mandatory parameters that are not set yet.
    fn get_missing_mandatory( &self ) -> Vec< &str >
    {
      let values = self.get_values();
      self
      .parameters()
      .list_mandatory()
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
    #[ subform_entry( setter = false ) ]
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
    pub fn list_mandatory( &self ) -> Vec< &str >
    {
      self.descriptors.iter().filter( | d | d.is_mandatory ).map( | d | d.parameter.as_str() ).collect()
    }
  }

  /// Parameter description.
  #[ derive( Debug, Default, Former ) ]
  pub struct TemplateParameterDescriptor
  {
    parameter : String,
    is_mandatory : bool,
  }

  impl< Definition > TemplateParametersFormer< Definition >
  where
    Definition : former::FormerDefinition< Storage = < TemplateParameters as former::EntityToStorage >::Storage >,
  {
    #[ inline( always ) ]
    pub fn parameter( self, name : &str ) ->
    TemplateParameterDescriptorAsSubformer< Self, impl TemplateParameterDescriptorAsSubformerEnd< Self > >
    {
      self._descriptors_subform_entry::< TemplateParameterDescriptorFormer< _ >, _ >()
      .parameter( name )
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
    mode : WriteMode
  }

  impl TemplateFileDescriptor
  {
    fn contents< FS : FileSystemPort >( &self, fs : &FS, path : &PathBuf, values : &TemplateValues )
    -> Result< String >
    {
      let contents = if self.is_template
      {
        self.build_template( values )?
      }
      else
      {
        self.data.to_owned()
      };
      match self.mode
      {
        WriteMode::Rewrite => Ok( contents ),
        WriteMode::TomlExtend =>
        {
          let instruction = FileReadInstruction { path : path.into() };
          if let Some(existing_contents) = fs.read( &instruction ).ok()
          {
            let document = contents.parse::< toml_edit::Document >().context( "Failed to parse template toml file" )?;
            let template_items = document.iter();
            let existing_toml_contents = String::from_utf8( existing_contents ).context( "Failed to read existing toml file as a UTF-8 String" )?;
            let mut existing_document = existing_toml_contents.parse::< toml_edit::Document >().context( "Failed to parse existing toml file" )?;
            for ( template_key, template_item ) in template_items
            {
              match existing_document.get_mut( &template_key )
              {
                Some( item ) => *item = template_item.to_owned(),
                None => existing_document[ &template_key ] = template_item.to_owned(),
              }
            }
            return Ok( existing_document.to_string() );
          }

          Ok( contents )
        }
      }
    }

    fn build_template( &self, values : &TemplateValues ) -> Result< String >
    {
      let mut handlebars = handlebars::Handlebars::new();
      handlebars.register_escape_fn( handlebars::no_escape );
      handlebars.register_template_string( "templated_file", self.data )?;
      handlebars.render( "templated_file", &values.to_serializable() ).context( "Failed creating a templated file" )
    }

    fn create_file< FS : FileSystemPort >( &self, fs : &FS, path : &Path, values : &TemplateValues ) -> Result< () >
    {
      let path = path.join( &self.path );
      let data = self.contents( fs, &path, values )?.as_bytes().to_vec();
      let instruction = FileWriteInstruction { path, data };
      fs.write( &instruction )?;
      Ok( () )
    }

  }

  /// Determines how the template file should be written.
  #[ derive( Debug, Default ) ]
  pub enum WriteMode
  {
    /// Overwrites existing files.
    #[default]
    Rewrite,
    /// Attempts to extend existing toml files.
    ///
    /// If files exists it searches for the same top-level items (tables, values)
    /// and replaces them with template defined ones.
    /// If file does not exist it creates a new one with contents provided by the template.
    TomlExtend
  }

  /// Helper builder for full template file list.
  #[ derive( Debug, Former ) ]
  pub struct TemplateFilesBuilder
  {
    /// Stores all file descriptors for current template.
    #[ subform_entry( setter = true ) ]
    #[ scalar( setter = false ) ]
    pub files : Vec< TemplateFileDescriptor >,
  }

  impl< Description > TemplateFilesBuilderFormer< Description >
  where
    Description : former::FormerDefinition< Storage = < TemplateFilesBuilder as former::EntityToStorage >::Storage >,
  {
    #[ inline( always ) ]
    pub fn file( self ) -> TemplateFileDescriptorAsSubformer< Self, impl TemplateFileDescriptorAsSubformerEnd< Self > >
    {
      self._files_subform_entry()
    }
  }

  /// Instruction for writing a file.
  #[ derive( Debug ) ]
  pub struct FileWriteInstruction
  {
    path : PathBuf,
    data : Vec< u8 >,
  }

  /// Instruction for reading from a file.
  #[ derive( Debug ) ]
  pub struct FileReadInstruction
  {
    path : PathBuf,
  }

  /// Describes how template file creation should be handled.
  pub trait FileSystemPort
  {
    /// Writing to file implementation.
    fn write( &self, instruction : &FileWriteInstruction ) -> Result< () >;

    /// Reading from a file implementation.
    fn read( &self, instruction : &FileReadInstruction ) -> Result< Vec< u8 > >;
  }

  // qqq : xxx : why not public?
  struct FileSystem;
  impl FileSystemPort for FileSystem
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

    fn read( &self, instruction : &FileReadInstruction ) -> Result< Vec< u8 > >
    {
      let FileReadInstruction { path } = instruction;
      fs::read( path ).context( "Failed reading a file" )
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
  orphan use FileSystemPort;
  orphan use FileWriteInstruction;
  orphan use FileReadInstruction;
  orphan use WriteMode;
}
