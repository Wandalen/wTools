# module::werror

Basic exceptions handling mechanism.

### Try out from the repository

``` shell test
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/werror_trivial
cargo run
```

### To add to your project

```
cargo add werror
```

### Sample

``` rust sample test
use werror::*;

let err1 = Error::new( "Some error" );
println!( "err1 : {}", err1 );
// < err1 : Some error
```
