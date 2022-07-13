
//!
//! Library of utility to operate packages from a command line.
//!

/// Protected namespace of the module.
pub mod protected
{
}

/// Orphan namespace of the module.
pub mod orphan
{
}

/// Exposed namespace of the module.
pub mod exposed
{
}

/// Prelude namespace of the module.
pub mod prelude
{
}

///
/// Work with bools.
///

#[ cfg( feature = "use_std" ) ]
pub mod bool;

///
/// Manipulate over files.
///

#[ cfg( feature = "use_std" ) ]
pub mod files;

///
/// Manipulate over manifest.
///

#[ cfg( feature = "use_std" ) ]
pub mod manifest;

///
/// Work with crate on `crates.io`.
///

#[ cfg( feature = "use_std" ) ]
pub mod http;

///
/// Run external processes.
///

#[ cfg( feature = "use_std" ) ]
pub mod process;

///
/// Make sha-1 hash for data.
///

#[ cfg( feature = "use_std" ) ]
pub mod digest;
