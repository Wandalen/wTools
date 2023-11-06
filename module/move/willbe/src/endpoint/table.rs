mod private 
{
  use core::fmt::Display;
  use std::{ fs, path::PathBuf };
  use std::io::BufReader;
  use std::io::BufWriter;
  use std::io::BufRead;
  use std::io::Write;
  use cargo_metadata::
  {
    MetadataCommand,
  };

  use lazy_format::prelude::*;

  use joinery::JoinableIterator;
  use std::fs::File;

  // use wtools::error::Result;
  use anyhow::*;

  /// Create table
  pub fn create_table() -> Result< () >
  {
    let workspace_root = get_workspace_root()?;
    let core_directories = get_directory_names( workspace_root.join( "module" ).join( "core" ) )?;
    let move_directories = get_directory_names( workspace_root.join( "module" ).join( "move" ) )?;
    let core_table = prepare_table(core_directories);
    let move_table = prepare_table(move_directories);
    write_tables_into_file(workspace_root.join( "Readme.md" ), vec![core_table, move_table])?;
    Ok( () )
  }

  fn get_directory_names(path: PathBuf) -> Result< Vec< String > >
  {
    let mut result = vec![];
    let entries = fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() 
        {
          if let Some(dir_name) = path.file_name() 
          {
            result.push( dir_name.to_string_lossy().into() );
          }
        }
    }
    Ok(result)
  }

  fn prepare_table( modules: Vec< String >) -> impl Display
  {
    let table = modules
    .into_iter()
    .map(|module_name| {
        lazy_format!("| {} | [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) | [![rust-status]({})]({}) | [![rust-status]({})]({}) | [![docs.rs](https://raster.shields.io/static/v1?label=&message=docs&color=eee)](https://docs.rs/{}) | [![Open in Gitpod](https://raster.shields.io/static/v1?label=&message=try&color=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2F{}_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20{}_trivial_sample/https://github.com/Wandalen/wTools) |", module_name, module_name, module_name, module_name, module_name, module_name, module_name, module_name)
    })
    .join_with("\n");
    table
  }

  fn get_workspace_root() -> Result< PathBuf >
  {
    let metadata = MetadataCommand::new().no_deps().exec()?;
    Ok(metadata.workspace_root.into_std_path_buf())
  }

  fn write_tables_into_file<T: Display>( file_path: PathBuf, params: Vec<T>) -> Result< () >
  {
    let file = File::open(&file_path)?;
    let temp_file_path = format!("temp{}.md", uuid::Uuid::new_v4());
    let temp_file = File::create(&temp_file_path)?;
    let mut reader = BufReader::new(file);
    let mut writer = BufWriter::new(temp_file);

    let mut line = String::new();
    let mut index = 0;
    while reader.read_line(&mut line)? > 0 {
        // Поиск и замена слова
        if line.contains( "KEYWORD") {
            line = line.replace("KEYWORD", &format!("| Module | Stability | Master | Alpha | Docs | Sample |\n|--------|-----------|--------|-------|:----:|:------:|\n{}", params[index]));
            index+=1;
        }
        writer.write_all(line.as_bytes())?;
        line.clear();
    }
    drop(reader);
    drop(writer);

    std::fs::remove_file(&file_path)?;
    std::fs::rename(&temp_file_path, &file_path)?;
    Ok(())
  }
}
crate::mod_interface!
{
  /// Create Table.
  prelude use create_table;
}
