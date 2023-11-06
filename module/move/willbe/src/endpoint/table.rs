mod private 
{
  use std::{ fs, path::PathBuf };
  use std::io::{ Read, self, Seek };
  use std::io::Write;
  use cargo_metadata::
  {
    MetadataCommand,
  };
  use wca::wtools::Itertools;
  use convert_case::Case;
  use convert_case::Casing;
  use std::fs::
  { 
    OpenOptions
  };

  use anyhow::*;

  /// Create table
  pub fn create_table() -> Result< () >
  {
    let workspace_root = get_workspace_root()?;
    let core_directories = get_directory_names( workspace_root.join( "module" ).join( "core" ) )?;
    let move_directories = get_directory_names( workspace_root.join( "module" ).join( "move" ) )?;
    let core_table = prepare_table( core_directories , "core".into() );
    let move_table = prepare_table( move_directories, "move".into() );
    write_tables_into_file( workspace_root.join( "Readme.md" ), vec![ core_table, move_table ] )?;
    Ok( () )
  }

  fn get_directory_names( path: PathBuf ) -> Result< Vec< String > >
  {
    let mut result = vec![];
    let entries = fs::read_dir( path )?;
    for entry in entries 
    {
      let entry = entry?;
      let path = entry.path();
      if path.is_dir() 
        {
          if let Some( dir_name ) = path.file_name() 
          {
            result.push( dir_name.to_string_lossy().into() );
          }
        }
    }
    Ok( result )
  }

  fn prepare_table( modules: Vec< String >, dir: String ) -> String
  {
    let table = modules
    .into_iter()
    .map
    (
      | ref module_name | 
      {
        let column_module = format!( "[{}](./module/{}/{})", &module_name, &dir, &module_name ); 
        let column_stability = format!( "[![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental)" );
        let column_master = format!( "[![rust-status](https://img.shields.io/github/actions/workflow/status/Wandalen/wTools/Module{}Push.yml?label=&branch=master)](https://github.com/Wandalen/wTools/actions/workflows/Module{}Push.yml)", &module_name.to_case( Case::Pascal ), &module_name.to_case( Case::Pascal ) );
        let column_alpha = format!( "[![rust-status](https://img.shields.io/github/actions/workflow/status/Wandalen/wTools/Module{}Push.yml?label=&branch=alpha)](https://github.com/Wandalen/wTools/actions/workflows/Module{}Push.yml)", &module_name.to_case( Case::Pascal ), &module_name.to_case( Case::Pascal ) );
        let column_docs = format!( "[![docs.rs](https://raster.shields.io/static/v1?label=&message=docs&color=eee)](https://docs.rs/{})", &module_name );
        let column_sample = format!( "[![Open in Gitpod](https://raster.shields.io/static/v1?label=&message=try&color=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2F{}_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20{}_trivial_sample/https://github.com/Wandalen/wTools)", &module_name, &module_name );
        format!( "| {} | {} | {} | {} | {} | {} |", column_module, column_stability, column_master, column_alpha, column_docs, column_sample )
      }
    )
    .join( "\n" );
    table
  }

  fn get_workspace_root() -> Result< PathBuf >
  {
    let metadata = MetadataCommand::new().no_deps().exec()?;
    Ok( metadata.workspace_root.into_std_path_buf() )
  }

  fn write_tables_into_file( file_path: PathBuf, params: Vec< String >) -> Result< () >
  {
    let header = "| Module | Stability | Master | Alpha | Docs | Online |\n|--------|-----------|--------|-------|:----:|:------:|\n";
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&file_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let updated_contents = contents.replace( "<!-- {{# generate.modules_index{core} #}} -->", &format!( "{}{}", &header,params[0] ) ).replace( "<!-- {{# generate.modules_index{move} #}} -->", &format!( "{}{}", &header, params[1] ) );

    file.set_len(0)?;  
    file.seek(io::SeekFrom::Start(0))?;

    file.write_all(updated_contents.as_bytes())?;

    Ok(())
  }
}
crate::mod_interface!
{
  /// Create Table.
  prelude use create_table;
}
