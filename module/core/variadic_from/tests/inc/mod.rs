use super::*;

#[ cfg( all( feature = "type_variadic_from" ) ) ]
mod variadic_from_manual_test;

#[ cfg( all( feature = "type_variadic_from" ) ) ]
mod variadic_from_manual_beyond_test;

// #[ cfg( all( feature = "derive_variadic_from", feature = "type_variadic_from" ) ) ]
// mod variadic_from_derive_test;
// xxx : fix

#[ cfg( all( feature = "derive_variadic_from", feature = "type_variadic_from" ) ) ]
mod variadic_from2_derive;
