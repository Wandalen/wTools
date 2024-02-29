use super::*;

mod basic_test;

// #[ cfg( any( feature = "derive_clone_dyn_use_std", feature = "derive_clone_dyn_use_alloc" ) ) ]
// mod clone_dyn_test;

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
mod from_inner_named_manual_test;
mod from_inner_multiple_named_manual_test;
mod from_inner_multiple_manual_test;
mod from_inner_unit_manual_test;
#[ cfg( feature = "derive_from" ) ]
mod from_inner_test;
#[ cfg( feature = "derive_from" ) ]
mod from_inner_named_test;
#[ cfg( feature = "derive_from" ) ]
mod from_inner_multiple_named_test;
#[ cfg( feature = "derive_from" ) ]
mod from_inner_unit_test;
#[ cfg( feature = "derive_from" ) ]
mod from_inner_multiple_test;

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

#[ cfg( feature = "derive_reflect" ) ]
mod reflect_common_test;
#[ cfg( feature = "derive_reflect" ) ]
mod reflect_primitive_test;
#[ cfg( feature = "derive_reflect" ) ]
mod reflect_struct_manual_test;
#[ cfg( feature = "derive_reflect" ) ]
mod reflect_struct_in_struct_manual_test;
#[ cfg( feature = "derive_reflect" ) ]
mod reflect_struct_with_lifetime_manual_test;
#[ cfg( feature = "derive_reflect" ) ]
mod reflect_slice_test;
#[ cfg( feature = "derive_reflect" ) ]
mod reflect_vec_test;
#[ cfg( feature = "derive_reflect" ) ]
mod reflect_hashset_test;
#[ cfg( feature = "derive_reflect" ) ]
mod reflect_hashmap_test;
#[ cfg( feature = "derive_reflect" ) ]
mod reflect_array_test;

// #[ cfg( all( feature = "type_variadic_from" ) ) ]
// mod variadic_from_manual_test;
//
// #[ cfg( all( feature = "type_variadic_from" ) ) ]
// mod variadic_from_manual_beyond_test;
//
// // #[ cfg( all( feature = "derive_variadic_from", feature = "type_variadic_from" ) ) ]
// // mod variadic_from_derive_test;
// // xxx : fix
// #[ cfg( all( feature = "derive_variadic_from", feature = "type_variadic_from" ) ) ]
// mod variadic_from2_derive;

// #[ cfg( any( feature = "derive_variadic_from", feature = "type_variadic_from" ) ) ]
// #[ path = "../../../../../module/core/variadic_from/tests/inc/mod.rs" ]
// mod variadic_tests;
