# Development

## Project Structure

```
zed-http/
├── Cargo.toml                    # Workspace root
├── src/
│   └── http.rs                   # Zed extension entry point
├── http-lsp/                     # Language Server
│   ├── Cargo.toml
│   ├── build.rs                  # Compiles tree-sitter grammar
│   ├── tree-sitter-http/         # Grammar C files
│   │   ├── parser.c
│   │   └── tree_sitter/
│   │       ├── parser.h
│   │       ├── alloc.h
│   │       └── array.h
│   └── src/
│       ├── main.rs               # LSP server
│       ├── document.rs           # Document management
│       ├── parser.rs             # Tree-sitter parsing
│       ├── code_lens.rs          # Code lens provider
│       ├── http_client.rs        # HTTP execution
│       └── commands.rs           # Command handlers
├── languages/
│   └── http/
│       ├── config.toml           # Language config
│       ├── highlights.scm        # Syntax highlighting
│       ├── injections.scm        # Language injections
│       └── runnables.scm         # Runnable markers
├── extension.toml                # Extension metadata
└── test/
    └── test.http                 # Test file
```

## Building

### Extension Only

```bash
cargo build -p zed_http
```

### LSP Only

```bash
cargo build -p http-lsp
```

### Full Build

```bash
cargo build --workspace
```

### Release Build

```bash
cargo build --workspace --release
```

## Tree-Sitter Grammar

The extension uses the [rest-nvim/tree-sitter-http](https://github.com/rest-nvim/tree-sitter-http) grammar.

### Updating Grammar Files

```bash
cd http-lsp/tree-sitter-http

# Download latest parser.c
curl -sSL "https://raw.githubusercontent.com/rest-nvim/tree-sitter-http/main/src/parser.c" -o parser.c

# Download headers
cd tree_sitter
curl -sSL "https://raw.githubusercontent.com/rest-nvim/tree-sitter-http/main/src/tree_sitter/parser.h" -o parser.h
curl -sSL "https://raw.githubusercontent.com/rest-nvim/tree-sitter-http/main/src/tree_sitter/alloc.h" -o alloc.h
curl -sSL "https://raw.githubusercontent.com/rest-nvim/tree-sitter-http/main/src/tree_sitter/array.h" -o array.h
```

## Testing the LSP

### Manual Testing

```bash
# Build LSP
cargo build -p http-lsp

# Run with stdio
./target/debug/http-lsp
```

### Testing with a Client

Create a test script:

```bash
#!/bin/bash
echo 'Content-Length: 123

{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"capabilities":{}}}' | ./target/debug/http-lsp
```

## Tree-Sitter Queries

### Highlights (highlights.scm)

```scheme
; Methods
(method) @function.method

; URLs
(target_url) @string.url

; Headers
(header name: (name) @property)
(header value: (value) @string)
```

### Injections (injections.scm)

```scheme
; JSON body injection
((json_body) @injection.content
 (#set! injection.language "json"))
```

### Runnables (runnables.scm)

```scheme
(
  (request
    (method) @run
  ) @http-request
  (#set! tag http-request)
)
```

## LSP Architecture

```
┌─────────────────────────────────────────┐
│  http-lsp                               │
├─────────────────────────────────────────┤
│  main.rs     │ Server setup, routing    │
│  document.rs │ Document sync, storage   │
│  parser.rs   │ Tree-sitter parsing      │
│  code_lens.rs│ Code lens generation     │
│  commands.rs │ Command execution        │
│  http_client │ reqwest HTTP calls       │
└─────────────────────────────────────────┘
```

## LSP Capabilities

| Capability | Status |
|------------|--------|
| `textDocument/didOpen` | Implemented |
| `textDocument/didChange` | Implemented |
| `textDocument/didClose` | Implemented |
| `textDocument/codeLens` | Implemented (awaiting Zed support) |
| `workspace/executeCommand` | Implemented |

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run `cargo fmt` and `cargo clippy`
5. Submit a pull request

## Release Process

1. Update version in `Cargo.toml` and `extension.toml`
2. Build release binaries for all platforms:
   ```bash
   # Linux x86_64
   cargo build -p http-lsp --release --target x86_64-unknown-linux-gnu

   # macOS x86_64
   cargo build -p http-lsp --release --target x86_64-apple-darwin

   # macOS ARM
   cargo build -p http-lsp --release --target aarch64-apple-darwin

   # Windows
   cargo build -p http-lsp --release --target x86_64-pc-windows-msvc
   ```
3. Create GitHub release with binaries
4. Publish to Zed extension registry
