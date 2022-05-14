# Module :: winterval [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ToolsRustPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ToolsRustPush.yml) [![stable](https://img.shields.io/badge/stability-stable-brightgreen.svg)](https://github.com/emersion/stability-badges#stable)

Interval adapter for both open/closed implementations of intervals ( ranges ).

### Sample

```rust
use winterval::*;

let src = 2..5;
assert_eq!( src.closed(), ( 2, 4 ) );

let src = 2..=4;
assert_eq!( src.closed(), ( 2, 4 ) );
```

### To add to your project

```sh
cargo add winterval
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/winterval_trivial
cargo run
```
