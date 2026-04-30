# Exclude task/ development artifacts from published crate

## Description

The published `error_tools` crate on crates.io includes the `task/` directory — internal task
management files used during development (e.g. `task/completed/009_fix_clippy_const_is_empty_warnings.md`,
`task/readme.md`, `task/tasks.md`, `task/no_std_refactoring_task.md`, etc.). These files are
invisible in normal Rust usage but inflate the published crate size and expose internal project
management details to consumers.

Add an `exclude` list to `Cargo.toml` to prevent development-only paths from being bundled in
the `.crate` archive.

## Requirements

- Add `exclude` field to `[package]` in `Cargo.toml` covering at minimum:
  ```toml
  exclude = [
    "task/**",
  ]
  ```
- Verify `docs/` should remain included (intentional — published for docs.rs)
- After adding exclude, run `cargo package --list` and confirm no `task/` files appear
- Do not exclude `docs/`, `examples/`, `tests/`, `src/`, `license`, `readme.md`

## Acceptance Criteria

- `Cargo.toml` contains an `exclude` list with `"task/**"` (and any other dev-only paths)
- `cargo package --list` output contains no files matching `task/`
- `cargo package --list` output still contains `docs/`, `src/`, `examples/`, `tests/`
- `cargo nextest run --all-features` exits 0

## Outcomes

