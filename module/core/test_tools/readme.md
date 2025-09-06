<!-- {{# generate.module_header{} #}} -->

# Module :: `test_tools`
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_test_tools_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_test_tools_push.yml) [![docs.rs](https://img.shields.io/docsrs/test_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/test_tools) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Ftest_tools%2Fexamples%2Ftest_tools_trivial.rs,RUN_POSTFIX=--example%20module%2Fcore%2Ftest_tools%2Fexamples%2Ftest_tools_trivial.rs/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Tools for writing and running tests.

## Architecture Overview

This crate serves as an **aggregation layer** that unifies testing tools from multiple ecosystem crates:
- `error_tools` - Error handling and assertions
- `collection_tools` - Collection constructor macros and utilities  
- `impls_index` - Implementation and test organization macros
- `mem_tools`, `typing_tools`, `diagnostics_tools` - Specialized testing utilities

### Key Design Patterns

**Namespace Re-exports:** The crate provides unified access through `own::*`, `orphan::*`, `exposed::*`, and `prelude::*` modules that re-export functionality from dependency crates.

**Macro Re-exports:** Collection constructor macros (`heap!`, `vec!`, etc.) require explicit re-export since `#[macro_export]` macros are not propagated through module re-exports.

**Feature Cascading:** Features are propagated to dependencies through Cargo.toml, with some requiring explicit handling in source code.

### Test Aggregation Strategy

Tests from dependency crates are included via path references to ensure re-export consistency. This requires the complete public API to remain visible during test compilation.

**⚠️ IMPORTANT:** Never hide public API modules with feature gates during test compilation. See troubleshooting documentation in the source code for details.

## Cross-Crate Testing

### Comprehensive Test Script

Use `test.sh` to run tests across all aggregated crates since changes in one can affect others:

```bash
# From module/core directory:
./test.sh       # Run all tests for test_tools + aggregated subcrates  
./test.sh quick # Quick compilation check only
```

**What it tests:**
- `error_tools` - Error handling and debug assertions (18 tests + 13 doc tests)
- `collection_tools` - Collection types and constructor macros (35 tests + 60 doc tests)
- `mem_tools` - Memory comparison utilities (7 tests + 0 doc tests) 
- `diagnostics_tools` - Runtime/compile-time assertions (4 tests + 8 doc tests)
- `impls_index` - Implementation indexing and test organization (30 tests + 0 doc tests)
- `test_tools` - Aggregated test suite (192 tests + 5 doc tests)

**Total Coverage:** 372 comprehensive tests across all 6 crates (286 unit/integration + 86 documentation tests)

✅ **Status:** All 6 crates pass comprehensive testing with zero warnings (nextest + doc tests + clippy analysis)

**Why cross-crate testing is needed:**
- Changes in `test_tools/src/standalone.rs` can break individual crate tests
- Changes in individual crates can break `test_tools` aggregation
- Macro changes affect all subcrates using `tests_impls!` and `tests_index!`
- Module structure changes can break `the_module` alias resolution

**Architecture:** Individual crates use `the_module` alias pattern that switches between `crate_name` (individual testing) and `test_tools` (aggregated testing), enabling the same test source code to work in both contexts.

📖 **For comprehensive documentation:** See [`CROSS_CRATE_TESTING.md`](../CROSS_CRATE_TESTING.md) for detailed architecture, troubleshooting, and implementation guidance.

## Troubleshooting

For test compilation issues, see the comprehensive troubleshooting documentation embedded in the source code:
- **Main troubleshooting guide:** See doc comments at the top of `src/lib.rs` 
- **Test-specific guidance:** See doc comments in `tests/tests.rs` and `tests/inc/mod.rs`
- **Inline warnings:** Critical sections have detailed prevention and resolution guidance
- **Historical context:** Each warning references the specific task that resolved the issue

### Basic use-case

<!-- {{# generate.module{} #}} -->

```rust
use test_tools::*;

#[ cfg( feature = "enabled" ) ]
#[ cfg( not( feature = "no_std" ) ) ]
tests_impls!
{
  fn pass1()
  {
    assert_eq!( true, true );
  }

  //

  fn pass2()
  {
    assert_eq!( 1, 1 );
  }
}

//
#[ cfg( feature = "enabled" ) ]
#[ cfg( not( feature = "no_std" ) ) ]
tests_index!
{
  pass1,
  pass2,
}
```

### To add to your project

```sh
cargo add test_tools --dev
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd examples/test_trivial
cargo run
```

# Sample

[![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
[![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=sample%2Frust%2Ftest_tools_trivial,SAMPLE_FILE=.%2Fsrc%2Fmain.rs/https://github.com/Wandalen/wTools)
[![docs.rs](https://raster.shields.io/static/v1?label=docs&message=online&color=eee&logo=docsdotrs&logoColor=eee)](https://docs.rs/test_tools)
