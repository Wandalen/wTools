# module::macro_tools

Tools for writing procedural macroses.

### Sample

``` rust sample test
use macro_tools::*;

fn main()
{
  let code = quote!( core::option::Option< i8, i16, i32, i64 > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = type_parameters( &tree_type, 0..=2 );
  got.iter().for_each( | e | println!( "{}", quote!( #e ) ) );
  // < i8
  // < i16
  // < i32
}
```

### To add to your project

```
cargo add macro_tools
```
