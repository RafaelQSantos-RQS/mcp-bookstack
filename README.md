# BookStack MCP Server (`mcp-bookstack`)

[![Rust](https://img.shields.io/badge/rust-2024%20edition-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![MCP](https://img.shields.io/badge/MCP-1.0-green.svg)](https://modelcontextprotocol.io)

A high-performance, read-only [Model Context Protocol (MCP)](https://modelcontextprotocol.io) server for [BookStack](https://www.bookstackapp.com/), implemented in Rust.

`mcp-bookstack` bridges AI assistants (such as Claude Desktop, Cursor, Antigravity, and other MCP clients) with your BookStack documentation instance. It enables language models to search knowledge bases, explore hierarchy trees (shelves, books, chapters), and read documentation pages rendered natively in clean Markdown.

---

## Features

- **Read-Only by Design**: Only exposes search and read tools. Zero risk of modifying, overwriting, or corrupting existing BookStack documentation or WYSIWYG editor content.
- **Markdown-Prioritized Output**: Reads pages using BookStack's native `/export/markdown` endpoint. Delivers clean, structured Markdown directly to the LLM to minimize token consumption and maximize comprehension.
- **Robust Fallback Strategy**: If direct markdown export is unavailable, falls back gracefully to page raw content or sanitized text representations.
- **Strict Transport Isolation (`stdio`)**: Implements standard MCP stdio communication. Diagnostic logs and tracing outputs are strictly routed to `stderr`, preserving `stdout` exclusively for valid JSON-RPC frames.
- **Fast & Lightweight**: Built with Rust, `tokio`, `reqwest`, and `rmcp`, with minimal CPU and memory overhead.

---

## Available Tools

The server exposes 6 read-only tools to MCP clients:

| Tool Name | Parameters | Description |
|:---|:---|:---|
| `search_content` | `query` *(string, required)*<br>`page` *(integer, optional)*<br>`count` *(integer, optional)* | Performs a full-text search across BookStack shelves, books, chapters, and pages. Returns IDs, titles, slugs, and sanitized match snippets. |
| `read_page` | `page_id` *(integer, required)* | Retrieves the full contents of a specific BookStack page formatted in Markdown. |
| `list_books` | `page` *(integer, optional)*<br>`count` *(integer, optional)* | Lists books in the instance with their IDs, names, slugs, and short descriptions. |
| `get_book_details`| `book_id` *(integer, required)* | Retrieves detailed book information, including its table of contents, chapters, and nested pages. |
| `list_shelves` | `page` *(integer, optional)*<br>`count` *(integer, optional)* | Lists all bookshelves in the BookStack instance with IDs, names, and slugs. |
| `read_chapter` | `chapter_id` *(integer, required)* | Retrieves chapter details, description, and list of child pages within the chapter. |

---

## Configuration

The server is configured via environment variables (which can also be placed in a `.env` file in the project root):

| Variable | Description | Example |
|:---|:---|:---|
| `BOOKSTACK_URL` | Base URL of your BookStack instance (trailing slashes are automatically trimmed, and `https://` is automatically prepended if omitted). | `https://wiki.example.com` or `wiki.example.com` |
| `BOOKSTACK_TOKEN_ID` | BookStack API Token ID. | `abcd1234efgh5678` |
| `BOOKSTACK_TOKEN_SECRET` | BookStack API Token Secret. | `ijkl9012mnop3456` |

### Generating API Tokens in BookStack

1. Log in to your BookStack instance.
2. Click your user profile in the top-right corner and select **Edit Profile**.
3. Scroll down to the **API Tokens** section and click **Create Token**.
4. Provide a token name and set an expiry date.
5. Copy both the **Token ID** and **Token Secret** (the secret is only shown once).
6. Ensure the associated BookStack user has the necessary read permissions for the shelves, books, and pages you want the assistant to access.

---

## Client Integration

### 1. Claude Desktop

Add the server to your `claude_desktop_config.json`:

- **macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
- **Linux**: `~/.config/Claude/claude_desktop_config.json`
- **Windows**: `%APPDATA%\Claude\claude_desktop_config.json`

```json
{
  "mcpServers": {
    "bookstack": {
      "command": "/absolute/path/to/mcp-bookstack",
      "env": {
        "BOOKSTACK_URL": "https://wiki.example.com",
        "BOOKSTACK_TOKEN_ID": "your_token_id_here",
        "BOOKSTACK_TOKEN_SECRET": "your_token_secret_here"
      }
    }
  }
}
```

Restart Claude Desktop after updating the file.

---

### 2. Cursor

Add the server in your user settings (`~/.cursor/mcp.json`) or project-level `.cursor/mcp.json`:

```json
{
  "mcpServers": {
    "bookstack": {
      "command": "/absolute/path/to/mcp-bookstack",
      "env": {
        "BOOKSTACK_URL": "https://wiki.example.com",
        "BOOKSTACK_TOKEN_ID": "your_token_id_here",
        "BOOKSTACK_TOKEN_SECRET": "your_token_secret_here"
      }
    }
  }
}
```

---

### 3. Antigravity / Gemini CLI

Add the server configuration to `~/.gemini/config/mcp_config.json`:

```json
{
  "mcpServers": {
    "bookstack": {
      "command": "/absolute/path/to/mcp-bookstack/target/release/mcp-bookstack",
      "env": {
        "BOOKSTACK_URL": "https://wiki.example.com",
        "BOOKSTACK_TOKEN_ID": "your_token_id_here",
        "BOOKSTACK_TOKEN_SECRET": "your_token_secret_here"
      }
    }
  }
}
```

---

## Building & Testing

### Prerequisites

- [Rust Toolchain](https://rustup.rs/) (edition 2024 compatible, Rust 1.85+)
- `cargo` package manager

### Build

To compile a release binary:

```bash
cargo build --release
```

The compiled binary will be located at:
```bash
./target/release/mcp-bookstack
```

### Running Tests

Run the full test suite, including unit tests and integration tests:

```bash
cargo test
```

Run only integration tests:

```bash
cargo test --test test_server
```

---

## Inspecting and Testing Interactively

You can test and inspect the MCP server using the official `@modelcontextprotocol/inspector`.

> [!IMPORTANT]
> The MCP Inspector spawns the child process in a clean environment by default. You **must** pass environment variables using the `-e` flag (or create a `.env` file in the project root).

### Using `-e` flags (Recommended):

```bash
npx @modelcontextprotocol/inspector \
  -e BOOKSTACK_URL="https://wiki.example.com" \
  -e BOOKSTACK_TOKEN_ID="your_token_id" \
  -e BOOKSTACK_TOKEN_SECRET="your_token_secret" \
  ./target/release/mcp-bookstack
```

*(Note: If `https://` is omitted from `BOOKSTACK_URL`, `mcp-bookstack` will automatically prepend `https://`)*.

### Using `.env` file:

Create a `.env` file in the project root:

```env
BOOKSTACK_URL=https://wiki.example.com
BOOKSTACK_TOKEN_ID=your_token_id
BOOKSTACK_TOKEN_SECRET=your_token_secret
```

And run:

```bash
npx @modelcontextprotocol/inspector ./target/release/mcp-bookstack
```

The inspector launches a local web interface where you can:
- Inspect registered tool schemas (`search_content`, `read_page`, etc.).
- Execute tool calls with custom parameters.
- View raw JSON-RPC request and response payloads.

---

## Troubleshooting

- **Server exits immediately on startup**:
  Verify that all three required environment variables (`BOOKSTACK_URL`, `BOOKSTACK_TOKEN_ID`, and `BOOKSTACK_TOKEN_SECRET`) are exported.
- **Protocol parse errors in MCP clients**:
  Ensure that no wrapper scripts print to `stdout`. All diagnostics must go to `stderr`. `mcp-bookstack` automatically sends all internal `tracing` logs to `stderr`.
- **Permission Denied / 403 Forbidden**:
  Verify in BookStack that the API token's user role has permission to access the requested books or shelves.
- **Self-signed SSL certificates**:
  For development instances using local self-signed certificates, ensure standard OS trust stores are updated, or route traffic through a local proxy.

---

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
