# Sample

[![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
[![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=sample%2Frust%2Fmod_interface_with_debug,SAMPLE_FILE=.%2Fsrc%2Fmain.rs/https://github.com/Wandalen/wTools)
[![docs.rs](https://raster.shields.io/static/v1?label=docs&message=online&color=eee&logo=docsdotrs&logoColor=eee)](https://docs.rs/mod_interface)

A sample demonstrates basic usage of macro `mod_interface`.

In file `inner.rs` demonstrated how to generate module interface from namespace `private` and its public routine.

In file `main.rs` demonstrated how to generate module interface from layer ( file with full module interface ).

The directive `#![ debug ]` in declaration of macro `mod_interface` allow to show generated module interface as the standard output in compile time.
