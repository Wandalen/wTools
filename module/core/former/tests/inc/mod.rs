
use super::*;
use test_tools::exposed::*;

#[ cfg( feature = "derive_former" ) ]
mod struct_tests;

#[ cfg( feature = "derive_former" ) ]
pub mod enum_unit_tests;
#[ cfg( feature = "derive_former" ) ]
pub mod enum_unnamed_tests;
#[ cfg( feature = "derive_former" ) ]
pub mod enum_named_tests;
#[ cfg( feature = "derive_former" ) ]
pub mod enum_complex_tests;
