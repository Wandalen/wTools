# Invariant: Heading Rendering

### Scope

- **Purpose**: Define the behavioral guarantees that caption rendering must maintain in all configurations and table styles.
- **Responsibility**: Documents three invariants: no-caption passthrough, width ceiling, and single-line output.
- **In Scope**: Absent caption behavior, table width ceiling, output line count guarantee.
- **Out of Scope**: Caption content format (see `feature/007_table_caption.md`), rendering algorithm (see `algorithm/007_caption_rendering.md`).

### Features

| File | Relationship |
|------|-------------|
| [007_table_caption.md](../feature/007_table_caption.md) | Feature whose invariants are defined here |

### Algorithms

| File | Relationship |
|------|-------------|
| [007_caption_rendering.md](../algorithm/007_caption_rendering.md) | Algorithm whose outputs these invariants constrain |

### Sources

| File | Relationship |
|------|-------------|
| [`src/formatters/table/row_rendering.rs`](../../src/formatters/table/row_rendering.rs) | `render_caption_if_present()` — caption rendering path |
| [`src/formatters/table/mod.rs`](../../src/formatters/table/mod.rs) | Call site in `format_internal()` — passes `primary_widths` to caption renderer |
| [`src/config/table_caption.rs`](../../src/config/table_caption.rs) | `Heading` optional field on `TableConfig` |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/table_caption_test.rs`](../../tests/table_caption_test.rs) | Caption invariant tests |

### Invariant Statement

#### Invariant 1 — No-Caption Passthrough

A table with no caption attached produces output byte-identical to the output produced by the same table before the caption feature existed. The caption path is a strict additive extension — no code runs on the caption path when no caption is set.

#### Invariant 2 — Width Ceiling

The caption line never exceeds the rendered table width. When caption content (lead prefix + content + trailing space) equals or exceeds the rendered table width, the trailing rule width is clamped to zero. The caption line may be shorter than the rendered table width but never longer. When content alone exceeds the rendered table width, the caption line may exceed it — the invariant guarantee is that the trailing rule is clamped to zero and content is never truncated.

#### Invariant 3 — Single Output Line

The caption always occupies exactly one output line ending with a newline character. No combination of title length, caption field count, or terminal width can cause the caption to span multiple lines.

### Enforcement Mechanism

Invariant 1 is enforced by an early-exit guard in `render_caption_if_present()` that skips the caption path entirely when the caption field on `TableConfig` is absent (`None`). Invariant 2 is enforced by the `saturating_sub` clamp applied to the trailing rule width computation using the rendered table width (`compute_total_row_width(primary_widths)`), not the terminal width. Invariant 3 is enforced by the single-string assembly in the rendering algorithm — exactly one newline character is appended at the end, and no newlines appear inside the content or lead prefix.

### Violation Consequences

| Invariant | Consequence of Violation |
|-----------|------------------------|
| No-caption passthrough | Silent regression for all existing callers — every table output changes without any user action |
| Width ceiling | Caption line overflows the rendered table width; output wraps or truncates unexpectedly in terminal display |
| Single output line | Table layout corrupts; the header row and subsequent rows shift down by an unexpected number of lines |
