# API: Storage Port

### Scope

- **Purpose**: Define the boundary between the action layer and the storage adapter in the hexagonal architecture.
- **Responsibility**: Documents all five async storage trait contracts and their operations.
- **In Scope**: Store, FeedStore, ConfigStore, FrameStore, and TableStore traits and their operations.
- **Out of Scope**: The FeedStorage sled-backed implementation; GlueSQL query syntax; SQL schema definitions.

### Abstract

Five async traits form a single storage port — the boundary all business logic crosses to read or write persistent state. Code in the action layer depends on these traits rather than on the concrete adapter, enabling the sled-backed implementation to be replaced without modifying business logic.

The five traits correspond to the five storage domains: arbitrary query execution, feed metadata, subscription config files, feed entry frames, and database schema inspection.

### Operations

**Store** — arbitrary SQL query execution:
- query_execute: accepts a GlueSQL SQL string, executes it against the database, returns all result payloads as a QueryReport.

**FeedStore** — feed metadata management:
- feeds_save: insert new feed rows from a list of Feed values.
- feeds_update: update existing feed metadata (title, description, timestamps) for feeds already in storage.
- feeds_process: three-way diff for a batch of fetched feeds — compare incoming entries against stored frames, insert new ones, update modified ones, skip unchanged.
- feeds_list: retrieve all stored feed rows as a formatted report.

**ConfigStore** — subscription config management:
- config_add: register a config file path and immediately sync its feeds.
- config_delete: remove a config file path and its associated feeds from storage.
- config_list: retrieve all stored config file paths.

**FrameStore** — article frame management:
- frames_save: insert new frame rows for entries not yet in storage.
- frames_update: update existing frames whose published date has changed.
- frames_list: retrieve all stored frame rows as a formatted report.

**TableStore** — schema inspection:
- tables_list: return a report listing all tables with their column descriptions.
- table_list: return all column metadata for a specific named table.

### Error Handling

All operations return error_tools::Result. Errors propagate from GlueSQL execution — invalid SQL syntax, storage I/O failures, schema violations (primary key conflicts, foreign key errors), and lock acquisition failures on the internal async mutex. Callers must not assume partial success on error; a failed operation may leave storage in an indeterminate state depending on whether GlueSQL issued any preceding writes.

### Compatibility Guarantees

These traits are internal to the unitore crate and carry no external stability guarantee. FeedStorage backed by SledStorage is the sole implementation. The Store trait carries a mockall automock attribute that generates MockStore for test purposes — MockStore is not part of the public API.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/entity/config.rs` | ConfigStore trait definition |
| source | `src/entity/feed.rs` | FeedStore trait definition |
| source | `src/entity/frame.rs` | FrameStore trait definition |
| source | `src/entity/table.rs` | TableStore trait definition |
| source | `src/sled_adapter/mod.rs` | Store trait definition and FeedStorage struct |
| source | `src/sled_adapter/config.rs` | ConfigStore implementation |
| source | `src/sled_adapter/feed.rs` | FeedStore implementation |
| source | `src/sled_adapter/frame.rs` | FrameStore implementation |
| source | `src/sled_adapter/table.rs` | TableStore implementation |
| doc | [feature/001_subscription_management.md](../feature/001_subscription_management.md) | ConfigStore usage context |
| doc | [feature/002_content_retrieval.md](../feature/002_content_retrieval.md) | FeedStore and FrameStore usage context |
| doc | [feature/003_data_access.md](../feature/003_data_access.md) | Store and TableStore usage context |
