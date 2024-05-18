use super::*;

#[ cfg( all( feature = "type_variadic_from" ) ) ]
mod from4_named_manual;
#[ cfg( all( feature = "type_variadic_from" ) ) ]
mod from4_tuple_manual;

// mod variadic_from_manual_test;

#[ cfg( all( feature = "type_variadic_from" ) ) ]
mod variadic_from_manual_beyond_test;

// #[ cfg( all( feature = "derive_variadic_from", feature = "type_variadic_from" ) ) ]
// mod variadic_from_derive_test;
// xxx : fix

// xxx
// #[ cfg( all( feature = "derive_variadic_from", feature = "type_variadic_from" ) ) ]
// mod auto_std_named_derive;

#[ cfg( all( feature = "derive_variadic_from", feature = "type_variadic_from" ) ) ]
mod auto_std_named_manual;
