# IDE Support for Former

This guide helps you get the best development experience when using Former with various IDEs and editors.

## Current State of IDE Support

Former is a procedural macro that generates code at compile time. IDE support for proc-macros varies by tool:

| IDE/Editor | Autocomplete | Go-to-Definition | Inline Errors | Rating |
|------------|--------------|------------------|---------------|--------|
| **rust-analyzer** (VS Code, Neovim, etc.) | ✅ Good | ⚠️ Partial | ✅ Good | ⭐⭐⭐⭐ |
| **IntelliJ IDEA** | ✅ Good | ✅ Good | ✅ Excellent | ⭐⭐⭐⭐⭐ |
| **CLion** | ✅ Good | ✅ Good | ✅ Excellent | ⭐⭐⭐⭐⭐ |
| **Vim/Neovim + CoC** | ⚠️ Limited | ❌ No | ⚠️ Limited | ⭐⭐ |
| **Emacs + lsp-mode** | ⚠️ Limited | ❌ No | ⚠️ Limited | ⭐⭐ |

---

## rust-analyzer (VS Code, Zed, Neovim)

### Status: ⭐⭐⭐⭐ (Good)

rust-analyzer has improved proc-macro support significantly. Former-generated methods are usually autocompleted correctly.

### Setup

1. **Install rust-analyzer**:
   ```bash
   # VS Code
   code --install-extension rust-lang.rust-analyzer

   # Neovim with Mason
   :MasonInstall rust-analyzer
   ```

2. **Enable proc-macro expansion** in settings:
   ```json
   // VS Code: settings.json
   {
     "rust-analyzer.procMacro.enable": true,
     "rust-analyzer.procMacro.attributes.enable": true
   }
   ```

3. **Reload window** after adding `#[derive(Former)]`

### Expected Behavior

✅ **Works**:
- Autocomplete for `.former()` method
- Autocomplete for field setters (`.field_name(value)`)
- Autocomplete for `.form()` finalization
- Type hints for setter parameters
- Inline errors for missing required fields

⚠️ **Partial**:
- Go-to-definition on `.former()` → shows derive macro, not generated code
- Subformer methods (`.end()`) may not autocomplete immediately

❌ **Doesn't Work**:
- Jump to generated setter implementation (limitation of proc-macros)

### Troubleshooting

**Problem**: Autocomplete doesn't show `.former()` or `.field_name()`

**Solutions**:
1. Save the file and wait 2-3 seconds for rust-analyzer to re-expand macros
2. Run "Rust Analyzer: Reload Workspace" command
3. Check Output panel → Rust Analyzer → look for proc-macro errors
4. Ensure your code compiles: `cargo check`
5. Clear rust-analyzer cache: `rm -rf target/.rust-analyzer-cache`

**Problem**: "Method not found" errors despite code compiling

**Cause**: rust-analyzer macro expansion failed or is outdated

**Solution**:
```bash
# Force macro re-expansion
touch src/lib.rs  # or main.rs
# Wait for rust-analyzer to catch up
```

---

## IntelliJ IDEA / CLion

### Status: ⭐⭐⭐⭐⭐ (Excellent)

JetBrains IDEs have the best proc-macro support for Former.

### Setup

1. Install the **Rust plugin**
2. Enable macro expansion:
   - Settings → Languages & Frameworks → Rust → Expand declarative macros: **ON**
   - Settings → Languages & Frameworks → Rust → Proc macro expansion: **ON**

### Expected Behavior

✅ **Works Perfectly**:
- Full autocomplete for all generated methods
- Parameter hints with types
- Go-to-definition (shows macro expansion output)
- Inline documentation for setters
- Error highlighting for type mismatches

### Tips

- **View expanded macro**: Right-click on `#[derive(Former)]` → "Show Macro Expansion"
- **Inspections**: Enable "Rust → Unresolved reference" for better error detection
- **Performance**: Macro expansion can slow down on large files - consider splitting

---

## Getting Better Autocomplete

### 1. Type Hints Help IDEs

rust-analyzer infers types better when you provide hints:

```rust
// ❌ Harder for IDE to infer
let config = Config::former()
  .host("localhost")  // IDE might not know .host() exists yet
  .form();

// ✅ Better: Type hint helps autocomplete
let config: Config = Config::former()
  .host("localhost")  // Autocomplete works immediately
  .form();
```

### 2. Incremental Builds

Former's code generation happens at compile time. The IDE needs a successful compilation to see generated methods.

**Best Practice**:
```bash
# Terminal in background
cargo watch -x check

# Or configure IDE to run cargo check on save
```

### 3. Use #[debug] for Inspection

Former supports a `#[debug]` attribute to print generated code:

```rust
#[derive(Former)]
#[debug]  // Prints generated code to console during compilation
pub struct MyStruct {
    field: String,
}
```

With `former_diagnostics_print_generated` feature:
```toml
[dependencies]
former = { version = "2.31", features = ["former_diagnostics_print_generated"] }
```

Then:
```bash
cargo build 2>&1 | less
# Search for "Generated code for MyStruct"
```

This shows exactly what methods are available!

---

## Workarounds for Limited IDE Support

### Manual Trait Bounds

If autocomplete fails, you can manually verify available methods by checking trait bounds:

```rust
use former::{ Former, FormerDefinition };

// This won't compile if MyStructFormer doesn't exist
fn assert_former_exists<T>()
where
  T: FormerDefinition
{
}

assert_former_exists::<MyStructFormerDefinition>();
```

### Consult Examples

When autocomplete fails, refer to:
- [examples/](../examples/) directory
- [spec.md](../spec.md) for generated API reference
- [advanced.md](../advanced.md) for subformer patterns

### Use `cargo expand`

Install and use `cargo-expand` to see generated code:

```bash
cargo install cargo-expand
cargo expand --lib > expanded.rs
# Search for your struct's Former implementation
```

---

## Requesting IDE Features

### rust-analyzer

If you encounter issues:

1. File an issue: https://github.com/rust-lang/rust-analyzer/issues
2. Mention "proc-macro expansion" and "derive Former"
3. Provide minimal reproduction case

### IntelliJ Rust Plugin

Feature requests: https://github.com/intellij-rust/intellij-rust/issues

---

## Future Improvements

### Potential Solutions

1. **Proc-macro hints**: Rust RFC for better IDE integration
   - Status: Under discussion
   - Impact: Would enable perfect autocomplete

2. **Former-specific LSP plugin**: Custom language server
   - Effort: High (6+ months development)
   - Benefit: Perfect autocomplete, go-to-definition

3. **Documentation generation**: Auto-generate docs for formers
   - Status: Planned for Former v3.0
   - Would add rustdoc comments to generated methods

---

## Tips by IDE

### VS Code

- Install "Error Lens" extension for inline error display
- Use "Rust Syntax" extension for better highlighting
- Set `"rust-analyzer.inlayHints.enable": true` for type hints

### Neovim

```lua
-- LSP config for better Former support
require('lspconfig').rust_analyzer.setup {
  settings = {
    ['rust-analyzer'] = {
      procMacro = {
        enable = true,
        attributes = {
          enable = true
        }
      }
    }
  }
}
```

### Vim (with CoC)

```json
// coc-settings.json
{
  "rust-analyzer.procMacro.enable": true
}
```

---

## Testing IDE Support

Create a test file to verify your IDE setup:

```rust
use former::Former;

#[derive(Former)]
pub struct TestIdeSupport {
    field_one: String,
    field_two: i32,
}

fn test() {
    let x = TestIdeSupport::former()
        .  // ← Autocomplete should show field_one, field_two, form
}
```

If autocomplete works here, your IDE is properly configured!

---

## FAQ

### Q: Why doesn't `.form()` autocomplete?

**A**: rust-analyzer might not have expanded the macro yet. Wait 2-3 seconds after saving.

### Q: Can I see generated code in the IDE?

**A**: IntelliJ: Yes ("Show Macro Expansion"). VS Code: Use `cargo expand` externally.

### Q: Why do I get "method not found" errors that don't appear in `cargo build`?

**A**: IDE and compiler run separately. The IDE's macro expansion might be stale. Reload workspace.

### Q: Will Former slow down my IDE?

**A**: Slightly. Proc-macro expansion adds ~100-200ms per file with Former derives. Noticeable on large files (1000+ lines).

---

**Contributing**: Found a way to improve IDE support? [Open an issue](https://github.com/Wandalen/wTools/issues)!

**Last Updated**: 2025-10-19
