# Tests — Inc — Snipet

Compile-fail `.rs` snippets and expected `.stderr` files used by the trybuild harness.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| [rta_id.rs](rta_id.rs) | Passing trybuild case: a_id success path |
| [rta_id_fail.rs](rta_id_fail.rs) | Failing trybuild case: a_id mismatch |
| [rta_id_fail.stderr](rta_id_fail.stderr) | Expected stderr for rta_id_fail compile-fail |
| [rta_not_id.rs](rta_not_id.rs) | Passing trybuild case: a_not_id success path |
| [rta_not_id_fail.rs](rta_not_id_fail.rs) | Failing trybuild case: a_not_id mismatch |
| [rta_not_id_fail.stderr](rta_not_id_fail.stderr) | Expected stderr for rta_not_id_fail compile-fail |
| [cta_true_fail.rs](cta_true_fail.rs) | Failing trybuild case: cta_true false condition |
| [cta_true_fail.stderr](cta_true_fail.stderr) | Expected stderr for cta_true_fail compile-fail |
| [cta_type_same_size_fail.rs](cta_type_same_size_fail.rs) | Failing trybuild case: cta_type_same_size mismatch |
| [cta_type_same_size_fail.stderr](cta_type_same_size_fail.stderr) | Expected stderr for cta_type_same_size_fail |
| [cta_type_same_align_fail.rs](cta_type_same_align_fail.rs) | Failing trybuild case: cta_type_same_align mismatch |
| [cta_type_same_align_fail.stderr](cta_type_same_align_fail.stderr) | Expected stderr for cta_type_same_align_fail |
| [cta_ptr_same_size_fail.rs](cta_ptr_same_size_fail.rs) | Failing trybuild case: cta_ptr_same_size mismatch |
| [cta_ptr_same_size_fail.stderr](cta_ptr_same_size_fail.stderr) | Expected stderr for cta_ptr_same_size_fail |
| [cta_mem_same_size_fail.rs](cta_mem_same_size_fail.rs) | Failing trybuild case: cta_mem_same_size mismatch |
| [cta_mem_same_size_fail.stderr](cta_mem_same_size_fail.stderr) | Expected stderr for cta_mem_same_size_fail |
