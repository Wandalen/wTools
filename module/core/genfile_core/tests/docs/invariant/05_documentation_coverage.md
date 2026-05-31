# Test Spec: Documentation Coverage

- **Source**: `docs/invariant/005_documentation_coverage.md`
- **Prefix**: `IN-05`
- **Min cases**: 2

## Cases

| ID | Name | Status |
|----|------|--------|
| IN-05-1 | doc_build_produces_zero_missing_doc_warnings | ⏳ |
| IN-05-2 | all_public_items_have_doc_comments | ⏳ |

---

### IN-05-1: doc_build_produces_zero_missing_doc_warnings

- **Given:** The `genfile_core` source with all public items
- **When:** `RUSTDOCFLAGS="-D warnings" cargo test --doc --all-features` is run
- **Then:** The command exits with code 0 and produces no missing-doc warnings

---

### IN-05-2: all_public_items_have_doc_comments

- **Given:** Every public trait, struct, enum, function, and method in `src/`
- **When:** `cargo doc --all-features` is run
- **Then:** No item is listed without a doc comment; the missing-docs lint enforced in `src/lib.rs` passes without errors
