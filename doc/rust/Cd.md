# Continious development of rust modules

The CD system for rust modules consists of 3 parts :

- common rust CD scripts for fast and full testing
- individual modules CD scripts
- CD script for pull request event

## Module status

The status badges on [main page](../Readme.md#rust-tools) show the last test run of individual module CD script.

To run CD script name should have either:
- Keyword `Merge`
- Keyword `!build`
- Keyword with name of the module

#### Keyword `Merge`

Example : `Merge branch 'fix' into master`.

Example : [CD script run](https://github.com/Wandalen/wTools/actions/runs/2343552303).

#### Keyword `!build`

Example : `!build rust modules`.

Example : `some change in module !build`.

#### Keyword with name of the module

Example for module wtools : `some important changes in module 'wtools'`.

Example for module wtools : `rust : wtools publish`.
