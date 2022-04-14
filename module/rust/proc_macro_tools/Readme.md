
# module::proc_macro_tools

Tools for writing procedural macroses.

### Sample

```rust
use proc_macro_tools::*;
use quote::quote;

let code = quote!( core::option::Option< i8, i16, i32, i64 > );
let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
let got = type_parameters( &tree_type, 0..=2 );
got.iter().for_each( | e | println!( "{}", quote!( #e ) ) );
// < i8
// < i16
// < i32
```

### To add to your project

```sh
cargo add proc_macro_tools
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/proc_macro_tools_trivial
cargo run
```
