# Remove Rust-specific syntax/terminology from prose in docs/api/002_help_api.md and docs/feature/002_cli_help_template.md

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

6 confirmed instances of Rust-specific syntax/terminology in prose violate `doc_des.rulebook.md`'s Documentation Anti-Patterns abstraction-first principle:
- `docs/api/002_help_api.md:94` — "Struct literal expressions... at compile time"
- `docs/api/002_help_api.md:111` — "No panics occur"
- `docs/api/002_help_api.md:115` — "struct fields"
- `docs/api/002_help_api.md:117` — "struct literals... fail to compile... struct update syntax" (3 instances on one line)
- `docs/feature/002_cli_help_template.md:32` — "construct it with a struct literal... compile-time enforcement"

Rewrite these language-agnostically (e.g., "direct construction is restricted; callers must use the default constructor with field assignment" instead of naming Rust's struct-literal/compile-time mechanics).

**Note:** `docs/feature/001_output_processing.md` was also audited (it was named alongside the other two in the originating finding) and found clean — zero violations. No changes needed there; do not expand scope to that file without a fresh finding.

## History

- **[2026-07-15]** `CREATED` — Draft filed from a documentation-audit finding. Independently re-verified this session against `doc_des.rulebook.md § Collection : Documentation Anti-Patterns` (rule quoted with citation) and all 3 originally-named files read in full — 2 of 3 confirmed with violations, 1 of 3 (`001_output_processing.md`) found clean, narrowing scope from the original 3-file claim.
