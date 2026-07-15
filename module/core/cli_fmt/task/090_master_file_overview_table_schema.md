# Normalize master file Overview Table column schema to ID/Name/Purpose/Status

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

`docs/feature/readme.md`, `docs/api/readme.md`, `docs/invariant/readme.md`, and `docs/pitfall/readme.md` all use the Overview Table column schema `# | File | Name | Status` instead of the canonical `ID | Name | Purpose | Status` defined in `doc_des.rulebook.md § Collection : Master File Documentation` (column minimum: ID, Name, Status required; Purpose expected or a domain-specific substitute). All four files deviate identically. Normalize all four to the canonical schema.

## History

- **[2026-07-15]** `CREATED` — Draft filed from a documentation-audit finding, independently confirmed this session by reading all 4 files' actual header rows and comparing against the rulebook's canonical schema (quoted with citation) — not taken on the audit's word alone.
