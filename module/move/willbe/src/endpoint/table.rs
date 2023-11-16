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
  use std::str::FromStr;

  use anyhow::*;

  lazy_static::lazy_static!{
    static ref TAG_TEMPLATE: regex::bytes::Regex = regex::bytes::Regex::new(r#"<!--\{ generate.healthtable\( '(\w+/\w+)' \) \} -->"#).unwrap();
    static ref CLOUSE_TAG: regex::bytes::Regex = regex::bytes::Regex::new(r#"<!--\{ generate\.healthtable\.end \} -->"#).unwrap();
  }

  /// Create table
  pub fn table_create() -> Result< () >
  {
    let workspace_root = workspace_root()?;
    let read_me_path = workspace_root.join("Readme.md");
    let mut file = OpenOptions::new()
      .read( true )
      .write( true )
      .open( &read_me_path )?;

    let mut contents = Vec::new();

    file.read_to_end( &mut contents )?;
    let mut buffer = vec![];
    let open_caps = TAG_TEMPLATE.captures_iter( &*contents );
    let close_caps = CLOUSE_TAG.captures_iter( &*contents );
    let mut tags_clousures = vec![];
    let mut tables = vec![];
    for c in open_caps.zip( close_caps )
    {
      for c in c.0.iter().zip( c.1.iter() )
      {
        if let ( Some( open ), Some( close ) ) = c
        {
          let path_content = &mut "".to_string();
          let _ = open.as_bytes().read_to_string(path_content);
          let module_path =
            PathBuf::from_str(
            std::str::from_utf8(
              TAG_TEMPLATE.captures(path_content.as_bytes())
                .ok_or(anyhow!("Fail to parse tag"))?
                .get(1)
                .ok_or(anyhow!("Fail to parse group"))?
                .as_bytes())?)?;
          tables.push(table_prepare(directory_names(&module_path)?, module_path.to_string_lossy().into()));
          tags_clousures.push( ( open.end(), close.start() ) );
        }
      }
    }
    let mut start: usize = 0;
    for ( t_c, con ) in tags_clousures.iter().zip( tables.iter() )
    {
      copy_range_to_target( &*contents, &mut buffer, start, t_c.0 )?;
      copy_range_to_target( con.as_bytes(), &mut buffer, 0,con.len()-1 )?;
      start = t_c.1;
    }
    copy_range_to_target( &*contents,&mut buffer,start,contents.len()-1 )?;

    file.set_len( 0 )?;
    file.seek( SeekFrom::Start( 0 ) )?;

    file.write_all( &buffer )?;

    Ok( () )
  }

  fn directory_names( path: &PathBuf ) -> Result< Vec< String > >
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
    let table_header = "| Module | Stability | Master | Alpha | Docs | Online |\n|--------|-----------|--------|-------|:----:|:------:|";
    let table_content = modules
    .into_iter()
    .map
    (
      | ref module_name | 
      {
        let column_module = format!( "[{}](./{}/{})", &module_name, &dir, &module_name );
        let column_stability = format!( "[![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental)" );
        let column_master = format!( "[![rust-status](https://img.shields.io/github/actions/workflow/status/Wandalen/wTools/Module{}Push.yml?label=&branch=master)](https://github.com/Wandalen/wTools/actions/workflows/Module{}Push.yml)", &module_name.to_case( Case::Pascal ), &module_name.to_case( Case::Pascal ) );
        let column_alpha = format!( "[![rust-status](https://img.shields.io/github/actions/workflow/status/Wandalen/wTools/Module{}Push.yml?label=&branch=alpha)](https://github.com/Wandalen/wTools/actions/workflows/Module{}Push.yml)", &module_name.to_case( Case::Pascal ), &module_name.to_case( Case::Pascal ) );
        let column_docs = format!( "[![docs.rs](https://raster.shields.io/static/v1?label=&message=docs&color=eee)](https://docs.rs/{})", &module_name );
        let column_sample = format!( "[![Open in Gitpod](https://raster.shields.io/static/v1?label=&message=try&color=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2F{}_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20{}_trivial_sample/https://github.com/Wandalen/wTools)", &module_name, &module_name );
        format!( "| {} | {} | {} | {} | {} | {} |", column_module, column_stability, column_master, column_alpha, column_docs, column_sample )
      }
    )
    .join( "\n" );
    format!( "{table_header}\n{table_content}\n" )
  }

  fn workspace_root() -> Result< PathBuf >
  {
    let metadata = MetadataCommand::new().no_deps().exec()?;
    Ok( metadata.workspace_root.into_std_path_buf() )
  }

  fn copy_range_to_target<T: Clone>(source: &[T], target: &mut Vec<T>, a: usize, b: usize) -> Result<()> {
    if a < source.len() && b < source.len() && a <= b {
      target.extend_from_slice(&source[a..=b]);
      return Ok( () )
    } else {
      bail!("Incorrect indexes")
    }
  }
}

crate::mod_interface!
{
  /// Create Table.
  prelude use table_create;
}
