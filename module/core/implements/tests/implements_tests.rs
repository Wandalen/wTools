// #![cfg_attr(docsrs, feature(doc_cfg))]
// #![ cfg_attr( feature = "nightly", feature( type_name_of_val ) ) ]
#![ cfg_attr( feature = "nightly", feature( trace_macros ) ) ]
#![ cfg_attr( feature = "nightly", feature( meta_idents_concat ) ) ]

use test_tools::exposed::*;

use implements as the_module;

mod inc;
