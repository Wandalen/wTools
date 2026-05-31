# Test Spec: Archive Self-Containment

- **Source**: `docs/feature/017_archive_self_containment.md`
- **Prefix**: `FT-17`
- **Min cases**: 4

## Cases

| ID | Name | Status |
|----|------|--------|
| FT-17-1 | archive_stores_parameter_values_inline | ⏳ |
| FT-17-2 | loaded_archive_generates_without_external_values | ⏳ |
| FT-17-3 | json_roundtrip_preserves_all_fields | ⏳ |
| FT-17-4 | yaml_roundtrip_preserves_all_fields | ⏳ |

---

### FT-17-1: archive_stores_parameter_values_inline

- **Given:** A template archive serialized to JSON with parameter values embedded in the document
- **When:** The JSON is parsed back into an archive
- **Then:** The archive's parameter values match the originals without requiring any external lookup

---

### FT-17-2: loaded_archive_generates_without_external_values

- **Given:** A template archive loaded from a self-contained JSON document that contains template content and all parameter values
- **When:** `generate` is called without supplying additional values
- **Then:** Generation succeeds and output files contain correctly substituted content

---

### FT-17-3: json_roundtrip_preserves_all_fields

- **Given:** An archive with file descriptors, parameter values, and metadata
- **When:** It is serialized to JSON and deserialized back
- **Then:** The deserialized archive is structurally identical to the original (all fields preserved)

---

### FT-17-4: yaml_roundtrip_preserves_all_fields

- **Given:** An archive with file descriptors, parameter values, and metadata
- **When:** It is serialized to YAML and deserialized back
- **Then:** The deserialized archive is structurally identical to the original (all fields preserved)
