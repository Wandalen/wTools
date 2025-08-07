# Sample: Debugging `mod_interface`

[![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
[![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=sample%2Frust%2Fmod_interface_with_debug,SAMPLE_FILE=.%2Fsrc%2Fmain.rs/https://github.com/Wandalen/wTools)
[![docs.rs](https://raster.shields.io/static/v1?label=docs&message=online&color=eee&logo=docsdotrs&logoColor=eee)](https://docs.rs/mod_interface)

This sample demonstrates basic usage of the `mod_interface!` macro and its debugging capabilities.

- In `child.rs`, the macro defines a simple module interface, exporting `inner_is` into the `prelude` exposure level from the `private` namespace.
- In `main.rs`, the macro uses the `layer` keyword to integrate the `child` module as a layer.

The directive `#![ debug ]` within the `mod_interface!` macro invocation causes the macro to print the generated code (including the module structure with its exposure levels like `own`, `orphan`, `exposed`, `prelude`) to standard output during compilation. This is useful for understanding how the macro expands and verifying the resulting module structure. Uncomment the `//#![ debug ]` line in `main.rs` to see this output.