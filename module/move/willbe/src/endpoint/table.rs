mod private
{
  use std::
  { 
    fs, 
    path::PathBuf
  };
  use std::collections::HashMap;
  use std::io::
  { 
    Read, 
    Seek,
    SeekFrom 
  };
  use std::io::Write;
  use cargo_metadata::
  {
    Dependency,
    DependencyKind,
    MetadataCommand,
    Package
  };
  use wca::wtools::Itertools;
  use convert_case::Case;
  use convert_case::Casing;
  use std::fs::OpenOptions;
  use std::path::Path;
  use std::str::FromStr;

  use error_tools::for_app::
  {
    Error,
    Result,
    bail,
  };
  use anyhow::anyhow;
  use crate::package::functions;
  use crate::package::functions::FilterMapOptions;
  use walkdir::WalkDir;
  use toml::Value;


  lazy_static::lazy_static!
  {
    static ref TAG_TEMPLATE: regex::bytes::Regex = regex::bytes::Regex::new( r#"<!--\{ generate.healthtable\( (path\s*:\s*[\w\/]+(\s*,\s*\w+\s*:\s*\w+)*) \) \} -->"# ).unwrap();
    static ref CLOUSE_TAG: regex::bytes::Regex = regex::bytes::Regex::new( r#"<!--\{ generate\.healthtable\.end \} -->"# ).unwrap();
  }

  #[ derive( Debug ) ]
  enum Stability
  {
    Stable,
    Experimental,
    Deprecated,
  }

  impl FromStr for Stability
  {
    type Err = Error;

    fn from_str( s: &str ) -> Result< Self, Self::Err >
    {
      match s
      {
        "stable" => Ok( Stability::Stable ),
        "experimental" => Ok( Stability::Experimental ),
        "deprecated" => Ok( Stability::Deprecated ),
        _ => Err( anyhow!( "Fail to parse stability" ) ),
      }
    }
  }

  fn get_stable_status( directories: &Vec< String >, dir: &Path ) -> Result< Vec< Stability  > >
  {
    let mut results = Vec::new();

    for directory in directories
    {
      for entry in WalkDir::new( dir.join(directory ) )
      {
        let entry = entry?;
        if entry.file_name() == "Cargo.toml"
        {
          let contents = fs::read_to_string( entry.path() )?;
          let value = contents.parse::< Value >()?;
          let stable_status = value
          .get( "package" )
          .and_then( | package | package.get( "metadata" ) )
          .and_then( | package | package.get( "stable_status" ) )
          .and_then( Value::as_str )
          .and_then( | s | s.parse::< Stability >().ok() );
          results.push( stable_status.unwrap_or( Stability::Stable ) );
        }
      }
    }
    Ok( results )
  }


  #[ derive( Debug ) ]
  struct GlobalTableParameters
  {
    core_url: String,
    user_and_repo: String,
    branches: Option< Vec< String > >,
  }

  #[ derive( Debug ) ]
  struct TableParameters
  {
    base_path: String,
    include_branches: bool,
    include_stability: bool,
    include_docs: bool,
    include_sample: bool,
  }

  impl From< HashMap< String, functions::Value > > for TableParameters
  {
    fn from(value: HashMap< String, functions::Value >) -> Self
    {
      let include_branches = value.get( "with_branches" ).map( | val | bool::from( val ) ).unwrap_or( true );
      let include_stability = value.get( "with_stability" ).map( | val | bool::from( val ) ).unwrap_or( true );
      let include_docs = value.get( "with_docs" ).map( | val | bool::from( val ) ).unwrap_or( true );
      let include_sample = value.get( "with_gitpod" ).map( | val | bool::from( val ) ).unwrap_or( true );
      let base_path = if let Some( functions::Value::StringValue( path ) ) = value.get( "path" )
      {
        path.as_ref()
      }
      else
      {
        "./"
      };
      Self { base_path: base_path.to_string(), include_branches, include_stability, include_docs, include_sample }
    }
  }


  impl GlobalTableParameters
  {
    fn new( path: &Path ) -> Result< Self >
    {
      let cargo_toml_path = path.join( "Cargo.toml" );
      if !cargo_toml_path.exists()
      {
        bail!( "Cannot find Cargo.toml" )
      }
      else
      {
        let contents = fs::read_to_string( cargo_toml_path )?;
        let value = contents.parse::< Value >()?;

        let core_url = value
        .get( "workspace" )
        .and_then( |workspace | workspace.get( "metadata" ) )
        .and_then( |metadata | metadata.get( "repo_url" ) )
        .and_then( Value::as_str )
        .map( String::from )
        .ok_or_else( || anyhow!( "Fail to find repo_url" ) )?;

        let branches = value
        .get( "workspace" )
        .and_then( | workspace | workspace.get( "metadata" ) )
        .and_then( | package | package.get( "branches" ) )
        .and_then( Value::as_array )
        .map
        (
          |array|
          array
          .iter()
          .filter_map( Value::as_str )
          .map( String::from )
          .collect()
        );

        let user_and_repo = Self::extract_repo( &core_url )?;

        Ok( Self { core_url, user_and_repo, branches } )
      }
    }

    fn extract_repo( url: &String ) -> Result< String >
    {
      let parts: Vec< &str > = url.split( '/' ).collect();
      if parts.len() >= 2
      {
        Ok( format!( "{}/{}", parts[ parts.len() - 2 ], parts[ parts.len() - 1 ] ) )
      }
      else
      {
        Err( anyhow!( "Fail to extract  git username and repository name" ) )
      }
    }

  }


  /// Create health table in README.md file
  ///
  /// The location and filling of tables is defined by a tag, for example record:
  /// ```md
  /// <!--{ generate.healthtable( 'module/core' ) } -->
  /// <!--{ generate.healthtable.end } -->
  /// ```
  /// will mean that at this place the table with modules located in the directory module/core will be generated.
  /// The tags do not disappear after generation.
  /// Anything between the opening and closing tag will be destroyed.
  pub fn table_create() -> Result< () >
  {
    let cargo_metadata = MetadataCommand::new().no_deps().exec()?;
    let workspace_root = workspace_root( &cargo_metadata )?;
    let parameters = GlobalTableParameters::new( &workspace_root )?;

    let read_me_path = readme_path(&workspace_root).ok_or_else( || anyhow!( "Fail to find README.md" ) )?;
    let mut file = OpenOptions::new()
    .read( true )
    .write( true )
    .open( &read_me_path )?;


    let mut contents = Vec::new();

    file.read_to_end( &mut contents )?;
    let mut buffer = vec![];
    let mut tags_closures = vec![];
    let mut tables = vec![];
    let open_caps = TAG_TEMPLATE.captures_iter( &*contents );
    let close_caps = CLOUSE_TAG.captures_iter( &*contents );
    // iterate by regex matches and generate table content for each dir which taken from open-tag
    for ( open_captures, close_captures ) in open_caps.zip( close_caps )
    {
      for captures in open_captures.iter().zip( close_captures.iter() )
      {
        if let ( Some( open ), Some( close ) ) = captures
        {
          let raw_table_params = std::str::from_utf8
          (
            TAG_TEMPLATE.captures( open.as_bytes() )
            .ok_or( anyhow!( "Fail to parse tag" ) )?
            .get( 1 )
            .ok_or( anyhow!( "Fail to parse group" ) )?
            .as_bytes()
          )?;
          let params: TableParameters  = functions::parse_string( raw_table_params ).into();
          let directory_names = directory_names( workspace_root.join( &params.base_path ), &cargo_metadata.packages );
          let stability = if params.include_stability
          {
            Some( get_stable_status(&directory_names, &workspace_root.join( &params.base_path ) )? )
          }
          else
          {
            None
          };
          let table = table_prepare(directory_names, stability, &parameters, &params );
          tables.push( table );
          tags_closures.push( ( open.end(), close.start() ) );
        }
      }
    }
    let mut start: usize = 0;
    for ( ( end_of_start_tag, start_of_end_tag ), con ) in tags_closures.iter().zip( tables.iter() )
    {
      copy_range_to_target( &*contents, &mut buffer, start, *end_of_start_tag )?;
      copy_range_to_target( con.as_bytes(), &mut buffer, 0,con.len() - 1 )?;
      start = *start_of_end_tag;
    }
    copy_range_to_target( &*contents,&mut buffer,start,contents.len() - 1 )?;

    file.set_len( 0 )?;
    file.seek( SeekFrom::Start( 0 ) )?;

    file.write_all( &buffer )?;

    Ok( () )
  }

  fn directory_names( path: PathBuf, packages: &[ Package ] ) -> Vec< String >
  {
    let path_clone = path.clone();
    let module_package_filter: Option< Box< dyn Fn( &Package ) -> bool > > = Some
    (
      Box::new
      (
        move | p |
        {
          p.publish.is_none() && p.manifest_path.starts_with( &path )
        }
      )
    );
    let module_dependency_filter: Option< Box< dyn Fn( &Package, &Dependency) -> bool > > = Some
    (
      Box::new
        (
          move | _, d |
          {
            d.path.is_some() && d.kind != DependencyKind::Development && d.path.as_ref().unwrap().starts_with( &path_clone )
          }
        )
    );
    let module_packages_map = functions::packages_filter_map
    (
      packages,
      FilterMapOptions{ package_filter: module_package_filter, dependency_filter: module_dependency_filter },
    );
    let module_graph = functions::graph_build( &module_packages_map);
    functions::toposort( module_graph )
  }

  fn table_prepare(modules: Vec< String >, stability: Option< Vec< Stability > >, parameters: &GlobalTableParameters, table_parameters: &TableParameters ) -> String
  {
    let table_header = generate_table_header( &parameters, table_parameters );
    let table_content = modules
    .iter()
    .enumerate()
    .map
    (
      | ( index, module_name) |
      {
        let mut rou = format!( "| [{}]({}/{}) |", &module_name, &table_parameters.base_path, &module_name );
        if table_parameters.include_stability
        {
          rou.push_str( &generate_stability( &stability.as_ref().unwrap()[ index ] ) );
        }
        if parameters.branches.is_some() && table_parameters.include_branches
        {
          rou.push_str( &generate_branch_cells( &parameters, &module_name ) );
        }
        if table_parameters.include_docs
        {
          rou.push_str( &format!( "[![docs.rs](https://raster.shields.io/static/v1?label=&message=docs&color=eee)](https://docs.rs/{}) | ", &module_name ) );
        }
        if table_parameters.include_sample
        {
          rou.push_str(&format!( "[![Open in Gitpod](https://raster.shields.io/static/v1?label=&message=try&color=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2F{}_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20{}_trivial_sample/{}) | ", &module_name, &module_name, parameters.core_url ) );
        }
        rou
      }
    )
    .join( "\n" );
    format!( "{table_header}\n{table_content}\n" )
  }

  fn generate_stability( stability: &Stability ) -> String
  {
    match stability
    {
      Stability::Experimental => "[![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) | ".into(),
      Stability::Stable => "[![stable](https://raster.shields.io/static/v1?label=&message=stable&color=green)](https://github.com/emersion/stability-badges#stable) | ".into(),
      Stability::Deprecated => "[![deprecated](https://raster.shields.io/static/v1?label=&message=deprecated&color=grey)](https://github.com/emersion/stability-badges#deprecated) | ".into()
    }
  }

  fn generate_table_header( parameters: &GlobalTableParameters, table_parameters: &TableParameters ) -> String
  {
    let mut header = String::from( "| Module |" );
    let mut separator = String::from( "|--------|" );

    if table_parameters.include_stability
    {
      header.push_str( " Stability |" );
      separator.push_str( "-----------|" );
    }

    if let Some( branches ) = &parameters.branches
    {
      if table_parameters.include_branches
      {
        for branch in branches
        {
          header.push_str( format!( " {} |", branch ).as_str() );
          separator.push_str( "--------|" );
        }
      }
    }

    if table_parameters.include_docs
    {
      header.push_str( " Docs |" );
      separator.push_str( ":----:|" );
    }

    if table_parameters.include_sample
    {
      header.push_str( " Sample |" );
      separator.push_str( ":------:|" );
    }

    format!( "{}\n{}", header, separator )
  }

  fn generate_branch_cells( table_parameters: &GlobalTableParameters, module_name: &String ) -> String
  {
    let cells = table_parameters
    .branches
    .as_ref()
    .unwrap()
    .iter()
    .map
    (
      | b |
      format!( "[![rust-status](https://img.shields.io/github/actions/workflow/status/{}/Module{}Push.yml?label=&branch={b})](https://{}/actions/workflows/Module{}Push.yml)", table_parameters.user_and_repo, &module_name.to_case( Case::Pascal ), table_parameters.core_url, &module_name.to_case( Case::Pascal ) )
    )
    .collect::< Vec< String > >()
    .join( " | " );
    format!( "{cells} | " )
  }

  fn workspace_root( metadata: &cargo_metadata::Metadata ) -> Result< PathBuf >
  {
    Ok( metadata.workspace_root.clone().into_std_path_buf() )
  }

  fn copy_range_to_target< T: Clone >( source: &[T], target: &mut Vec< T >, from: usize, to: usize ) -> Result< () >
  {
    if from < source.len() && to < source.len() && from <= to
    {
      target.extend_from_slice( &source[ from..= to ] );
      return Ok( () )
    }
    else
    {
      bail!( "Incorrect indexes" )
    }
  }

  /// Searches for a README file in specific subdirectories of the given directory path.
  ///
  /// This function attempts to find a README file in the following subdirectories: ".github",
  /// the root directory, and "./docs". It returns the path to the first found README file, or
  /// `None` if no README file is found in any of these locations.
  fn readme_path( dir_path : &Path ) -> Option< PathBuf >
  {
    if let Some( path )  = readme_in_dir_find( &dir_path.join( ".github" ) )
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
