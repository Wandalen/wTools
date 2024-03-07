mod private
{
  use crate::*;
  use std::path::Path;
  use error_tools::Result;
  use tool::template::*;

  /// Template for creating deploy files.
  ///
  /// Includes terraform deploy options to GCP, and Hetzner,
  /// a Makefile for useful commands, and a key directory.
  #[ derive( Debug ) ]
  pub struct DeployTemplate
  {
    files : DeployTemplateFiles,
    parameters : TemplateParameters,
    values : TemplateValues,
  }

  impl Template< DeployTemplateFiles > for DeployTemplate
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

  /// Files for the deploy template.
  ///
  /// Default implementation contains all required files.
  #[ derive( Debug ) ]
  pub struct DeployTemplateFiles( Vec< TemplateFileDescriptor > );

  impl Default for DeployTemplateFiles
  {
    fn default() -> Self
    {
      let formed = TemplateFilesBuilder::former()
      // root
      .file().data( include_str!( "../../template/deploy/Makefile" ) ).path( "./Makefile" ).is_template( true ).end()
      // /key
      .file().data( include_str!( "../../template/deploy/key/pack.sh" ) ).path( "./key/pack.sh" ).end()
      .file().data( include_str!( "../../template/deploy/key/Readme.md" ) ).path( "./key/Readme.md" ).end()
      // /terraform/
      .file().data( include_str!( "../../template/deploy/terraform/Dockerfile" ) ).path( "./terraform/Dockerfile" ).end()
      .file().data( include_str!( "../../template/deploy/terraform/Readme.md" ) ).path( "./terraform/Readme.md" ).end()
      // /terraform/gar
      .file().data( include_str!( "../../template/deploy/terraform/gar/Readme.md" ) ).path( "./terraform/gar/Readme.md" ).end()
      .file().data( include_str!( "../../template/deploy/terraform/gar/main.tf" ) ).path( "./terraform/gar/main.tf" ).end()
      .file().data( include_str!( "../../template/deploy/terraform/gar/outputs.tf" ) ).path( "./terraform/gar/outputs.tf" ).end()
      .file().data( include_str!( "../../template/deploy/terraform/gar/variables.tf" ) ).path( "./terraform/gar/variables.tf" ).end()
      // /terraform/gce
      .file().data( include_str!( "../../template/deploy/terraform/gce/Readme.md" ) ).path( "./terraform/gce/Readme.md" ).end()
      .file().data( include_str!( "../../template/deploy/terraform/gce/main.tf" ) ).path( "./terraform/gce/main.tf" ).end()
      .file().data( include_str!( "../../template/deploy/terraform/gce/outputs.tf" ) ).path( "./terraform/gce/outputs.tf" ).end()
      .file().data( include_str!( "../../template/deploy/terraform/gce/variables.tf" ) ).path( "./terraform/gce/variables.tf" ).end()
      // /terraform/gce/templates
      .file().data( include_str!( "../../template/deploy/terraform/gce/templates/cloud-init.tpl" ) ).path( "./terraform/gce/templates/cloud-init.tpl" ).end()
      // /terraform/gcs
      .file().data( include_str!( "../../template/deploy/terraform/gcs/main.tf" ) ).path( "./terraform/gcs/main.tf" ).end()
      // /terraform/hetzner
      .file().data( include_str!( "../../template/deploy/terraform/hetzner/main.tf" ) ).path( "./terraform/hetzner/main.tf" ).end()
      .file().data( include_str!( "../../template/deploy/terraform/hetzner/outputs.tf" ) ).path( "./terraform/hetzner/outputs.tf" ).end()
      .file().data( include_str!( "../../template/deploy/terraform/hetzner/variables.tf" ) ).path( "./terraform/hetzner/variables.tf" ).end()
      // /terraform/hetzner/templates
      .file().data( include_str!( "../../template/deploy/terraform/hetzner/templates/cloud-init.tpl" ) ).path( "./terraform/hetzner/templates/cloud-init.tpl" ).end()
      .form();

      Self( formed.files )
    }
  }

  impl TemplateFiles for DeployTemplateFiles {}
  impl IntoIterator for DeployTemplateFiles
  {
    type Item = TemplateFileDescriptor;

    type IntoIter = std::vec::IntoIter< Self::Item >;

    fn into_iter( self ) -> Self::IntoIter
    {
      self.0.into_iter()
    }
  }

  /// Creates deploy template
  pub fn deploy_renew
  (
    path : &Path,
    template : DeployTemplate
  ) -> Result< () >
  {
    template.create_all( path )?;
    Ok( () )
  }

}

crate::mod_interface!
{
  orphan use deploy_renew;
  orphan use DeployTemplate;
}
