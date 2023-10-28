// #![cfg_attr(docsrs, feature(doc_cfg))]
#![ cfg_attr( feature = "nightly", feature( type_name_of_val ) ) ]
// #![ cfg_attr( feature = "nightly", feature( trace_macros ) ) ]
// #![ cfg_attr( feature = "nightly", feature( meta_idents_concat ) ) ]

#[ allow( unused_imports ) ]
use inspect_type as TheModule;
#[ allow( unused_imports ) ]
use test_tools::exposed::*;

mod inc;
