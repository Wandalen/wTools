mod private
{
  use crate::*;
  use _path::AbsolutePath;
  use action::readme_health_table_renew::{ readme_path, Stability, stability_generate };
  use package::Package;
  use wtools::error::
  {
    err,
    for_app::{ Result, Error },
  };
  use std::borrow::Cow;
  use std::fs::{ OpenOptions };
  use std::io::{ Read, Seek, SeekFrom, Write };
  use std::path::{Path, PathBuf};
  use convert_case::{ Case, Casing };
  use regex::Regex;
  use crate::action::readme_health_table_renew::find_example_file;
  // aaa : for Petro : rid off crate::x. ask
  // aaa : add `use crate::*` first

  static TAGS_TEMPLATE : std::sync::OnceLock< Regex > = std::sync::OnceLock::new();

  fn regexes_initialize()
  {
    TAGS_TEMPLATE.set( Regex::new( r"<!--\{ generate\.module_header\.start(\(\)|\{\}|\(.*?\)|\{.*?\}) \}-->(.|\n|\r\n)+<!--\{ generate\.module_header\.end \}-->" ).unwrap() ).ok();
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
    fn from_cargo_toml( package : Package, default_discord_url : &Option< String >, workspace_path : &str ) -> Result< Self >
    {
      let stability = package.stability()?;

      let module_name = package.name()?;

      let repository_url = package.repository()?.ok_or_else::< Error, _ >( || err!( "Fail to find repository_url in module`s Cargo.toml" ) )?;

      let discord_url = package.discord_url()?.or_else( || default_discord_url.clone() );
      Ok
        (
          Self
          {
            module_path: package.manifest_path().parent().unwrap().as_ref().strip_prefix( workspace_path ).unwrap().to_path_buf(),
            stability,
            module_name,
            repository_url,
            discord_url,
          }
        )
    }

    /// Convert `ModuleHeader`to header.
    fn to_header( self ) -> Result< String >
    {
      let discord = self.discord_url.map( | discord_url |
        format!( "\n[![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)]({discord_url})" )
      )
      .unwrap_or_default();
      let path = self.module_path.to_string_lossy().replace( "/", "%2F" );
      let repo_url = url::extract_repo_url( &self.repository_url ).and_then( | r | url::git_info_extract( &r ).ok() ).ok_or_else::< Error, _ >( || err!( "Fail to parse repository url" ) )?;
      let example = if let Some( name ) = find_example_file( self.module_path.as_path(), &self.module_name )
      {
        let p = name.replace( "\\","%2F");
        let name = name.split( "\\" ).last().unwrap().split( "." ).next().unwrap();
        format!( "[![Open in Gitpod](https://raster.shields.io/static/v1?label=&message=try&color=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE={p},RUN_POSTFIX=--example%20{}/https://github.com/{})", name, repo_url )
      }
      else
      {
        "".into()
      };
      Ok( format!
      (
        "{} \
        [![rust-status](https://github.com/{}/actions/workflows/module_{}_push.yml/badge.svg)](https://github.com/{}/actions/workflows/module_{}_push.yml) \
        [![docs.rs](https://img.shields.io/docsrs/{}?color=e3e8f0&logo=docs.rs)](https://docs.rs/{}) \
        {} {}",
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
  pub fn readme_modules_headers_renew( path : AbsolutePath ) -> Result< () >
  {
    regexes_initialize();
    let cargo_metadata = Workspace::with_crate_dir( CrateDir::try_from( path )? )?;
    let discord_url = cargo_metadata.discord_url()?;
    for path in cargo_metadata.packages()?.into_iter().filter_map( | p | AbsolutePath::try_from( p.manifest_path() ).ok())
    {
      let read_me_path =  path
      .parent()
      .unwrap()
      .join( readme_path( path.parent().unwrap().as_ref() ).ok_or_else::< Error, _ >( || err!( "Fail to find README.md" ) )? );

      let pakage = Package::try_from( path )?;
      let header = ModuleHeader::from_cargo_toml( pakage, &discord_url, cargo_metadata.workspace_root()?.to_str().unwrap() )?;

      let mut file = OpenOptions::new()
      .read( true )
      .write( true )
      .open( &read_me_path )?;

      let mut content = String::new();
      file.read_to_string( &mut content )?;

      let raw_params = TAGS_TEMPLATE
      .get()
      .unwrap()
      .captures( &content )
      .and_then( | c | c.get( 1 ) )
      .map( | m | m.as_str() )
      .unwrap_or_default();

      _ = query::parse( raw_params )?;

      let content = header_content_generate( &content, header, raw_params )?;

      file.set_len( 0 )?;
      file.seek( SeekFrom::Start( 0 ) )?;
      file.write_all( content.as_bytes() )?;
    }
    Ok( () )
  }

  fn header_content_generate< 'a >( content : &'a str, header : ModuleHeader, raw_params : &str ) -> Result< Cow< 'a, str > >
  {
    let header = header.to_header()?;
    let result = TAGS_TEMPLATE.get().unwrap().replace( &content, &format!( "<!--{{ generate.module_header.start{raw_params} }}-->\n{header}\n<!--{{ generate.module_header.end }}-->" ) );
    Ok( result )
  }
}

crate::mod_interface!
{
  /// Generate headers in modules
  orphan use readme_modules_headers_renew;
}