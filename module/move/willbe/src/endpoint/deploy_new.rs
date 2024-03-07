mod private {
  use crate::*;
  use std::path::Path;
  use std::path::PathBuf;
  use error_tools::for_app::Context;
  use error_tools::Result;

  use tools::template::*;

  /// todo
  #[ derive( Debug ) ]
  pub struct DeployTemplate
  {
    files : DeployTemplateFiles,
    parameters : TemplateParameters,
    values : TemplateValues,
  }

  impl Template< DeployTemplateFiles, DeployFileDescriptor > for DeployTemplate
  {
    fn create_all( self, path : &Path ) -> Result< () >
    {
      self.files.create_all( path, &self.values )
    }

    fn parameters( &self ) -> &TemplateParameters
    {
      &self.parameters
    }
    
    fn set_values( &mut self, values : TemplateValues )
    {
      self.values = values
    }
  }

  impl Default for DeployTemplate
  {
    fn default() -> Self
    {
      Self
      {
        files : Default::default(),
        parameters : TemplateParameters::new
          (
            &
            [
              "gcp_project_id",
              "gcp_region",
              "gcp_artifact_repo_name",
              "docker_image_name"
            ]
          ),
        values : Default::default(),
      }
    }
  }

  /// todo
  #[ derive( Debug ) ]
  pub struct DeployTemplateFiles( Vec< DeployFileDescriptor > );

  impl Default for DeployTemplateFiles
  {
    fn default() -> Self
    {
      let mut files = vec![];
      let templated_files =
      [
        // root
        ( "Makefile", include_str!( "../../template/deploy/Makefile" ), "./" ),
      ];
      let non_templated_files =
      [
        // /key
        ( "pack.sh", include_str!( "../../template/deploy/key/pack.sh" ), "./key" ),
        ( "Readme.md", include_str!( "../../template/deploy/key/Readme.md" ), "./key" ),
        // /terraform/
        ( "Dockerfile", include_str!( "../../template/deploy/terraform/Dockerfile" ), "./terraform" ),
        ( "Readme.md", include_str!( "../../template/deploy/terraform/Readme.md" ), "./terraform" ),
        // /terraform/gar
        ( "Readme.md", include_str!( "../../template/deploy/terraform/gar/Readme.md" ), "./terraform/gar" ),
        ( "main.tf", include_str!( "../../template/deploy/terraform/gar/main.tf" ), "./terraform/gar" ),
        ( "outputs.tf", include_str!( "../../template/deploy/terraform/gar/outputs.tf" ), "./terraform/gar" ),
        ( "variables.tf", include_str!( "../../template/deploy/terraform/gar/variables.tf" ), "./terraform/gar" ),
        // /terraform/gce
        ( "Readme.md", include_str!( "../../template/deploy/terraform/gce/Readme.md" ), "./terraform/gce" ),
        ( "main.tf", include_str!( "../../template/deploy/terraform/gce/main.tf" ), "./terraform/gce" ),
        ( "outputs.tf", include_str!( "../../template/deploy/terraform/gce/outputs.tf" ), "./terraform/gce" ),
        ( "variables.tf", include_str!( "../../template/deploy/terraform/gce/variables.tf" ), "./terraform/gce" ),
        // /terraform/gce/templates
        ( "cloud-init.tpl", include_str!( "../../template/deploy/terraform/gce/templates/cloud-init.tpl" ), "./terraform/gce/templates" ),
        // /terraform/gcs
        ( "main.tf", include_str!( "../../template/deploy/terraform/gcs/main.tf" ), "./terraform/gcs" ),
        // /terraform/hetzner
        ( "main.tf", include_str!( "../../template/deploy/terraform/hetzner/main.tf" ), "./terraform/hetzner" ),
        ( "outputs.tf", include_str!( "../../template/deploy/terraform/hetzner/outputs.tf" ), "./terraform/hetzner" ),
        ( "variables.tf", include_str!( "../../template/deploy/terraform/hetzner/variables.tf" ), "./terraform/hetzner" ),
        // /terraform/hetzner/templates
        ( "cloud-init.tpl", include_str!( "../../template/deploy/terraform/hetzner/templates/cloud-init.tpl" ), "./terraform/hetzner/templates" ),
      ];
      for (filename, data, path ) in templated_files
      {
        let file = DeployFileDescriptor::builder( filename )
          .data( data )
          .templated( true )
          .path( path )
          .build();
        files.push( file );
      }
      for (filename, data, path ) in non_templated_files
      {
        let file = DeployFileDescriptor::builder( filename )
          .data( data )
          .path( path )
          .build();
        files.push( file );
      } 

      Self(files)
    }
  }

  impl TemplateFiles< DeployFileDescriptor > for DeployTemplateFiles {}
  impl IntoIterator for DeployTemplateFiles
  {
    type Item = DeployFileDescriptor;
  
    type IntoIter = std::vec::IntoIter< Self::Item >;
  
    fn into_iter( self ) -> Self::IntoIter
    {
      self.0.into_iter()
    }
  }
  
  /// todo
  #[ derive( Debug ) ]
  pub struct DeployFileDescriptor
  {
    path : PathBuf,
    filename : String,
    data : &'static str,
    is_template : bool,
  }

  impl TemplateFileDescriptor for DeployFileDescriptor
  {
    fn new
    (
      path : PathBuf,
      filename : String,
      data : &'static str,
      is_template : bool,
    ) -> Self {
      Self
      {
        path,
        filename,
        data,
        is_template : is_template,
      }  
    }

    fn path( &self ) -> &Path
    {
      &self.path
    }
  
    fn filename( &self ) -> &str
    {
      &self.filename
    }
    
    fn data( &self ) -> &'static str
    {
      self.data
    }
    
    fn templated( &self ) -> bool
    {
      self.is_template
    }
    
    fn build_template( data : &'static str, values : &TemplateValues ) -> Result< String >
    {
      let mut handlebars = handlebars::Handlebars::new();
      handlebars.register_escape_fn( handlebars::no_escape );
      handlebars.register_template_string( "templated_file", data )?;
      handlebars.render( "templated_file", &values.to_serializable() ).context( "Failed creating a templated file" )
    }
    
  }

  /// Creates deploy template
  pub fn deploy_new
  (
    path: &Path,
    template: DeployTemplate
  ) -> Result< () >
  {
    template.create_all( path )?;
    Ok( () )
  }
}

crate::mod_interface!
{
  exposed use deploy_new;
  orphan use DeployTemplate;
}
