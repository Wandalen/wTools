# Switch cli_fmt's strs_tools dependency to workspace = true

## Execution State

- **Executor Type:** any
- **filed_by:** ai
- **actor:** null
- **started_at:** null
- **expires_at:** null
- **round:** 1
- **state:** 📝 (Draft)
- **closes:** null
- **unit_type:** module
- **unit:** lib/yrd_core/wtools/dev/module/core/cli_fmt
- **validated_by:** null
- **validation_date:** null
- **blocked_by:** null

## Goal

`Cargo.toml:14` declares `strs_tools = { version = "~0.49.1", features = [ "ansi", "string_split", "std" ], optional = true }` with an explicit version pin rather than `workspace = true`. Confirm workspace-level dependency centralization is intended for this crate (check the workspace root `Cargo.toml`'s `[workspace.dependencies]` for an existing `strs_tools` entry) and switch, consistent with dep_l1 workspace dependency hygiene conventions.

## History

- **[2026-07-15]** `CREATED` — Draft filed from a finding carried from an earlier session; confirmed still current this session via direct read of `Cargo.toml:14`.
