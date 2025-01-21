#[ allow( clippy::std_instead_of_alloc, clippy::std_instead_of_core ) ]
mod private
{
  #[ allow( clippy::wildcard_imports ) ]
  use crate::*;

  use std::
  {
    fs::{ OpenOptions, File },
    io::{ Write, Read, Seek, SeekFrom },
  };
  use path::{ Path, PathBuf };
  use convert_case::Casing;
  use toml_edit::Document;
  use regex::bytes::Regex;
  use collection::HashMap;

  use error::
  {
    Error,
    untyped::
    {
      // Error as wError, // xxx
      // Result,
      Context,
      format_err,
    }
  };
  use manifest::repo_url;
  // use path::AbsolutePath;

  static TAG_TEMPLATE: std::sync::OnceLock< Regex > = std::sync::OnceLock::new();
  static CLOSE_TAG: std::sync::OnceLock< Regex > = std::sync::OnceLock::new();

  /// Initializes two global regular expressions that are used to match tags.
  fn regexes_initialize()
  {
    TAG_TEMPLATE.set
    (
      regex::bytes::Regex::new
      (
        r"<!--\{ generate.healthtable(\(\)|\{\}|\(.*?\)|\{.*?\}) \} -->"
      ).unwrap()
    ).ok();
    CLOSE_TAG.set
    (
      regex::bytes::Regex::new
      (
        r"<!--\{ generate\.healthtable\.end \} -->"
      ).unwrap()
    ).ok();
  }

  #[ derive( Debug, Error )]
  pub enum HealthTableRenewError
  {
    // qqq : rid of the branch
    #[ error( "Common error: {0}" ) ]
    Common( #[ from ] error::untyped::Error ),
    #[ error( "I/O error: {0}" ) ]
    IO( #[ from ] std::io::Error ),
    #[ error( "Path error: {0}" ) ]
    Path( #[ from ] PathError ),
    #[ error( "Workspace error: {0}" ) ]
    Workspace( #[ from ] WorkspaceInitError ),
    #[ error( "Utf8Error error: {0}" ) ]
    Utf8Error( #[ from ] std::str::Utf8Error ),
    #[ error( "Toml edit error: {0}" ) ]
    Toml( #[ from ] toml_edit::TomlError )
  }

  /// `Stability` is an enumeration that represents the stability level of a feature.
  #[ derive( Debug, derive_tools::FromStr ) ]
  #[ display( style = "snake_case" ) ]
  pub enum Stability
  {
    /// The feature is still being tested and may change.
    Experimental,
    /// The feature is not fully tested and may be unstable.
    Unstable,
    /// The feature is tested and stable.
    Stable,
    /// The feature is stable and will not change in future versions.
    Frozen,
    /// The feature is no longer recommended for use and may be removed in future versions.
    Deprecated,
  }

  // aaa : qqq : derive?
  // aaa : add

  /// Retrieves the stability level of a package from its `Cargo.toml` file.
  fn stability_get( package_path : &Path ) -> Result< Stability, HealthTableRenewError >
  {
    let path = package_path.join( "Cargo.toml" );
    if path.exists()
    {
      let mut contents = String::new();
      File::open( path )?.read_to_string( &mut contents )?;
      let doc = contents.parse::< Document >()?;

      let stable_status = doc
      .get( "package" )
      .and_then( | package | package.get( "metadata" ) )
      .and_then( | metadata | metadata.get( "stability" ) )
      .and_then( | i | i.as_str() )
      .and_then( | s | s.parse::< Stability >().ok() );

      Ok( stable_status.unwrap_or( Stability::Experimental ) )
    }
    else
    {
      // qqq : for Petro : use typed error
      Err( HealthTableRenewError::Common( error::untyped::Error::msg( "Cannot find Cargo.toml" )))
    }
  }

  /// Represents parameters that are common for all tables
  #[ derive( Debug ) ]
  struct GlobalTableOptions
  {
    /// Path to the root repository.
    core_url : String,
    /// User and repository name, written through '/'.
    user_and_repo : String,
    /// List of branches in the repository.
    branches : Option< Vec< String > >,
    /// workspace root
    workspace_root : PathBuf,
    // aaa : for Petro : is not that path?
    // aaa : done
  }

  /// Structure that holds the parameters for generating a table.
  #[ derive( Debug ) ]
  #[ allow( clippy::struct_excessive_bools ) ]
  struct TableOptions
  {
    // Relative path from workspace root to directory with modules
    base_path : String,
    // include branches column flag
    include_branches : bool,
    // include stability column flag
    include_stability : bool,
    // include docs column flag
    include_docs : bool,
    // include sample column flag
    include : bool,
  }

  impl From< HashMap< String, query::Value > > for TableOptions
  {
    fn from( value : HashMap< String, query::Value > ) -> Self
    {
      let include_branches = value
      .get( "with_branches" )
      .map_or( true, bool::from );

      let include_stability = value
      .get( "with_stability" )
      .map_or( true, bool::from );

      let include_docs = value
      .get( "with_docs" )
      .map_or( true, bool::from );

      let include = value
      .get( "with_gitpod" )
      .map_or( true, bool::from );

      let b_p = value.get( "1" );
      let base_path = if let Some( query::Value::String( path ) ) = value.get( "path" ).or( b_p )
      {
        path
      }
      else
      {
        "./"
      };
      Self
      {
        base_path: base_path.to_string(),
        include_branches,
        include_stability,
        include_docs,
        include
      }
    }
  }

  impl GlobalTableOptions
  {
    /// Initializes the struct's fields from a `Cargo.toml` file located at a specified path.
    fn initialize_from_path( path : &Path ) -> Result< Self, HealthTableRenewError >
    {

      let cargo_toml_path = path.join( "Cargo.toml" );
      if !cargo_toml_path.exists()
      {
        return Err( HealthTableRenewError::Common( error::untyped::Error::msg( "Cannot find Cargo.toml" )))
      }

      let mut contents = String::new();
      File::open( cargo_toml_path )?.read_to_string( &mut contents )?;
      let doc = contents.parse::< Document >()?;

      let core_url =
      doc
      .get( "workspace" )
      .and_then( | workspace  | workspace.get( "metadata" ) )
      .and_then( | metadata | metadata.get( "repo_url" ) )
      .and_then( | url | url.as_str() )
      .map( String::from );

      let branches =
      doc
      .get( "workspace" )
      .and_then( | workspace | workspace.get( "metadata" ) )
      .and_then( | metadata | metadata.get( "branches" ) )
      .and_then( | branches | branches.as_array())
      .map
      (
        | array |
        array
        .iter()
        .filter_map( | value | value.as_str() )
        .map( String::from )
        .collect::< Vec< String > >()
      );
      let mut user_and_repo = String::new();
      if let Some( core_url ) = &core_url
      {
        user_and_repo = url::git_info_extract( core_url )?;
      }
      Ok
      (
        Self
        {
          core_url : core_url.unwrap_or_default(),
          user_and_repo,
          branches,
          workspace_root : path.to_path_buf()
        }
      )
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
  ///
  /// # Errors
  /// qqq: doc
  ///
  /// # Panics
  /// qqq: doc
  // aaa : for Petro : typed errors
  // aaa : done
  pub fn readme_health_table_renew( path : &Path ) -> Result< (), HealthTableRenewError >
  {
    regexes_initialize();
    let workspace = Workspace::try_from( CrateDir::try_from( path )? )?;
    let workspace_root = workspace.workspace_root();
    let mut parameters = GlobalTableOptions::initialize_from_path
    (
      &workspace_root
    )?;

    let read_me_path = workspace_root
    .join( repository::readme_path( &workspace_root )? );
    let mut file = OpenOptions::new()
    .read( true )
    .write( true )
    .open( &read_me_path )?;

    let mut contents = Vec::new();

    file.read_to_end( &mut contents )?;

    let mut tags_closures = vec![];
    let mut tables = vec![];
    let open_caps = TAG_TEMPLATE.get().unwrap().captures_iter( &contents );
    let close_caps = CLOSE_TAG.get().unwrap().captures_iter( &contents );
    // iterate by regex matches and generate table content for each dir which taken from open-tag
    for ( open_captures, close_captures ) in open_caps.zip( close_caps )
    {
      for captures in open_captures.iter().zip( close_captures.iter() )
      {
        if let ( Some( open ), Some( close ) ) = captures
        {
          let raw_table_params = std::str::from_utf8
          (
          TAG_TEMPLATE.get().unwrap().captures( open.as_bytes() )
          .ok_or( format_err!( "Fail to parse tag" ) )?
          .get( 1 )
          .ok_or( format_err!( "Fail to parse group" ) )?
          .as_bytes()
          )?;
          let params: TableOptions  = query::parse
          (
            raw_table_params
          ).unwrap()
          .into_map( vec![] )
          .into();
          let table = package_readme_health_table_generate
          (
            &workspace,
            &params,
            &mut parameters
          )?;
          tables.push( table );
          tags_closures.push( ( open.end(), close.start() ) );
        }
      }
    }
    tables_write_into_file( tags_closures, tables, contents, file )?;

    Ok( () )
  }

  /// Writes tables into a file at specified positions.
  #[ allow( clippy::needless_pass_by_value ) ]
  fn tables_write_into_file
  (
    tags_closures : Vec< ( usize, usize ) >,
    tables: Vec< String >,
    contents: Vec< u8 >,
    mut file: File
  ) -> Result< (), HealthTableRenewError >
  {
    let mut buffer: Vec< u8 > = vec![];
    let mut start: usize = 0;
    for
    (
      ( end_of_start_tag, start_of_end_tag ),
      con
    )
    in tags_closures.iter().zip( tables.iter() )
    {
      range_to_target_copy( &contents, &mut buffer, start, *end_of_start_tag )?;
      range_to_target_copy( con.as_bytes(), &mut buffer, 0,con.len() - 1 )?;
      start = *start_of_end_tag;
    }
    range_to_target_copy( &contents,&mut buffer,start,contents.len() - 1 )?;
    file.set_len( 0 )?;
    file.seek( SeekFrom::Start( 0 ) )?;
    file.write_all( &buffer )?;
    Ok(())
  }

  /// Generate table from `table_parameters`.
  /// Generate header, iterate over all modules in package (from `table_parameters`) and append row.
  fn package_readme_health_table_generate
  (
    workspace : &Workspace,
    table_parameters: &TableOptions,
    parameters: &mut GlobalTableOptions,
  ) -> Result< String, HealthTableRenewError >
  {
    let directory_names = directory_names
    (
      workspace
      .workspace_root()
      .join( &table_parameters.base_path ).to_path_buf(),
      workspace
      .packages()
    )?;
    let mut table = table_header_generate( parameters, table_parameters );
    for package_name in directory_names
    {
      let stability = if table_parameters.include_stability
      {
        Some
        (
          // qqq : use new-type. for example `CrateDir`
          stability_get
          (
            // qqq : the folder name may not match the package name
            &workspace.workspace_root().join( &table_parameters.base_path ).join( &package_name )
          )?
        )
      }
      else
      {
        None
      };
      if parameters.core_url.is_empty()
      {
        let module_path = workspace
        .workspace_root()
        .join( &table_parameters.base_path )
        .join( &package_name );
        // parameters.core_url = repo_url( &module_path )
        parameters.core_url = repo_url( &module_path.clone().try_into()? )
        .context
        (
          // aaa : for Petro : unreadable : check other lines of code which are long
          // aaa : done
          format_err!
          (
            "Can not find Cargo.toml in {} or Fail to extract repository url from git remote.\n\
specify the correct path to the main repository in Cargo.toml of workspace (in the [workspace.metadata] section named repo_url) in {} \
OR in Cargo.toml of each module (in the [package] section named repository, specify the full path to the module) for example {} OR\
ensure that at least one remotest is present in git. ",
            module_path.display(),
            workspace.workspace_root().join( "Cargo.toml" ).display(),
            module_path.join( "Cargo.toml" ).display()
          )
        )?;
        parameters.user_and_repo = url::git_info_extract( &parameters.core_url )?;
      }
      table.push_str
      (
        &row_generate
        (
          &package_name,
          stability.as_ref(),
          parameters,
          table_parameters
        )
      );
    }
    Ok( table )
  }

  /// Return topologically sorted modules name, from packages list, in specified directory.
  // fn directory_names( path : PathBuf, packages : &[ WorkspacePackageRef< '_ > ] ) -> Result< Vec< String > >
  #[ allow( clippy::type_complexity, clippy::unnecessary_wraps ) ]
  fn directory_names< 'a >
  (
    path : PathBuf,
    packages : impl Iterator< Item = WorkspacePackageRef< 'a > >,
  ) -> Result< Vec< String >, HealthTableRenewError >
  {
    let path_clone = path.clone();
    let module_package_filter : Option< Box< dyn Fn( WorkspacePackageRef< '_ > ) -> bool > > = Some
    (
      Box::new
      (
        move | p |
        {
          let manifest_file = p.manifest_file();
          if let Ok( pa ) = manifest_file
          {
            p.publish().is_none() && pa.starts_with( &path )
          }
          else
          {
            false
          }
        } // aaa : rid of unwraps
        // aaa : done
      )
    );
    let module_dependency_filter : Option< Box< dyn Fn( WorkspacePackageRef< '_ >, DependencyRef< '_ > ) -> bool > > = Some
    (
      Box::new
      (
        move | _, d |
        d.crate_dir().is_some() &&
        d.kind() !=
        DependencyKind::Development &&
        d.crate_dir().as_ref().unwrap().starts_with( &path_clone )
      )
    );
    let module_packages_map = packages::filter
    (
      packages,
      packages::FilterMapOptions
      {
        package_filter : module_package_filter,
        dependency_filter : module_dependency_filter
      },
    );
    let module_graph = graph::construct( &module_packages_map );
    let names : Vec< String > = graph::topological_sort_with_grouping( module_graph )
    .into_iter()
    .flat_map
    (
      | mut group |
      {
        group.sort();
        group
      }
    )
    .map( | n | n.to_string() )
    .collect();

    Ok( names )
  }

  /// Generate row that represents a module, with a link to it in the repository and optionals for stability, branches, documentation and links to the gitpod.
  fn row_generate
  (
    module_name : &str,
    stability : Option< &Stability >,
    parameters : &GlobalTableOptions,
    table_parameters : &TableOptions
  ) -> String
  {
    let mut rou = format!
    (
      "| [{}]({}/{}) |",
      &module_name,
      &table_parameters.base_path,
      &module_name
    );
    if table_parameters.include_stability
    {
      let mut stability = stability_generate( stability.as_ref().unwrap() );
      stability.push_str( " |" );
      rou.push_str( &stability );
    }
    if parameters.branches.is_some() && table_parameters.include_branches
    {
      rou.push_str( &branch_cells_generate( parameters, module_name ) );
    }
    if table_parameters.include_docs
    {
      rou.push_str
      (
        &format!
        (
          " [![docs.rs](https://raster.shields.io/static/v1?label=&message=docs&color=eee)](https://docs.rs/{}) |",
          &module_name
        )
      );
    }
    if table_parameters.include
    {
      let path = Path::new( table_parameters.base_path.as_str() ).join( module_name );
      let p = Path::new( &parameters.workspace_root ).join( &path );
      // let path = table_parameters.base_path.
      let example = if let Some( name ) = find_example_file( p.as_path(), module_name )
      {
        let path = path.to_string_lossy().replace( '\\', "/" ).replace( '/', "%2F" );
        let tmp = name.to_string_lossy().replace( '\\', "/" );
        let file_name = tmp.split( '/' ).last().unwrap();
        let name = file_name.strip_suffix( ".rs" ).unwrap();
        format!
        (
          "[![Open in Gitpod](https://raster.shields.io/static/v1?label=&message=try&color=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE={}%2Fexamples%2F{},RUN_POSTFIX=--example%20{}/{})",
          path,
          file_name,
          name,
          parameters.core_url,
        )
      }
      else
      {
        String::new()
      };
      rou.push_str( &format!( " {example} |" ) );
    }
    format!( "{rou}\n" )
  }

  /// todo
  #[ must_use ]
  pub fn find_example_file( base_path : &Path, module_name : &str ) -> Option< PathBuf >
  {
    let examples_dir = base_path.join("examples" );

    if examples_dir.exists() && examples_dir.is_dir()
    {
      if let Ok( entries ) = std::fs::read_dir( &examples_dir )
      {
        for entry in entries.flatten()
        {

          let file_name = entry.file_name();
          if let Some( file_name_str ) = file_name.to_str()
          {
            if file_name_str == format!( "{module_name}_trivial.rs" )
            {
              return Some( entry.path() )
            }
          }

        }
      }
    }

    // If module_trivial.rs doesn't exist, return any other file in the examples directory
    if let Ok( entries ) = std::fs::read_dir( &examples_dir )
    {
      for entry in entries.flatten()
      {

        let file_name = entry.file_name();
        if let Some( file_name_str ) = file_name.to_str()
        {
          if std::path::Path::new( file_name_str )
          .extension()
          .map_or( false, | ext | ext.eq_ignore_ascii_case( "rs" ) )
          {
            return Some( entry.path() )
          }
        }

      }
    }

    None
  }

  /// Generate stability cell based on stability
  #[ must_use ]
  pub fn stability_generate( stability : &Stability ) -> String
  {
    match stability
    {
      Stability::Experimental => " [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental)".into(),
      Stability::Stable => " [![stability-stable](https://img.shields.io/badge/stability-stable-green.svg)](https://github.com/emersion/stability-badges#stable)".into(),
      Stability::Deprecated => " [![stability-deprecated](https://img.shields.io/badge/stability-deprecated-red.svg)](https://github.com/emersion/stability-badges#deprecated)".into(),
      Stability::Unstable => " [![stability-unstable](https://img.shields.io/badge/stability-unstable-yellow.svg)](https://github.com/emersion/stability-badges#unstable)".into(),
      Stability::Frozen => " [![stability-frozen](https://img.shields.io/badge/stability-frozen-blue.svg)](https://github.com/emersion/stability-badges#frozen)".into(),
    }
  }

  /// Generate table header
  fn table_header_generate
  (
    parameters : &GlobalTableOptions,
    table_parameters : &TableOptions
  ) -> String
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
          header.push_str( format!( " {branch} |" ).as_str() );
          separator.push_str( "--------|" );
        }
      }
    }

    if table_parameters.include_docs
    {
      header.push_str( " Docs |" );
      separator.push_str( ":----:|" );
    }

    if table_parameters.include
    {
      header.push_str( " Sample |" );
      separator.push_str( ":------:|" );
    }

    format!( "{header}\n{separator}\n" )
  }

  /// Generate cells for each branch
  fn branch_cells_generate( table_parameters : &GlobalTableOptions, module_name : &str ) -> String
  {
    let cells = table_parameters
    .branches
    .as_ref()
    .unwrap()
    .iter()
    .map
    (
      | b |
      format!
      (
        "[![rust-status](https://img.shields.io/github/actions/workflow/status/{}/module_{}_push.yml?label=&branch={})]({}/actions/workflows/module_{}_push.yml?query=branch%3A{})",
        table_parameters.user_and_repo,
        &module_name.to_case( convert_case::Case::Snake ),
        b,
        table_parameters.core_url,
        &module_name.to_case( convert_case::Case::Snake ),
        b,
      )
    )
    .collect::< Vec< String > >()
    .join( " | " );
    format!( " {cells} |" )
  }

  fn range_to_target_copy< T : Clone >
  (
    source : &[ T ],
    target : &mut Vec< T >,
    from : usize,
    to : usize
  ) -> Result< (), HealthTableRenewError >
  {
    if from < source.len() && to < source.len() && from <= to
    {
      target.extend_from_slice( &source[ from..= to ] );
      return Ok( () )
    }
    Err( HealthTableRenewError::Common( error::untyped::Error::msg( "Incorrect indexes" )))
  }
}

crate::mod_interface!
{
  // /// Return workspace root
  // own use workspace_root;
  /// Stability
  own use Stability;
  /// Generate Stability badge
  own use stability_generate;
  own use find_example_file;
  /// Create Table.
  orphan use readme_health_table_renew;
}
