# Claude Code JSONL Format Specification

## Overview

Claude Code stores conversation history in JSONL (JSON Lines) files, where each line is a complete JSON object representing a single conversation entry (message).

**Location**: `~/.claude/projects/{project-id}/{session-id}.jsonl`

**Format**: One JSON object per line, no trailing commas, valid JSON on each line.

## Entry Types

There are two primary entry types:
1. **User messages** - Messages sent by the human user
2. **Assistant messages** - Responses from Claude AI

Each entry type has different structures but shares common fields.

## Common Fields

These fields appear in all entries:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `uuid` | string | ✅ | Unique identifier for this entry (UUID v4) |
| `parentUuid` | string \| null | ✅ | UUID of parent entry (null for first message) |
| `timestamp` | string | ✅ | ISO 8601 timestamp (e.g., "2025-11-08T23:30:10.039Z") |
| `type` | string | ✅ | Entry type: "user" or "assistant" |
| `cwd` | string | ✅ | Working directory when message was sent |
| `sessionId` | string | ✅ | Session UUID this entry belongs to |
| `version` | string | ✅ | Claude Code version (e.g., "2.0.31") |
| `gitBranch` | string \| null | ✅ | Git branch name (null if not in git repo) |
| `userType` | string | ✅ | User type: "external" (human) |
| `isSidechain` | boolean | ✅ | Whether this is a sidechain conversation (sub-agent) |

## User Message Format

User messages have this structure:

```json
{
  "parentUuid": null,
  "isSidechain": false,
  "userType": "external",
  "cwd": "/home/user1/pro",
  "sessionId": "8d795a1c-c81d-4010-8d29-b4e678272419",
  "version": "2.0.31",
  "gitBranch": "master",
  "type": "user",
  "message": {
    "role": "user",
    "content": "command to repeat something every hour?"
  },
  "uuid": "a6f3bd8c-5575-4eab-82b0-b856f7a02833",
  "timestamp": "2025-11-08T23:30:10.039Z",
  "thinkingMetadata": {
    "level": "high",
    "disabled": false,
    "triggers": [
      {
        "start": 58,
        "end": 68,
        "text": "ultrathink"
      }
    ]
  }
}
```

### User-Specific Fields

| Field | Type | Description |
|-------|------|-------------|
| `message.role` | string | Always "user" |
| `message.content` | string | The user's message text |
| `thinkingMetadata` | object | Configuration for extended thinking |
| `thinkingMetadata.level` | string | Thinking level: "low", "medium", "high" |
| `thinkingMetadata.disabled` | boolean | Whether thinking is disabled |
| `thinkingMetadata.triggers` | array | Trigger words that enabled thinking |
| `thinkingMetadata.triggers[].start` | number | Character offset where trigger starts |
| `thinkingMetadata.triggers[].end` | number | Character offset where trigger ends |
| `thinkingMetadata.triggers[].text` | string | The trigger word (e.g., "ultrathink") |

## Assistant Message Format

Assistant messages are more complex, with structured API response data:

```json
{
  "parentUuid": "a6f3bd8c-5575-4eab-82b0-b856f7a02833",
  "isSidechain": false,
  "userType": "external",
  "cwd": "/home/user1/pro",
  "sessionId": "8d795a1c-c81d-4010-8d29-b4e678272419",
  "version": "2.0.31",
  "gitBranch": "master",
  "message": {
    "model": "claude-sonnet-4-5-20250929",
    "id": "msg_01AEieWYMdbGML9PEKCmB36v",
    "type": "message",
    "role": "assistant",
    "content": [
      {
        "type": "thinking",
        "thinking": "...",
        "signature": "..."
      },
      {
        "type": "text",
        "text": "Looking at options..."
      },
      {
        "type": "tool_use",
        "id": "toolu_...",
        "name": "Bash",
        "input": {...}
      }
    ],
    "stop_reason": "end_turn",
    "stop_sequence": null,
    "usage": {
      "input_tokens": 9,
      "cache_creation_input_tokens": 10156,
      "cache_read_input_tokens": 12112,
      "cache_creation": {
        "ephemeral_5m_input_tokens": 10156,
        "ephemeral_1h_input_tokens": 0
      },
      "output_tokens": 6,
      "service_tier": "standard"
    }
  },
  "requestId": "req_011CUwHuh7iPfwQNAXEeEYrP",
  "type": "assistant",
  "uuid": "56a226b5-0ec6-4214-af16-b13cc326f8dc",
  "timestamp": "2025-11-08T23:30:21.913Z"
}
```

### Assistant-Specific Fields

| Field | Type | Description |
|-------|------|-------------|
| `message.model` | string | Claude model used (e.g., "claude-sonnet-4-5-20250929") |
| `message.id` | string | API message ID |
| `message.type` | string | Always "message" |
| `message.role` | string | Always "assistant" |
| `message.content` | array | Array of content blocks |
| `message.stop_reason` | string \| null | Why generation stopped: "end_turn", "stop_sequence", "max_tokens" |
| `message.stop_sequence` | string \| null | The stop sequence that triggered (if any) |
| `message.usage` | object | Token usage statistics |
| `requestId` | string | API request ID |

## Content Block Types

The `message.content` array can contain several block types:

### 1. Text Block

Plain text response from Claude:

```json
{
  "type": "text",
  "text": "Here's the answer..."
}
```

### 2. Thinking Block

Extended thinking/reasoning (when ultrathink is enabled):

```json
{
  "type": "thinking",
  "thinking": "Let me analyze this...",
  "signature": "base64-encoded-signature"
}
```

**Note**: Signature is cryptographically signed thinking content to verify authenticity.

### 3. Tool Use Block

When Claude invokes a tool:

```json
{
  "type": "tool_use",
  "id": "toolu_01ABC123",
  "name": "Bash",
  "input": {
    "command": "ls -la",
    "description": "List files"
  }
}
```

### 4. Tool Result Block

Results from tool execution:

```json
{
  "type": "tool_result",
  "tool_use_id": "toolu_01ABC123",
  "content": "file1.txt\nfile2.txt",
  "is_error": false
}
```

## Usage Object

Token usage tracking for billing and analytics:

```json
{
  "input_tokens": 9,
  "cache_creation_input_tokens": 10156,
  "cache_read_input_tokens": 12112,
  "cache_creation": {
    "ephemeral_5m_input_tokens": 10156,
    "ephemeral_1h_input_tokens": 0
  },
  "output_tokens": 6,
  "service_tier": "standard"
}
```

| Field | Type | Description |
|-------|------|-------------|
| `input_tokens` | number | Regular input tokens (uncached) |
| `cache_creation_input_tokens` | number | Tokens used to create cache |
| `cache_read_input_tokens` | number | Tokens read from cache |
| `cache_creation.ephemeral_5m_input_tokens` | number | 5-minute cache tokens |
| `cache_creation.ephemeral_1h_input_tokens` | number | 1-hour cache tokens |
| `output_tokens` | number | Tokens generated by assistant |
| `service_tier` | string | API service tier: "standard" |

## Conversation Threading

Entries are linked via `parentUuid`:

```
Entry 1 (User):    uuid=A, parentUuid=null
  ↓
Entry 2 (Assistant): uuid=B, parentUuid=A
  ↓
Entry 3 (User):    uuid=C, parentUuid=B
  ↓
Entry 4 (Assistant): uuid=D, parentUuid=C
```

## Sidechain Sessions

Sub-agent conversations (e.g., Task agents) are stored in separate files:

**Main session**: `8d795a1c-c81d-4010-8d29-b4e678272419.jsonl`
**Sub-agent**: `agent-167331a7.jsonl`

Sidechain entries have `isSidechain: true`.

## Parsing Considerations

### Line-by-Line Processing

JSONL files should be parsed line-by-line:

```rust
for (line_num, line) in file_content.lines().enumerate() {
  if line.trim().is_empty() {
    continue; // Skip empty lines
  }

  let entry: Entry = parse_json_line(line)?;
  // Process entry...
}
```

### Error Handling

- **Malformed JSON**: Skip line, log warning, continue
- **Missing required fields**: Error for that entry
- **Unknown fields**: Ignore (forward compatibility)

### Memory Efficiency

For large sessions:
- Stream line-by-line instead of loading entire file
- Process entries as they're parsed
- Don't accumulate in memory unless needed

## Schema Validation

Recommended validation checks:

1. ✅ `uuid` is valid UUID v4 format
2. ✅ `timestamp` is valid ISO 8601
3. ✅ `type` is "user" or "assistant"
4. ✅ `parentUuid` references exist (except first entry)
5. ✅ `sessionId` matches filename
6. ✅ User messages have `message.content` as string
7. ✅ Assistant messages have `message.content` as array

## Format Evolution

### Version Compatibility

The `version` field indicates Claude Code version. Parsers should:
- Support all versions gracefully
- Ignore unknown fields (forward compatibility)
- Provide defaults for missing optional fields (backward compatibility)

### Known Variations

**v2.0.x**:
- Current format as documented above
- Thinking signatures added in 2.0.30+
- Cache tracking added in 2.0.25+

**v1.x**:
- Legacy format (not documented here)
- Different message structure
- No thinking metadata

## Example: Minimal Valid Entry

**User message**:
```json
{"uuid":"a6f3bd8c-5575-4eab-82b0-b856f7a02833","parentUuid":null,"timestamp":"2025-11-08T23:30:10.039Z","type":"user","cwd":"/home/user","sessionId":"8d795a1c-c81d-4010-8d29-b4e678272419","version":"2.0.31","gitBranch":null,"userType":"external","isSidechain":false,"message":{"role":"user","content":"Hello"},"thinkingMetadata":{"level":"low","disabled":true,"triggers":[]}}
```

**Assistant message**:
```json
{"uuid":"56a226b5-0ec6-4214-af16-b13cc326f8dc","parentUuid":"a6f3bd8c-5575-4eab-82b0-b856f7a02833","timestamp":"2025-11-08T23:30:21.913Z","type":"assistant","cwd":"/home/user","sessionId":"8d795a1c-c81d-4010-8d29-b4e678272419","version":"2.0.31","gitBranch":null,"userType":"external","isSidechain":false,"message":{"model":"claude-sonnet-4-5-20250929","id":"msg_01ABC","type":"message","role":"assistant","content":[{"type":"text","text":"Hi there!"}],"stop_reason":"end_turn","stop_sequence":null,"usage":{"input_tokens":5,"output_tokens":3,"cache_creation_input_tokens":0,"cache_read_input_tokens":0,"service_tier":"standard"}},"requestId":"req_01ABC"}
```

## Testing Data

For parser testing, use these scenarios:

1. **Empty file** - Should return empty vec
2. **Single user message** - Parse successfully
3. **Single assistant message** - Parse successfully
4. **Conversation thread** - Multiple entries with correct parentUuid chain
5. **Malformed JSON line** - Skip gracefully, warn
6. **Missing required field** - Error for that entry
7. **Unknown fields** - Ignore, parse rest
8. **Empty lines** - Skip
9. **Unicode content** - Handle correctly
10. **Very long messages** - Stream efficiently

## References

- **JSONL Spec**: https://jsonlines.org/
- **UUID v4**: RFC 4122
- **ISO 8601**: https://www.iso.org/iso-8601-date-and-time-format.html
- **Claude API**: https://docs.anthropic.com/en/api/
