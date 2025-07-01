<!-- {{# generate.module_header{} #}} -->
# Module :: `clone_dyn`
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_clone_dyn_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_clone_dyn_push.yml) [![docs.rs](https://img.shields.io/docsrs/clone_dyn?color=e3e8f0&logo=docs.rs)](https://docs.rs/clone_dyn) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Fclone_dyn%2Fexamples%2Fclone_dyn_trivial.rs,RUN_POSTFIX=--example%20module%2Fcore%2Fclone_dyn%2Fexamples%2Fclone_dyn_trivial.rs/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Derive to clone dyn structures.

This crate is a facade that re-exports `clone_dyn_types` (for core traits and logic) and `clone_dyn_meta` (for procedural macros). It provides a convenient way to enable cloning for trait objects. By default, Rust does not support cloning for trait objects due to the `Clone` trait requiring compile-time knowledge of the type's size. The `clone_dyn` crate addresses this limitation through its procedural macros, allowing for cloning collections of trait objects. The crate's purpose is straightforward: it allows for easy cloning of `dyn< Trait >` with minimal effort and complexity, accomplished by applying the `#[clone_dyn]` attribute to the trait.

### Alternative

There are few alternatives [dyn-clone](https://github.com/dtolnay/dyn-clone), [dyn-clonable](https://github.com/kardeiz/objekt-clonable). Unlike other options, this solution is more concise and demands less effort to use, all without compromising the quality of the outcome.

## Basic use-case

This example demonstrates the usage of the `#[clone_dyn]` attribute macro to enable cloning for trait objects.

```rust
#[ cfg( feature = "derive_clone_dyn" ) ]
#[ clone_dyn ]
pub trait Trait1
{
  fn f1( &self );
}

#[ cfg( feature = "derive_clone_dyn" ) ]
#[ clone_dyn ]
pub trait Trait2 : Trait1
{
  fn f2( &self );
}

// Example usage of Trait1 and Trait2 with cloning
let obj1: Box<dyn Trait1> = Box::new(10i32);
let cloned_obj1 = obj1.clone();
// ... further usage ...
```

<details>
<summary>If you use multithreading or asynchronous paradigms implement trait `Clone` also for `Send` and `Sync`</summary>

```rust, ignore

#[ allow( non_local_definitions ) ]
impl< 'c, T > Clone for Box< dyn IterTrait< 'c, T > + 'c >
{
  #[ inline ]
  fn clone( &self ) -> Self
  {
    clone_dyn::clone_into_box( &**self )
  }
}

#[ allow( non_local_definitions ) ]
impl< 'c, T > Clone for Box< dyn IterTrait< 'c, T > + Send + 'c >
{
  #[ inline ]
  fn clone( &self ) -> Self
  {
    clone_dyn::clone_into_box( &**self )
  }
}

#[ allow( non_local_definitions ) ]
impl< 'c, T > Clone for Box< dyn IterTrait< 'c, T > + Sync + 'c >
{
  #[ inline ]
  fn clone( &self ) -> Self
  {
    clone_dyn::clone_into_box( &**self )
  }
}

#[ allow( non_local_definitions ) ]
impl< 'c, T > Clone for Box< dyn IterTrait< 'c, T > + Send + Sync + 'c >
{
  #[ inline ]
  fn clone( &self ) -> Self
  {
    clone_dyn::clone_into_box( &**self )
  }
}

```

</details>

<br/>

Try out `cargo run --example clone_dyn_trivial`.
<br/>
[See code](./examples/clone_dyn_trivial.rs).

### To add to your project

```sh
cargo add clone_dyn
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd examples/clone_dyn_trivial
cargo run
