# Rust CD system

The CD system for rust modules consists of 3 parts :

- common rust CD scripts for fast and full testing
- individual modules CD scripts
- CD script for pull request event

## Individual modules statuses

The status badges on [main page](../Readme.md#rust-tools) show the last test run of individual module CD script.

To run CD script one of the condition should be met:

#### Keyword `Merge` at the start of the commit message

Example : `Merge branch 'fix' into master`

Example : [CD script run](https://github.com/Wandalen/wTools/actions/runs/2343552303)

#### Keyword `[build]` at the start of the commit message

Example : `[build] rust modules`

#### Keyword with module name in commit message

Example for module `wtools` : `build module 'wtools'`.

Example for module `wtools` : `rust : wtools publish`.
