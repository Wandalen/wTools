# module::instance_of

Macro to answer the question: does it implement a trait?

This solution has a limitation:

- In case enity is a function and trat is `Fn`/`FnMut`/`FnOnce` which current entity does not implement you will get compile-time error instead of `false`.

This is alias for [module::implements](https://github.com/Wandalen/wTools/tree/master/module/rust/implements).

### Sample

```rust
use instance_of::*;

dbg!( instance_of!( 13_i32 => Copy ) );
// < instance_of!( 13_i32 => Copy ) : true
dbg!( instance_of!( Box::new( 13_i32 ) => Copy ) );
// < instance_of!( 13_i32 => Copy ) : false
```

### Try out from the repository

```
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/meta_implements_trivial
cargo run
```

### To add to your project

```
cargo add implements
```
