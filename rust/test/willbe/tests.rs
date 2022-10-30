use std::path::PathBuf;

use willbe::prelude::
{
  *,
  entities::*, //? may be moved to prelude
};

#[ test ]
fn package_from_path()
{
  // current package is a package
  assert!( Package::try_from( PathBuf::from( "" ) ).is_ok() );
}

#[ test ]
fn workspace_from_path()
{
  // current package is a package, not a workspace
  assert!( Workspace::try_from( PathBuf::from( "" ) ).is_err() );
  // main workspace is a workspace
  assert!( Workspace::try_from( PathBuf::from( "../../../" ) ).is_ok() );
}

#[ test ]
fn workspace_iterator()
{
  let workspace = Workspace::try_from( PathBuf::from( "../../../" ) ).unwrap();

  let packages = workspace.packages_iterate( OrderStrategy::Random ).collect::< Vec< _ > >();

  assert!( !packages.is_empty() );
}