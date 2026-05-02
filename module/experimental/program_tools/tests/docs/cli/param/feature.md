# Parameter Spec: feature

### Scope

- **Purpose**: Verify `--feature` enables Cargo features in the target program.
- **Responsibility**: Edge cases for absent (no features), single feature, multiple features, unknown feature, empty value, and repeated value.
- **In Scope**: Feature activation in generated manifests; collection merge semantics; unknown feature rejection.
- **Out of Scope**: Environment variable injection (→ `param/env.md`).

### EC-1 (Divergence A): Absent — no features enabled

**Given:** A program with a `my_feature` feature gate that prints different output based on the feature
**When:** `program_tools run main.rs` (no `--feature`; `my_feature` absent from manifest)
**Then:** Exit code `0`; `stdout` contains the output for the feature-absent branch; feature-gated code is not compiled
**Commands:** run

### EC-2 (Divergence B): Single feature enabled

**Given:** The same program with a `my_feature` gate
**When:** `program_tools run --feature my_feature main.rs`
**Then:** Exit code `0`; `stdout` contains the output for the feature-present branch; the feature is active
**Commands:** run

### EC-3: Two features — collection merge

**Given:** A program with two independent feature gates `feat_a` and `feat_b`
**When:** `program_tools run --feature feat_a --feature feat_b main.rs`
**Then:** Exit code `0`; both features active; output reflects both feature branches; second `--feature` appends rather than replaces
**Commands:** run

### EC-4: Unknown feature name

**Given:** A program whose manifest declares no features; `--feature no_such_feature` supplied
**When:** `program_tools run --feature no_such_feature main.rs`
**Then:** Exit code non-zero; `stderr` contains Cargo's unknown-feature diagnostic; `stdout` is empty
**Commands:** run

### EC-5: Empty feature name

**Given:** Any compilable program
**When:** `program_tools run --feature "" main.rs`
**Then:** Documented behaviour: accepted and passed to Cargo (Cargo may reject), or rejected early with exit code `1`; exit code is consistent and non-zero if rejected
**Commands:** run

### EC-6: Repeated identical feature

**Given:** A program with a `my_feature` gate
**When:** `program_tools run --feature my_feature --feature my_feature main.rs`
**Then:** Exit code `0`; behaviour is identical to supplying it once; no error from deduplication or double-activation
**Commands:** run
