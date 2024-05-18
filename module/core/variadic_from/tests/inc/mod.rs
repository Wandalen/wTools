use super::*;

#[ cfg( all( feature = "type_variadic_from" ) ) ]
mod from4_named_manual;
#[ cfg( all( feature = "type_variadic_from" ) ) ]
mod from4_tuple_manual;

// xxx
// #[ cfg( all( feature = "derive_variadic_from", feature = "type_variadic_from" ) ) ]
// mod auto_std_named_derive;

#[ cfg( all( feature = "derive_variadic_from", feature = "type_variadic_from" ) ) ]
mod auto_std_named_manual;

// #[ cfg( all( feature = "derive_variadic_from", feature = "type_variadic_from" ) ) ]
// mod variadic_from_derive_test;
// xxx : fix

#[ cfg( all( feature = "type_variadic_from" ) ) ]
mod variadic_from_manual_beyond_test;
