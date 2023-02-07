/// Internal namespace.
pub( crate ) mod private
{
  use cargo_edit::LocalManifest;
  use wtools::{ Result, err };

  use crate::Package;

  /// -
  #[ derive( Debug ) ]
  pub struct PackageDependency
  {
    package : Package,
    name : String,
    section : String,
    dry : bool,
  }

  impl Package
  {
    /// -
    pub fn dependency( self, name : impl Into< String > ) -> PackageDependency
    {
      let section = "dependencies".to_owned();

      PackageDependency
      {
        package : self,
        name : name.into(),
        section,
        dry : false,
      }
    }
  }

  impl PackageDependency
  {
    /// This dependency will be added to dev dependencies
    pub fn dev( mut self ) -> Self
    {
      self.section = "dev-dependencies".into();
      self
    }

    /// This dependency will be added to build dependencies
    pub fn build( mut self ) -> Self
    {
      self.section = "build-dependencies".into();
      self
    }
  }

  impl PackageDependency
  {
    /// Changes will NOT be applied
    pub fn dry( mut self ) -> Self
    {
      self.dry = true;
      self
    }

    /// Changes will be applied
    pub fn no_dry( mut self ) -> Self
    {
      self.dry = false;
      self
    }
  }

  impl PackageDependency
  {
    /// Add the dependency to the Package
    pub fn add( self ) -> Result< Package >
    {
      println!( "[Adding] {dep} to {section}", dep = self.name, section = self.section );
      let mut cmd = std::process::Command::new( "cargo" );

      cmd
      .current_dir( self.package.path() )
      .args([ "add", &self.name ]);
      match self.section.as_str()
      {
        "dev-dependencies" => { cmd.arg( "--dev" ); },
        "build-dependencies" => { cmd.arg( "--build" ); },
        _ => {},
      };

      if self.dry
      { 
        cmd.arg( "--dry-run" );
        println!( "[Adding] aborting due to dry run" )
      }

      cmd.output()
      .map_err( | _ | err!( "Can not add dependency") )?;

      Ok( self.package )
    }

    /// Remove the dependency from the Package
    pub fn remove( self ) -> Result< Package >
    {
      let mut manifest = LocalManifest::find( Some( &self.package.path() ) ).map_err( | _ | err!( "Can not find package manifest" ) )?;
      let dep = self.name.as_ref();

      println!( "[Removing] {dep} from {sec}", sec = self.section );
      manifest.remove_from_table( &[ self.section ], dep ).map_err( | _ | err!( "Can not remove dependency" ) )?;
      manifest.gc_dep( dep );

      if self.dry
      { println!( "[Removing] aborting due to dry run" ) }
      else
      { manifest.write().map_err( | _ | err!( "Can not write to manifest" ) )? }

      Ok( self.package )
    }
  }
}

//

wtools::mod_interface!
{
  prelude use PackageDependency;
}
