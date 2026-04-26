# file_tools Manual Testing Plan

Manual testing for the `file_tools` alias crate (re-exports `fs_tools::*`).

### Scope

Verify all `fs_tools` exports are accessible through `file_tools`. Exhaustive
file-manipulation corner-case testing lives in `fs_tools/tests/manual/readme.md`.

### Checklist

- [ ] Compiles `--features full` — zero errors, zero warnings
- [ ] Compiles `--no-default-features` — zero errors, zero warnings
- [ ] `use file_tools::*` exposes equivalent public API as `use fs_tools::*`
- [ ] `w3 .test level::1` passes clean
