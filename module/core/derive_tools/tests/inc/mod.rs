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
#[ path = "from" ]
mod tests
{
	#[ allow( unused_imports ) ]
	use super::*;

	mod named_test;
	mod named_manual_test;

	mod manual_test;
	mod multiple_named_manual_test;
	mod multiple_manual_test;
	mod unit_manual_test;
	mod test;
	mod multiple_named_test;
	mod unit_test;
	mod multiple_test;

	mod variants_manual;
	mod variants_derive;

	mod variants_duplicates_all_off;
	mod variants_duplicates_some_off;
	mod variants_duplicates_some_off_default_off;

	mod variants_generics;
	mod variants_generics_where;
	mod variants_collisions;
}

#[ cfg( feature = "derive_inner_from" ) ]
#[ path = "inner_from" ]
mod inner_from_tests
{
	#[ allow( unused_imports ) ]
	use super::*;

	mod manual_test;
	mod named_manual_test;
	mod multiple_named_manual_test;
	mod multiple_manual_test;
	mod unit_manual_test;
	mod test;
	mod named_test;
	mod multiple_named_test;
	mod unit_test;
	mod multiple_test;

}
