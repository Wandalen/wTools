mod private
{
	use std::fs::{ File, OpenOptions };
	use std::io::{ Read, Seek, SeekFrom, Write };
	use std::path::Path;
	use convert_case::{ Case, Casing };
	use regex::Regex;
	use toml_edit::Document;
	use crate::path::AbsolutePath;
	use crate::{ CrateDir, query, url, Workspace };
	use crate::endpoint::table::{ readme_path, Stability, stability_generate };
	use crate::wtools::error::
	{
		err,
	  for_app::{ bail, Result, Error },
	};

	static TAGS_TEMPLATE: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();

	fn regexes_initialize()
	{
		TAGS_TEMPLATE.set( Regex::new( r"<!--\{ generate\.module_header\.start\((.+|)\) \}-->(.|\n|\r\n)+<!--\{ generate\.module_header\.end \}-->" ).unwrap() ).ok();
	}

	/// The `ModuleHeader` structure represents a set of parameters, used for creating url for header.
	struct ModuleHeader
	{
		stability: Stability,
	  module_name: String,
	  repository_url: String,
	  discord_url: Option< String >,
	}

	impl ModuleHeader
	{

		/// Create `ModuleHeader` instance from the folder where Cargo.toml is stored.
		fn from_cargo_toml( path: &Path ) -> Result< Self >
		{
			if !path.exists()
			{
				bail!( "Cannot find Cargo.toml" )
			}
			let mut contents = String::new();

			File::open( path )?.read_to_string( &mut contents )?;

			let doc = contents.parse::< Document >()?;

			let stability = doc
			.get( "package" )
			.and_then( | package | package.get( "metadata" ) )
			.and_then( | metadata | metadata.get( "stability" ) )
			.and_then( | i | i.as_str() )
			.and_then( | s | s.parse::< Stability >().ok() )
			.unwrap_or( Stability::Experimental );

			let repository_url = doc
			.get( "package" )
			.and_then( | metadata | metadata.get( "repository" ) )
			.and_then( | url | url.as_str() )
			.map( String::from )
			.ok_or_else::< Error, _>( || err!( "package.repository not found in module Cargo.toml" ) )?;

			let module_name = doc
			.get( "package" )
			.and_then( | workspace  | workspace.get( "name" ) )
			.and_then( | url | url.as_str() )
			.map( String::from )
			.ok_or_else::< Error, _>( || err!( "master_branch not found in module Cargo.toml" ) )?;

			let discord_url = doc
			.get( "package" )
			.and_then( | workspace  | workspace.get( "metadata" ) )
			.and_then( | metadata | metadata.get( "discord_url" ) )
			.and_then( | url | url.as_str() )
			.map( String::from );

			Ok
				(
					Self
					{
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
			let discord = if self.discord_url.is_some()
			{
				format!("\n[![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)]({})", self.discord_url.unwrap())
			} else
			{
				"".into()
			};
			let repo_url = url::extract_repo_url( &self.repository_url ).and_then( | r | url::git_info_extract( &r ).ok() ).ok_or_else::< Error, _ >( || err!( "Fail to parse repository url" ) )?;
			Ok(format!
			(
				"{}\
				[![rust-status](https://github.com/{}/actions/workflows/Module{}Push.yml/badge.svg)](https://github.com/{}/actions/workflows/Module{}Push.yml)\
				[![docs.rs](https://img.shields.io/docsrs/{}?color=e3e8f0&logo=docs.rs)](https://docs.rs/{})\
				[![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2F{}_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20{}_trivial_sample/https://github.com/{}){}",
				stability_generate( &self.stability ),
				repo_url, self.module_name.to_case( Case::Pascal ), repo_url, self.module_name.to_case( Case::Pascal ),
				self.module_name, self.module_name,
				self.module_name, self.module_name, repo_url,
				discord,
			))
		}
	}

	/// Generate header in modules Readme.md.
	/// The location of header is defined by a tag:
	/// ``` md
	/// <!--{ generate.module_header.start() }-->
	/// <!--{ generate.module_header.end }-->
	/// ```
	/// To use it you need to add these fields to Cargo.toml each module workspace:
	/// ``` toml
	/// [package]
	/// name = "test_module"
	/// repository = "https://github.com/Wandalen/wTools/tree/master/module/move/test_module"
	/// ...
	/// [package.metadata]
	/// stability = "stable" (Optional)
	/// discord_url = "https://discord.gg/m3YfbXpUUY" (Optional)
	/// ```
	/// Result example:
	/// ``` md
	/// <!--{ generate.main_header.start }-->
	/// <!--{ generate.module_header.start() }-->
	/// [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) | [![rust-status](https://github.com/Username/test/actions/workflows/ModuleChainOfPackagesAPush.yml/badge.svg)](https://github.com/Username/test/actions/workflows/ModuleChainOfPackagesAPush.yml)[![docs.rs](https://img.shields.io/docsrs/_chain_of_packages_a?color=e3e8f0&logo=docs.rs)](https://docs.rs/_chain_of_packages_a)[![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2F_chain_of_packages_a_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20_chain_of_packages_a_trivial_sample/https://github.com/Username/test)
	/// <!--{ generate.module_header.end }-->
	/// <!--{ generate.main_header.end }-->
	/// ```
	pub fn generate_modules_headers( path: AbsolutePath ) -> Result< () >
	{
		regexes_initialize();
		let cargo_metadata = Workspace::with_crate_dir( CrateDir::try_from( path )? )?;
		for path in cargo_metadata.packages_get()?.into_iter().map( |p| p.manifest_path.as_std_path() )
		{
			let header = ModuleHeader::from_cargo_toml( path )?.to_header()?;
			let read_me_path =  path
			.parent()
			.unwrap()
			.join( readme_path( path.parent().unwrap() ).ok_or_else::< Error, _ >( || err!( "Fail to find README.md" ) )?);

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
			
			let content: String = TAGS_TEMPLATE.get().unwrap().replace( &content, &format!( "<!--{{ generate.module_header.start({raw_params}) }}-->\n{header}\n<!--{{ generate.module_header.end }}-->" ) ).into();
			
			file.set_len( 0 )?;
			file.seek( SeekFrom::Start( 0 ) )?;
			file.write_all( content.as_bytes() )?;
		}
		Ok( () )
	}
}

crate::mod_interface!
{
  /// Generate headers in modules
  prelude use generate_modules_headers;
}