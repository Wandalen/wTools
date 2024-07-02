mod private
{
  use crate::*;
  use std::path::Path;
  use error::{ untyped::Context, Result };
  use tool::template::*;

  // /// Template for creating deploy files.
  // ///
  // /// Includes terraform deploy options to GCP, and Hetzner,
  // /// a Makefile for useful commands, and a key directory.
  // #[ derive( Debug ) ]
  // pub struct DeployTemplate
  // {
  //   files : DeployTemplateFiles,
  //   parameters : TemplateParameters,
  //   values : TemplateValues,
  // }

  // // qqq : for Viktor : why DeployTemplate can't be part of template.rs?

  // impl Template< DeployTemplateFiles > for DeployTemplate
  // {
  //   fn create_all( self, path : &Path ) -> Result< () >
  //   {
  //     self.files.create_all( path, &self.values )
  //   }

  //   fn parameters( &self ) -> &TemplateParameters
  //   {
  //     &self.parameters
  //   }

  //   fn set_values( &mut self, values : TemplateValues )
  //   {
  //     self.values = values
  //   }

  //   fn get_values( &self ) -> &TemplateValues
  //   {
  //     &self.values
  //   }

  //   fn get_values_mut( &mut self ) -> &mut TemplateValues
  //   {
  //     &mut self.values
  //   }

  //   fn parameter_storage( &self ) -> &Path {
  //     "./.deploy_template.toml".as_ref()
  //   }

  //   fn template_name( &self ) -> &'static str {
  //     "deploy"
  //   }
  // }

  // impl Default for DeployTemplate
  // {
  //   fn default() -> Self
  //   {
  //     let parameters = TemplateParameters::former()
  //     .parameter( "gcp_project_id" ).is_mandatory( true ).end()
  //     .parameter( "gcp_region" ).end()
  //     .parameter( "gcp_artifact_repo_name" ).end()
  //     .parameter( "docker_image_name" ).end()
  //     .form();

  //     Self
  //     {
  //       files : Default::default(),
  //       parameters,
  //       values : Default::default(),
  //     }
  //   }
  // }

  // // qqq : for Viktor : is that structure required?
  // /// Files for the deploy template.
  // ///
  // /// Default implementation contains all required files.
  // #[ derive( Debug ) ]
  // pub struct DeployTemplateFiles( Vec< TemplateFileDescriptor > );

  // impl Default for DeployTemplateFiles
  // {
  //   fn default() -> Self
  //   {
  //     let formed = TemplateFilesBuilder::former()
  //     // root
  //     .file().data( include_str!( "../../template/deploy/.deploy_template.toml.hbs" ) ).path( "./.deploy_template.toml" ).mode( WriteMode::TomlExtend ).is_template( true ).end()
  //     .file().data( include_str!( "../../template/deploy/Makefile.hbs" ) ).path( "./Makefile" ).is_template( true ).end()
  //     // /key
  //     .file().data( include_str!( "../../template/deploy/key/pack.sh" ) ).path( "./key/pack.sh" ).end()
  //     .file().data( include_str!( "../../template/deploy/key/Readme.md" ) ).path( "./key/Readme.md" ).end()
  //     // /deploy/
  //     .file().data( include_str!( "../../template/deploy/deploy/Dockerfile" ) ).path( "./deploy/Dockerfile" ).end()
  //     .file().data( include_str!( "../../template/deploy/deploy/Readme.md" ) ).path( "./deploy/Readme.md" ).end()
  //     // /deploy/gar
  //     .file().data( include_str!( "../../template/deploy/deploy/gar/Readme.md" ) ).path( "./deploy/gar/Readme.md" ).end()
  //     .file().data( include_str!( "../../template/deploy/deploy/gar/main.tf" ) ).path( "./deploy/gar/main.tf" ).end()
  //     .file().data( include_str!( "../../template/deploy/deploy/gar/outputs.tf" ) ).path( "./deploy/gar/outputs.tf" ).end()
  //     .file().data( include_str!( "../../template/deploy/deploy/gar/variables.tf" ) ).path( "./deploy/gar/variables.tf" ).end()
  //     // /deploy/gce
  //     .file().data( include_str!( "../../template/deploy/deploy/gce/Readme.md" ) ).path( "./deploy/gce/Readme.md" ).end()
  //     .file().data( include_str!( "../../template/deploy/deploy/gce/main.tf" ) ).path( "./deploy/gce/main.tf" ).end()
  //     .file().data( include_str!( "../../template/deploy/deploy/gce/outputs.tf" ) ).path( "./deploy/gce/outputs.tf" ).end()
  //     .file().data( include_str!( "../../template/deploy/deploy/gce/variables.tf" ) ).path( "./deploy/gce/variables.tf" ).end()
  //     // /deploy/gce/templates
  //     .file().data( include_str!( "../../template/deploy/deploy/gce/templates/cloud-init.tpl" ) ).path( "./deploy/gce/templates/cloud-init.tpl" ).end()
  //     // /deploy/gcs
  //     .file().data( include_str!( "../../template/deploy/deploy/gcs/main.tf" ) ).path( "./deploy/gcs/main.tf" ).end()
  //     // /deploy/hetzner
  //     .file().data( include_str!( "../../template/deploy/deploy/hetzner/main.tf" ) ).path( "./deploy/hetzner/main.tf" ).end()
  //     .file().data( include_str!( "../../template/deploy/deploy/hetzner/outputs.tf" ) ).path( "./deploy/hetzner/outputs.tf" ).end()
  //     .file().data( include_str!( "../../template/deploy/deploy/hetzner/variables.tf" ) ).path( "./deploy/hetzner/variables.tf" ).end()
  //     // /deploy/hetzner/templates
  //     .file().data( include_str!( "../../template/deploy/deploy/hetzner/templates/cloud-init.tpl" ) ).path( "./deploy/hetzner/templates/cloud-init.tpl" ).end()
  //     // /deploy/aws
  //     .file().data( include_str!( "../../template/deploy/deploy/aws/main.tf" ) ).path( "./deploy/aws/main.tf" ).end()
  //     .file().data( include_str!( "../../template/deploy/deploy/aws/outputs.tf" ) ).path( "./deploy/aws/outputs.tf" ).end()
  //     .file().data( include_str!( "../../template/deploy/deploy/aws/variables.tf" ) ).path( "./deploy/aws/variables.tf" ).end()
  //     // /deploy/aws/templates
  //     .file().data( include_str!( "../../template/deploy/deploy/aws/templates/cloud-init.tpl" ) ).path( "./deploy/aws/templates/cloud-init.tpl" ).end()
  //     .form();

  //     Self( formed.files )
  //   }
  // }

  // // qqq : for Viktor : should not be required
  // impl TemplateFiles for DeployTemplateFiles {}
  // // qqq : for Viktor : should not be required
  // impl IntoIterator for DeployTemplateFiles
  // {
  //   type Item = TemplateFileDescriptor;

  //   type IntoIter = std::vec::IntoIter< Self::Item >;

  //   fn into_iter( self ) -> Self::IntoIter
  //   {
  //     self.0.into_iter()
  //   }
  // }

  // aaa : for Petro : redundant function
  // aaa : this function not my, but ok I'll remove it.

  fn dir_name_to_formatted( dir_name : &str, separator : &str ) -> String
  {
    dir_name
    .replace( ' ', separator )
    .replace( '_', separator )
    .to_lowercase()
  }

  /// Creates deploy template
  pub fn deploy_renew
  (
    path : &Path,
    mut template : TemplateHolder
  ) -> Result< () >
  {
    if let None = template.load_existing_params( path )
    {
      let current_dir = std::env::current_dir()?;
      // qqq : for Petro : use file_name
      // qqq : for Kos : bad description
      let current_dir = current_dir
      .components()
      .last()
      .context( "Invalid current directory" )?;

      let current_dir = current_dir.as_os_str().to_string_lossy();
      let artifact_repo_name = dir_name_to_formatted( &current_dir, "-" );
      let docker_image_name = dir_name_to_formatted( &current_dir, "_" );
      template
      .values
      .insert_if_empty( "gcp_artifact_repo_name", wca::Value::String( artifact_repo_name ) );

      template
      .values
      .insert_if_empty( "docker_image_name", wca::Value::String( docker_image_name ) );
      template
      .values
      .insert_if_empty( "gcp_region", wca::Value::String( "europe-central2".into() ) );
    }
    template.create_all( path )?;
    Ok( () )
  }

}

crate::mod_interface!
{
  orphan use deploy_renew;
  //orphan use DeployTemplate;
}
