use super::*;

mod basic_test;

#[ cfg( any( feature = "derive_clone_dyn_use_std", feature = "derive_clone_dyn_use_alloc" ) ) ]
mod clone_dyn_test;

mod all_manual_test;
#[ cfg
(
	all
	(
		feature = "derive_as_mut",
		feature = "derive_as_ref",
		feature = "derive_deref",
		feature = "derive_deref_mut",
		feature = "derive_from",
		feature = "derive_inner_from",
	)
)]
mod all_test;

mod as_mut_manual_test;
#[ cfg( feature = "derive_as_mut" ) ]
mod as_mut_test;

mod as_ref_manual_test;
#[ cfg( feature = "derive_as_ref" ) ]
mod as_ref_test;

mod deref_manual_test;
#[ cfg( feature = "derive_deref" ) ]
mod deref_test;

mod deref_mut_manual_test;
#[ cfg( feature = "derive_deref_mut" ) ]
mod deref_mut_test;

mod from_inner_manual_test;
#[ cfg( feature = "derive_from" ) ]
mod from_inner_test;

mod inner_from_manual_test;
#[ cfg( feature = "derive_inner_from" ) ]
mod inner_from_test;

#[ cfg( all( feature = "type_from" ) ) ]
mod variadic_from_manual_test;

#[ cfg( all( feature = "type_from" ) ) ]
mod variadic_from_manual_beyond_test;

// #[ cfg( all( feature = "derive_variadic_from", feature = "type_from" ) ) ]
// mod variadic_from_derive_test;
// xxx : fix

#[ cfg( all( feature = "derive_variadic_from", feature = "type_from" ) ) ]
mod variadic_from2_derive;
