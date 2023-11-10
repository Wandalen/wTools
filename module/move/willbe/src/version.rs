/// Internal namespace.
mod private
{
  pub fn bump( version : &str ) -> anyhow::Result< String >
  {
    let mut splits : Vec< &str > = version.split( '.' ).collect();
    let patch_version = splits[ 2 ].parse::< u32 >()? + 1;
    let v = &patch_version.to_string();
    splits[ 2 ] = v;

    Ok( splits.join( "." ) )
  }
}

//

crate::mod_interface!
{
  /// Bump version.
  protected( crate ) use bump;
}
