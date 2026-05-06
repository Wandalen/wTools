
# Module :: program_tools
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://img.shields.io/github/actions/workflow/status/Wandalen/wTools/workspace_push.yml?label=&branch=master&job=program_tools)](https://github.com/Wandalen/wTools/actions/workflows/workspace_push.yml) [![docs.rs](https://img.shields.io/docsrs/program_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/program_tools) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fexperimental%2Fprogram_tools%2Fexamples%2Fbasic.rs,RUN_POSTFIX=--example%20basic/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Rust script runner — compile and execute Rust files as scripts with output capture.

### To add to your project

```bash
cargo add program_tools
```

### Quick Start — programmatic (test use case)

Run a Rust file from a test with one expression and assert on its output:

```rust
use program_tools::*;

let output = run_file( "tests/asset/hello/src/main.rs" ).expect( "run failed" );
output.assert_stdout_eq( "hello\n" );
```

Run inline source code without any files on disk:

```rust
use program_tools::*;

let output = run_source( r#"fn main() { println!( "hello" ); }"# ).expect( "run failed" );
output.assert_stdout_contains( "hello" );
```

Build the execution plan manually for full control:

```rust
use program_tools::*;

let plan = Plan::former()
  .program()
    .source()
      .file_path( "src/main.rs" )
      .data( "fn main() { println!( \"hello\" ); }" )
      .end()
    .end()
  .form();

let output = run( plan ).expect( "run failed" );
output.assert_exit_ok();
output.assert_stdout_eq( "hello\n" );
```

### Quick Start — CLI

```bash
# Run a Rust file as a script
program_tools run main.rs

# Run with release profile and a custom package name
program_tools run --profile release --name my_script main.rs
```

### Features

- **Script execution** — run any Rust file or Cargo project as a script; all Cargo complexity is hidden behind a single call
- **Output capture** — stdout and stderr are captured into separate buffers; compare with expected strings using assertion methods
- **Artifact management** — auto-generated Cargo.toml, isolated temp workspaces, optional persistent build cache across runs
- **Test integration** — single-expression invocations and panic-on-failure assertion methods designed for Rust test functions
- **Configuration** — all parameters (build profile, timeout, features, env vars, edition) accessible via both builder API and CLI flags
