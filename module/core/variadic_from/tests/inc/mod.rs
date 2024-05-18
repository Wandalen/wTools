use super::*;

#[ cfg( all( feature = "type_variadic_from" ) ) ]
mod from4_named_manual;
#[ cfg( all( feature = "type_variadic_from" ) ) ]
mod from4_unnamed_manual;

// xxx
// #[ cfg( all( feature = "derive_variadic_from", feature = "type_variadic_from" ) ) ]
// mod auto_std_named_derive;

#[ cfg( all( feature = "derive_variadic_from", feature = "type_variadic_from" ) ) ]
mod auto_std_named_manual;

// #[ cfg( all( feature = "derive_variadic_from", feature = "type_variadic_from" ) ) ]
// mod variadic_from_derive;
// xxx : fix

#[ cfg( all( feature = "type_variadic_from" ) ) ]
mod from4_named_manual_beyond;
#[ cfg( all( feature = "type_variadic_from" ) ) ]
mod from4_unnamed_manual_beyond;

#[ cfg( all( feature = "derive_variadic_from", feature = "type_variadic_from" ) ) ]
mod sample;
