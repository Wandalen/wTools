<!-- {{# generate.module_header{} #}} -->

# Module :: pth
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_pth_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_pth_push.yml) [![docs.rs](https://img.shields.io/docsrs/pth?color=e3e8f0&logo=docs.rs)](https://docs.rs/pth) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Collection of algorithms and structures to handle paths properly.

All functions in the crate don't touch file system, but only process paths.

### Type `AbsolutePath`

The AbsolutePath type ensures that paths are absolute, which helps reduce issues and maintenance costs associated with relative paths. Relative paths can be problematic as they introduce additional variables and complexities, making code analysis, integration, refactoring, and testing more difficult. By using absolute paths, software architecture can be improved, similar to how avoiding global variables can enhance code quality. It is recommended to use relative paths only at the outskirts of an application.

### Trait `AsPath`

This trait is used to avoid redundant allocation of memory by providing a reference to a Path. It is implemented only for types that can either be referenced or are references to Path itself. Unlike `TryIntoPath`, it does not allocate memory on the heap. However, `TryIntoPath` is implemented for a wider range of types because it is not restricted from allocating memory. Unlike `AsRef< Path >`, `AsPath` is implemented for a wider number of types, including those that are not directly convertible to a Path using `AsRef`. This is because `AsPath` is designed to provide a more flexible interface for path-like types, accommodating various representations that can logically be treated as paths.

### Trait `TryIntoPath`

This trait is used to convert any path-like type into an owned PathBuf. Unlike `TryIntoCowPath`, it always returns an owned PathBuf, so there is no need to differentiate between borrowed and owned paths at runtime. Unlike `AsPath`, it is implemented for a wider range of path-like types, similar to `TryIntoCowPath`.

### Trait `TryIntoCowPath`

This trait is designed to avoid redundant memory allocation. Unlike TryIntoPath, it does not allocate memory on the heap if it’s not necessary. Unlike `AsPath`, it is implemented for a wider number of path-like types, similar to TryIntoPath. The drawback is the necessity to differentiate borrowed and owned paths at runtime.

<!-- ### Basic use-case

```rust
use pth::*;

fn main()
{
}
```

### To add to your project

```bash
cargo add pth
```

### Try out from the repository

``` shell test
git clone https://github.com/Wandalen/wTools
cd wTools
cargo run --example pth_trivial
cargo run
``` -->
