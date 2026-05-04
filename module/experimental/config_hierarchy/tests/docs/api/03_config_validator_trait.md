# API Spec: ConfigValidator Trait

### Scope

- **Element:** `api/003_config_validator_trait`
- **Source:** `docs/api/003_config_validator_trait.md`
- **Feature flag:** `enabled`
- **Prefix:** `AP-`
- **Minimum cases:** 4

## Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| AP-01 | no_validator_passes_all_values | nominal | ✅ |
| AP-02 | validate_parameter_rejects_invalid_value | behavioral_divergence | ⏳ |
| AP-03 | validate_parameter_accepts_valid_value | behavioral_divergence | ⏳ |
| AP-04 | validate_all_detects_cross_param_violation | nominal | ⏳ |
| AP-05 | validate_all_returns_empty_for_valid_config | boundary | ⏳ |
| AP-06 | validator_type_param_governs_behavior | cannot_be_faked | ⏳ |

---

### AP-01: NoValidator accepts all values without error

- **Given:** `ConfigManager< D, P, NoValidator >` is the manager type
- **When:** `validate_parameter("any_param", &any_value)` is called
- **Then:** Returns `Ok(())` regardless of the value — NoValidator never rejects
- **Tests:** `tests/validator_tests.rs::test_no_validator_accepts_all`

### AP-02: validate_parameter() returns Err for invalid value

- **Given:** A validator that rejects negative numbers; value is `-1`
- **When:** `validate_parameter("count", &JsonValue::Number((-1).into()))` is called
- **Then:** Returns `Err(ValidationError)` with `parameter == "count"` — rejection propagated
- **Tests:** `tests/validator_tests.rs::test_validator_rejects_negative` ⏳ (not yet written)

### AP-03: validate_parameter() returns Ok for valid value

- **Given:** Same negative-rejecting validator; value is `5`
- **When:** `validate_parameter("count", &JsonValue::Number(5.into()))` is called
- **Then:** Returns `Ok(())` — valid value accepted
- **Tests:** `tests/validator_tests.rs::test_validator_accepts_positive` ⏳ (not yet written)

### AP-04: validate_all() detects cross-parameter constraint violation

- **Given:** A validator that requires `timeout > 0` whenever `retries > 0`; resolved config has `retries=3, timeout=0`
- **When:** `validate_all_config(&config)` is called
- **Then:** Returns `Vec` with one `ValidationError` for `"timeout"` — cross-parameter constraint violated
- **Tests:** `tests/validator_tests.rs::test_validate_all_cross_parameter_constraint` ⏳ (not yet written)

### AP-05: validate_all() returns empty Vec for valid config

- **Given:** Same cross-parameter validator; resolved config has `retries=3, timeout=30`
- **When:** `validate_all_config(&config)` is called
- **Then:** Returns empty `Vec` — no violations when constraint satisfied
- **Tests:** `tests/validator_tests.rs::test_validate_all_valid_config` ⏳ (not yet written)

### AP-06: validator type parameter governs behavior (cannot-be-faked)

- **Given:** Two manager types — `ConfigManager< D, P, NoValidator >` and `ConfigManager< D, P, NegativeRejectValidator >`; value is `-42`
- **When:** `validate_parameter("x", &JsonValue::Number((-42).into()))` is called on each
- **Then:** `NoValidator` returns `Ok(())`; `NegativeRejectValidator` returns `Err` — different `V` type produces different result, proving `V` is consulted
- **Tests:** `tests/validator_tests.rs::test_validator_type_param_governs_behavior` ⏳ (not yet written)
