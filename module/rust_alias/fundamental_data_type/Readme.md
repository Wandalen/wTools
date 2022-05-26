# Module :: fundamental_data_type
[![experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleFundamentalDataTypePush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleFundamentalDataTypePush.yml) [![docs.rs](https://img.shields.io/docsrs/fundamental_data_type?color=e3e8f0&logo=docs.rs)](https://docs.rs/fundamental_data_type) [![discord](https://img.shields.io/discord/872391416519737405?color=e3e8f0&logo=discord&logoColor=e3e8f0)](https://discord.gg/JwTG6d2b)

Fundamental data types and type constructors, like Single, Pair, Homopair, Many.

In Rust, you often need to wrap a given type into a new one.
The role of the orphan rules in particular is basically to prevent you from implementing external traits for external types.
To overcome the restriction developer usually wrap the external type into a tuple introducing a new type.
Type constructor does exactly that and auto-implement traits From, Into, Deref and few more for the constructed type.

Besides type constructor for single element there are type constructors for `pair`, `homopair` and `many`:

- `Single` to wrap single element.
- `Pair` to wrap pair of distinct elements.
- `HomoPair` to wrap pair of elements with the same type.
- `Many` to wrap `Vec` of elements.

<!-- qqq : for Dima : bad -->

### To add to your project

``` shell
cargo add type_constructor
```

## Try out from the repository

``` shell test
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/type_constructor_trivial_sample
cargo run
```
