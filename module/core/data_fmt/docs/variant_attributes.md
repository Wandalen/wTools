# Comprehensive Variant Attributes List

This document defines all 46 attributes used to describe each output format variant.

## Identity & Classification

1. **formatter** - Parent formatter name
   - Example: `TableFormatter`, `HtmlFormatter`, `JsonFormatter`

2. **variant** - Variant name
   - Example: `plain`, `bordered`, `markdown`, `Bootstrap`

3. **is_default** - Whether this is the default variant for its formatter
   - Example: `Yes`, `No`

4. **category** - Format category
   - Example: `Visual`, `Data`, `Markup`, `Query`, `Logging`

## Build & Dependencies

5. **feature_flag** - Cargo feature required
   - Example: `default`, `format_json`, `format_html`, `format_sql`

6. **runtime_deps** - Runtime dependencies
   - Example: `None`, `serde`, `serde+serde_json`, `serde+serde_yaml`

7. **zero_dependency** - Whether variant needs zero external crates
   - Example: `Yes`, `No`

## Character Set & Encoding

8. **charset** - Character encoding
   - Example: `ASCII`, `Unicode`, `UTF-8`

9. **border_charset** - Character set for borders specifically
   - Example: `ASCII`, `Unicode`, `None`

10. **requires_unicode_terminal** - Whether terminal must support Unicode
    - Example: `Yes`, `No`

11. **supports_ansi_colors** - Whether ANSI color codes are supported
    - Example: `Yes`, `No`

## Visual Structure

12. **has_borders** - Border presence
    - Example: `Yes`, `No`, `Partial`

13. **border_style** - Border rendering
    - Example: `None`, `ASCII-Pipes`, `ASCII-Grid`, `Unicode-Box`, `Markdown`

14. **column_separator** - What separates columns
    - Example: `Spaces`, `Pipes |`, `Commas ,`, `Tabs \t`, `None`

15. **row_separator** - What separates rows
    - Example: `Newline`, `Dashes`, `Grid-Lines`, `None`

16. **header_separator** - Header separator style
    - Example: `None`, `Dashes`, `ASCII-Grid`, `Unicode`, `Markdown`

17. **outer_padding** - Padding at table edges
    - Example: `Yes`, `No`

18. **inner_padding** - Padding within cells (spaces)
    - Example: `0`, `1`, `2`

## Data Representation

19. **machine_parseable** - Designed for machine parsing
    - Example: `Yes`, `No`, `Partial`

20. **human_readable** - Designed for human reading
    - Example: `Yes`, `No`

21. **supports_hierarchical** - Can represent tree/nested data
    - Example: `Yes`, `No`

22. **supports_tabular** - Can represent rows/columns
    - Example: `Yes`, `No`

23. **preserves_structure** - Maintains data structure
    - Example: `Yes`, `No`, `Partial`

24. **supports_multiline_values** - Can handle newlines in values
    - Example: `Yes`, `No`, `Escaped`

## Output Characteristics

25. **output_compactness** - Space efficiency
    - Example: `Minimal`, `Compact`, `Standard`, `Rich`, `Verbose`

26. **visual_complexity** - Visual richness
    - Example: `Minimal`, `Simple`, `Standard`, `Rich`

27. **alignment** - Data alignment support
    - Example: `Left`, `Right`, `Both`, `None`

28. **column_alignment** - Column-aligned output
    - Example: `Yes`, `No`

## Usage Context

29. **primary_use_case** - Main purpose
    - Example: `CLI tools output`, `Database export`, `Documentation`, `Web display`

30. **terminal_optimized** - Designed for terminal display
    - Example: `Yes`, `No`, `Partial`

31. **file_export_suitable** - Good for file export
    - Example: `Yes`, `No`, `Primary`

32. **streaming_friendly** - Can be parsed line-by-line
    - Example: `Yes`, `No`

33. **grep_friendly** - Easy to search with grep/awk
    - Example: `Yes`, `No`

## Technical Details

34. **escaping_rules** - How special characters are escaped
    - Example: `None`, `Quotes`, `HTML-Entities`, `SQL-Quotes`, `Backslash`

35. **output_format** - MIME/format type
    - Example: `text/plain`, `application/json`, `text/html`, `text/csv`

36. **standards_compliance** - Follows standard
    - Example: `None`, `Markdown-GFM`, `SQL-ANSI`, `JSON-RFC8259`, `CSV-RFC4180`

37. **supports_custom_colors** - Formatter parameters for colors
    - Example: `Yes`, `No`

## API & Construction

38. **constructor** - How to create
    - Example: `TableConfig::plain()`, `HtmlVariant::Bootstrap`, `default`, `new()`

39. **config_type** - Config struct name
    - Example: `TableConfig`, `HtmlVariant`, `ExpandedConfig`, `None`

40. **customizable_parameters** - Number of customizable formatter parameters
    - Example: `0`, `5`, `15`, `20+`

41. **builder_pattern** - Uses fluent builder API
    - Example: `Yes`, `No`

## Performance & Size

42. **output_overhead** - Extra characters vs raw data
    - Example: `Minimal`, `Low`, `Medium`, `High`, `Very-High`

43. **memory_efficiency** - Memory usage pattern
    - Example: `Streaming`, `Buffered`, `Hybrid`

## Compatibility

44. **works_on_windows** - Windows console compatible
    - Example: `Yes`, `No`, `Partial`

45. **works_in_ci** - CI/CD environment friendly
    - Example: `Yes`, `No`

46. **copy_paste_friendly** - Easy to copy from terminal
    - Example: `Yes`, `No`, `Partial`

---

**Total: 46 attributes per variant**

Each variant descriptor file in `docs/variant/` contains all these attributes filled out with specific values.
