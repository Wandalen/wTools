# Module :: werror [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ToolsRustPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ToolsRustPush.yml) [![stable](https://img.shields.io/badge/stability-stable-brightgreen.svg)](https://github.com/emersion/stability-badges#stable)

Basic exceptions handling mechanism.

### Sample

```rust
use werror::*;

let err1 = Error::new( "Some error" );
println!( "err1 : {}", err1 );
// < err1 : Some error
```

### To add to your project

```sh
cargo add werror
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/werror_trivial
cargo run
```
