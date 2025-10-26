# Implement preserved quotes stripping in parse_from_argv

## Description

Enhance `parse_from_argv` to detect and strip literal quote characters that occur when users over-quote parameters. For example, when a user types `'param::"value"'`, the shell preserves the inner double quotes as literal characters, resulting in `param::"value"` being passed to the parser. Currently this creates double-quoting issues.

This is a lower-priority enhancement that improves handling of edge cases. The natural syntax (without over-quoting) already works correctly.

## Requirements

-   All work must strictly adhere to all applicable rulebooks
    (discover via `prompt .rulebooks.relevant`)

## Acceptance Criteria

-   Detection logic added to identify literal quote characters in parameter values
-   Quote stripping logic implemented while preserving intentionally escaped quotes
-   Tests created covering over-quoting scenarios
-   Test `test_argv_multiword_parameter_with_shell_quotes_preserved` passes (currently ignored)
-   No regressions in existing functionality
-   Documentation updated explaining quote handling behavior
