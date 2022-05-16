
// #![ feature( type_name_of_val ) ]
#![ cfg_attr( feature = "nightly", feature( type_name_of_val ) ) ]
// #![ cfg_attr( all( feature = "nightly" ), feature( type_name_of_val ) ) ]

#[ cfg( feature = "nightly" ) ]
compile_error!( "nightly!" );

//
// To run this sample, please make sure you are on nightly rustc and switched on feature "nightly"
//
// To switch to nightly rustc run:
// ```
// rustup default nightly && rustup update
// ```
//
// To run the sample with switched on feature "nightly" run:
// ```
// cargo run --features nightly
// ```
//

pub use inspect_type::*;

// #[ rustversion::nightly ]
#[ cfg( feature = "nightly" ) ]
fn main()
{
  inspect_type_of!( &[ 1, 2, 3 ][ .. ] );
  // < sizeof( &[1, 2, 3][..] : &[i32] ) = 16
  inspect_type_of!( &[ 1, 2, 3 ] );
  // < sizeof( &[1, 2, 3] : &[i32; 3] ) = 8
}

// #[ rustversion::stable ]
#[ cfg( not( feature = "nightly" ) ) ]
fn main()
{
}
