# DEPRECATED: Module :: willbe2
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://img.shields.io/github/actions/workflow/status/Wandalen/wTools/workspace_push.yml?label=&branch=master&job=willbe_2)](https://github.com/Wandalen/wTools/actions/workflows/workspace_push.yml) [![docs.rs](https://img.shields.io/docsrs/willbe2?color=e3e8f0&logo=docs.rs)](https://docs.rs/willbe2) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Utility to publish multi-crate and multi-workspace environments and maintain their consistency.

`willbe2` is a binary alias for [willbe](../willbe/readme.md). It re-exports the entire `willbe` library API and delegates all CLI commands to the core `willbe` implementation. Use this crate when you need the `willbe2` binary name.

### Basic use-case

```bash
cargo install willbe2 --features full
willbe2 .list
```

### To add to your project

```bash
cargo add willbe2 --features full
```

### Try out from the repository

```shell
git clone https://github.com/Wandalen/wTools
cd wTools
cargo run --manifest-path module/experimental/willbe2/Cargo.toml --features enabled -- .list
```
