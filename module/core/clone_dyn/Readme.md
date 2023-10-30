<!-- {{# generate.module_header{} #}} -->
# Module :: clone_dyn
[![experimental](https://raster.shields.io/static/v1?label=stability&message=experimental&color=orange&logoColor=eee)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleCloneDynPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleCloneDynPush.yml) [![docs.rs](https://img.shields.io/docsrs/clone_dyn?color=e3e8f0&logo=docs.rs)](https://docs.rs/clone_dyn) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Fclone_dyn_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20clone_dyn_trivial_sample/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)

Derive to clone dyn structures.

The purpose of the crate is very simple, making `dyn< Trait >` clonable with minimum efforts and complexity, simply by applying to derive to the trait.

### Alternative

There are few alternatives [dyn-clone](https://github.com/dtolnay/dyn-clone), [dyn-clonable](https://github.com/kardeiz/objekt-clonable). Unlike alternatives, this solution is more compact and requires fewer efforts to use without loss of quality of the solution. Also, you can ask an inquiry and get answers, which is problematic in the case of alternatives.

### Basic use-case.

<!-- begin {{# generate.module_sample( "example/clone_dyn_trivail_sample.rs" ) #}} -->

```rust
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
{
  use clone_dyn::clone_dyn;

  #[ clone_dyn ]
  trait Trait1
  {
  }

  let vec = Vec::< Box< dyn Trait1 > >::new();
  let vec2 = vec.clone(); /* <- it does not work without `clone_dyn` */
}
```

<!-- end -->

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
```
