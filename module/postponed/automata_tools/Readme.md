<!-- {{# generate.module_header{} #}} -->

# Module :: automata_tools
<!--{ generate.module_header.start() }-->
[![experimental](https://raster.shields.io/static/v1?label=stability&message=experimental&color=orange&logoColor=eee)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleAutomataToolsPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleAutomataToolsPush.yml) [![docs.rs](https://img.shields.io/docsrs/automata_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/automata_tools) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Fautomata_tools_trivial%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20automata_tools_trivial/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Automata tools.

### Basic use-case

<!-- {{# generate.module{} #}} -->

```rust ignore
use automata_tools::prelude::*;
use wtools::prelude::*;
let node : automata_tools::canonical::Node<i32, i32> = automata_tools::canonical::Node::_make_with_id( 13 );
assert_eq!( node.id(), 13.into() );
println!( "{:?}", node );
/* print : node::13 */
```

### To add to your project

```bash
cargo add automata_tools
```

### Try out from the repository

``` shell test
git clone https://github.com/Wandalen/wTools
cd wTools
cd examples/automata_tools_trivial
cargo run
```
