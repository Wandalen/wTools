# Rust CD system

The CD system for rust modules consists of 3 parts :

- common rust workflows for fast and full testing
- individual modules workflows
- workflow for pull request event

## Individual modules statuses

The status badges on [main page](../Readme.md#rust-tools) show the last test run of individual module workflow.

One of the next condition will run an individual module workflow :

- keyword `Merge` at the start of the commit message
  - example of commit message : `Merge branch 'fix' into master`
  - [workflow run](https://github.com/Wandalen/wTools/actions/runs/2343552303)
- keyword `[build]` at the start of the commit message
  - example of commit message : `[build] rust modules`
- keyword with module name in commit message
  - example of commit message that should run workflow of module `wtools` : `build module 'wtools'`
  - example of commit message that should run workflow of module `wtools` : `rust : wtools publish`
