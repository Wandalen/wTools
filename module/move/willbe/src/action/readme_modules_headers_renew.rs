mod private
{
  use crate::*;
  use std::
  {
    borrow::Cow,
    fs::OpenOptions,
    fmt,
    io::
    {
      Read,
      Seek,
      Write,
      SeekFrom,
    }
  };
  use std::fs::File;
  use collection::BTreeSet;
  // use path::AbsolutePath;
  use action::readme_health_table_renew::{ Stability, stability_generate, find_example_file };
  use package::Package;
  use error::
  {
    err,
    untyped::
    {
      // Result,
      Error as wError,
      Context,
    },
  };
  // aaa : for Petro : group properly, don't repeat std::
  // aaa : done
  use std::path::PathBuf;
  use convert_case::{ Case, Casing };
  // use rayon::scope_fifo;
  use regex::Regex;
  use toml_edit::Document;
  use entity::{ WorkspaceInitError, PathError };
  use package::PackageError;
  use error::typed::Error;
  use workspace_md_extension::WorkspaceMdExtension;
  // use error::ErrWith;

  static TAGS_TEMPLATE : std::sync::OnceLock< Regex > = std::sync::OnceLock::new();

  fn regexes_initialize()
  {
    TAGS_TEMPLATE.set
    (
      Regex::new
      (
        r"<!--\{ generate\.module_header\.start(\(\)|\{\}|\(.*?\)|\{.*?\}) \}-->(.|\n|\r\n)+<!--\{ generate\.module_header\.end \}-->"
      ).unwrap()
    ).ok();
  }

  /// Report.
  #[ derive( Debug, Default, Clone ) ]
  pub struct ModulesHeadersRenewReport
  {
    found_files : BTreeSet< PathBuf >,
    touched_files : BTreeSet< PathBuf >,
  }

  impl fmt::Display for ModulesHeadersRenewReport
  {
    fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      if self.touched_files.len() < self.found_files.len()
      {
        writeln!
        (
          f,
          "Something went wrong.\n{}/{} was touched.",
          self.found_files.len(),
          self.touched_files.len()
        )?;
        return Ok(())
      }
      writeln!( f, "Touched files :" )?;
      let mut count = self.found_files.len();
      for path in &self.touched_files
      {
        if let Some( file_path ) = path.to_str()
        {
          writeln!( f, "{file_path}" )?;
          count -= 1;
        }
      }
      if count != 0
      {
        writeln!( f, "Other {count} files contains non-UTF-8 characters." )?;
      }
      Ok( () )
    }
  }

  /// The `ModulesHeadersRenewError` enum represents the various errors that can occur during
  /// the renewal of module headers.
  #[ derive( Debug, Error ) ]
  pub enum ModulesHeadersRenewError
  {
    /// Represents a common error.
    #[ error( "Common error: {0}" ) ]
    Common(#[ from ] wError ),
    /// Represents an I/O error.
    #[ error( "I/O error: {0}" ) ]
    IO( #[ from ] std::io::Error ),
    /// Represents an error related to workspace initialization.
    #[ error( "Workspace error: {0}" ) ]
    Workspace( #[ from ] WorkspaceInitError ),
    /// Represents an error related to a package.
    #[ error( "Package error: {0}" ) ]
    Package( #[ from ] PackageError ),
    /// Represents an error related to directory paths.
    #[ error( "Directory error: {0}" ) ]
    Directory( #[ from ] PathError ),
    /// Represents an error related to toml editing.
    #[ error( "Toml error: {0}" ) ]
    Toml( #[ from ] toml_edit::TomlError ),
  }

  /// The `ModuleHeader` structure represents a set of parameters, used for creating url for header.
  struct ModuleHeader< 'a >
  {
    module_path : PathBuf,
    stability : Stability,
    module_name : String,
    repository_url : String,
    discord_url : Option< String >,
    branches : Option< &'a [ String ] >,
  }

  impl< 'a > ModuleHeader < 'a >
  {

    /// Create `ModuleHeader` instance from the folder where Cargo.toml is stored.
    fn from_cargo_toml< 'b >
    (
      package : Package< 'b >,
      branches : Option< &'a [ String ] >,
      default_discord_url : &Option< String >,
    )
    -> Result< Self, ModulesHeadersRenewError >
    {
      let stability = package.stability()?;
      let module_name = package.name()?;
      let repository_url = package.repository()?
      .ok_or_else::< wError, _ >( || err!( "Fail to find repository_url in module`s Cargo.toml" ) )?;

      let discord_url = package
      .discord_url()?
      .or_else( || default_discord_url.clone() );
      Ok
        (
          Self
          {
            module_path: package.manifest_file().parent().unwrap().as_ref().to_path_buf(),
            stability,
            module_name : module_name.to_string(),
            repository_url,
            discord_url,
            branches,
          }
        )
    }

    /// Convert `ModuleHeader`to header.
    fn to_header( self, workspace_path : &str ) -> Result< String, ModulesHeadersRenewError >
    {
      let discord = self.discord_url.map( | discord_url |
        format!
        (
          " [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)]({})",
          discord_url
        )
      )
      .unwrap_or_default();

      let repo_url = url::repo_url_extract( &self.repository_url )
      .and_then( | r | url::git_info_extract( &r ).ok() )
      .ok_or_else::< wError, _ >( || err!( "Fail to parse repository url" ) )?;
      let example= if let Some( name ) = find_example_file
      (
        self.module_path.as_path(),
        &self.module_name
      )
      {
        let relative_path = proper_path_tools::path::path_relative
        (
          workspace_path.try_into().unwrap(),
          name
        )
        .to_string_lossy()
        .to_string();
        #[ cfg( target_os = "windows" ) ]
        let relative_path = relative_path.replace( "\\", "/" );
        // aaa : for Petro : use path_toools
        // aaa : used
        let p = relative_path.replace( "/","%2F" );
        format!
        (
          " [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE={},RUN_POSTFIX=--example%20{}/https://github.com/{})",
          p,
          p,
          repo_url
        )
      }
      else
      {
        "".into()
      };
      let stable = if let Some( branches ) = self.branches
      {
        let mut base = "".to_string();
        for branch in branches
        {
          base = format!
          (
            "{} \
            {} [![rust-status](https://github.com/{}/actions/workflows/module_{}_push.yml/badge.svg?branch={})](https://github.com/{}/actions/workflows/module_{}_push.yml?query=branch%3A{})",
            base,
            branch, repo_url, self.module_name.to_case( Case::Snake ), branch, repo_url, self.module_name.to_case( Case::Snake ), branch, 
          );
        }
        base
      }
      else
      {
        format!("[![rust-status](https://github.com/{}/actions/workflows/module_{}_push.yml/badge.svg)](https://github.com/{}/actions/workflows/module_{}_push.yml)", repo_url, self.module_name.to_case( Case::Snake ), repo_url, self.module_name.to_case( Case::Snake ) )
      };
      Ok( format!
      (
        "{} \
        {} \
        [![docs.rs](https://img.shields.io/docsrs/{}?color=e3e8f0&logo=docs.rs)](https://docs.rs/{}){}{}",
        stability_generate( &self.stability ),
        stable,
        self.module_name, self.module_name,
        example,
        discord,
      ) )
    }
  }

  /// Generate header in modules Readme.md.
  /// The location of header is defined by a tag :
  /// ``` md
  /// <!--{ generate.module_header.start() }-->
  /// <!--{ generate.module_header.end }-->
  /// ```
  /// To use it you need to add these fields to Cargo.toml each module workspace :
  /// ``` toml
  /// [package]
  /// name = "test_module"
  /// repository = "https://github.com/Wandalen/wTools/tree/master/module/move/test_module"
  /// ...
  /// [package.metadata]
  /// stability = "stable" (Optional)
  /// discord_url = "https://discord.gg/m3YfbXpUUY" (Optional)
  /// ```
  /// Result example :
  /// ``` md
  /// <!--{ generate.module_header.start() }-->
  /// [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) | [![rust-status](https://github.com/Username/test/actions/workflows/ModuleChainOfPackagesAPush.yml/badge.svg)](https://github.com/Username/test/actions/workflows/ModuleChainOfPackagesAPush.yml)[![docs.rs](https://img.shields.io/docsrs/_chain_of_packages_a?color=e3e8f0&logo=docs.rs)](https://docs.rs/_chain_of_packages_a)[![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2F_chain_of_packages_a_trivial%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20_chain_of_packages_a_trivial/https://github.com/Username/test)
  /// <!--{ generate.module_header.end }-->
  /// ```
  pub fn readme_modules_headers_renew( crate_dir : CrateDir )
  -> ResultWithReport< ModulesHeadersRenewReport, ModulesHeadersRenewError >
  // -> Result< ModulesHeadersRenewReport, ( ModulesHeadersRenewReport, ModulesHeadersRenewError ) >
  {
    let mut report = ModulesHeadersRenewReport::default();
    regexes_initialize();
    let workspace = Workspace::try_from( crate_dir )
    .err_with_report( &report )?;
    let discord_url = workspace.discord_url();

    // qqq : inspect each collect in willbe and rid of most of them

    let paths : Vec< AbsolutePath > = workspace
    .packages()
    .filter_map( | p | p.manifest_file().ok().and_then( | a | Some( a.inner() ) ) )
    .collect();

    report.found_files = paths
    .iter()
    .map( | ap | ap.as_ref().to_path_buf() )
    .collect();

    let mut contents = String::new();
    File::open( workspace.workspace_root().join( "Cargo.toml" ) )
    .err_with_report( &report )?
    .read_to_string( &mut contents )
    .err_with_report( &report )?;
    
    let doc = contents.parse::< Document >().err_with_report( &report )?;

    let branches = doc
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

    for path in paths
    {
      let read_me_path =  path
      .parent()
      .unwrap()
      .join
      (
        repository::readme_path( path.parent().unwrap().as_ref() )
        // .ok_or_else::< wError, _ >( || err!( "Fail to find README.md at {}", &path ) )
        .err_with_report( &report )?
      );

      let pakage = Package::try_from
      (
        CrateDir::try_from
        (
          &path
          .parent()
          .unwrap()
        )
        .err_with_report( &report )?
      )
      .err_with_report( &report )?;

      let header = ModuleHeader::from_cargo_toml( pakage.into(), branches.as_ref().map( | b | b.as_slice() ), &discord_url )
      .err_with_report( &report )?;

      let mut file = OpenOptions::new()
      .read( true )
      .write( true )
      .open( &read_me_path )
      .err_with_report( &report )?;

      let mut content = String::new();
      file.read_to_string( &mut content ).err_with_report( &report )?;

      let raw_params = TAGS_TEMPLATE
      .get()
      .unwrap()
      .captures( &content )
      .and_then( | c | c.get( 1 ) )
      .map( | m | m.as_str() )
      .unwrap_or_default();

      _ = query::parse( raw_params ).context( "Fail to parse raw params." );
      // qqq : for Petro : why ignored?

      let content = header_content_generate
      (
        &content,
        header,
        raw_params,
        workspace.workspace_root().to_str().unwrap()
      ).err_with_report( &report )?;

      file.set_len( 0 ).err_with_report( &report )?;
      file.seek( SeekFrom::Start( 0 ) ).err_with_report( &report )?;
      file.write_all( content.as_bytes() ).err_with_report( &report )?;
      report.touched_files.insert( path.as_ref().to_path_buf() );
    }
    Ok( report )
  }

  fn header_content_generate< 'a >
  (
    content : &'a str,
    header : ModuleHeader< 'a >,
    raw_params : &str,
    workspace_root : &str
  )
  // qqq : use typed error
  -> error::untyped::Result< Cow< 'a, str > >
  {
    let header = header.to_header( workspace_root )?;
    let result = TAGS_TEMPLATE
    .get()
    .unwrap()
    .replace
    (
      &content,
      &format!
      (
        "<!--{{ generate.module_header.start{} }}-->\n{}\n<!--{{ generate.module_header.end }}-->",
        raw_params,
        header
      )
    );
    Ok( result )
  }
}

crate::mod_interface!
{
  /// Generate headers in modules
  orphan use readme_modules_headers_renew;
  /// report
  orphan use ModulesHeadersRenewReport;
  /// Error.
  orphan use ModulesHeadersRenewError;
}