# Feature: Content Retrieval

### Scope

- **Purpose**: Fetch RSS/Atom feed content from remote sources and browse stored articles.
- **Responsibility**: Documents the frame download, frame listing, and feed listing commands.
- **In Scope**: `.frame.download`, `.frame.list`, `.feed.list` commands and the diff-based update behavior.
- **Out of Scope**: Subscription config management (→ `feature/001`); raw SQL access (→ `feature/003`).

### Design

`.frame.download` reads all stored config file paths, fetches each declared feed over HTTPS, and applies a three-way diff against stored frames: new entries are inserted, modified entries (same id but different published date) are updated, unchanged entries are skipped. This makes repeated downloads idempotent.

`.feed.list` displays all stored feeds with title, URL, polling interval, and source config file.

`.frame.list` displays all stored frames (articles) across all feeds, showing title, content, links, authors, and timestamps.

| Command | Phrase | Description |
|---------|--------|-------------|
| Download | `.frame.download` | Fetch all feeds and apply diff to storage |
| List frames | `.frame.list` | Display all stored article entries |
| List feeds | `.feed.list` | Display all stored feed metadata |

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/action/feed.rs` | Business logic for feed listing |
| source | `src/action/frame.rs` | Business logic for download and list operations |
| source | `src/command/frame.rs` | wca command builders for frame commands |
| source | `src/command/feed.rs` | wca command builder for feed list |
| source | `src/retriever.rs` | HTTPS feed fetcher using hyper |
| source | `src/sled_adapter/feed.rs` | feeds_process diff algorithm implementation |
| source | `src/sled_adapter/frame.rs` | frames_save and frames_update implementation |
| test | `tests/frames_download.rs` | Integration tests for download and deduplication |
| doc | [api/001_storage_port.md](../api/001_storage_port.md) | FeedStore and FrameStore trait contracts |
| doc | [invariant/001_frame_deduplication.md](../invariant/001_frame_deduplication.md) | Deduplication guarantee during download |
| doc | [invariant/002_feed_url_primary_key.md](../invariant/002_feed_url_primary_key.md) | Feed URL uniqueness enforced at schema level |
