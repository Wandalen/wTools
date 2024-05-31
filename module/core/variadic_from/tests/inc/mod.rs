#![ allow( unused_imports ) ]

use super::*;

#[ cfg( all( feature = "type_variadic_from" ) ) ]
mod from2_named_manual;
#[ cfg( all( feature = "derive_variadic_from", feature = "type_variadic_from" ) ) ]
mod from2_named_derive;

#[ cfg( all( feature = "type_variadic_from" ) ) ]
mod from2_unnamed_manual;
#[ cfg( all( feature = "derive_variadic_from", feature = "type_variadic_from" ) ) ]
mod from2_unnamed_derive;

#[ cfg( all( feature = "type_variadic_from" ) ) ]
mod from4_named_manual;
#[ cfg( all( feature = "type_variadic_from" ) ) ]
mod from4_unnamed_manual;

#[ cfg( all( feature = "type_variadic_from" ) ) ]
mod from4_beyond_named;
#[ cfg( all( feature = "type_variadic_from" ) ) ]
mod from4_beyond_unnamed;

#[ cfg( all( feature = "type_variadic_from" ) ) ]
mod from0_named_manual;
#[ cfg( all( feature = "derive_variadic_from", feature = "type_variadic_from" ) ) ]
mod from0_named_derive;
#[ cfg( all( feature = "derive_variadic_from", feature = "type_variadic_from" ) ) ]
mod from0_unnamed_derive;

#[ cfg( all( feature = "derive_variadic_from", feature = "type_variadic_from" ) ) ]
mod sample;
#[ cfg( all( feature = "type_variadic_from" ) ) ]
mod exports;
