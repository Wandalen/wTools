# examples/

Runnable examples demonstrating `genfile_core` API usage patterns.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| basic_template.rs | Minimal create-add-materialize workflow |
| archive_with_parameters.rs | Parameter definition and auto-discovery |
| binary_files.rs | Binary file handling with base64 encoding |
| serialization.rs | JSON and YAML archive serialization roundtrip |
| custom_storage.rs | Custom `ContentStorage` trait implementation |
| external_content.rs | External file references via `FileRef` and `ContentSource` |
| low_level_template.rs | Direct `Template<V,R,FS>` API without archive wrapper |
