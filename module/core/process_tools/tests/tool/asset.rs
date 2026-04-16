pub const ASSET_PATH : &str = "tests/asset";

macro_rules! ERR_MSG {
  () =>
  {
    "Create `.cargo/config.toml` file at root of your project and append it by
```
[env]
WORKSPACE_PATH = { value = \".\", relative = true }
```"
  };
}

pub fn path() -> std ::io ::Result< std ::path ::PathBuf >
{
  use std ::
  {
    path ::Path,
    io ::{ self, ErrorKind },
  };
  let workspace_path = Path ::new( env!( "WORKSPACE_PATH", ERR_MSG! {} ) );
  let dir_path = workspace_path.join( Path ::new( file!() ) );
  let dir_path = dir_path.canonicalize()?;
  let test_dir = dir_path
  .parent()
  .ok_or_else( ||
  {
    io ::Error ::new(
      ErrorKind ::NotFound,
      format!( "Failed to find parent directory {}", dir_path.display() ),
    )
  })?
  .parent()
  .ok_or_else( ||
  {
    io ::Error ::new(
      ErrorKind ::NotFound,
      format!( "Failed to find parent directory {}", dir_path.display() ),
    )
  })?
  .parent()
  .ok_or_else( ||
  {
    io ::Error ::new(
      ErrorKind ::NotFound,
      format!( "Failed to find parent directory {}", dir_path.display() ),
    )
  })?;
  let assets_path = test_dir.join( Path ::new( ASSET_PATH ) );
  Ok( assets_path )
}
