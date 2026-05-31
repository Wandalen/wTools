# Test Spec: Backward Compatibility

- **Source**: `docs/invariant/007_backward_compatibility.md`
- **Prefix**: `IN-07`
- **Min cases**: 2

## Cases

| ID | Name | Status |
|----|------|--------|
| IN-07-1 | semver_checks_passes_on_minor_release | ⏳ |
| IN-07-2 | public_api_removals_require_major_version_bump | ⏳ |

[PENDING — `cargo semver-checks` CI integration not yet in place]

---

### IN-07-1: semver_checks_passes_on_minor_release

- **Given:** The current public API of `genfile_core` and a candidate minor or patch release
- **When:** `cargo semver-checks` is run comparing the candidate against the previous published version
- **Then:** The tool reports no breaking changes and exits with code 0

[PENDING — requires `cargo semver-checks` CI integration — see task]

---

### IN-07-2: public_api_removals_require_major_version_bump

- **Given:** A pull request that removes or changes the signature of any public item in `src/`
- **When:** `cargo semver-checks` is run in CI
- **Then:** The tool reports a breaking change; the PR must increment the major version before merging
