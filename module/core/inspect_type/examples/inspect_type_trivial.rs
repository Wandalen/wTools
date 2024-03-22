//! qqq : write proper description
#![ cfg_attr( feature = "type_name_of_val", feature( type_name_of_val ) ) ]

// // #![ cfg_attr( feature = "nightly", feature( type_name_of_val ) ) ]
// #![ rustversion::attr( nightly, feature( type_name_of_val ) ) ]

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
fn main()
{
  // #[ cfg( feature = "nightly" ) ]
  // {
  //   inspect_type_of!( &[ 1, 2, 3 ][ .. ] );
  //   // < sizeof( &[1, 2, 3][..] : &[i32] ) = 16
  //   inspect_type_of!( &[ 1, 2, 3 ] );
  //   // < sizeof( &[1, 2, 3] : &[i32; 3] ) = 8
  // }
  // #[ cfg( not( feature = "nightly" ) ) ]
  // {
  //   println!( "\nTo run sample correctly, run sample on nightly rustup channel. To change channel run :" );
  //   println!( "rustup default nightly\n" );
  //   println!( "The command from the root of the sample :" );
  //   println!( "cargo run --features nightly\n" );
  //   println!( "The command from the root of module :" );
  //   println!( "cargo run --example inspect_type_trivial --features nightly" );
  // }
}
