# genfile_core Manual Test Plan

Manual testing procedures for scenarios not covered by automated unit tests.

### Responsibility Table

| File | Responsibility |
|------|----------------|

## Trigger

Run when: serialization format changes, filesystem integration changes, or before release.

## Procedures

### 1. Real Filesystem Materialization

Verify `TemplateArchive::materialize` writes files to a real filesystem.

```rust
use genfile_core::{ TemplateArchive, FileContent, WriteMode, Value };
use std::path::Path;

let mut archive = TemplateArchive::new( "smoke" );
archive.add_file( "output.txt".into(), FileContent::Text( "Hello, {{name}}!".into() ), WriteMode::Rewrite );
archive.values_mut().insert( "name".into(), Value::String( "World".into() ) );
let report = archive.materialize( Path::new( "/tmp/genfile_manual_test" ) ).unwrap();
assert_eq!( report.files_created.len(), 1 );
```

Expected: `/tmp/genfile_manual_test/output.txt` contains `Hello, World!`.

### 2. JSON Serialization Roundtrip via File

Verify save/load roundtrip preserves archive content exactly.

```rust
use genfile_core::{ TemplateArchive, FileContent, WriteMode };
use std::path::Path;

let mut archive = TemplateArchive::new( "roundtrip" );
archive.add_file( "a.txt".into(), FileContent::Text( "content".into() ), WriteMode::Rewrite );
archive.save_to_file( Path::new( "/tmp/roundtrip.json" ) ).unwrap();

let loaded = TemplateArchive::load_from_file( Path::new( "/tmp/roundtrip.json" ) ).unwrap();
assert_eq!( loaded.name, "roundtrip" );
assert_eq!( loaded.file_count(), 1 );
```

Expected: loaded archive matches original in name and file count.

### 3. Binary File Preservation

Verify binary files survive pack/materialize without corruption.

```rust
use genfile_core::{ TemplateArchive, FileContent, WriteMode };
use std::path::Path;

let bytes = vec![ 0u8, 1, 2, 255, 254, 253 ];
let mut archive = TemplateArchive::new( "binary" );
archive.add_file( "data.bin".into(), FileContent::Binary( bytes.clone() ), WriteMode::Rewrite );
let report = archive.materialize( Path::new( "/tmp/binary_test" ) ).unwrap();
let written = std::fs::read( "/tmp/binary_test/data.bin" ).unwrap();
assert_eq!( written, bytes );
```

Expected: binary content preserved byte-for-byte.
