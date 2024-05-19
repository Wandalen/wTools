use super::*;

// = import tests of clone_dyn

#[ cfg( feature = "derive_clone_dyn" ) ]
#[ path = "../../../../core/clone_dyn/tests/inc/mod.rs" ]
mod clone_dyn_test;

// = import tests of variadic_from

#[ cfg( any( feature = "derive_variadic_from", feature = "type_variadic_from" ) ) ]
#[ path = "../../../../../module/core/variadic_from/tests/inc/mod.rs" ]
mod variadic_from_test;

// = own tests

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

mod basic_test;

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

#[ cfg( feature = "derive_from" ) ]
mod from_inner_named_test;
mod from_inner_named_manual_test;

mod from_inner_manual_test;
mod from_inner_multiple_named_manual_test;
mod from_inner_multiple_manual_test;
mod from_inner_unit_manual_test;
#[ cfg( feature = "derive_from" ) ]
mod from_inner_test;
#[ cfg( feature = "derive_from" ) ]
mod from_inner_multiple_named_test;
#[ cfg( feature = "derive_from" ) ]
mod from_inner_unit_test;
#[ cfg( feature = "derive_from" ) ]
mod from_inner_multiple_test;

#[ cfg( feature = "derive_from" ) ]
mod from_inner_variants_manual;
#[ cfg( feature = "derive_from" ) ]
mod from_inner_variants_derive;
#[ cfg( feature = "derive_from" ) ]
mod from_inner_variants_duplicates;
#[ cfg( feature = "derive_from" ) ]
mod from_inner_variants_generics;

mod inner_from_manual_test;
mod inner_from_named_manual_test;
mod inner_from_multiple_named_manual_test;
mod inner_from_multiple_manual_test;
mod inner_from_unit_manual_test;
#[ cfg( feature = "derive_inner_from" ) ]
mod inner_from_test;
#[ cfg( feature = "derive_inner_from" ) ]
mod inner_from_named_test;
#[ cfg( feature = "derive_inner_from" ) ]
mod inner_from_multiple_named_test;
#[ cfg( feature = "derive_inner_from" ) ]
mod inner_from_unit_test;
#[ cfg( feature = "derive_inner_from" ) ]
mod inner_from_multiple_test;

// xxx
