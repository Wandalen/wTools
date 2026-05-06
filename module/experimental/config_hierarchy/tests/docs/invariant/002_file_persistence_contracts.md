# Invariant Spec: File Persistence Contracts

### Scope

- **Element:** `invariant/002_file_persistence_contracts`
- **Source:** `docs/invariant/002_file_persistence_contracts.md`
- **Feature flag:** `file_ops`
- **Prefix:** `FP-`
- **Minimum cases:** 3

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| FP-01 | created_at_preserved_on_resave | invariant | ✅ |
| FP-02 | last_modified_updated_on_every_save | invariant | ✅ |
| FP-03 | concurrent_writes_do_not_corrupt | invariant | ✅ |
| FP-04 | atomic_modify_reads_before_writing | invariant | ✅ |

---

### FP-01: created_at is preserved on re-save

- **Given:** A config file with an existing `created_at` timestamp is saved again
- **When:** Save operation is performed a second time with updated parameter values
- **Then:** The `created_at` field in the saved file equals the original timestamp — it is not overwritten with the current time
- **Tests:** `tests/edge_cases_tests.rs::test_created_at_preserved_on_resave`

### FP-02: last_modified is updated on every save

- **Given:** A config file exists with a `last_modified` timestamp from an earlier save
- **When:** Save operation is performed
- **Then:** The `last_modified` field reflects the current save time — later than the previous value
- **Tests:** `tests/edge_cases_tests.rs`

### FP-03: concurrent writes do not corrupt the file

- **Given:** Multiple threads or processes attempt to write to the same config file simultaneously
- **When:** All write operations complete
- **Then:** The resulting file is valid YAML parseable without error — no partial writes, no interleaved content
- **Tests:** `tests/concurrent_access_tests.rs`

### FP-04: atomic modify reads current state before writing

- **Given:** A config file contains existing parameter values
- **When:** Atomic modify operation is called with a closure that modifies one value
- **Then:** The closure receives the current file state (including values set by previous saves); the written file reflects both the closure's changes and any pre-existing values the closure did not touch
- **Tests:** `tests/concurrent_access_tests.rs`
