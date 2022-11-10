/// Internal namespace.
pub( crate ) mod private
{
  use crate::protected::*;
  use std::env;
  use wtools::error::BasicError;

  ///
  /// Verify and publish a package
  /// 

  pub fn publish( instruction : &crate::instruction::Instruction ) -> Result< (), BasicError >
  {
    let current_path = env::current_dir().unwrap();
    let package = Package::try_from( current_path )?;
    let info = PackageMetadata::try_from( package.to_owned() )?;

    println!
    (
      "=== Verification ===\nLicense: {}\nReadme: {}\nDocumentation: {}\nTests: {}",
      if info.has_license() { "Yes" } else { "No" },
      if info.has_readme() { "Yes" } else { "No" },
      if info.has_documentation() { "Yes" } else { "No" },
      if info.is_tests_passed() { "Passed" } else { "Failed" }
    );

    // TODO: Check if verified before pushing

    if let Some( remote_url ) = instruction.properties_map.get( "push_remote" )
    {
      let url = remote_url.clone().primitive().unwrap();

      let mut package_rep = PackageRepository::try_from( package )?;
      // TODO: Think about refs. Who should set branch to push?
      package_rep.add_refspec( "refs/heads/master:refs/heads/master" );

      package_rep
      // if package is inside workspace - `*` will be replaced with `<path to package>/*`
      .add( "*" )
      .commit( format!( "AUTO: {pn}", pn = info.all().name ) )?;
      let mut rem = package_rep.remote_by_url( url )?;
      package_rep.push( &mut rem )?;
    }

    Ok( () )
  }
}

//

crate::mod_interface!
{
  prelude use publish;
}
