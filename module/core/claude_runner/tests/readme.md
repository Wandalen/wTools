# tests/

| File | Responsibility |
|------|----------------|
| `commands_yaml_test.rs` | Verify YAML defines `.claude`, `.claude.help`, and rejects `.please`. |
| `cli_args_test.rs` | CLI flag parsing: correct translation to builder calls. |
| `dry_run_test.rs` | Dry-run output: env vars and command line structure. |
| `stale_ref_guard_test.rs` | Guard against stale claude_runner_plugin references. |
| `manual/readme.md` | Manual testing plan: live Claude Code invocation. |
