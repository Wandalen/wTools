# Test Spec: Handlebars Renderer

- **Source**: `docs/feature/007_handlebars_renderer.md`
- **Prefix**: `FT-07`
- **Min cases**: 4

## Cases

| ID | Name | Status |
|----|------|--------|
| FT-07-1 | variable_substitution_replaces_placeholder | ⏳ |
| FT-07-2 | html_escaping_is_disabled | ⏳ |
| FT-07-3 | unknown_variable_renders_empty | ⏳ |
| FT-07-4 | renderer_is_reusable_across_calls | ⏳ |

---

### FT-07-1: variable_substitution_replaces_placeholder

- **Given:** A Handlebars renderer and a template `"Name: {{name}}"` with value map `{name: "Alice"}`
- **When:** `render` is called
- **Then:** The output is `"Name: Alice"`

---

### FT-07-2: html_escaping_is_disabled

- **Given:** A Handlebars renderer and a template `"{{value}}"` with value `"<script>"`
- **When:** `render` is called
- **Then:** The output is `"<script>"` (not `"&lt;script&gt;"`), confirming HTML escaping is off

---

### FT-07-3: unknown_variable_renders_empty

- **Given:** A Handlebars renderer and a template `"{{missing_var}}"` with an empty value map
- **When:** `render` is called
- **Then:** The output is an empty string or the rendering proceeds without error (per Handlebars default behavior)

---

### FT-07-4: renderer_is_reusable_across_calls

- **Given:** A single Handlebars renderer instance
- **When:** `render` is called twice with different template strings and value maps
- **Then:** Both calls return correct independent results with no state leakage between calls
