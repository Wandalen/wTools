# module::winterval

Interval adapter for both open/closed implementations of intervals ( ranges ).

### Sample

```rust
use winterval::*;

let src = 2..5;
assert_eq!( src.closed(), ( 2, 4 ) );

let src = 2..=4;
assert_eq!( src.closed(), ( 2, 4 ) );
```

### To add to your project

```sh
cargo add winterval
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/winterval_trivial
cargo run
```
