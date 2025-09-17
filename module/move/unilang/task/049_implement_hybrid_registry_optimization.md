# Implement hybrid registry optimization

## Description

Implement the hybrid registry optimization that enhances both static and dynamic command lookup performance while maintaining full backward compatibility. This involves optimizing DynamicCommandMap with better data structures (IndexMap for cache locality, LruCache for hot commands, StringInterner for memory efficiency), adding RegistryMode selection, and implementing intelligent caching. Links to task 048 for test foundation.

## Requirements

- All work must strictly adhere to the rules defined in the following rulebooks:
  - `$PRO/genai/code/rules/code_design.rulebook.md`
  - `$PRO/genai/code/rules/code_style.rulebook.md`

## Acceptance Criteria

- DynamicCommandMap implemented with IndexMap, LruCache, and StringInterner
- RegistryMode enum with StaticOnly, DynamicOnly, Hybrid, Auto variants
- Intelligent caching layer for hot command optimization
- Performance improvements: 2-3x dynamic lookup, 50% memory reduction
- Zero breaking changes - all existing APIs work unchanged
- All tests from task 048 pass
- Implementation validated with `ctest1` verification