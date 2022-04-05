# module::is_slice

Macro to answer the question: is it a slice?

### Sample

``` rust sample test
use is_slice::*;

fn main()
{

  dbg!( is_slice!( Box::new( true ) ) );
  // < is_slice!(Box :: new(true)) = false
  dbg!( is_slice!( &[ 1, 2, 3 ] ) );
  // < is_slice!(& [1, 2, 3]) = false
  dbg!( is_slice!( &[ 1, 2, 3 ][ .. ] ) );
  // < is_slice!(& [1, 2, 3] [..]) = true

}
```

### To add to your project

```
cargo add is_slice
```

### Try out from the repository

``` shell test
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/is_slice_trivial
cargo run
```
