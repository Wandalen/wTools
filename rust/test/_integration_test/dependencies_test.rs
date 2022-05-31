use test_tools_local::*;

tests_impls!
{
  fn dependencies_check()
  {
    let manifest_dir = std::path::PathBuf::from( env!( "CARGO_MANIFEST_DIR" ) );
    let mut manifest_path = manifest_dir.clone();
    manifest_path.push( "Cargo.toml" );
    let read = std::fs::read_to_string( manifest_path ).unwrap();
    let manifest = read.parse::< toml_edit::Document >().unwrap();
    let dependencies = &manifest[ "dependencies" ];

    let root_dirs = [ "alias", "move", "rust" ];

    let _ = root_dirs.iter().map( | path |
    {
      let mut current_root = manifest_dir.clone();
      current_root.push( ".." );
      current_root.push( ".." );
      current_root.push( path );
      let current_root = current_root.canonicalize().unwrap();

      let dirs = std::fs::read_dir( &current_root ).unwrap();
      for p in dirs
      {
        let entry = p.unwrap();
        let file_type = entry.file_type().unwrap();
        if file_type.is_dir()
        {
          let mut dependency_name = entry.file_name().into_string().unwrap();
          dependency_name.push_str( "_local" );
          if !dependency_name.starts_with( '_' )
          {
            assert!
            (
              dependencies.get( &dependency_name ).is_some(),
              "local version of dependency :: {} is not included",
              &dependency_name
            );
          }
        }
      }
    }).collect::< Vec< _ > >();
  }
}

tests_index!
{
  dependencies_check,
}
