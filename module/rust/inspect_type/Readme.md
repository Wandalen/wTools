# module::inspect_type

Diagnostic-purpose tools to inspect type of a variable and its size.

### Sample

``` rust test
#![ feature( type_name_of_val ) ]

pub use inspect_type::*;

fn main()
{
  inspect_type_of!( &[ 1, 2, 3 ][ .. ] );
  // < sizeof( &[1, 2, 3][..] : &[i32] ) = 16
  inspect_type_of!( &[ 1, 2, 3 ] );
  // < sizeof( &[1, 2, 3] : &[i32; 3] ) = 8
}
```

### To add to your project

``` shell
cargo add implements
```

### Try out from the repository

``` shell test
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/inspect_type_trivial
cargo run
```
