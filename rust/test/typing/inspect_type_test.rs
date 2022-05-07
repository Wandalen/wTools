#![ cfg_attr( feature = "nightly", feature( type_name_of_val ) ) ]

#[ cfg( feature = "nightly" ) ]
use inspect_type as TheModule;
include!( "./common/inspect_type_test.rs" );
