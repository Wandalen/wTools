# fs_tools Manual Testing Plan

Manual testing for the `fs_tools` alias crate (re-exports `file_tools::*`).

### Scope

Verify all `file_tools` exports are accessible through `fs_tools`. Exhaustive
file-manipulation corner-case testing lives in `file_tools/tests/manual/readme.md`.

### Checklist

- [ ] Compiles `--features full` — zero errors, zero warnings
- [ ] Compiles `--no-default-features` — zero errors, zero warnings
- [ ] `use fs_tools::*` exposes equivalent public API as `use file_tools::*`
- [ ] `w3 .test level::1` passes clean
