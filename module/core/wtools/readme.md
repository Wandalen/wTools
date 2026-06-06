
# Module :: wtools
<!--{ generate.module_header.start() }-->
 [![stable](https://raster.shields.io/static/v1?label=&message=stable&color=brightgreen)](https://github.com/emersion/stability-badges#stable) [![rust-status](https://img.shields.io/github/actions/workflow/status/Wandalen/wTools/workspace_push.yml?label=&branch=master&job=wtools)](https://github.com/Wandalen/wTools/actions/workflows/workspace_push.yml) [![docs.rs](https://img.shields.io/docsrs/wtools?color=e3e8f0&logo=docs.rs)](https://docs.rs/wtools) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Fwtools%2Fexamples%2Fwtools_trivial.rs,RUN_POSTFIX=--example%20wtools_trivial/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Single entry-point for collection utilities. Re-exports `collection_tools`: variadic constructors (`hmap!`, `vec!`, `hset!`, …) and unified `HashMap`/`HashSet`/`Vec` support with optional `no_std`/`hashbrown` backend.

### To add to your project

```sh
cargo add wtools
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cargo run --example wtools_trivial
```
