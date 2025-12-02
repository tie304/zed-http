# Configuration

## Language Configuration

The extension automatically associates with `.http` files. No additional configuration needed.

## LSP Configuration

Configure the HTTP LSP in your Zed settings (`~/.config/zed/settings.json`):

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

## Task Configuration

Create `.zed/tasks.json` in your project root:

```json
[
  {
    "label": "Send HTTP Request",
    "command": "httpyac",
    "args": ["send", "--line", "$ZED_ROW", "$ZED_FILE"],
    "tags": ["http-request"],
    "reveal": "always"
  },
  {
    "label": "Send All Requests",
    "command": "httpyac",
    "args": ["send", "$ZED_FILE"],
    "tags": ["http-request"]
  }
]
```

### Task Variables

| Variable | Description |
|----------|-------------|
| `$ZED_FILE` | Current file path |
| `$ZED_ROW` | Current line number |
| `$ZED_COLUMN` | Current column number |

## Syntax Highlighting

Highlighting is handled via tree-sitter queries. The following elements are highlighted:

| Element | Highlight Group |
|---------|-----------------|
| HTTP methods (GET, POST, etc.) | `@function.method` |
| URLs | `@string.url` |
| Header names | `@property` |
| Header values | `@string` |
| Status codes | `@constant.numeric` |
| HTTP version | `@keyword` |
| Variables | `@variable` |
| Comments | `@comment` |

## Language Injections

Request bodies automatically get proper syntax highlighting:

| Body Type | Injected Language |
|-----------|-------------------|
| JSON | `json` |
| XML | `xml` |
| GraphQL | `graphql` |

## HTTP File Format

### Request Separator

Use `###` to separate multiple requests:

```http
### First request
GET https://api.example.com/users

### Second request
POST https://api.example.com/users
Content-Type: application/json

{"name": "test"}
```

### Comments

```http
# This is a comment
// This is also a comment
### This separates requests AND is a comment
```

### Variables

```http
@hostname = api.example.com
@token = your-bearer-token

GET https://{{hostname}}/users
Authorization: Bearer {{token}}
```

### Environment Files

Create `.env` or `http-client.env.json` for environment-specific variables.
