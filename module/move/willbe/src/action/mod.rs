crate::mod_interface!
{
  /// Deploy new.
  layer deploy_renew;
  /// List packages.
  layer list;
  /// Main Header.
  layer main_header;
  /// Publish packages.
  layer publish;
  /// Return the differences between a local and remote package versions.
  layer publish_diff;
  /// Generates health table in main Readme.md file of workspace.
  // aaa : for Petro : give high quality explanations
  // aaa : add more details to description
  layer readme_health_table_renew;
  /// Module headers.
  layer readme_modules_headers_renew;
  /// Run all tests
  layer test;
  /// Workflow.
  layer cicd_renew;
  /// Workspace new.
  layer workspace_renew;
  /// List features.
  layer features;
}
