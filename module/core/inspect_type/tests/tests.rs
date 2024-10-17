// #![ allow( unexpected_cfgs ) ]

// #![ no_std ]

// #![ cfg_attr( feature = "no_std", no_std ) ]
// #![ cfg( custom_inner_attributes ) ]
// #![ test_tools::nightly ]
// #![ cfg_attr( feature = "type_name_of_val", feature( type_name_of_val ) ) ]
// #![ cfg_attr( rustversion::nightly, feature( type_name_of_val ) ) ]
// #![cfg_attr(docsrs, feature(doc_cfg))]
// // #![ cfg_attr( feature = "nightly", feature( type_name_of_val ) ) ]
// #![ cfg_attr( feature = "nightly", feature( trace_macros ) ) ]
// #![ cfg_attr( feature = "nightly", feature( meta_idents_concat ) ) ]

// #![ cfg_attr( RUSTC_IS_NIGHTLY, feature( type_name_of_val ) ) ]

#[ allow( unused_imports ) ]
use inspect_type as the_module;
#[ allow( unused_imports ) ]
use test_tools::exposed::*;

mod inc;
