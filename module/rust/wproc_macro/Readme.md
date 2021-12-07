
# module::wproc_macro

Tools for writing procedural macroses.

### Try out from the repository

``` shell test
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/wproc_macro_trivial
cargo run
```

### To add to your project

``` shell test
cargo add wproc_macro
```

### Sample

``` rust test
let code = quote!( core::option::Option< i8, i16, i32, i64 > );
let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
let got = type_parameters( &tree_type, 0..=2 );
got.iter().for_each( | e | println!( "{}", quote!( #e ) ) );
// < i8
// < i16
// < i32
```