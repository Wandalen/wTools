mod private
{
  use crate::*;
  use std::fs::
  {
    OpenOptions
  };
  use std::io::
  {
    Read,
    Seek,
    SeekFrom,
    Write
  };
  use regex::Regex;
  use wtools::error::err;
  use error_tools::Result;
  use wca::wtools::anyhow::Error;
  use action::readme_health_table_renew::
  {
    readme_path,
    workspace_root
  };
  use path::AbsolutePath;
  use { CrateDir, query, url, Workspace, wtools };
  use wtools::error::anyhow::
  {
    format_err
  };

  static TAGS_TEMPLATE : std::sync::OnceLock< Regex > = std::sync::OnceLock::new();

  fn regexes_initialize()
  {
    TAGS_TEMPLATE.set( Regex::new( r"<!--\{ generate\.main_header\.start(\(\)|\{\}|\(.*?\)|\{.*?\}) \}-->(.|\n|\r\n)+<!--\{ generate\.main_header\.end \}-->" ).unwrap() ).ok();
  }

  /// The `HeaderParameters` structure represents a set of parameters, used for creating url for header.
  struct HeaderParameters
  {
    master_branch : String,
    repository_url : String,
    workspace_name : String,
    discord_url : Option< String >,
  }

  impl HeaderParameters
  {
    /// Create `HeaderParameters` instance from the folder where Cargo.toml is stored.
    fn from_cargo_toml( workspace : Workspace ) -> Result< Self >
    {
      let repository_url = workspace.repository_url()?.ok_or_else::< Error, _ >( || err!( "repo_url not found in workspace Cargo.toml" ) )?;
      let master_branch = workspace.master_branch()?.unwrap_or( "master".into() );
      let workspace_name = workspace.workspace_name()?.ok_or_else::< Error, _ >( || err!( "workspace_name not found in workspace Cargo.toml" ) )?;
      let discord_url = workspace.discord_url()?;

      Ok
      (
        Self
        {
          master_branch,
          repository_url,
          workspace_name,
          discord_url,
        }
      )
    }

    /// Convert `Self`to header.
    fn to_header( self ) -> Result< String >
    {
      let discord = self.discord_url.map( | discord |
        format!( "\n[![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)]({discord})" )
      )
      .unwrap_or_default();

      Ok
      (
        format!
        (
          r#"[![{}](https://img.shields.io/github/actions/workflow/status/{}/StandardRustScheduled.yml?branch=master&label={}&logo=github)](https://github.com/{}/actions/workflows/StandardRustStatus.yml){}
[![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2F{}_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20{}_trivial_sample/https://github.com/{})
[![docs.rs](https://raster.shields.io/static/v1?label=docs&message=online&color=eee&logo=docsdotrs&logoColor=eee)](https://docs.rs/{})"#,
          self.master_branch, url::git_info_extract( &self.repository_url )?, self.master_branch, url::git_info_extract( &self.repository_url )?,
          discord,
          self.workspace_name, self.workspace_name, url::git_info_extract( &self.repository_url )?,
          self.workspace_name,
        )
      )
    }
  }

  /// Generate header in main Readme.md.
  /// The location of header is defined by a tag :
  /// ``` md
  /// <!--{ generate.main_header.start() }-->
  /// <!--{ generate.main_header.end() }-->
  /// ```
  /// To use it you need to add these fields to Cargo.toml of workspace :
  /// ``` toml
  /// [workspace.metadata]
  /// master_branch = "alpha" (Optional)
  /// workspace_name = "wtools"
  /// repo_url = "https://github.com/Wandalen/wTools"
  /// discord_url = "https://discord.gg/123123" (Optional)
  /// ```
  /// Result example :
  /// ``` md
  /// <!--{ generate.main_header.start }-->
  /// [![alpha](https://img.shields.io/github/actions/workflow/status/Wandalen/wTools/StandardRustScheduled.yml?branch=master&label=alpha&logo=github)](https://github.com/Wandalen/wTools/actions/workflows/StandardRustStatus.yml)
  /// [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/123123)
  /// [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Fwtools_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20wtools_trivial_sample/https://github.com/Wandalen/wTools)
  /// [![docs.rs](https://raster.shields.io/static/v1?label=docs&message=online&color=eee&logo=docsdotrs&logoColor=eee)](https://docs.rs/wtools)
  /// <!--{ generate.main_header.end }-->
  /// ```
  pub fn readme_header_renew( path : AbsolutePath ) -> Result< () >
  {
    regexes_initialize();

    let mut cargo_metadata = Workspace::with_crate_dir( CrateDir::try_from( path )? )?;
    let workspace_root = workspace_root( &mut cargo_metadata )?;
    let header_param = HeaderParameters::from_cargo_toml( cargo_metadata )?;
    let read_me_path = workspace_root.join( readme_path( &workspace_root ).ok_or_else( || format_err!( "Fail to find README.md" ) )?);
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

    let header = header_param.to_header()?;
    let content : String = TAGS_TEMPLATE.get().unwrap().replace( &content, &format!( "<!--{{ generate.main_header.start{raw_params} }}-->\n{header}\n<!--{{ generate.main_header.end }}-->" ) ).into();
    file.set_len( 0 )?;
    file.seek( SeekFrom::Start( 0 ) )?;
    file.write_all( content.as_bytes() )?;
    Ok( () )
  }
}

crate::mod_interface!
{
  /// Generate header.
  orphan use readme_header_renew;
}