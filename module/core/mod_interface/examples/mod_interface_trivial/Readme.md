### Example: Using Layers and Entities

[![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
[![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=sample%2Frust%2Fmod_interface_trivial,SAMPLE_FILE=.%2Fsrc%2Fmain.rs/https://github.com/Wandalen/wTools)
[![docs.rs](https://raster.shields.io/static/v1?label=docs&message=online&color=eee&logo=docsdotrs&logoColor=eee)](https://docs.rs/mod_interface)

In this example, we demonstrate the basic use case of one layer utilizing another layer. For a module to be used as a layer, it must contain all the necessary chapters: `orphan`, `exposed`, and `prelude`. Generally, a layer should also have the `own` and `private` chapters, but these are typically not modified directly by the user unless explicitly defined, with the `private` chapter remaining inaccessible from outside the module.

Below is a simple example where a parent layer imports a `child` layer. The `child` layer defines several functions, each with a different propagation strategy, resulting in each function being placed in a different chapter of the parent layer, while some functions do not reach the parent layer at all.
