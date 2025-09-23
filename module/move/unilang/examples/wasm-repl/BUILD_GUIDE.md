# 🔨 WebAssembly Build Guide

Complete guide for building and deploying the UniLang WebAssembly REPL.

## 📋 Prerequisites

### Required Tools

1. **Rust with WebAssembly target**
   ```bash
   # Install Rust (if not already installed)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Add WebAssembly target
   rustup target add wasm32-unknown-unknown
   ```

2. **wasm-pack (Optional but Recommended)**
   ```bash
   # Install wasm-pack for optimized builds
   curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
   ```

3. **Local Web Server**
   ```bash
   # Python (usually pre-installed)
   python3 -m http.server --version
   
   # Or Node.js
   npx serve --version
   ```

## 🚀 Build Process

### Method 1: Using Cargo (Manual)

```bash
# Navigate to the WASM REPL directory
cd examples/wasm-repl

# Build for WebAssembly (release mode for optimal size)
cargo build --target wasm32-unknown-unknown --release

# The WASM binary will be generated at:
# target/wasm32-unknown-unknown/release/unilang_wasm_repl.wasm
```

**Generated Files:**
- `target/wasm32-unknown-unknown/release/unilang_wasm_repl.wasm` (~2.3MB)

### Method 2: Using wasm-pack (Recommended)

```bash
# Navigate to the WASM REPL directory  
cd examples/wasm-repl

# Build with wasm-pack (automatically generates JS bindings)
wasm-pack build --target web --release

# Generated files will be in pkg/ directory:
# - pkg/unilang_wasm_repl.js (JS bindings)
# - pkg/unilang_wasm_repl_bg.wasm (WASM binary)
# - pkg/unilang_wasm_repl.d.ts (TypeScript definitions)
```

**Generated Files:**
- `pkg/unilang_wasm_repl.js` - JavaScript bindings
- `pkg/unilang_wasm_repl_bg.wasm` - Optimized WASM binary
- `pkg/unilang_wasm_repl.d.ts` - TypeScript definitions
- `pkg/package.json` - NPM package metadata

## 📊 Build Optimization

### Size Optimization

The `Cargo.toml` is already configured for optimal WASM builds:

```toml
[profile.release]
# Optimize for small code size in WebAssembly
opt-level = "s"      # Optimize for size
lto = true          # Link-time optimization
```

### Features Configuration

```toml
# WebAssembly-compatible features only
default = ["console_error_panic_hook", "wee_alloc"]

# Memory allocator optimized for WASM
wee_alloc = { version = "0.4", optional = true }

# Better error messages in development
console_error_panic_hook = { version = "0.1", optional = true }
```

## 🧪 Testing the Build

### 1. Run Automated Tests

```bash
# Run the comprehensive test suite
./test_runner.sh

# Or run tests manually:
cargo test --tests                    # Native tests
wasm-pack test --chrome --headless   # WASM tests (requires wasm-pack)
```

### 2. Serve the Application

```bash
# Navigate to the web directory
cd www

# Start a local web server (Python)
python3 -m http.server 8000

# Or using Node.js
npx serve . -p 8000

# Open in browser
open http://localhost:8000
```

### 3. Verify Functionality

In the browser console, you should see:
```
UniLang WASM REPL initialized successfully! 🚀
```

Test these commands:
- `.help` - Show available commands
- `.demo.echo Hello World` - Echo text
- `.calc.add 5 3` - Simple calculator

## 📈 Performance Metrics

### Build Size Analysis

**Release Build (cargo):**
- WASM Binary: ~2.3MB
- Includes full unilang framework
- SIMD-optimized parsing when supported

**Optimized Build (wasm-pack):**
- WASM Binary: ~800KB-1.2MB (compressed)
- JavaScript Bindings: ~50KB
- TypeScript Definitions: ~10KB

### Runtime Performance

- **Cold Start**: ~100-200ms (first command)
- **Command Execution**: <1ms (subsequent commands)  
- **Memory Usage**: ~5-10MB (including JavaScript heap)
- **Bundle Load Time**: ~50-100ms (depending on network)

## 🔧 Troubleshooting

### Common Build Issues

1. **Missing WebAssembly target**
   ```bash
   error[E0463]: can't find crate for `std`
   
   # Solution:
   rustup target add wasm32-unknown-unknown
   ```

2. **wasm-pack not found**
   ```bash
   # Install wasm-pack:
   curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
   ```

3. **Large binary size**
   - Ensure `opt-level = "s"` in `[profile.release]`
   - Enable `lto = true` for link-time optimization
   - Consider disabling debug symbols: `debug = false`

### Browser Compatibility

**Supported Browsers:**
- ✅ Chrome 67+ 
- ✅ Firefox 61+
- ✅ Safari 11.1+
- ✅ Edge 79+

**Required Features:**
- WebAssembly support
- ES6 modules
- Fetch API

### Development Tips

1. **Debug Build for Development**
   ```bash
   # Faster builds, larger size, better error messages
   cargo build --target wasm32-unknown-unknown
   ```

2. **Enable Console Logging**
   ```rust
   // In browser console
   console.log("WASM log:", wasmModule.log("debug message"));
   ```

3. **Inspect Generated Code**
   ```bash
   # View the generated JavaScript bindings
   cat pkg/unilang_wasm_repl.js
   
   # Inspect WASM binary info
   wasm-objdump -h pkg/unilang_wasm_repl_bg.wasm
   ```

## 🚢 Deployment

### Static Site Deployment

The built application can be deployed to any static hosting service:

1. **GitHub Pages**
2. **Netlify**
3. **Vercel**
4. **AWS S3 + CloudFront**
5. **Any web server capable of serving static files**

### Files to Deploy

```
www/
├── index.html          # Main application
├── style.css          # Styles
├── bootstrap.js       # WASM loader
└── pkg/               # Generated WASM package
    ├── unilang_wasm_repl.js
    ├── unilang_wasm_repl_bg.wasm
    └── package.json
```

### CORS Considerations

Ensure your web server serves WASM files with correct MIME type:

```apache
# .htaccess for Apache
AddType application/wasm .wasm
```

```nginx
# nginx.conf
location ~* \.wasm$ {
    add_header Content-Type application/wasm;
}
```

## 📚 Additional Resources

- [WebAssembly Official Documentation](https://webassembly.org/)
- [wasm-pack Guide](https://rustwasm.github.io/wasm-pack/)
- [Rust and WebAssembly Book](https://rustwasm.github.io/book/)
- [UniLang Framework Documentation](../../readme.md)

---

🎉 **Your UniLang WebAssembly REPL is now ready for production deployment!**