mod private
{
  use crate::*;
  use std::path::Path;
  use error::{ untyped::Context };
  use tool::template::*;

  /// Template for creating deploy files.
  ///
  /// Includes terraform deploy options to GCP, and Hetzner,
  /// a Makefile for useful commands, and a key directory.
  #[ derive( Debug ) ]
  pub struct DeployTemplate;

  impl DeployTemplate
  {
    /// Creates am instance of `[TemplateHolder]` for deployment template.
    /// 
    /// Used for properly initializing a template 
    pub fn default() -> TemplateHolder
    {
      let parameters = TemplateParameters::former()
      .parameter( "gcp_project_id" ).is_mandatory( true ).end()
      .parameter( "gcp_region" ).end()
      .parameter( "gcp_artifact_repo_name" ).end()
      .parameter( "docker_image_name" ).end()
      .form();

      TemplateHolder
      {
        files : get_deploy_template_files(),
        parameters,
        values : Default::default(),
        parameter_storage : "./.deploy_template.toml".as_ref(),
        template_name : "deploy",
      }
    }
  }

  fn get_deploy_template_files() -> Vec< TemplateFileDescriptor >
  {
    let formed = TemplateFilesBuilder::former()
    // root
    .file().data( include_str!( "../../template/deploy/.deploy_template.toml.hbs" ) ).path( "./.deploy_template.toml" ).mode( WriteMode::TomlExtend ).is_template( true ).end()
    .file().data( include_str!( "../../template/deploy/Makefile.hbs" ) ).path( "./Makefile" ).is_template( true ).end()
    // /key
    .file().data( include_str!( "../../template/deploy/key/pack.sh" ) ).path( "./key/pack.sh" ).end()
    .file().data( include_str!( "../../template/deploy/key/Readme.md" ) ).path( "./key/Readme.md" ).end()
    // /deploy/
    .file().data( include_str!( "../../template/deploy/deploy/Dockerfile" ) ).path( "./deploy/Dockerfile" ).end()
    .file().data( include_str!( "../../template/deploy/deploy/Readme.md" ) ).path( "./deploy/Readme.md" ).end()
    // /deploy/gar
    .file().data( include_str!( "../../template/deploy/deploy/gar/Readme.md" ) ).path( "./deploy/gar/Readme.md" ).end()
    .file().data( include_str!( "../../template/deploy/deploy/gar/main.tf" ) ).path( "./deploy/gar/main.tf" ).end()
    .file().data( include_str!( "../../template/deploy/deploy/gar/outputs.tf" ) ).path( "./deploy/gar/outputs.tf" ).end()
    .file().data( include_str!( "../../template/deploy/deploy/gar/variables.tf" ) ).path( "./deploy/gar/variables.tf" ).end()
    // /deploy/gce
    .file().data( include_str!( "../../template/deploy/deploy/gce/Readme.md" ) ).path( "./deploy/gce/Readme.md" ).end()
    .file().data( include_str!( "../../template/deploy/deploy/gce/main.tf" ) ).path( "./deploy/gce/main.tf" ).end()
    .file().data( include_str!( "../../template/deploy/deploy/gce/outputs.tf" ) ).path( "./deploy/gce/outputs.tf" ).end()
    .file().data( include_str!( "../../template/deploy/deploy/gce/variables.tf" ) ).path( "./deploy/gce/variables.tf" ).end()
    // /deploy/gce/templates
    .file().data( include_str!( "../../template/deploy/deploy/gce/templates/cloud-init.tpl" ) ).path( "./deploy/gce/templates/cloud-init.tpl" ).end()
    // /deploy/gcs
    .file().data( include_str!( "../../template/deploy/deploy/gcs/main.tf" ) ).path( "./deploy/gcs/main.tf" ).end()
    // /deploy/hetzner
    .file().data( include_str!( "../../template/deploy/deploy/hetzner/main.tf.hbs" ) ).path( "./deploy/hetzner/main.tf" ).is_template( true ).end()
    .file().data( include_str!( "../../template/deploy/deploy/hetzner/outputs.tf.hbs" ) ).path( "./deploy/hetzner/outputs.tf" ).is_template( true ).end()
    .file().data( include_str!( "../../template/deploy/deploy/hetzner/variables.tf" ) ).path( "./deploy/hetzner/variables.tf" ).end()
    // /deploy/hetzner/templates
    .file().data( include_str!( "../../template/deploy/deploy/hetzner/templates/cloud-init.tpl.hbs" ) ).path( "./deploy/hetzner/templates/cloud-init.tpl" ).end()
    // /deploy/aws
    .file().data( include_str!( "../../template/deploy/deploy/aws/main.tf" ) ).path( "./deploy/aws/main.tf" ).end()
    .file().data( include_str!( "../../template/deploy/deploy/aws/outputs.tf" ) ).path( "./deploy/aws/outputs.tf" ).end()
    .file().data( include_str!( "../../template/deploy/deploy/aws/variables.tf" ) ).path( "./deploy/aws/variables.tf" ).end()
    // /deploy/aws/templates
    .file().data( include_str!( "../../template/deploy/deploy/aws/templates/cloud-init.tpl.hbs" ) ).path( "./deploy/aws/templates/cloud-init.tpl" ).is_template( true ).end()
    .form();

    formed.files
  }

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
  )
  -> error::untyped::Result< () >
  // qqq : typed error
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
    template.files.create_all( path, &template.values )?;
    Ok( () )
  }

}

crate::mod_interface!
{
  orphan use deploy_renew;
  orphan use DeployTemplate;
}
