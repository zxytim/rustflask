# RustFlask Examples

This directory contains various examples demonstrating the RustFlask web framework and its UTF-8 encoding fixes.

## Examples Available:

### UTF-8 Encoding Fixed ✅
1. **`fixed_beautiful_showcase.rs`** - Comprehensive demo with proper UTF-8 encoding
2. **`test_encoding.rs`** - Simple UTF-8 encoding test
3. **`combined_demo.rs`** - Complete feature demonstration with UTF-8

### Original Examples
4. **`showcase.rs`** - Basic feature demonstration
5. **`beautiful_showcase.rs`** - Beautiful HTML demo (may have encoding issues)
6. **`fancy_showcase.rs`** - Interactive playground demo
7. **`enhanced_example.rs`** - Enhanced functionality demo

## Key Features Demonstrated:

### ✅ UTF-8 Encoding Support
- All HTML responses now have `Content-Type: text/html; charset=utf-8`
- All text responses now have `Content-Type: text/plain; charset=utf-8`  
- All JSON responses now have `Content-Type: application/json; charset=utf-8`
- Proper Unicode character support in all responses

### 🌟 Core Framework Features
- Async/await performance with Tokio and Hyper
- HTTP method routing (GET, POST, etc.)
- URL parameter extraction with `{param}` syntax
- JSON serialization with Serde
- Configuration management
- Memory safety with Rust's type system

### 🎮 Interactive Features
- Beautiful HTML pages with emojis and Unicode
- Live dashboard and status pages
- Command-line examples with cURL
- Unicode character testing endpoints

## 🚀 How to Run Examples

### Quick Start Commands

```bash
# Modern tech magazine style (Recommended)
cargo run --example combined_demo      # http://localhost:8085

# Executive corporate style (Professional)
cargo run --example fixed_beautiful_showcase  # http://localhost:8084

# Simple UTF-8 encoding test
cargo run --example test_encoding      # http://localhost:8001

# Basic feature demonstration
cargo run --example showcase           # http://localhost:8081

# Interactive playground
cargo run --example fancy_showcase     # http://localhost:8082

# Enhanced functionality demo
cargo run --example enhanced_example   # http://localhost:8083
```

### After Starting an Example

Open your browser and navigate to the URLs shown above. For example:

```bash
# Start the example
cargo run --example combined_demo

# In another terminal, test endpoints:
# Test UTF-8 encoding
curl http://localhost:8085/utf8 | jq '.emojis[0]'

# Test Unicode text
curl http://localhost:8085/hello/世界

# Test multiple URL parameters
curl http://localhost:8085/users/42/posts/tutorial

# Test POST method
curl -X POST http://localhost:8085/echo

# View JSON responses
curl http://localhost:8085/json | jq '.'

# Test server status
curl http://localhost:8085/status

# Test configuration info
curl http://localhost:8085/config

# View HTML page (recommended in browser)
open http://localhost:8085/
```

### Testing UTF-8 Encoding
Once any example is running, you can test the UTF-8 encoding fix:

## Architecture:

The framework is built using:
- **Tokio** - Async runtime
- **Hyper** - HTTP server
- **Serde** - JSON serialization
- **Chrono** - Date/time handling
- **Rust** - Memory safety and performance