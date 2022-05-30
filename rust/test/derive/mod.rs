// qqq : for Dima : bad

// #[ cfg( all( feature = "derive_from", feature = "derive_into", feature = "derive_display", feature = "derive_from_str" ) ) ]
#[ cfg( feature = "derive" ) ]
use wtools::derive as TheModule;
// #[ cfg( all( feature = "derive_from", feature = "derive_into", feature = "derive_display", feature = "derive_from_str" ) ) ]
#[ cfg( feature = "derive" ) ]
mod basic_test;
