# http extension for Zed

## Overview
The `zed-http` extension provides robust syntax highlighting and runnable HTTP requests for `.http` files in the Zed editor, enhancing the development and testing of HTTP requests directly within the editor. This extension aims to replicate and eventually expand upon the functionality similar to the HTTP request capabilities seen in JetBrains editors, as described [here](https://github.com/JetBrains/http-request-in-editor-spec/blob/master/spec.md).

## Features
- Syntax highlighting for HTTP methods, headers, URLs, and bodies
- Supports standard HTTP methods such as GET, POST, PUT, DELETE, PATCH, and OPTIONS
- Runnable HTTP requests directly from the editor
- Execute individual requests or all requests in a file

## Usage

### Running HTTP Requests

To execute HTTP requests from within Zed, you can choose to install [httpYac](https://httpyac.github.io/) (or any other http CLI tool):

```bash
npm install -g httpyac
# OR
yarn global add httpyac
```

_For more information, visit the [httpYac documentation](https://httpyac.github.io/)._

To connect the runnable queries to httpYac commands, add the following task configuration to your `.zed/tasks.json` file in your project:

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
