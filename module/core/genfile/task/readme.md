# Tasks

### Scope

Active implementation tasks for `genfile`.

## Tasks Index

| Order | ID | Advisability | Value | Easiness | Safety | Priority | State | Executor | UnitType | Unit | Task | Purpose |
|-------|----|--------------|-------|----------|--------|----------|-------|----------|----------|------|------|---------|
| 1 | [001](completed/001_fill_test_surface_gaps.md) | 0 | — | — | — | 0 | ✅ (Completed) | any | module | lib/yrd_core/wtools/dev/module/core/genfile | [Implement missing tests to back all test surface spec cases in genfile](completed/001_fill_test_surface_gaps.md) | Implement test functions covering all deferred spec cases in tests/docs/ |
| 2 | [002](completed/002_expand_cli_test_surface.md) | 1 | — | — | — | 1 | ✅ (Completed) | any | module | lib/yrd_core/wtools/dev/module/core/genfile | [Expand CLI test surface spec cases to rulebook minimums](completed/002_expand_cli_test_surface.md) | Add 77 param + 12 command spec cases; add BD cases to 6 params; fix Minimum cases headers |
| 3 | [003](003_implement_fr9_help_system.md) | 2 | — | — | — | 2 | 🎯 (Verified) | any | module | lib/yrd_core/wtools/dev/module/core/genfile | [Implement FR9 help system commands](003_implement_fr9_help_system.md) | Patch .help output to list real commands; register . alias; implement 4 test functions covering FT-01..FT-04 |
| 4 | [004](004_implement_deferred_source_behaviors.md) | 3 | — | — | — | 3 | 🎯 (Verified) | any | module | lib/yrd_core/wtools/dev/module/core/genfile | [Implement deferred source behaviors to resolve open feature spec cases](004_implement_deferred_source_behaviors.md) | Add duplicate-param guard, undefined-param guard, pack internalize-refs; flip FT-05/003, FT-04/004, FT-01/007 to ✅ |
