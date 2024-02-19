mod private
{
	use std::fs::{File, OpenOptions};
	use std::io::{Read, Seek, SeekFrom, Write};
	use std::path::Path;
	use convert_case::{Case, Casing};
	use toml_edit::Document;
	use crate::path::AbsolutePath;
	use crate::{CrateDir, url, Workspace};
	use crate::endpoint::table::{readme_path, Stability, stability_generate};
	use crate::wtools::error::
	{
		err,
	  for_app::{ bail, Result, Error },
	};

	struct ModuleHeader
	{
		stability: Stability,
	  module_name: String,
	  repository_url: String,
	}

	impl ModuleHeader
	{
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

			Ok
				(
					Self
					{
						stability,
						module_name,
						repository_url,
					}
				)
		}

		fn to_header( self ) -> Result< String >
		{
			Ok(format!
			(
				"{}\
				[![rust-status](https://github.com/{}/actions/workflows/Module{}Push.yml/badge.svg)](https://github.com/{}/actions/workflows/Module{}Push.yml)\
				[![docs.rs](https://img.shields.io/docsrs/{}?color=e3e8f0&logo=docs.rs)](https://docs.rs/{})\
				[![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2F{}_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20{}_trivial_sample/https://github.com/{})\
				[![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)",
				stability_generate( &self.stability ),
				url::git_info_extract( &self.repository_url )?, self.module_name.to_case( Case::Pascal ), url::git_info_extract( &self.repository_url )?, self.module_name.to_case( Case::Pascal ),
				self.module_name, self.module_name,
				self.module_name, self.module_name, url::git_info_extract( &self.repository_url )?
			))
		}
	}

	/// Generates headers for each module
	pub fn generate_modules_headers( path: &Path ) -> Result< () >
	{
		let absolute_path = AbsolutePath::try_from( path )?;
		let cargo_metadata = Workspace::with_crate_dir( CrateDir::try_from( absolute_path )? )?;
		for path in cargo_metadata.packages_get()?.into_iter().map(|p| p.manifest_path.as_std_path() ) {
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
			let content = content.replace( "<!--{ generate.module_header }-->", &format!( "<!--{{ generate.module_header }}-->\n{header}" ) );
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