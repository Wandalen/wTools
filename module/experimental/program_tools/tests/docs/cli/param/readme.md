# Parameter Specs

### Scope

- **Purpose**: Catalog test specifications for each CLI parameter and flag.
- **Responsibility**: One spec file per parameter; EC- prefix; min 6 cases per spec including behavioral divergence.
- **In Scope**: All 10 named flags and the `<TARGET>` positional.
- **Out of Scope**: Command-level integration (→ `command/`); invariant verification (→ `invariant/`).

### Overview Table

| Name | Purpose | Status |
|------|---------|--------|
| `target.md` | `parameter` spec for `<TARGET>` positional | ⏳ |
| `profile.md` | `parameter` spec for `--profile` | ⏳ |
| `target_dir.md` | `parameter` spec for `--target-dir` | ⏳ |
| `cargo.md` | `parameter` spec for `--cargo` | ⏳ |
| `timeout.md` | `parameter` spec for `--timeout` | ⏳ |
| `feature.md` | `parameter` spec for `--feature` | ⏳ |
| `env.md` | `parameter` spec for `--env` | ⏳ |
| `edition.md` | `parameter` spec for `--edition` | ⏳ |
| `name.md` | `parameter` spec for `--name` | ⏳ |
| `capture.md` | `parameter` spec for `--capture` | ⏳ |
| `keep.md` | `parameter` spec for `--keep` | ⏳ |
