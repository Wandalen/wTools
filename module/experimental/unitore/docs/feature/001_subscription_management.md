# Feature: Subscription Management

### Scope

- **Purpose**: Manage the set of feed configuration files that unitore monitors.
- **Responsibility**: Documents the three subscription commands — adding, removing, and listing config files.
- **In Scope**: `.config.add`, `.config.delete`, `.config.list` commands and path canonicalization behavior.
- **Out of Scope**: Feed content download triggered by adding a config (→ `feature/002`); TOML subscription file syntax.

### Design

Config files are TOML files listing one or more feed URLs with their polling interval. Adding a config file path stores it in the database and immediately fetches all feeds declared in that file. Removing a config file path removes the config entry; the feeds it sourced remain in the feed table.

Paths are canonicalized to their absolute filesystem path before storage. This prevents the same file from being registered twice under different relative forms.

| Command | Phrase | Description |
|---------|--------|-------------|
| Add | `.config.add path::./feeds.toml` | Register a TOML config file and sync its feeds |
| Delete | `.config.delete path::./feeds.toml` | Remove a config file from monitoring |
| List | `.config.list` | Display all registered config file paths |

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/action/config.rs` | Business logic for add, delete, list operations |
| source | `src/command/config.rs` | wca command builders for config commands |
| source | `src/entity/config.rs` | ConfigStore trait definition |
| source | `src/sled_adapter/config.rs` | ConfigStore implementation against GlueSQL |
| source | `src/feed_config.rs` | TOML config file reader |
| doc | [api/001_storage_port.md](../api/001_storage_port.md) | ConfigStore trait contract |
| doc | [invariant/002_feed_url_primary_key.md](../invariant/002_feed_url_primary_key.md) | Uniqueness guarantee for feeds added from config |
