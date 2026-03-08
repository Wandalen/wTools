# Manual Testing - claude_runner_core

This directory contains manual testing procedures, plans, and results for the `claude_runner_core` crate.

## Purpose

Manual testing verifies crate functionality with real Claude Code binary execution, testing corner cases and edge conditions that may not be practical in automated tests.

## Files

| File | Responsibility |
|------|----------------|
| `readme.md` | Manual testing overview and procedures |
| `-corner_cases_exhaustive.md` | Exhaustive corner case matrix |

Note: Files prefixed with `-` are temporary/working files per project conventions.

## Running Manual Tests

### Quick Start

Run all manual tests:
```bash
cd /home/user1/pro/lib/wip_core/wtools/dev/module/experimental/claude_runner_core
cargo test --test manual_execution_test -- --ignored --nocapture
```

Run specific test:
```bash
cargo test --test manual_execution_test manual_test_1_default_token_limit -- --ignored --nocapture
```

### Prerequisites

- Claude Code binary must be in PATH (`which claude` should succeed)
- Network connectivity (Claude API calls)
- Valid Claude API credentials configured

### Available Manual Tests

The `tests/manual_execution_test.rs` file contains 20 manual tests:

**Core Functionality (Tests 1-10):**
1. `manual_test_1_default_token_limit` - Verify default 200K token limit
2. `manual_test_2_nonexistent_directory` - Test error handling for missing paths
3. `manual_test_3_empty_message` - Test empty message string
4. `manual_test_4_message_with_shell_metacharacters` - **Security test** - verify no command injection
5. `manual_test_5_full_builder_chain` - Test builder method chaining
6. `manual_test_6_token_limit_explicit_override` - Test explicit token limit override
7. `manual_test_7_working_directory_with_spaces` - Test paths with spaces
8. `manual_test_8_very_long_message` - Test 10,000 character message
9. `manual_test_9_utf8_message` - Test unicode (emoji, Japanese, Russian, Arabic)
10. `manual_test_10_custom_arguments` - Test custom argument accumulation

**Extended Corner Cases (Tests 11-20):**
11. `manual_test_11_system_prompt` - Test system prompt functionality
12. `manual_test_12_verbose_mode` - Test verbose output mode
13. `manual_test_13_temperature_zero` - Test temperature 0.0 (deterministic)
14. `manual_test_14_newlines_in_message` - Test multiline messages
15. `manual_test_15_json_in_message` - Test JSON content in message
16. `manual_test_16_permission_denied_directory` - Test permission denied error
17. `manual_test_17_model_selection` - Test explicit model selection
18. `manual_test_18_action_mode_allow` - Test ActionMode::Allow setting
19. `manual_test_19_log_level_debug` - Test LogLevel::Debug setting
20. `manual_test_20_sampling_parameters` - Test top_p and top_k parameters

Each test is marked with `#[ignore]` to prevent accidental execution during regular test runs, since they require real Claude API access.

## Testing Methodology

Manual testing follows this workflow:

1. **Phase 0:** Read organizational governance rulebook
2. **Phase 1:** Read test organization rulebook
3. **Phase 2:** Create exhaustive corner case plan
4. **Phase 3:** Execute real tests with Claude Code binary
5. **Phase 4:** Fix all found issues (Round 0 → Round 1 pattern)
6. **Phase 5:** Iterate until zero issues remain

## Latest Test Results

**Date:** 2025-12-20
**Status:** ✅ ALL TESTS PASS
**Issues Found:** 0 functional issues
**Pass Rate:** 20/20 (100%)

See `-corner_cases_exhaustive.md` for complete corner case analysis.

## Corner Case Coverage

### Tested ✅
- Default token limit (200K)
- Token limit explicit override
- Nonexistent working directory
- Working directory with spaces
- Permission denied directory
- Empty message
- Shell metacharacters (security)
- Very long message (10K chars)
- UTF-8 unicode (emoji + international text)
- Newlines in message (multiline)
- JSON content in message
- Full builder chain (22+ methods)
- Custom arguments
- System prompt
- Verbose mode
- Temperature (0.0 deterministic)
- Model selection
- ActionMode::Allow
- LogLevel::Debug
- Sampling parameters (top_p, top_k)

### Covered by Automated Tests (build_command_for_test)
- Token limits: 0, 1, max u32
- Override semantics (last wins)
- Argument accumulation
- All environment variables
- All builder methods

### Not Yet Tested ⚠️
- execute_interactive() TTY mode (requires real terminal)
- Claude binary not in PATH (requires PATH manipulation)
- Continuation flag with missing/corrupted session
- API key with invalid values (security sensitive)
- Very large output (>1GB) - impractical

## Security Testing

**Critical Security Test:** `manual_test_4_message_with_shell_metacharacters`

This test verifies that shell special characters in messages are NOT interpreted as commands:

```rust
let dangerous_message = "Tell me: what is in $PATH directory? And `whoami` result?";
```

**Result:** ✅ PASS - No command injection vulnerability. Characters treated as literal text.

## Adding New Manual Tests

To add a new manual test:

1. Add test function to `tests/manual_execution_test.rs`
2. Mark with `#[ignore = "Manual test - run explicitly with --ignored flag"]`
3. Document expected behavior in test comments
4. Update this readme.md with test description
5. Run test manually to verify
6. Update corner case coverage in this readme.md

## Test Maintenance

Manual tests should be reviewed and updated when:
- New builder methods are added
- New execution modes are introduced
- New CLI flags are supported
- Security concerns are identified
- Edge cases are discovered in production

## Troubleshooting

### "Claude binary not found"
Ensure Claude Code is installed and in PATH:
```bash
which claude
# Should output: /path/to/claude
```

### "API authentication failed"
Verify Claude API credentials are configured:
```bash
claude --help
# Should not error about authentication
```

### Tests hang indefinitely
Check network connectivity and Claude API status. Set timeout in test if needed.

## Contact

For questions about manual testing:
- See specification: `../../spec.md`
- See main readme: `../../readme.md`
- Review test code: `../manual_execution_test.rs`
