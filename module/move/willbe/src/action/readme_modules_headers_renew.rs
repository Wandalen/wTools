mod private
{
  use crate::*;
  use _path::AbsolutePath;
  use action::readme_health_table_renew::{ readme_path, Stability, stability_generate, find_example_file };
  use package::Package;
  use wtools::error::
  {
    err,
    for_app::
    { 
      Result, 
      Error as wError,
      Context,
    },
  };
  use std::borrow::Cow;
  use std::collections::BTreeSet;
  use std::fmt::{Display, Formatter};
  use std::fs::{ OpenOptions };
  use std::io::{ Read, Seek, SeekFrom, Write };
  use std::path::PathBuf;
  use convert_case::{ Case, Casing };
  use regex::Regex;
  use entity::WorkspaceError;
  use manifest::private::CrateDirError;
  use package::PackageError;
  use error_tools::for_lib::Error;
  use error_tools::dependency::*;
  // aaa : for Petro : rid off crate::x. ask
  // aaa : add `use crate::*` first

  static TAGS_TEMPLATE : std::sync::OnceLock< Regex > = std::sync::OnceLock::new();

  fn regexes_initialize()
  {
    TAGS_TEMPLATE.set( Regex::new( r"<!--\{ generate\.module_header\.start(\(\)|\{\}|\(.*?\)|\{.*?\}) \}-->(.|\n|\r\n)+<!--\{ generate\.module_header\.end \}-->" ).unwrap() ).ok();
  }

  /// Report.
  #[ derive( Debug, Default, Clone ) ]
  pub struct ModulesHeadersRenewReport
  { 
    found_files : BTreeSet< PathBuf >, 
    touched_files : BTreeSet< PathBuf >,
  }

  impl Display for ModulesHeadersRenewReport
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      if self.touched_files.len() < self.found_files.len() 
      {
        writeln!( f, "Something went wrong.\n{}/{} was touched.", self.found_files.len(), self.touched_files.len() )?;
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

  #[ derive( Debug, Error ) ]
  pub enum ModulesHeadersRenewError
  {
    #[ error( "Common error: {0}" ) ]
    Common(#[ from ] wError ),
    #[ error( "I/O error: {0}" ) ]
    IO( #[ from ] std::io::Error ),
    #[ error( "Workspace error: {0}" ) ]
    Workspace( #[ from ] WorkspaceError),
    #[ error( "Package error: {0}" ) ]
    Package( #[ from ] PackageError),
    #[ error( "Directory error: {0}" ) ]
    Directory( #[ from ] CrateDirError ),
  }

  /// The `ModuleHeader` structure represents a set of parameters, used for creating url for header.
  struct ModuleHeader
  {
    module_path : PathBuf,
    stability : Stability,
    module_name : String,
    repository_url : String,
    discord_url : Option< String >,
  }

  impl ModuleHeader
  {

    /// Create `ModuleHeader` instance from the folder where Cargo.toml is stored.
    fn from_cargo_toml( package : Package, default_discord_url : &Option< String > ) -> Result< Self, ModulesHeadersRenewError >
    {
      let stability = package.stability()?;

      let module_name = package.name()?;

      let repository_url = package.repository()?.ok_or_else::< wError, _ >( || err!( "Fail to find repository_url in module`s Cargo.toml" ) )?;

      let discord_url = package.discord_url()?.or_else( || default_discord_url.clone() );
      Ok
        (
          Self
          {
            module_path: package.manifest_path().parent().unwrap().as_ref().to_path_buf(),
            stability,
            module_name,
            repository_url,
            discord_url,
          }
        )
    }

    /// Convert `ModuleHeader`to header.
    fn to_header( self, workspace_path : &str ) -> Result< String, ModulesHeadersRenewError >
    {
      let discord = self.discord_url.map( | discord_url |
        format!( " [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)]({discord_url})" )
      )
      .unwrap_or_default();

      let repo_url = url::extract_repo_url( &self.repository_url ).and_then( | r | url::git_info_extract( &r ).ok() ).ok_or_else::< wError, _ >( || err!( "Fail to parse repository url" ) )?;
      let example = if let Some( name ) = find_example_file( self.module_path.as_path(), &self.module_name )
      {
        // qqq : for Petro : Hardcoded Strings, would be better to use `PathBuf` to avoid separator mismatch on Windows and Unix
        let p = name.strip_prefix( workspace_path ).unwrap().get( 1.. ).unwrap().replace( "\\","%2F" );
        let name = name.replace( "/", "\\" );
        let name = name.split( "\\" ).last().unwrap().split( "." ).next().unwrap();
        format!( " [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE={p},RUN_POSTFIX=--example%20{}/https://github.com/{})", name, repo_url )
      }
      else
      {
        "".into()
      };
      Ok( format!
      (
        "{} \
        [![rust-status](https://github.com/{}/actions/workflows/module_{}_push.yml/badge.svg)](https://github.com/{}/actions/workflows/module_{}_push.yml) \
        [![docs.rs](https://img.shields.io/docsrs/{}?color=e3e8f0&logo=docs.rs)](https://docs.rs/{}){}{}",
        stability_generate( &self.stability ),
        repo_url, self.module_name.to_case( Case::Snake ), repo_url, self.module_name.to_case( Case::Snake ),
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
  pub fn readme_modules_headers_renew( path : AbsolutePath ) -> Result< ModulesHeadersRenewReport, ( ModulesHeadersRenewReport, ModulesHeadersRenewError ) >
  {
    let mut report = ModulesHeadersRenewReport::default();
    regexes_initialize();
    let cargo_metadata = Workspace::with_crate_dir( CrateDir::try_from( path ).map_err( | e | ( report.clone(), e.into() ) )? ).map_err( | e | ( report.clone(), e.into() ) )?;
    let discord_url = cargo_metadata.discord_url().map_err( | e | ( report.clone(), e.into() ) )?;
    let paths = cargo_metadata.packages().map_err( | e | ( report.clone(), e.into() ) )?.into_iter().filter_map( | p | AbsolutePath::try_from( p.manifest_path() ).ok()).collect::< Vec< _ > >();
    report.found_files = paths.iter().map( | ap | ap.as_ref().to_path_buf() ).collect();
    for path in paths
    {
      let read_me_path =  path
      .parent()
      .unwrap()
      .join( readme_path( path.parent().unwrap().as_ref() ).ok_or_else::< wError, _ >( || err!( "Fail to find README.md" ) ).map_err( | e | ( report.clone(), e.into() ) )? );

      let pakage = Package::try_from( path.clone() ).map_err( | e | ( report.clone(), e.into() ) )?;
      let header = ModuleHeader::from_cargo_toml( pakage.into(), &discord_url ).map_err( | e | ( report.clone(), e.into() ) )?;

      let mut file = OpenOptions::new()
      .read( true )
      .write( true )
      .open( &read_me_path )
      .map_err( | e | ( report.clone(), e.into() ) )?;

      let mut content = String::new();
      file.read_to_string( &mut content ).map_err( | e | ( report.clone(), e.into() ) )?;

      let raw_params = TAGS_TEMPLATE
      .get()
      .unwrap()
      .captures( &content )
      .and_then( | c | c.get( 1 ) )
      .map( | m | m.as_str() )
      .unwrap_or_default();

      _ = query::parse( raw_params ).context( "Fail to parse raw params." );

      let content = header_content_generate( &content, header, raw_params, cargo_metadata.workspace_root().map_err( | e | ( report.clone(), e.into() ) )?.to_str().unwrap() ).map_err( | e | ( report.clone(), e.into() ) )?;

      file.set_len( 0 ).map_err( | e | ( report.clone(), e.into() ) )?;
      file.seek( SeekFrom::Start( 0 ) ).map_err( | e | ( report.clone(), e.into() ) )?;
      file.write_all( content.as_bytes() ).map_err( | e | ( report.clone(), e.into() ) )?;
      report.touched_files.insert( path.as_ref().to_path_buf() );
    }
    Ok( report )
  }

  fn header_content_generate< 'a >( content : &'a str, header : ModuleHeader, raw_params : &str, workspace_root : &str ) -> Result< Cow< 'a, str > >
  {
    let header = header.to_header( workspace_root )?;
    let result = TAGS_TEMPLATE.get().unwrap().replace( &content, &format!( "<!--{{ generate.module_header.start{raw_params} }}-->\n{header}\n<!--{{ generate.module_header.end }}-->" ) );
    Ok( result )
  }
}

crate::mod_interface!
{
  /// Generate headers in modules
  orphan use readme_modules_headers_renew;
  /// report
  orphan use ModulesHeadersRenewReport;
}