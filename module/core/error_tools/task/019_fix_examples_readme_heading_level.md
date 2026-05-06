# Fix examples/readme.md heading level regression

## Description

`examples/readme.md` has a heading-level regression introduced in a recent edit:
the `Responsibility Table` heading was changed from `##` (h2) to `###` (h3), skipping
a heading level under the `# Examples Directory` h1 header.

Valid markdown heading hierarchy requires no level skips: `# → ## → ###`. Jumping
from `#` directly to `###` is a structural error that breaks document outline tools
and accessibility tree navigation.

The fix is a one-line change.

## Requirements

- Change `### Responsibility Table` back to `## Responsibility Table` in `examples/readme.md`
- Verify no other heading-level skips exist in the same file

## Acceptance Criteria

- `examples/readme.md` contains `## Responsibility Table` (not `###`)
- All other headings in the file form a valid sequence (no level skips)

## Outcomes

