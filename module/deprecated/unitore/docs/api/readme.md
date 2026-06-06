# API Doc Entity

### Scope

- **Purpose**: Document the trait contracts forming the storage port of unitore's hexagonal architecture.
- **Responsibility**: Lists all async storage trait contracts and their operation sets.
- **In Scope**: Store, FeedStore, ConfigStore, FrameStore, and TableStore traits.
- **Out of Scope**: The FeedStorage sled-backed implementation; GlueSQL SQL syntax and internals.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Storage Port](001_storage_port.md) | Five async traits forming the storage boundary | ✅ |
