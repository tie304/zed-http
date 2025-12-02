# Installation

## From Zed Extension Registry

1. Open Zed
2. Open the extensions panel (`cmd+shift+x`)
3. Search for "http"
4. Click Install

## Manual Installation

### Building from Source

```bash
# Clone the repository
git clone https://github.com/anthropics/zed-http.git
cd zed-http

# Build the extension
cargo build --release

# Build the LSP (optional)
cargo build -p http-lsp --release
```

### Installing the Extension

Copy the built extension to Zed's extensions directory:

```bash
# macOS
cp -r . ~/Library/Application\ Support/Zed/extensions/http/

# Linux
cp -r . ~/.config/zed/extensions/http/
```

## Setting Up HTTP Execution

To execute HTTP requests, you need to configure a task runner.

### Option 1: Using httpYac

1. Install httpYac:
   ```bash
   npm install -g httpyac
   ```

2. Create `.zed/tasks.json` in your project:
   ```json
   [
     {
       "label": "Send HTTP Request",
       "command": "httpyac",
       "args": ["send", "--line", "$ZED_ROW", "$ZED_FILE"],
       "tags": ["http-request"]
     }
   ]
   ```

### Option 2: Using curl

Create `.zed/tasks.json`:
```json
[
  {
    "label": "Send HTTP Request (curl)",
    "command": "bash",
    "args": ["-c", "curl -v $(sed -n '${ZED_ROW}p' $ZED_FILE)"],
    "tags": ["http-request"]
  }
]
```

### Option 3: Using the Built-in LSP

Configure the LSP binary path in Zed settings:

```json
{
  "lsp": {
    "http-lsp": {
      "binary": {
        "path": "/path/to/http-lsp"
      }
    }
  }
}
```

> **Note**: Code lens buttons (Send, Headers, Save) require Zed to implement code lens support. Currently pending [Issue #11565](https://github.com/zed-industries/zed/issues/11565).
