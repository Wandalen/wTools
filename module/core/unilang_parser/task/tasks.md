#### Tasks

| Task | Status | Priority | Responsible |
|---|---|---|---|
| [`001_zero_copy_tokens.md`](./001_zero_copy_tokens.md) | Not Started | High | @user |
| [`implement_parser_rules_task.md`](./implement_parser_rules_task.md) | Not Started | Medium | @user |

---

### Issues Index

| ID | Name | Status | Priority |
|---|---|---|---|
| [ISSUE-STRS-001](#issue-strs-001--strs_tools-unescaping-bug) | `strs_tools` Unescaping Bug | Open | High |

---

### Issues

###### ISSUE-STRS-001 : `strs_tools` Unescaping Bug

*   **Issue Description:** The `strs_tools::string::split` function, when `quoting(true)` is enabled, does not correctly unescape quoted strings containing escaped quotes (`\"`) or escaped backslashes (`\\`). The `SplitFastIterator`'s logic for finding the end of a quoted segment is flawed, leading to incorrect input for the `unescape_str` function.
*   **Location:** `module/core/strs_tools/src/string/split.rs`
*   **Issue Rationale:** This bug prevents `unilang_instruction_parser` from correctly parsing command arguments that contain escaped characters within quoted strings, leading to functional errors. A fix is required in `strs_tools` to unblock `unilang_instruction_parser` development.
*   **Related Proposal:** `module/core/strs_tools/task.md`
