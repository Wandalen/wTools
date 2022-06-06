use test_tools_local::*;

tests_impls!
{
  fn dependencies_check()
  {
    let manifest_dir = std::path::PathBuf::from( env!( "CARGO_MANIFEST_DIR" ) );
    let mut manifest_path = manifest_dir.clone();
    manifest_path.push( "Cargo.toml" );
    let read = std::fs::read_to_string( &manifest_path ).unwrap();
    let manifest = read.parse::< toml_edit::Document >().unwrap();
    let dependencies = &manifest[ "dependencies" ];

    let root_dirs = [ "alias", "move", "rust" ];

    let non_existed = root_dirs.iter().filter_map( | path |
    {
      let mut current_root = manifest_dir.clone();
      current_root.push( ".." );
      current_root.push( ".." );
      current_root.push( path );
      let current_root = current_root.canonicalize().unwrap();

      let dirs = std::fs::read_dir( &current_root ).unwrap();
      let mut missed = vec![];
      for p in dirs
      {
        let entry = p.unwrap();
        let file_type = entry.file_type().unwrap();
        if file_type.is_dir()
        {
          let dependency_name = entry.file_name().into_string().unwrap();
          let mut local_dependency_name = dependency_name.clone();
          local_dependency_name.push_str( "_local" );
          if !local_dependency_name.starts_with( '_' )
          {
            if dependencies.get( &local_dependency_name ).is_none()
            {
              missed.push( ( path, dependency_name ) );
            }
          }
        }
      }

      if missed.len() > 0
      {
        return Some( missed );
      }

      None
    }).collect::< Vec< _ > >();

    /* */

    if non_existed.len() > 0
    {
      let header = format!
      (
        "\nManifest of integration test at {:?} has some missed dependencies.\nPlease, insert next code to manifest:\n```",
        manifest_path.to_str().unwrap()
      );
      let mut msg = String::from( header );

      for v in non_existed
      {
        for ( root, dependency ) in v
        {
          let dependencies_string = format!
          (
"
{dep}_local = {{ version = \"*\", path = \"../../{root}/{dep}\", package = \"{dep}\" }}
{dep}_published = {{ version = \"*\", package = \"{dep}\" }}
",
            root = root,
            dep = dependency
          );
          msg.push_str( &dependencies_string );
        }
      }
      msg.push_str( "```\n" );

      assert!( false, "{}", &msg );
    }
  }
}

tests_index!
{
  dependencies_check,
}
