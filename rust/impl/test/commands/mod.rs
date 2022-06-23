
/// Perform smoke testing.
#[ cfg( feature = "use_std" ) ]
mod smoke;

/// Init aggregator commands.
#[ cfg( feature = "use_std" ) ]
mod init;
#[ cfg( feature = "use_std" ) ]
pub use init::*;
