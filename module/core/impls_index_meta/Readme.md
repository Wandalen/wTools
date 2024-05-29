<!-- {{# generate.module_header{} #}} -->

# Module :: impls_index_meta
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_impls_index_meta_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_impls_index_meta_push.yml) [![docs.rs](https://img.shields.io/docsrs/impls_index_meta?color=e3e8f0&logo=docs.rs)](https://docs.rs/impls_index_meta) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Several of macros to put each function under a named macro to index every function in a class.

It encourages writing better code, having index of components stripped of details of implementation is very important for comprehension of the code and ability to see the big picture.

Not intended to be used without runtime. This module and runtime is aggregate in module::impls_index is [here](https://github.com/Wandalen/wTools/tree/master/module/core/impls_index).

### To add to your project

```sh
cargo add impls_index
```
