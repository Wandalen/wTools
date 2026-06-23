# Test Surface: Invariant — Zero-Copy Contract

### Source

- **Doc Instance:** [invariant/001_zero_copy_contract.md](../../../docs/invariant/001_zero_copy_contract.md)

### Cases

| # | Status | Case |
|---|--------|------|
| IN-1 | ✅ | Split segments borrow from source |
| IN-2 | ✅ | Isolation slices borrow from source |
| IN-3 | ✅ | Stripping breaks zero-copy — returns owned |
| IN-4 | ✅ | Number parsing breaks zero-copy — returns numeric value |

### IN-1 — Split segments borrow from source

- **Given:** A source string and a single-byte delimiter
- **When:** Split is invoked with stripping disabled and unescaping disabled
- **Then:** Each yielded segment is a subslice of the original source (pointer within source bounds)
- **Test:** `tests/inc/split_test.rs`

### IN-2 — Isolation slices borrow from source

- **Given:** A source string with left and right delimiters present
- **When:** Isolation is invoked without content transformation
- **Then:** The returned slice borrows from the source (no allocation)
- **Test:** `tests/inc/isolate_test.rs`

### IN-3 — Stripping breaks zero-copy — returns owned

- **Given:** A source string containing quote characters around content
- **When:** Isolation is invoked with stripping enabled
- **Then:** The returned value is an owned string, not a subslice of the source
- **Test:** `tests/invariant_contract_test.rs` — `inv_001_in_3_stripping_breaks_zero_copy`

### IN-4 — Number parsing breaks zero-copy — returns numeric value

- **Given:** A string containing numeric text like `"42"`
- **When:** Number parsing converts the string to a numeric type
- **Then:** The returned value is a numeric value, not a string slice — zero-copy invariant intentionally broken
- **Test:** `tests/inc/number_test.rs` — `integer_basic` (returns `Ok(42i32)`, not a slice)
