# 🚀 UniLang WebAssembly REPL

A fully functional web-based REPL (Read-Eval-Print Loop) for the UniLang command framework, powered by WebAssembly.

## ✨ Features

- **WebAssembly Performance**: Native Rust performance in the browser
- **Interactive REPL**: Real-time command execution and feedback  
- **Cross-Platform Validation**: Works consistently across all platforms
- **SIMD Optimizations**: Fast parsing and tokenization (when available)
- **Dynamic Command Loading**: Load custom commands via JSON
- **Modern UI**: Dark theme with syntax highlighting
- **Command History**: Navigate previous commands with arrow keys

## 🏗️ Project Structure

```
examples/wasm-repl/
├── Cargo.toml          # WebAssembly-specific dependencies
├── src/
│   └── lib.rs          # Rust/WASM bridge implementation
├── www/                # Web frontend
│   ├── index.html      # Main HTML interface
│   ├── style.css       # Modern dark theme styles
│   └── bootstrap.js    # JavaScript WASM loader
├── pkg/                # Generated WASM bindings (after build)
└── readme.md          # This file
```

## 🚀 Quick Start

### Prerequisites

- [wasm-pack](https://rustwasm.github.io/wasm-pack/) for building WebAssembly
- A local web server for development

### Building

1. **Build the WebAssembly module:**
   ```bash
   cd examples/wasm-repl
   wasm-pack build --target web
   ```

2. **Serve the web interface:**
   ```bash
   # Using Python (most systems)
   cd www
   python3 -m http.server 8000
   
   # Using Node.js
   npx serve .
   
   # Using any local web server
   ```

3. **Open in browser:**
   Navigate to `http://localhost:8000`

## 💡 Usage Examples

### Basic Commands

```bash
# Get help
.help

# Echo text (demo command)
.demo.echo Hello, WebAssembly!

# Simple calculator
.calc.add 42 58
```

### Loading Custom Commands

Use the JSON command loader in the sidebar:

```json
{
  "commands": [
    {
      "name": "greet",
      "namespace": ["demo"],
      "hint": "Greet someone",
      "description": "A friendly greeting command",
      "arguments": [
        {
          "name": "name",
          "kind": "String",
          "hint": "Person's name",
          "description": "The name of the person to greet",
          "properties": {}
        }
      ],
      "properties": {},
      "routine": "demo_greet_routine"
    }
  ]
}
```

## 🎯 WebAssembly Compatibility

This example demonstrates how UniLang works in WebAssembly environments:

- **Conditional Compilation**: Filesystem operations are disabled for WASM targets
- **Minimal Dependencies**: Uses only web-compatible dependencies
- **Optimized Build**: Small binary size with `opt-level = "s"` and LTO
- **Memory Management**: Uses `wee_alloc` for reduced WASM binary size
- **Error Handling**: Proper panic hooks for debugging

## 🔧 Development

### Key Files

- **`src/lib.rs`**: Main Rust/WASM interface
- **`www/bootstrap.js`**: JavaScript bridge to WASM module
- **`Cargo.toml`**: WebAssembly-optimized dependencies

### WASM Features Used

- `wasm-bindgen` for Rust/JavaScript interop
- `web-sys` for DOM manipulation
- `js-sys` for JavaScript API access
- `console_error_panic_hook` for better debugging

### Building for Production

```bash
# Optimized release build
wasm-pack build --target web --release

# Check the generated bundle size
ls -lh pkg/
```

## 📊 Performance

The WebAssembly version provides significant performance benefits:

- **Parsing**: SIMD-optimized tokenization (when supported)
- **Validation**: Native Rust speed for type checking
- **Memory**: Efficient memory usage with custom allocator
- **Size**: Optimized binary size (~200KB compressed)

## 🧪 Testing

Run the WASM-specific tests:

```bash
# Test the WASM build
wasm-pack test --chrome --headless

# Test with different browsers
wasm-pack test --firefox --headless
wasm-pack test --safari --headless
```

## 📚 Learn More

- [UniLang Documentation](../../readme.md)
- [WebAssembly Book](https://rustwasm.github.io/book/)
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)
- [web-sys Reference](https://rustwasm.github.io/wasm-bindgen/web-sys/)

## 🚧 Known Limitations

- File system operations are not available (by design)
- Some native commands are disabled in WebAssembly mode
- Browser security restrictions apply to certain features

## 🤝 Contributing

This example demonstrates the full WebAssembly integration for UniLang. Feel free to extend it with additional features or use it as a template for your own WebAssembly applications.