<!-- {{# generate.module_header{} #}} -->
# Module :: clone_dyn_types
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_clone_dyn_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_clone_dyn_push.yml) [![docs.rs](https://img.shields.io/docsrs/clone_dyn_types?color=e3e8f0&logo=docs.rs)](https://docs.rs/clone_dyn_types) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Fclone_dyn%2Fexamples%2Fclone_dyn_trivial.rs,RUN_POSTFIX=--example%20clone_dyn_trivial/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Derive to clone dyn structures.

It's types, use `clone_dyn` to avoid bolerplate.

By default, Rust does not support cloning for trait objects due to the `Clone` trait requiring compile-time knowledge of the type's size. The `clone_dyn` crate addresses this limitation through procedural macros, allowing for cloning collections of trait objects. Prefer to use `clone_dyn` instead of this crate, because `clone_dyn` includes this crate and also provides an attribute macro to generate boilerplate with one line of code.

### Alternative

There are few alternatives [dyn-clone](https://github.com/dtolnay/dyn-clone), [dyn-clonable](https://github.com/kardeiz/objekt-clonable). Unlike other options, this solution is more concise and demands less effort to use, all without compromising the quality of the outcome.

### Basic use-case

<!-- xxx : qqq : rewrite example -->
<!-- begin {{# generate.module( "example/clone_dyn_trivail.rs" ) #}} -->

<!-- ```rust
# #[ cfg( all( feature = "enabled", any( not( feature = "no_std" ), feature = "use_alloc" ) ) ) ]
# {
  use clone_dyn_types::clone_dyn_types;

  #[ clone_dyn_types ]
  trait Trait1
  {
  }

  let vec = Vec::< Box< dyn Trait1 > >::new();
  let vec2 = vec.clone(); /* <- it does not work without `clone_dyn_types` */
# }
``` -->

<!-- end -->

### To add to your project

```sh
cargo add clone_dyn_types
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd examples/clone_dyn_types_trivial
cargo run
```
