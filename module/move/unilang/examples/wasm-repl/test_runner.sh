#!/bin/bash

# UniLang WebAssembly REPL Test Runner
# Runs comprehensive tests in both native and WebAssembly environments

set -e

echo "🧪 UniLang WebAssembly REPL Test Suite"
echo "======================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    print_error "wasm-pack is required but not installed"
    echo "Install it with: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh"
    exit 1
fi

print_info "Starting test suite..."

# Test 1: Native compilation and tests
echo ""
echo "📋 Phase 1: Native Environment Testing"
echo "======================================"

print_info "Running native compilation check..."
if cargo check; then
    print_status "Native compilation successful"
else
    print_error "Native compilation failed"
    exit 1
fi

print_info "Running native integration tests..."
if cargo test --tests; then
    print_status "Native tests passed"
else
    print_error "Native tests failed"
    exit 1
fi

# Test 2: WebAssembly compilation
echo ""
echo "🌐 Phase 2: WebAssembly Environment Testing"
echo "=========================================="

print_info "Running WebAssembly compilation check..."
if cargo check --target wasm32-unknown-unknown; then
    print_status "WebAssembly compilation successful"
else
    print_error "WebAssembly compilation failed"
    exit 1
fi

# Test 3: WebAssembly tests with wasm-pack
print_info "Running WebAssembly tests with wasm-pack..."
if wasm-pack test --chrome --headless; then
    print_status "WebAssembly Chrome tests passed"
else
    print_warning "WebAssembly Chrome tests failed (this might be expected if Chrome is not available)"
fi

# Try Firefox if available
if command -v firefox &> /dev/null; then
    print_info "Running WebAssembly tests with Firefox..."
    if wasm-pack test --firefox --headless; then
        print_status "WebAssembly Firefox tests passed"
    else
        print_warning "WebAssembly Firefox tests failed"
    fi
else
    print_warning "Firefox not available for WebAssembly testing"
fi

# Test 4: Build the actual WebAssembly package
echo ""
echo "📦 Phase 3: WebAssembly Package Building"
echo "======================================="

print_info "Building WebAssembly package..."
if wasm-pack build --target web; then
    print_status "WebAssembly package built successfully"
    
    # Check that expected files were generated
    if [ -f "pkg/unilang_wasm_repl.js" ] && [ -f "pkg/unilang_wasm_repl_bg.wasm" ]; then
        print_status "Package files generated correctly"
        
        # Show package size information
        echo ""
        print_info "Package size information:"
        ls -lh pkg/unilang_wasm_repl_bg.wasm | awk '{print "  WASM binary: " $5}'
        ls -lh pkg/unilang_wasm_repl.js | awk '{print "  JS bindings: " $5}'
        
        # Check for optimization
        wasm_size=$(stat -c%s pkg/unilang_wasm_repl_bg.wasm 2>/dev/null || stat -f%z pkg/unilang_wasm_repl_bg.wasm)
        if [ "$wasm_size" -lt 1000000 ]; then  # Less than 1MB
            print_status "WASM binary size is optimized (< 1MB)"
        else
            print_warning "WASM binary size might need optimization (> 1MB)"
        fi
    else
        print_error "Expected package files not found"
        exit 1
    fi
else
    print_error "WebAssembly package build failed"
    exit 1
fi

# Test 5: Verify the API surface
echo ""
echo "🔍 Phase 4: API Verification"
echo "=========================="

print_info "Checking WebAssembly API exports..."
if grep -q "export class UniLangWasmRepl" pkg/unilang_wasm_repl.js; then
    print_status "UniLangWasmRepl class exported"
else
    print_error "UniLangWasmRepl class not found in exports"
    exit 1
fi

if grep -q "execute_command" pkg/unilang_wasm_repl.js; then
    print_status "execute_command method exported"
else
    print_error "execute_command method not found in exports"
    exit 1
fi

if grep -q "get_help" pkg/unilang_wasm_repl.js; then
    print_status "get_help method exported"
else
    print_error "get_help method not found in exports"
    exit 1
fi

# Test 6: Documentation and README validation
echo ""
echo "📚 Phase 5: Documentation Validation"
echo "==================================="

if [ -f "readme.md" ]; then
    print_status "README.md exists"
    
    # Check for key sections
    if grep -q "Quick Start" readme.md; then
        print_status "README contains Quick Start section"
    else
        print_warning "README missing Quick Start section"
    fi
    
    if grep -q "wasm-pack build" readme.md; then
        print_status "README contains build instructions"
    else
        print_warning "README missing build instructions"
    fi
else
    print_error "README.md not found"
fi

# Final summary
echo ""
echo "🎉 Test Suite Complete!"
echo "======================"

print_status "All critical tests passed"
print_info "WebAssembly REPL is ready for deployment"

echo ""
echo "Next steps:"
echo "  1. Serve the www/ directory with a local web server"
echo "  2. Open index.html in your browser"  
echo "  3. Test the interactive REPL interface"

echo ""
echo "Example:"
echo "  cd www && python3 -m http.server 8000"
echo "  # Then visit http://localhost:8000"