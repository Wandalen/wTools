mod private
{
  use crate::*;
  use std::fmt::{ Display, Formatter };
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
  use std::path::PathBuf;
  use regex::Regex;
  use { CrateDir, query, url, Workspace };
  use entity::{ PathError, WorkspaceInitError };
  use error::
  {
    err,
    Result,
    untyped::
    {
      Error,
      Context,
    },
  };
  use workspace_md_extension::WorkspaceMdExtension;
  // use error::ErrWith;

  static TAGS_TEMPLATE : std::sync::OnceLock< Regex > = std::sync::OnceLock::new();

  fn regexes_initialize()
  {
    TAGS_TEMPLATE.set
    (
      Regex::new
      (
        r"<!--\{ generate\.main_header\.start(\(\)|\{\}|\(.*?\)|\{.*?\}) \}-->(.|\n|\r\n)+<!--\{ generate\.main_header\.end \}-->"
      )
      .unwrap()
    ).ok();
  }

  /// Report.
  #[ derive( Debug, Default, Clone ) ]
  pub struct MainHeaderRenewReport
  {
    found_file : Option< PathBuf >,
    touched_file : PathBuf,
    success : bool,
  }

  impl Display for MainHeaderRenewReport
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      if self.success
      {
        if let Some( file_path ) = self.touched_file.to_str()
        {
          writeln!( f, "File successful changed : {file_path}." )?;
        }
        else
        {
          writeln!( f, "File successful changed but contains non-UTF-8 characters." )?;
        }
      }
      else
      {
        if let Some( Some( file_path ) ) = self
        .found_file
        .as_ref()
        .map( | p | p.to_str() )
        {
          writeln!( f, "File found but not changed : {file_path}." )?;
        }
        else
        {
          writeln!( f, "File not found or contains non-UTF-8 characters." )?;
        }
      }
      Ok( () )
    }
  }

  /// The `MainHeaderRenewError` enum represents the various errors that can occur during
  /// the renewal of the main header.
  #[ derive( Debug, error::Error ) ]
  pub enum MainHeaderRenewError
  {
    /// Represents a common error.
    #[ error( "Common error: {0}" ) ]
    Common(#[ from ] Error ),
    /// Represents an I/O error.
    #[ error( "I/O error: {0}" ) ]
    IO( #[ from ] std::io::Error ),
    /// Represents an error related to workspace initialization.
    #[ error( "Workspace error: {0}" ) ]
    Workspace( #[ from ] WorkspaceInitError ),
    /// Represents an error related to directory paths.
    #[ error( "Directory error: {0}" ) ]
    Directory( #[ from ] PathError ),
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
    fn from_cargo_toml( workspace : &Workspace ) -> Result< Self, MainHeaderRenewError >
    {
      // aaa : for Petro : too long lines, review all files
      // aaa : done
      let repository_url = workspace
      .repository_url()
      .ok_or_else::< Error, _ >
      ( || err!( "repo_url not found in workspace Cargo.toml" ) )?;

      let master_branch = workspace.master_branch().unwrap_or( "master".into() );
      let workspace_name = workspace
      .workspace_name()
      .ok_or_else::< Error, _ >
      ( || err!( "workspace_name not found in workspace Cargo.toml" ) )?;

      let discord_url = workspace.discord_url();

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
    fn to_header( self ) -> Result< String, MainHeaderRenewError >
    {
      let discord = self.discord_url
      .map
      (
        | discord |
        format!
        (
          "\n[![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)]({})",
          discord
        )
      )
      .unwrap_or_default();

      Ok
      (
        format!
        (
         r#"[![{}](https://img.shields.io/github/actions/workflow/status/{}/standard_rust_scheduled.yml?label={}&logo=github&branch={})](https://github.com/{}/actions/workflows/standard_rust_scheduled.yml){}
[![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2F{}_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20{}_trivial_sample/https://github.com/{})
[![docs.rs](https://raster.shields.io/static/v1?label=docs&message=online&color=eee&logo=docsdotrs&logoColor=eee)](https://docs.rs/{})"#,
          self.workspace_name, url::git_info_extract( &self.repository_url )?, self.workspace_name, self.master_branch, url::git_info_extract( &self.repository_url )?,
          discord,
          self.workspace_name.to_lowercase(), self.workspace_name.to_lowercase(), url::git_info_extract( &self.repository_url )?,
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
  /// [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Fwtools_trivial%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20wtools_trivial/https://github.com/Wandalen/wTools)
  /// [![docs.rs](https://raster.shields.io/static/v1?label=docs&message=online&color=eee&logo=docsdotrs&logoColor=eee)](https://docs.rs/wtools)
  /// <!--{ generate.main_header.end }-->
  /// ```
  pub fn readme_header_renew( crate_dir : CrateDir )
  -> Result< MainHeaderRenewReport, ( MainHeaderRenewReport, MainHeaderRenewError ) >
  {
    let mut report = MainHeaderRenewReport::default();
    regexes_initialize();

    let workspace = Workspace::try_from
    (
      crate_dir
    )
    .err_with( || report.clone() )?;

    let workspace_root = workspace
    .workspace_root();

    let header_param = HeaderParameters::from_cargo_toml( &workspace )
    .err_with( || report.clone() )?;

    let read_me_path = workspace_root.join
    (
      repository::readme_path( &workspace_root )
      .err_with( || report.clone() )?
    );

    report.found_file = Some( read_me_path.clone().to_path_buf() );

    let mut file = OpenOptions::new()
    .read( true )
    .write( true )
    .open( &read_me_path )
    .err_with( || report.clone() )?;

    let mut content = String::new();
    file.read_to_string( &mut content ).err_with( || report.clone() )?;

    let raw_params = TAGS_TEMPLATE
    .get()
    .unwrap()
    .captures( &content )
    .and_then( | c | c.get( 1 ) )
    .map( | m | m.as_str() )
    .unwrap_or_default();

    _ = query::parse( raw_params ).context( "Fail to parse arguments" );

    let header = header_param.to_header().err_with( || report.clone() )?;
    let content : String = TAGS_TEMPLATE.get().unwrap().replace
    (
      &content,
      &format!
      (
        "<!--{{ generate.main_header.start{} }}-->\n{}\n<!--{{ generate.main_header.end }}-->",
        raw_params,
        header,
      )
    ).into();

    file.set_len( 0 ).err_with( || report.clone() )?;
    file.seek( SeekFrom::Start( 0 ) ).err_with( || report.clone() )?;
    file.write_all( content.as_bytes() ).err_with( || report.clone() )?;
    report.touched_file = read_me_path.to_path_buf();
    report.success = true;
    Ok( report )
  }
}

crate::mod_interface!
{
  /// Generate header.
  orphan use readme_header_renew;
  /// Report.
  orphan use MainHeaderRenewReport;
  /// Error.
  orphan use MainHeaderRenewError;
}