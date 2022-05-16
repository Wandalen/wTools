# Module :: werror [![experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModulewErrorPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModulewErrorPush.yml) [![docs.rs](https://img.shields.io/docsrs/werror?color=e3e8f0&logo=docs.rs)](https://docs.rs/werror) [![discord](https://img.shields.io/discord/872391416519737405?color=e3e8f0&logo=discord&logoColor=e3e8f0)](https://discord.gg/JwTG6d2b)

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
