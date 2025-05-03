# Project Plan: Refactor `component_model_meta`

## Progress

*   [✅] Increment 1: Review `src/lib.rs` for `// former_...` comments or unused code blocks - remove if clearly garbage.
*   [✅] Increment 2: Review `src/component/*.rs` for `// former_...` comments or unused code blocks - remove if clearly garbage.
*   [✅] Increment 3: Review `plan.md` - assess if the file splitting plan is still relevant or completed. Update/remove as necessary.
*   [✅] Increment 4: Rename `src/derive_former.rs` to `src/derive_component_model.rs`.
*   [❌] Increment 5: Rename `src/derive_former/` directory to `src/component/`.
*   [ ] Increment 6: Rename `src/derive_component_model/former_struct.rs` to `src/derive_component_model/component_model_struct.rs`.
*   [ ] Increment 7: Rename `src/derive_component_model/former_enum.rs` to `src/derive_component_model/component_model_enum.rs`.
*   [ ] Increment 8: Rename `src/derive_component_model/former_enum/` directory to `src/derive_component_model/component_model_enum/`.
*   [ ] Increment 9: Update `mod` declarations in `src/lib.rs` and `src/derive_component_model.rs` to reflect renames.

## Notes & Insights

*   Performing basic renaming.
*   Could not rename `derive_component_model` directory because it is locked.