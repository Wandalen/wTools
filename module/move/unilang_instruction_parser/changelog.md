# Changelog

* [Increment 1 | 2025-07-05 10:34 UTC] Added failing test for incorrect command path parsing.
* [Increment 2 | 2025-07-05 10:58 UTC] Correctly parse command paths instead of treating them as arguments.
*   Investigated and documented the correct usage of `strs_tools::string::split::SplitOptionsFormer` with dynamic delimiters to resolve lifetime issues.
* [Increment 1 | 2025-07-06 06:42 UTC] Investigated `strs_tools` API issues and proposed switching to `regex` for string splitting.
- **Increment 1:** Refactored the parser engine to use official, unified data structures, establishing a consistent foundation.