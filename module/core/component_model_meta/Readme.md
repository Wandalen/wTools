<!-- {{# generate.module_header{} #}} -->

# Module :: `component_model_meta`
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_component_model_meta_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_component_model_meta_push.yml) [![docs.rs](https://img.shields.io/docsrs/component_model_meta?color=e3e8f0&logo=docs.rs)](https://docs.rs/component_model_meta) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

A flexible implementation of the Builder pattern supporting nested builders and collection-specific subcomponent_models. Implementation of its derive macro. Should not be used independently, instead use `module::component_model` which relies on the module.

Not intended to be used without runtime. This module and runtime is aggregate in `module::component_model` is [here](https://github.com/Wandalen/wTools/tree/master/module/core/component_model).

### To add to your project

```sh
cargo add component_model_meta
```
