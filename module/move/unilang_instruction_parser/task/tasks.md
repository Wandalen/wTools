#### Tasks

| Task | Status | Priority | Responsible |
|---|---|---|---|
| [`clarify_parsing_spec_task.md`](./clarify_parsing_spec_task.md) | Not Started | High | @user |
| [`fix_command_parsing_revised_completed_20250707_202343.md`](./fix_command_parsing_revised_completed_20250707_202343.md) | Completed | High | @user |
| [`implement.md`](./implement.md) | Not Started | High | @user |
| [`task_plan.md`](./task_plan.md) | Paused | High | @user |

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
