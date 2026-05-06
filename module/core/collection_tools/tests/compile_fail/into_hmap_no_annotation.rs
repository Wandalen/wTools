// tests/compile_fail/into_hmap_no_annotation.rs
//
// FT-02 (feature/002_into_constructors): type annotation required for into map macros.
//
// Given:  feature `collection_into_constructors` is enabled; no type annotation on the binding.
// When:   `into_hmap!` is invoked without specifying `HashMap< K, V >`.
// Then:   compilation fails; the compiler cannot determine K and V for the `.into()` calls.
//
// Root cause: `into_hmap!` uses `Into::into( $key )` and `Into::into( $value )`, which
// require the target key and value types to be known at the call site. Without a type
// annotation on the binding, type inference cannot resolve K and V.

fn main()
{
  let _m = collection_tools ::into_hmap!{ "a" => 1 };
}
