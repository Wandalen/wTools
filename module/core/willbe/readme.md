<!-- {{# generate.module_header{} #}} -->

# `Module`:: willbe
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_willbe_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_willbe_push.yml) [![docs.rs](https://img.shields.io/docsrs/willbe?color=e3e8f0&logo=docs.rs)](https://docs.rs/willbe) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Utility to publish multi-crate and multi-workspace environments and maintain their consistency.

### Purpose

1. **Multi-Workspace Consistency**: In a project setup involving multiple workspaces with shared dependencies, `willbe` maintains consistency. It ensures all workspaces are updated with compatible dependency versions.

2. **Publishing Multi-Crate Projects**: If your project structure includes multiple crates that need simultaneous publishing, `willbe` manages the process seamlessly. It ensures the crates are published in the right order while staying in version sync.

3. **Dependency Analysis**: `willbe` can perform a thorough analysis of your project's dependencies, providing a detailed highlight of inconsistencies or areas that need attention.

4. **Test Execution With Varied Configurations**: `willbe` can execute tests with varying configurations, such as different combinations of crate features. This assists in ensuring comprehensive coverage of test scenarios.

5. **Generate Health Table**: Quickly visualize important project metrics like build status for each crate, creating a single, easy-to-read table.

6. **Automating CI/CD Workflow Generation**: Automatically generate a series of CI/CD operations suitable for the task at hand to enhance productivity and improve the development process.

### To install

```bash
cargo install willbe
will .
```

### Try out from the repository

``` shell test
git clone https://github.com/Wandalen/wTools
cd wTools/module/move/willbe
cargo install --path .
will .
```
