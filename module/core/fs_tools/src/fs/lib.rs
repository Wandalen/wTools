#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https: //raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc(
  html_favicon_url = "https: //raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico"
) ]
#![ doc( html_root_url = "https: //docs.rs/fs_tools/latest/fs_tools/" ) ]
#![ cfg_attr( doc, doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "readme.md" ) ) ) ]
#![ cfg_attr( not( doc ), doc = "File system utilities" ) ]

/// Collection of primal data types.
pub mod fs;

/// Re-export of the glob crate for filesystem pattern matching.
///
/// Provides Unix shell-style glob pattern matching for finding files
/// and directories. Available when the `glob` feature is enabled.
///
/// # Example
///
/// ```
/// # #[ cfg( feature = "glob" ) ]
/// # {
/// use fs_tools::glob::glob;
///
/// // Find all Rust files in current directory
/// for entry in glob( "*.rs" ).expect( "valid pattern" )
/// {
///   if let Ok( path ) = entry
///   {
///     println!( "{:?}", path );
///   }
/// }
/// # }
/// ```
#[ cfg( feature = "glob" ) ]
#[ doc( inline ) ]
pub use ::glob;

/// Namespace with dependencies.
#[ cfg( feature = "enabled" ) ]
pub mod dependency
{
  /// Re-export of the glob crate.
  #[ cfg( feature = "glob" ) ]
  #[ doc( inline ) ]
  pub use ::glob;
}

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own 
{
  use super :: *;
  #[ doc( inline ) ]
  pub use orphan :: *;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super ::fs ::orphan :: *;
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own :: *;

/// Shared with parent namespace of the module
#[ allow( unused_imports ) ]
pub mod orphan 
{
  use super :: *;
  #[ doc( inline ) ]
  pub use exposed :: *;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed 
{
  use super :: *;
  #[ doc( inline ) ]
  pub use prelude :: *;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super ::fs ::exposed :: *;
}

/// Prelude to use essentials: `use my_module ::prelude :: *`.
#[ allow( unused_imports ) ]
pub mod prelude 
{
  use super :: *;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super ::fs ::prelude :: *;
}
