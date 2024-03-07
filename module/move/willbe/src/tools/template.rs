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

  /// todo
  pub trait Template< F > : Sized
  where
    F : TemplateFiles + Default
  {
    /// todo
    fn create_all( self, path : &Path ) -> Result< () >;

    /// todo
    fn parameters( &self ) -> &TemplateParameters;

    /// todo
    fn set_values( &mut self, values : TemplateValues );
  }

  /// todo
  pub trait TemplateFiles : IntoIterator< Item = TemplateFileDescriptor > + Sized
  {
    /// todo
    fn create_all( self, path : &Path, values : &TemplateValues ) -> Result< () >
    {
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
          file.create_file( path, values )?;
        }
      }
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

    /// todo
    pub fn values_from_props( &self, props : &Props ) -> TemplateValues
    {
      let values = self.0.iter().map( | param | ( param.clone(), props.get( param ).map( Value::clone ) ) ).collect();
      TemplateValues( values )
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
                  Value::String( val ) => val.to_string(),
                  Value::Number( val ) => val.to_string(),
                  Value::Path( _ ) => "unsupported".to_string(),
                  Value::Bool( val ) => val.to_string(),
                  Value::List( _ ) => "unsupported".to_string(),
                }
              }
            )
            .unwrap_or( "UNSPECIFIED_DURING_CREATING_FROM_TEMPLATE".to_string() );
          ( key.to_owned(), value )
        }
      )
      .collect()
    }
  }

  /// todo
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
    /// todo
    fn build_template( &self, values : &TemplateValues ) -> Result< String >
    {
      let mut handlebars = handlebars::Handlebars::new();
      handlebars.register_escape_fn( handlebars::no_escape );
      handlebars.register_template_string( "templated_file", self.data )?;
      handlebars.render( "templated_file", &values.to_serializable() ).context( "Failed creating a templated file" )
    }
    /// todo
    fn create_file( &self, path : &Path, values : &TemplateValues ) -> Result< () >
    {
      let mut file = fs::File::create( path.join( &self.path ) )?;
      file.write_all( self.contents( values )?.as_bytes() )?;
      Ok( () )
    }
  }

  /// todo
  #[ derive( Debug, Former ) ]
  pub struct TemplateFilesBuilder
  {
    /// todo
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
}
