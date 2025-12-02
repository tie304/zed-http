# http extension for Zed

## Overview
The `zed-http` extension provides robust syntax highlighting and an integrated HTTP client for `.http` files in the Zed editor, enhancing the development and testing of HTTP requests directly within the editor. This extension aims to replicate and eventually expand upon the functionality similar to the HTTP request capabilities seen in JetBrains editors, as described [here](https://github.com/JetBrains/http-request-in-editor-spec/blob/master/spec.md).

## Features
- Syntax highlighting for HTTP methods, headers, URLs, and bodies
- Supports standard HTTP methods such as GET, POST, PUT, DELETE, PATCH, and OPTIONS
- **Built-in HTTP client** - Execute requests directly without external tools
- **Inlay hints** - Visual buttons displayed inline: `▶ Send | 👁 Show | 💾 Save | ◉ Headers`
- **Code actions** - Access commands via `Ctrl+.` / `Cmd+.`
- **Hover info** - View request details and cached response status on hover
- Response caching for quick re-display

## Setup

### Required Settings

Add the following to your Zed settings (`.zed/settings.json` or global settings):

```json
{
  "languages": {
    "http": {
      "inlay_hints": {
        "enabled": true
      }
    }
  }
}
```

This enables the inline action buttons for HTTP requests.

## Usage

### Built-in HTTP Client

The extension includes a built-in Language Server that provides interactive commands for HTTP requests.

#### Available Commands

| Command | Description | Access |
|---------|-------------|--------|
| **▶ Send** | Execute the HTTP request and display response | `Ctrl+.` / `Cmd+.` |
| **👁 Show** | Display cached response (without re-executing) | `Ctrl+.` / `Cmd+.` |
| **💾 Save** | Execute and save response to file | `Ctrl+.` / `Cmd+.` |
| **◉ Headers** | Show only response headers | `Ctrl+.` / `Cmd+.` |

#### How to Use

1. Place your cursor anywhere on an HTTP request block
2. Press `Ctrl+.` (Windows/Linux) or `Cmd+.` (macOS)
3. Select the desired action from the menu

You can also hover over a request line to see request info and available actions.

### Alternative: External Tools (httpYac)

If you prefer using external tools, you can install [httpYac](https://httpyac.github.io/):

```bash
npm install -g httpyac
# OR
yarn global add httpyac
```

_For more information, visit the [httpYac documentation](https://httpyac.github.io/)._

Add the following task configuration to your `.zed/tasks.json` file:

```json
[
  {
    "label": "Run HTTP Request",
    "command": "httpyac",
    "args": [
      "send",
      "--line",
      "$ZED_ROW",
      "$ZED_FILE"
    ],
    "tags": [
      "http-request"
    ],
    "reveal": "always"
  },
  {
    "label": "Run All HTTP Requests",
    "command": "httpyac",
    "args": [
      "send",
      "$ZED_FILE"
    ],
    "tags": [
      "http-request"
    ],
    "reveal": "always"
  }
]
```
