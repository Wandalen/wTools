/// Define a private namespace for all its items.
mod private
{
  #[ allow( unused_imports, clippy::wildcard_imports ) ]
  use crate::tool::*;

  use std::
  {
    fs,
    path::
    {
      Path,
      PathBuf
    },
  };
  use error::untyped::Context;

  /// Container for templates.
  ///
  /// Includes files to create, parameters that those templates accept,
  /// and values for those parameters.
  #[ derive( Debug ) ]
  pub struct TemplateHolder
  {
    /// Files of the template.
    pub files : Vec< TemplateFileDescriptor >,
    /// Parameters definitions.
    pub parameters : TemplateParameters,
    /// The values associated with the template.
    pub values : TemplateValues,
    /// Path to the parameter storage for recovering values
    /// for already generated templated files.
    pub parameter_storage : &'static Path,
    /// Name of the template to generate
    pub template_name : &'static str,
  }

  impl TemplateFiles for Vec< TemplateFileDescriptor > {}


  impl TemplateHolder
  {
    /// Creates all files in the specified path using the template values.
    ///
    /// # Parameters
    ///
    /// - `path`: A reference to the path where the files will be created.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok` if the files are created successfully, or an `Err` otherwise.
    ///
    /// # Errors
    /// qqq: doc
    pub fn create_all( self, path : &path::Path ) -> error::untyped::Result< () > // qqq : use typed error
    {
      self.files.create_all( path, &self.values )
    }

    /// Returns a reference to the template parameters.
    ///
    /// # Returns
    ///
    /// A reference to `TemplateParameters`.
    #[ must_use ]
    pub fn parameters( &self ) -> &TemplateParameters
    {
      &self.parameters
    }

    /// Sets the template values.
    ///
    /// # Parameters
    ///
    /// - `values`: The new `TemplateValues` to be set.
    pub fn set_values( &mut self, values : TemplateValues )
    {
      self.values = values;
    }

    /// Returns a reference to the template values.
    ///
    /// # Returns
    ///
    /// A reference to `TemplateValues`.
    #[ must_use ]
    pub fn get_values( &self ) -> &TemplateValues
    {
      &self.values
    }

    /// Returns a mutable reference to the template values.
    ///
    /// # Returns
    ///
    /// A mutable reference to `TemplateValues`.
    pub fn get_values_mut( &mut self ) -> &mut TemplateValues
    {
      &mut self.values
    }

    /// Loads existing parameters from the specified path and updates the template values.
    ///
    /// # Parameters
    ///
    /// - `path`: A reference to the path where the parameter file is located.
    ///
    /// # Returns
    ///
    /// An `Option` which is `Some(())` if the parameters are loaded successfully, or `None` otherwise.
    pub fn load_existing_params( &mut self, path : &Path ) -> Option< () >
    {
      let data = fs::read_to_string( path.join( self.parameter_storage ) ).ok()?;
      let document = data.parse::< toml_edit::Document >().ok()?;
      let parameters : Vec< _ > = self.parameters().descriptors.iter().map( | d | &d.parameter ).cloned().collect();
      let template_table = document.get( self.template_name )?;
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
          self.get_values_mut().insert_if_empty( &parameter, wca::Value::String( value.into() ) );
        }
      }
      Some( () )
    }

    /// Fetches mandatory parameters that are not set yet.
    #[ must_use ]
    pub fn get_missing_mandatory( &self ) -> Vec< &str >
    {
      let values = self.get_values();
      self
      .parameters()
      .list_mandatory()
      .into_iter()
      .filter( | key | values.0.get( *key ).and_then( | val | val.as_ref() ).is_none() )
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
    ///
    /// # Errors
    /// qqq: doc
    fn create_all( self, path : &Path, values : &TemplateValues ) -> error::untyped::Result< () > // qqq : use typed error
    {
      let fsw = FileSystem;
      for file in self
      {
        file.create_file( &fsw, path, values )?;
      }
      Ok( () )
    }
  }

  /// Parameters required for the template.
  #[ derive( Debug, Default, former::Former ) ]
  pub struct TemplateParameters
  {
    #[ subform_entry( setter = false ) ]
    descriptors : Vec< TemplateParameterDescriptor >
  }

  impl TemplateParameters
  {
    /// Extracts template values from props for parameters required for this template.
    #[ must_use ]
    pub fn values_from_props( &self, props : &wca::executor::Props ) -> TemplateValues
    {
      let values = self.descriptors
      .iter()
      .map( | d | &d.parameter )
      .map( | param | ( param.clone(), props.get( param ).cloned() ) )
      .collect();
      TemplateValues( values )
    }

    /// Get a list of all mandatory parameters.
    #[ must_use ]
    pub fn list_mandatory( &self ) -> Vec< &str >
    {
      self.descriptors.iter().filter( | d | d.is_mandatory ).map( | d | d.parameter.as_str() ).collect()
    }
  }

  /// Parameter description.
  #[ derive( Debug, Default, former::Former ) ]
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
  pub struct TemplateValues( collection::HashMap< String, Option< wca::Value > > );

  impl TemplateValues
  {
    /// Converts values to a serializable object.
    ///
    /// Currently only `String`, `Number`, and `Bool` are supported.
    #[ must_use ]
    pub fn to_serializable( &self ) -> collection::BTreeMap< String, String >
    {
      self.0.iter().map
      (
        | ( key, value ) |
        {
          let value = value.as_ref().map_or
          (
            "___UNSPECIFIED___".to_string(),
            | value |
            {
              match value
              {
                wca::Value::String( val ) => val.to_string(),
                wca::Value::Number( val ) => val.to_string(),
                wca::Value::Bool( val ) => val.to_string(),
                wca::Value::Path( _ ) |
                wca::Value::List( _ ) => "unsupported".to_string(),
              }
            }
          );
          ( key.to_owned(), value )
        }
      )
      .collect()
    }

    /// Inserts new value if parameter wasn't initialized before.
    pub fn insert_if_empty( &mut self, key : &str, value : wca::Value )
    {
      if self.0.get( key ).and_then( | v | v.as_ref() ).is_none()
      {
        self.0.insert( key.into() , Some( value ) );
      }
    }

    /// Interactively asks user to provide value for a parameter.
    pub fn interactive_if_empty( &mut self, key : &str )
    {
      if self.0.get( key ).and_then( | v | v.as_ref() ).is_none()
      {
        println! ("Parameter `{key}` is not set" );
        let answer = wca::ask( "Enter value" );
        self.0.insert( key.into(), Some( wca::Value::String( answer ) ) );
      }
    }
  }

  /// File descriptor for the template.
  ///
  /// Holds raw template data, relative path for the file, and a flag that
  /// specifies whether the raw data should be treated as a template.
  #[ derive( Debug, former::Former ) ]
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
    -> error::untyped::Result< String >
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
          if let Ok( existing_contents ) = fs.read( &instruction )
          {
            let document = contents.parse::< toml_edit::Document >().context( "Failed to parse template toml file" )?;
            let template_items = document.iter();
            let existing_toml_contents = String::from_utf8( existing_contents ).context( "Failed to read existing toml file as a UTF-8 String" )?;
            let mut existing_document = existing_toml_contents.parse::< toml_edit::Document >().context( "Failed to parse existing toml file" )?;
            for ( template_key, template_item ) in template_items
            {
              match existing_document.get_mut( template_key )
              {
                Some( item ) => template_item.clone_into( item ),
                None => template_item.clone_into( &mut existing_document[ template_key ] ),
              }
            }
            return Ok( existing_document.to_string() );
          }

          Ok( contents )
        }
      }
    }

    // qqq : use typed error
    fn build_template( &self, values : &TemplateValues ) -> error::untyped::Result< String >
    {
      let mut handlebars = handlebars::Handlebars::new();
      handlebars.register_escape_fn( handlebars::no_escape );
      handlebars.register_template_string( "templated_file", self.data )?;
      handlebars.render( "templated_file", &values.to_serializable() ).context( "Failed creating a templated file" )
    }

    fn create_file< FS : FileSystemPort >( &self, fs : &FS, path : &Path, values : &TemplateValues ) -> error::untyped::Result< () > // qqq : use typed error
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
  #[ derive( Debug, former::Former ) ]
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
    /// # Errors
    /// qqq: doc
    fn write( &self, instruction : &FileWriteInstruction ) -> error::untyped::Result< () >; // qqq : use typed error

    /// Reading from a file implementation.
    /// # Errors
    /// qqq: doc
    fn read( &self, instruction : &FileReadInstruction ) -> error::untyped::Result< Vec< u8 > >; // qqq : use typed error
  }

  // zzz : why not public?
  struct FileSystem;
  impl FileSystemPort for FileSystem
  {
    fn write( &self, instruction : &FileWriteInstruction ) -> error::untyped::Result< () > // qqq : use typed error
    {
      let FileWriteInstruction { path, data } = instruction;
      let dir = path.parent().context( "Invalid file path provided" )?;
      if !dir.exists()
      {
        fs::create_dir_all( dir )?;
      }
      fs::write( path, data ).context( "Failed creating and writing to file" )
    }

  // qqq : use typed error
    fn read( &self, instruction : &FileReadInstruction ) -> error::untyped::Result< Vec< u8 > >
    {
      let FileReadInstruction { path } = instruction;
      fs::read( path ).context( "Failed reading a file" )
    }

  }

}

//

crate::mod_interface!
{
  //orphan use Template;
  orphan use TemplateHolder;
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
