
# Module :: `process_tools`
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://img.shields.io/github/actions/workflow/status/Wandalen/wTools/workspace_push.yml?label=&branch=master&job=process_tools)](https://github.com/Wandalen/wTools/actions/workflows/workspace_push.yml) [![docs.rs](https://img.shields.io/docsrs/process_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/process_tools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Ergonomic subprocess execution with output capture, environment variable management, and CI/CD environment detection. Provides a builder-pattern `Run` type for configuring and launching child processes with full stdout/stderr capture across platforms. Includes `exit_status` synthesis for platform-agnostic `ExitStatus` construction and a `lifecycle` module for signal mapping, process-alive checking, and Unix daemonization.

### To add to your project

```bash
cargo add process_tools
```
