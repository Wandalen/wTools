
use crate::error::BasicError;

/// Type alias for Result with BasicError.
pub type Result< T, E = BasicError > = std::result::Result< T, E >;
