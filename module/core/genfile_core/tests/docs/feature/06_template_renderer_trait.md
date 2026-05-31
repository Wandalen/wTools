# Test Spec: Template Renderer Trait

- **Source**: `docs/feature/006_template_renderer_trait.md`
- **Prefix**: `FT-06`
- **Min cases**: 4

## Cases

| ID | Name | Status |
|----|------|--------|
| FT-06-1 | trait_render_returns_rendered_string | ⏳ |
| FT-06-2 | trait_render_fails_on_invalid_template | ⏳ |
| FT-06-3 | custom_renderer_can_implement_trait | ⏳ |
| FT-06-4 | renderer_is_swappable_in_holder | ⏳ |

---

### FT-06-1: trait_render_returns_rendered_string

- **Given:** A renderer implementing the template renderer trait and a valid template `"Hello {{name}}"` with values `{name: "World"}`
- **When:** `render` is called
- **Then:** The returned string is `"Hello World"`

---

### FT-06-2: trait_render_fails_on_invalid_template

- **Given:** A renderer and a template string containing invalid syntax
- **When:** `render` is called
- **Then:** An error is returned (not a panic); the error contains a description of the syntax problem

---

### FT-06-3: custom_renderer_can_implement_trait

- **Given:** A custom struct implementing the renderer trait that simply returns the template string unchanged
- **When:** The struct is used as the renderer type in a template holder
- **Then:** The code compiles and the custom render method is called during generation

---

### FT-06-4: renderer_is_swappable_in_holder

- **Given:** Two different renderer implementations of the same trait
- **When:** Each is used in an otherwise identical template holder
- **Then:** Both holders compile and produce output; each uses its respective renderer's logic
