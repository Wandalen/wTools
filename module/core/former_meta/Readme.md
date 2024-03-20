<!-- {{# generate.module_header{} #}} -->

# Module :: former_meta
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) |[![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleFormerMetaPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleFormerMetaPush.yml)[![docs.rs](https://img.shields.io/docsrs/former_meta?color=e3e8f0&logo=docs.rs)](https://docs.rs/former_meta)[![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Fformer_meta_trivial%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20former_meta_trivial/https://github.com/Wandalen/wTools)
<!--{ generate.module_header.end }-->

Former - a variation of builder pattern. Implementation of its derive macro. Should not be used independently, instead use module::former which relies on the module.

Not intended to be used without runtime. This module and runtime is aggregate in module::former is [here](https://github.com/Wandalen/wTools/tree/master/module/core/former).

### To add to your project

```sh
cargo add former_meta
```
