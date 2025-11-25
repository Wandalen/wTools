# Change Proposal for `willbe`

### Task ID
*   TASK-20250701-110200-RemovePthStdFeatureDependency

### Requesting Context
*   **Requesting Crate/Project:** `module/core/derive_tools`
*   **Driving Feature/Task:** Fixing compilation errors in `derive_tools` due to dependency conflicts.
*   **Link to Requester's Plan:** `module/core/derive_tools/task.md`
*   **Date Proposed:** 2025-07-01

### Overall Goal of Proposed Change
*   Modify `willbe`'s `Cargo.toml` to remove the explicit dependency on the `std` feature of the `pth` crate. This is necessary because `pth` is intended to be compiled without `std` features at this stage, and `willbe`'s current dependency is causing compilation failures across the workspace.

### Problem Statement / Justification
*   The `pth` crate is currently configured to "ignore no_std" support, meaning it does not expose a `std` feature. However, `willbe`'s `Cargo.toml` explicitly depends on `pth` with the `std` feature enabled (`pth = { workspace = true, features = [ "default", "path_utf8", "std" ] }`). This creates a compilation error: "package `willbe` depends on `pth` with feature `std` but `pth` does not have that feature." This error prevents the entire workspace from compiling, including the `derive_tools` crate which is the primary focus of the current task.

### Proposed Solution / Specific Changes
*   **File to modify:** `module/move/willbe/Cargo.toml`
*   **Section to modify:** `[dependencies]`
*   **Specific change:** Remove `", "std"` from the `pth` dependency line.

```diff
--- a/module/move/willbe/Cargo.toml
+++ b/module/move/willbe/Cargo.toml
@@ -91,7 +91,7 @@
  component_model = { workspace = true, features = [ "default" ] }
  iter_tools = { workspace = true, features = [ "default" ] }
  mod_interface = { workspace = true, features = [ "default" ] }
  wca = { workspace = true, features = [ "default" ] }
- pth = { workspace = true, features = [ "default", "path_utf8", "std" ] }
+ pth = { workspace = true, features = [ "default", "path_utf8" ] }
  process_tools = { workspace = true, features = [ "default" ] }
  derive_tools = { workspace = true, features = [ "derive_display", "derive_from_str", "derive_deref", "derive_from", "derive_as_ref" ] }
  data_type = { workspace = true, features = [ "either" ] }
```

### Expected Behavior & Usage Examples (from Requester's Perspective)
*   After this change, `willbe` should no longer attempt to enable the `std` feature for `pth`. This should resolve the compilation error and allow the workspace (and thus `derive_tools`) to compile successfully.

### Acceptance Criteria (for this proposed change)
*   `willbe` compiles successfully without errors related to `pth`'s `std` feature.
*   The entire workspace compiles successfully.

### Potential Impact & Considerations
*   **Breaking Changes:** No breaking changes are anticipated for `willbe`'s functionality, as `pth`'s `std` feature was causing a compilation error, implying it was not being used correctly or was not essential for `willbe`'s operation.
*   **Dependencies:** This change affects `willbe`'s dependency on `pth`.
*   **Performance:** No performance impact is expected.
*   **Security:** No security implications.
*   **Testing:** Existing tests for `willbe` should continue to pass.

### Alternatives Considered (Optional)
*   Re-introducing the `std` feature in `pth`: This was considered but rejected as it contradicts the user's instruction to "ignore no_std" for `pth` at this stage.

### Notes & Open Questions
*   This change is a prerequisite for continuing the `derive_tools` task.