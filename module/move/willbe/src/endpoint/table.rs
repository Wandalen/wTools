mod private
{
  use std::
  { 
    fs, 
    path::PathBuf
  };
  use std::io::
  { 
    Read, 
    Seek,
    SeekFrom 
  };
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
  use std::path::Path;

  use error_tools::for_app::
  {
    Result,
    anyhow,
  };
  


  /// Create table
  pub fn table_create() -> Result< () >
  {
    let workspace_root = workspace_root()?;
    let core_directories = directory_names( workspace_root.join( "module" ).join( "core" ) )?;
    let move_directories = directory_names( workspace_root.join( "module" ).join( "move" ) )?;
    let core_table = table_prepare( core_directories , "core".into() );
    let move_table = table_prepare( move_directories, "move".into() );
    let read_me_path = readme_path(&workspace_root).ok_or( anyhow!("Cannot found README.md file") )?;
    tables_write_into_file( read_me_path, vec![ core_table, move_table ] )?;
    Ok( () )
  }

  fn directory_names( path: PathBuf ) -> Result< Vec< String > >
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

  fn table_prepare( modules: Vec< String >, dir: String ) -> String
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

  fn workspace_root() -> Result< PathBuf >
  {
    let metadata = MetadataCommand::new().no_deps().exec()?;
    Ok( metadata.workspace_root.into_std_path_buf() )
  }

  fn tables_write_into_file( file_path: PathBuf, params: Vec< String >) -> Result< () >
  {
    let header = "| Module | Stability | Master | Alpha | Docs | Online |\n|--------|-----------|--------|-------|:----:|:------:|\n";

    let mut file = OpenOptions::new()
    .read( true )
    .write( true )
    .open( &file_path )?;

    let mut contents = Vec::new();
    file.read_to_end( &mut contents )?;

    let core_old_text = "<!-- {{# generate.modules_index{core} #}} -->";
    let core_new_text = &format!( "{core_old_text}\n{}{}", &header, params[ 0 ] );
    let move_old_text = "<!-- {{# generate.modules_index{move} #}} -->";
    let move_new_text = &format!( "{move_old_text}\n{}{}", &header, params[ 0 ] );

    let updated_contents = contents
    .windows(core_old_text.len())
    .enumerate()
    .fold(Vec::new(), | mut acc, ( index, window ) |
    {
      match ( window == core_old_text.as_bytes(), window == move_old_text.as_bytes() )
      {
        ( true, false ) => acc.extend_from_slice( core_new_text.as_bytes() ),
        ( false, true ) => acc.extend_from_slice( move_new_text.as_bytes() ),
        ( false, false ) | ( true, true ) => acc.push( contents[ index ] ),
      }
      acc
    });

    file.set_len( 0 )?;
    file.seek( SeekFrom::Start( 0 ) )?;

    file.write_all( &updated_contents )?;

    Ok( () )
  }

  /// Searches for a README file in specific subdirectories of the given directory path.
  ///
  /// This function attempts to find a README file in the following subdirectories: ".github",
  /// the root directory, and "./docs". It returns the path to the first found README file, or
  /// `None` if no README file is found in any of these locations.
  fn readme_path( dir_path : &Path ) -> Option< PathBuf >
  {
    if let Some( path )  = readme_in_dir_find(&dir_path.join( ".github" ))
    {
      Some( path )
    } 
    else if let Some( path )  = readme_in_dir_find( dir_path )
    {
      Some( path )
    }
    else if let Some( path )  = readme_in_dir_find( &dir_path.join( "docs" ) )
    {
      Some( path )
    }
    else
    {
      None
    }
  }


  /// Searches for a file named "readme.md" in the specified directory path.
  ///
  /// Given a directory path, this function searches for a file named "readme.md" in the specified
  /// directory.
  fn readme_in_dir_find( path: &Path ) -> Option< PathBuf >
  {
    fs::read_dir( path )
    .ok()?
    .filter_map( Result::ok )
    .filter( | p | p.path().is_file() )
    .filter_map( | f |
    {
      let l_f = f.file_name().to_ascii_lowercase();
      if l_f == "readme.md"
      {
        return Some( f.file_name() )
      }
      None
    })
    .max()
    .map( PathBuf::from )
  }
}



crate::mod_interface!
{
  /// Create Table.
  prelude use table_create;
}
