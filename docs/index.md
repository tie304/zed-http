# zed-http

A Zed editor extension for `.http` files with syntax highlighting, language injections, and runnable HTTP requests.

## Features

- **Syntax Highlighting** - Full highlighting for HTTP requests, headers, URLs, methods, and more
- **Language Injections** - JSON, XML, and GraphQL bodies get proper syntax highlighting
- **Runnable Requests** - Execute HTTP requests directly from the editor via Zed tasks
- **Language Server** - Built-in LSP with code lens support (pending Zed support)

## Quick Start

1. Install the extension from Zed's extension registry
2. Open any `.http` file
3. Use the run button or `cmd+shift+r` to execute requests

## File Format

The extension supports the [JetBrains HTTP Client](https://www.jetbrains.com/help/idea/http-client-in-product-code-editor.html) file format:

```http
### GET Request
GET https://api.example.com/users
Accept: application/json

### POST Request with JSON body
POST https://api.example.com/users
Content-Type: application/json

{
  "name": "John Doe",
  "email": "john@example.com"
}

### Request with headers
GET https://api.example.com/protected
Authorization: Bearer your-token-here
X-Custom-Header: value
```

## Documentation

- [Installation](installation.md)
- [Configuration](configuration.md)
- [Development](development.md)

## Requirements

- Zed editor (latest version recommended)
- For running requests: Configure a task runner (httpyac, curl, etc.)

## License

Apache-2.0
