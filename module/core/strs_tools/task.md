# Change Proposal for strs_tools

### Task ID
*   TASK-20250708-202400-StrsToolsUnescape

### Requesting Context
*   **Requesting Crate/Project:** `unilang_instruction_parser`
*   **Driving Feature/Task:** Refactoring `unilang_instruction_parser` to be robust and spec-compliant, which requires correct tokenization and unescaping of quoted strings.
*   **Link to Requester's Plan:** `module/move/unilang_instruction_parser/task/task_plan.md`
*   **Date Proposed:** 2025-07-08

### Overall Goal of Proposed Change
*   To enhance `strs_tools::string::split` functionality to correctly parse and unescape quoted strings, treating them as single tokens and removing escape sequences, when `quoting(true)` is enabled.

### Problem Statement / Justification
The `unilang_instruction_parser` crate relies on `strs_tools` for tokenization, specifically for handling quoted strings. Current behavior of `strs_tools::split` with `quoting(true)` does not correctly:
1.  Treat an entire quoted string (e.g., `"value with spaces"`) as a single `Delimeted` token when internal delimiters (like spaces) are present. Instead, it splits the quoted string by internal delimiters.
2.  Perform unescaping of escape sequences (e.g., `\"`, `\\`) within quoted strings. The `string` field of the `Split` struct retains the raw, escaped content.

This prevents `unilang_instruction_parser` from correctly parsing instructions with quoted arguments, leading to parsing errors and requiring complex, redundant unescaping logic in the consumer crate.

**Minimal Reproducible Example (`strs_tools_mre.rs`):**
```rust
//! Minimal reproducible example for strs_tools unescaping bug.

use strs_tools::string::split::Split;

fn main()
{
  let input = r#"cmd key::"value with \"quotes\" and \\slash\\""#;
  let splits_iter = strs_tools::split()
      .src( input )
      .delimeter( vec![ " ", "::" ] )
      .preserving_delimeters( true )
      .quoting( true )
      .form()
      .split(); // Use the full iterator

  let splits: Vec< Split<'_> > = splits_iter.collect();
  println!( "{:#?}", splits );
}
```
**Current Output of MRE:**
```
[
    Split {
        string: "cmd",
        typ: Delimeted,
        start: 0,
        end: 3,
    },
    Split {
        string: " ",
        typ: Delimiter,
        start: 3,
        end: 4,
    },
    Split {
        string: "key",
        typ: Delimeted,
        start: 4,
        end: 7,
    },
    Split {
        string: "::",
        typ: Delimiter,
        start: 7,
        end: 9,
    },
    Split {
        string: "\"value with \\\"quotes\\\" and \\\\slash\\\"",
        typ: Delimeted,
        start: 9,
        end: 45,
    },
]
```
Expected output for the last `Split` item (after fix):
`Split { string: "value with \"quotes\" and \slash\", typ: Delimeted, start: 9, end: 45 }` (unescaped content)

### Proposed Solution / Specific Changes
Modify the `strs_tools::string::split::SplitIterator` to:
1.  Ensure that when `quoting(true)` is enabled, the iterator consumes the entire quoted segment (from opening to closing quote, respecting escape sequences) as a single `Split` item, regardless of internal delimiters.
2.  Perform unescaping of standard escape sequences (e.g., `\"`, `\\`, `\n`, `\t`, `\r`) within the quoted string content.
3.  **API Change Consideration:** Ideally, the `Split` struct's `string` field should be `Cow<'a, str>` to allow returning an owned `String` for unescaped content. If this is not feasible without a major version bump, a compromise might be to provide an `unescaped_string()` method on `Split` or a separate unescaping utility. However, the primary goal is for `Split.string` to contain the unescaped value directly when `quoting(true)` is used.

### Expected Behavior & Usage Examples (from Requester's Perspective)
Given the input: `cmd key::"value with \"quotes\" and \\slash\\"`
When `strs_tools::split().src(input).quoting(true).form().split()` is called:
The resulting `Split` for the quoted segment should be:
`Split { string: "value with \"quotes\" and \slash\", typ: Delimeted, start: 9, end: 45 }`
(Note: The `string` field here should contain the *unescaped* value, i.e., `value with "quotes" and \slash\`. The current MRE output shows it's still escaped.)

### Acceptance Criteria (for this proposed change)
1.  The `strs_tools_mre.rs` (provided in the `Problem Statement` section of this `task.md`) when run, produces a `Split` output for the quoted string where:
    *   The entire quoted string is a single `Split` item.
    *   The `string` field of this `Split` item contains the *unescaped* content (e.g., `value with "quotes" and \slash\`).
2.  No regressions are introduced to existing `strs_tools` functionality.

### Potential Impact & Considerations
*   **Breaking Changes:** Changing `Split.string` from `&'a str` to `Cow<'a, str>` would be a breaking change. If this is not desired, an alternative unescaping mechanism would be needed, but it would be less ergonomic.
*   **Performance:** Unescaping involves allocation for owned strings. This should be considered for performance-critical paths.
*   **Testing:** New unit and integration tests should be added to `strs_tools` to cover various quoting and unescaping scenarios.

### Alternatives Considered
*   Implementing unescaping logic directly in `unilang_instruction_parser`: Rejected, as it duplicates functionality that should ideally reside in the tokenization layer (`strs_tools`) and contradicts the architectural mandate to use `strs_tools` as the core tokenizer.

### Notes & Open Questions
*   Clarification on the intended behavior of `quoting(true)` regarding unescaping.
*   Guidance on whether a breaking change to `Split` (e.g., `Cow<'a, str>`) is acceptable for this functionality.