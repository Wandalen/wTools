# unitore

Feed reader with the ability to set updates frequency and local storage.

## Overview

`unitore` is an RSS/Atom feed aggregator CLI tool that downloads feeds from configured sources, stores them in a local database, and provides querying capabilities. It supports configurable update frequencies per feed and uses GlueSQL with sled storage backend for persistence.

The tool is designed for users who want to aggregate and query feed content locally without relying on external feed reader services.

### Scope

#### Responsibility

unitore is responsible for downloading RSS/Atom feeds from configured sources at specified intervals, storing feed content in a local database, and providing CLI commands for querying and managing stored feeds.

#### In-Scope

- **Feed downloading**: HTTP(S) retrieval of RSS/Atom feeds
- **Feed parsing**: Parse RSS and Atom feed formats via `feed-rs`
- **Local storage**: Persist feeds and frames in sled-backed GlueSQL database
- **Configuration management**: TOML-based feed source configuration
- **Update scheduling**: Configurable per-feed update frequencies
- **SQL querying**: Execute SQL queries against stored feed data
- **CLI interface**: Command-line access via wca framework

#### Out-of-Scope

- **GUI/Web interface**: CLI-only tool
- **Real-time notifications**: No push/alert system
- **Feed discovery**: Manual configuration only
- **Content rendering**: Raw data storage and retrieval
- **Authentication**: No support for authenticated feeds
- **Podcast support**: Feed metadata only, no media handling

#### Boundaries

- **Upstream**: Uses `feed-rs` for parsing, `hyper` for HTTP, `gluesql` for storage
- **Downstream**: Used by developers/users wanting local feed aggregation
- **Storage**: Local sled database, configurable via `UNITORE_STORAGE_PATH`

## Architecture

### Module Structure

```
unitore/
├── src/
│   ├── lib.rs              # Crate root
│   ├── main.rs             # Binary entry point
│   ├── retriever.rs        # HTTP feed fetching
│   ├── feed_config.rs      # Configuration parsing
│   ├── executor.rs         # Command executor
│   ├── entity/             # Data structures
│   │   ├── config.rs       # Config entity
│   │   ├── feed.rs         # Feed entity
│   │   ├── frame.rs        # Frame (entry) entity
│   │   └── table.rs        # Table metadata
│   ├── command/            # CLI command definitions
│   │   ├── config.rs       # Config commands
│   │   ├── feed.rs         # Feed commands
│   │   ├── frame.rs        # Frame commands
│   │   ├── query.rs        # Query commands
│   │   └── table.rs        # Table commands
│   ├── action/             # Action implementations
│   │   ├── config.rs       # Config actions
│   │   ├── feed.rs         # Feed actions
│   │   ├── frame.rs        # Frame actions
│   │   ├── query.rs        # Query actions
│   │   └── table.rs        # Table actions
│   ├── sled_adapter/       # Database storage layer
│   │   ├── config.rs       # Config storage
│   │   ├── feed.rs         # Feed storage
│   │   ├── frame.rs        # Frame storage
│   │   └── table.rs        # Table operations
│   └── tool/               # Utilities
│       └── table_display.rs
├── tests/
├── Cargo.toml
├── readme.md
└── spec.md
```

### Data Flow

```
┌─────────────────────────────────────────────────────────────┐
│                      Configuration                           │
│     TOML files → feed_config → sled_adapter/config          │
├─────────────────────────────────────────────────────────────┤
│                      Download Flow                           │
│  config → retriever (HTTP) → feed-rs (parse) → storage      │
├─────────────────────────────────────────────────────────────┤
│                      Storage Layer                           │
│     sled_adapter → GlueSQL → sled (embedded KV store)       │
├─────────────────────────────────────────────────────────────┤
│                      Query Layer                             │
│     CLI → command → action → SQL → sled_adapter → results   │
└─────────────────────────────────────────────────────────────┘
```

## Public API

### CLI Commands

#### `.config.add`

Add a configuration file to storage.

```bash
unitore .config.add ./config/feeds.toml
```

#### `.config.delete`

Remove a configuration file from storage.

```bash
unitore .config.delete ./config/feeds.toml
```

#### `.config.list`

List all configuration files in storage.

```bash
unitore .config.list
```

#### `.frames.download`

Download feeds from all configured sources.

```bash
unitore .frames.download
```

#### `.frames.list`

List all frames (feed entries) in storage.

```bash
unitore .frames.list
```

#### `.feeds.list`

List all feeds in storage.

```bash
unitore .feeds.list
```

#### `.query.execute`

Execute a SQL query against the storage database.

```bash
unitore .query.execute 'SELECT title, links, published FROM frame WHERE published > "2024-01-01"'
```

#### `.tables.list`

List all tables in the database.

```bash
unitore .tables.list
```

### Configuration Format

```toml
# feeds.toml
[[config]]
update_period = "1h"
link = "https://feeds.bbci.co.uk/news/world/rss.xml"

[[config]]
update_period = "30min"
link = "https://rss.nytimes.com/services/xml/rss/nyt/World.xml"

[[config]]
update_period = "2days 5h"
link = "https://example.com/feed.xml"
```

**Configuration fields:**
- `update_period`: Human-readable duration (e.g., `12h`, `1h 20min`, `2days 5h`)
- `link`: URL of the RSS/Atom feed

### Database Schema

```sql
-- Feeds table
CREATE TABLE feed (
  id TEXT PRIMARY KEY,
  title TEXT,
  link TEXT,
  description TEXT,
  published TIMESTAMP,
  updated TIMESTAMP
);

-- Frames table (feed entries)
CREATE TABLE frame (
  id TEXT PRIMARY KEY,
  feed_id TEXT,
  title TEXT,
  links TEXT,
  summary TEXT,
  content TEXT,
  published TIMESTAMP,
  updated TIMESTAMP,
  authors TEXT,
  categories TEXT
);

-- Config table
CREATE TABLE config (
  path TEXT PRIMARY KEY,
  update_period TEXT,
  link TEXT
);
```

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `UNITORE_STORAGE_PATH` | `./_data` | Path to storage directory |

## Usage Patterns

### Basic Workflow

```bash
# 1. Create configuration file
cat > feeds.toml << 'EOF'
[[config]]
update_period = "1h"
link = "https://feeds.bbci.co.uk/news/rss.xml"
EOF

# 2. Add configuration to storage
unitore .config.add ./feeds.toml

# 3. Download feeds
unitore .frames.download

# 4. View downloaded frames
unitore .frames.list

# 5. Query specific data
unitore .query.execute 'SELECT title, published FROM frame ORDER BY published DESC LIMIT 10'
```

### Multiple Feed Sources

```bash
# Add multiple config files
unitore .config.add ./news_feeds.toml
unitore .config.add ./tech_feeds.toml
unitore .config.add ./blog_feeds.toml

# Download all feeds
unitore .frames.download

# List all configs
unitore .config.list
```

### Custom Storage Location

```bash
# Set custom storage path
export UNITORE_STORAGE_PATH=/path/to/feeds_db

# All commands will use this location
unitore .frames.download
```

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `enabled` | ✓ | Enable the crate |
| `full` | - | All features |

## Dependencies and Consumers

### Dependencies

| Dependency | Purpose |
|------------|---------|
| `feed-rs` | RSS/Atom feed parsing |
| `hyper` | HTTP client |
| `hyper-tls` | HTTPS support |
| `gluesql` | SQL database engine |
| `sled` | Embedded key-value store (via gluesql) |
| `tokio` | Async runtime |
| `toml` | Configuration parsing |
| `wca` | CLI framework |
| `cli-table` | Table output formatting |
| `humantime-serde` | Duration parsing |

### Potential Consumers

- Developers wanting local feed aggregation
- Data analysts collecting feed data
- Researchers archiving feed content
- Automation pipelines processing feeds

## Design Rationale

### Why Local Storage?

1. **Offline access**: Query feeds without network
2. **Historical data**: Keep past entries even after feed updates
3. **Custom queries**: Full SQL access to feed data
4. **No external dependencies**: No service accounts required

### Why GlueSQL + sled?

1. **Embedded**: No external database server needed
2. **SQL interface**: Familiar query language
3. **Persistent**: Data survives restarts
4. **Lightweight**: Minimal resource usage

### Why Configurable Update Periods?

Feeds have different update frequencies:
- News feeds: Update frequently (minutes/hours)
- Blogs: Update infrequently (days/weeks)
- Configurable periods prevent unnecessary requests

## Testing Strategy

### Test Categories

1. **Unit tests**: Entity and parsing tests
2. **Integration tests**: Full download/storage flow
3. **Mock tests**: HTTP request mocking via `mockall`

### Running Tests

```bash
# Standard tests
cargo test

# With all features
cargo test --features full
```

## Future Considerations

### Potential Enhancements

1. **Scheduled downloads**: Background daemon mode
2. **Feed discovery**: Auto-discover feeds from URLs
3. **Export formats**: JSON, CSV, OPML export
4. **Filter rules**: Keyword-based frame filtering
5. **Deduplication**: Detect and merge duplicate entries

### Known Limitations

1. **Manual updates**: No automatic background downloads
2. **No authentication**: Cannot access protected feeds
3. **Experimental status**: API may change
4. **No media handling**: Text content only

## Adoption Guidelines

### When to Use

- Need local feed storage and querying
- Want SQL access to feed data
- Prefer CLI over GUI feed readers
- Need to process feeds in automation pipelines

### When Not to Use

- Want real-time notifications
- Need GUI feed reader
- Require authenticated feed access
- Need podcast/media management

### Integration Pattern

```bash
# Install
cargo install unitore

# Initialize with feeds
unitore .config.add ./my_feeds.toml

# Cron job for periodic updates
0 * * * * cd /path/to/data && unitore .frames.download

# Query in scripts
unitore .query.execute 'SELECT COUNT(*) FROM frame WHERE published > DATE("now", "-1 day")'
```

## Related Crates

| Crate | Relationship |
|-------|--------------|
| `feed-rs` | Upstream - feed parsing |
| `gluesql` | Upstream - database engine |
| `wca` | Internal - CLI framework |
| `rss` | Alternative - RSS-only parser |
| `atom_syndication` | Alternative - Atom-only parser |

## References

- [RSS 2.0 Specification](https://www.rssboard.org/rss-specification)
- [Atom Syndication Format](https://tools.ietf.org/html/rfc4287)
- [GlueSQL Documentation](https://gluesql.org/)
