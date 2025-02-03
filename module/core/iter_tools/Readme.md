<!-- {{# generate.module_header{} #}} -->

# Module :: `iter_tools`
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_iter_tools_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_iter_tools_push.yml) [![docs.rs](https://img.shields.io/docsrs/iter_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/iter_tools) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Fiter_tools%2Fexamples%2Fiter_tools_trivial.rs,RUN_POSTFIX=--example%20iter_tools_trivial/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Collection of general purpose tools to iterate. Currently it simply reexports itertools.

### Basic use-case

<!-- {{# generate.module{} #}} -->

```rust
# #[ cfg( feature = "itertools" ) ]
# {
  use iter_tools::*;

  /* standard functions */
  let vec = vec![ 5, 1, -2 ];
  let min = min( &vec );
  assert_eq!( *min.unwrap(), -2 );

  /* non standard functions */
  let vec = vec![ 5, 1, -2 ];
  let added = vec![ "a", "b", "c" ];
  let mut result = vec![];
  let zipped = zip( &vec, &added );
  for ( left, right ) in zipped
  {
    result.push( ( *left, *right ) );
  }
  assert_eq!( result, vec![ ( 5, "a" ), ( 1, "b" ), ( -2, "c" ) ] );
# }
```

<!-- # qqq : for Rust dev : please add --> <!-- aaa : done -->

### To add to your project

```sh
cargo add iter_tools
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd examples/iter_tools_trivial
cargo run
```


### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd examples/iter_tools_trivial
cargo run
```
