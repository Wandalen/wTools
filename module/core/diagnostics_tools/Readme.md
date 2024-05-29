<!-- {{# generate.module_header{} #}} -->

# Module :: diagnostics_tools
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_diagnostics_tools_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_diagnostics_tools_push.yml) [![docs.rs](https://img.shields.io/docsrs/diagnostics_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/diagnostics_tools) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Fdiagnostics_tools%2Fexamples%2Fdiagnostics_tools_trivial.rs,RUN_POSTFIX=--example%20diagnostics_tools_trivial/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Diagnostics tools.

### Basic use-case

<!-- {{# generate.module{} #}} -->

```rust
#[ test ]
#[ should_panic ]
fn a_id_panic_test()
{
  a_id!( 1, 2 );
  /*
    print :
    ...

thread 'a_id_panic_test' panicked at 'assertion failed: `(left == right)`

Diff < left / right > :
<1
>2
...
  */
}
```
<!-- zzz : qqq : add --> <!-- aaa : added -->

### To add to your project

```sh
cargo add diagnostics_tools
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd examples/diagnostics_tools_trivial
cargo run
```
