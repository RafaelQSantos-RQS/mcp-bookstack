# BookStack MCP Server (`mcp-bookstack`)

[![Rust](https://img.shields.io/badge/rust-2024%20edition-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![MCP](https://img.shields.io/badge/MCP-1.0-green.svg)](https://modelcontextprotocol.io)
[![M8ven Score](https://m8ven.ai/badge/mcp/rafaelqsantos-rqs-mcp-bookstack-1tot2a?v=ec94dbe7c3305c7b79bb441c4d2453cb)](https://m8ven.ai/mcp/rafaelqsantos-rqs-mcp-bookstack-1tot2a?s=readme)

A high-performance [Model Context Protocol (MCP)](https://modelcontextprotocol.io) server for [BookStack](https://www.bookstackapp.com/), implemented in Rust.

`mcp-bookstack` bridges AI assistants (such as Claude Desktop, Cursor, Antigravity, and other MCP clients) with your BookStack documentation instance. It enables language models to search knowledge bases, explore hierarchy trees (shelves, books, chapters), read documentation pages rendered natively in clean Markdown, and safely create, update, and manage documentation with dual-guard deletion safety.

---

## Features

- **Full Documentation Management**: Tools to search, read, create, update, and manage pages, chapters, books, and bookshelves.
- **Dual-Layer Deletion Protection**: Deletion tools are guarded by both an environment variable (`BOOKSTACK_ALLOW_DELETE=true`) and an explicit `confirm: true` parameter, preventing accidental data loss.
- **Markdown-Prioritized Output**: Reads and writes pages using BookStack's native Markdown format. Delivers clean, structured Markdown directly to the LLM to minimize token consumption and maximize comprehension.
- **Robust Fallback Strategy**: If direct markdown export is unavailable, falls back gracefully to page raw content or sanitized text representations.
- **Strict Transport Isolation (`stdio`)**: Implements standard MCP stdio communication. Diagnostic logs and tracing outputs are strictly routed to `stderr`, preserving `stdout` exclusively for valid JSON-RPC frames.
- **Fast & Lightweight**: Built with Rust, `tokio`, `reqwest`, and `rmcp`, with minimal CPU and memory overhead.

---

## Available Tools

The server exposes 19 tools to MCP clients:

### Reading & Discovery Tools
| Tool Name | Parameters | Description |
|:---|:---|:---|
| `search_content` | `query` *(string, required)*<br>`page` *(integer, optional)*<br>`count` *(integer, optional)* | Performs a full-text search across BookStack shelves, books, chapters, and pages. Returns IDs, titles, slugs, and sanitized match snippets. |
| `read_page` | `page_id` *(integer, required)* | Retrieves the full contents of a specific BookStack page formatted in Markdown. |
| `list_books` | `page` *(integer, optional)*<br>`count` *(integer, optional)* | Lists books in the instance with their IDs, names, slugs, and short descriptions. |
| `get_book_details`| `book_id` *(integer, required)* | Retrieves detailed book information, including its table of contents, chapters, and nested pages. |
| `list_shelves` | `page` *(integer, optional)*<br>`count` *(integer, optional)* | Lists all bookshelves in the BookStack instance with IDs, names, and slugs. |
| `list_chapters` | `book_id` *(integer, optional)* | Lists chapters in the instance, optionally filtered by parent book ID. |
| `read_chapter` | `chapter_id` *(integer, required)* | Retrieves chapter details, description, and list of child pages within the chapter. |

### Content Creation & Update Tools
| Tool Name | Parameters | Description |
|:---|:---|:---|
| `create_page` | `name` *(string, required)*<br>`book_id` *(integer, optional)*<br>`chapter_id` *(integer, optional)*<br>`markdown` *(string, optional)*<br>`html` *(string, optional)*<br>`tags` *(array, optional)*<br>`priority` *(integer, optional)* | Creates a new page within a book or chapter. |
| `update_page` | `page_id` *(integer, required)*<br>`name` *(string, optional)*<br>`book_id` *(integer, optional)*<br>`chapter_id` *(integer, optional)*<br>`markdown` *(string, optional)*<br>`html` *(string, optional)*<br>`tags` *(array, optional)*<br>`priority` *(integer, optional)* | Updates an existing page's name, Markdown/HTML content, or parent location. |
| `create_chapter` | `book_id` *(integer, required)*<br>`name` *(string, required)*<br>`description` *(string, optional)*<br>`tags` *(array, optional)*<br>`priority` *(integer, optional)* | Creates a new chapter within a book. |
| `update_chapter` | `chapter_id` *(integer, required)*<br>`book_id` *(integer, optional)*<br>`name` *(string, optional)*<br>`description` *(string, optional)*<br>`tags` *(array, optional)*<br>`priority` *(integer, optional)* | Updates an existing chapter's name, description, or parent book. |
| `create_book` | `name` *(string, required)*<br>`description` *(string, optional)*<br>`tags` *(array, optional)* | Creates a new book in the instance. |
| `update_book` | `book_id` *(integer, required)*<br>`name` *(string, optional)*<br>`description` *(string, optional)*<br>`tags` *(array, optional)* | Updates an existing book's name or description. |
| `create_shelf` | `name` *(string, required)*<br>`description` *(string, optional)*<br>`books` *(array of int, optional)*<br>`tags` *(array, optional)* | Creates a new bookshelf with optional book assignments. |
| `update_shelf` | `shelf_id` *(integer, required)*<br>`name` *(string, optional)*<br>`description` *(string, optional)*<br>`books` *(array of int, optional)*<br>`tags` *(array, optional)* | Updates an existing shelf's name, description, or assigned books. |

### Guarded Deletion Tools
*Requires `BOOKSTACK_ALLOW_DELETE=true` in environment and `confirm: true` parameter.*

| Tool Name | Parameters | Description |
|:---|:---|:---|
| `delete_page` | `page_id` *(integer, required)*<br>`confirm` *(boolean, required)* | Deletes a page and sends it to the recycle bin. |
| `delete_chapter` | `chapter_id` *(integer, required)*<br>`confirm` *(boolean, required)* | Deletes a chapter and sends it to the recycle bin. |
| `delete_book` | `book_id` *(integer, required)*<br>`confirm` *(boolean, required)* | Deletes a book and sends it to the recycle bin. |
| `delete_shelf` | `shelf_id` *(integer, required)*<br>`confirm` *(boolean, required)* | Deletes a shelf and sends it to the recycle bin. |

---

## Example Tool Usages

Here are practical examples of tool calls sent by MCP clients:

### 1. Searching and Reading Content
```json
// search_content
{
  "query": "deployment",
  "count": 5
}

// read_page
{
  "page_id": 42
}
```

### 2. Creating a Page with Markdown
```json
// create_page (under a book)
{
  "book_id": 10,
  "name": "Production Deployment Guide",
  "markdown": "# Production Deployment\n\nSteps to deploy to production:\n1. Run migrations\n2. Deploy containers\n3. Verify health checks",
  "tags": [
    { "name": "Environment", "value": "Production" },
    { "name": "Type", "value": "Runbook" }
  ]
}

// create_page (under a chapter)
{
  "chapter_id": 20,
  "name": "Database Migrations",
  "markdown": "## Migration Procedures\n\nAlways take a backup before running migrations."
}
```

### 3. Updating an Existing Page
```json
// update_page
{
  "page_id": 42,
  "name": "Production Deployment Guide (v2)",
  "markdown": "# Production Deployment Guide (v2)\n\nUpdated with automated rollback instructions."
}
```

### 4. Managing Books, Chapters, and Shelves
```json
// create_book
{
  "name": "DevOps Handbook",
  "description": "Guides and playbooks for cloud operations"
}

// create_chapter
{
  "book_id": 10,
  "name": "Kubernetes Operations",
  "description": "Managing k8s clusters and workloads"
}

// create_shelf (grouping books by IDs)
{
  "name": "Engineering Knowledge Base",
  "description": "All core engineering handbooks and standards",
  "books": [10, 12, 15]
}
```

### 5. Safe Deletions (Dual-Guard)
*Requires `BOOKSTACK_ALLOW_DELETE=true` in environment and `confirm: true` in the tool call.*
```json
// delete_page
{
  "page_id": 42,
  "confirm": true
}
```

---

## Configuration

The server is configured via environment variables (which can also be placed in a `.env` file in the project root):

| Variable | Description | Example |
|:---|:---|:---|
| `BOOKSTACK_URL` | Base URL of your BookStack instance (trailing slashes are automatically trimmed, and `https://` is automatically prepended if omitted). | `https://wiki.example.com` or `wiki.example.com` |
| `BOOKSTACK_TOKEN_ID` | BookStack API Token ID. | `abcd1234efgh5678` |
| `BOOKSTACK_TOKEN_SECRET` | BookStack API Token Secret. | `ijkl9012mnop3456` |
| `BOOKSTACK_ALLOW_DELETE` | Optional safety flag to enable deletion tools (`true` or `false`, defaults to `false`). | `true` |

### Generating API Tokens in BookStack

1. Log in to your BookStack instance.
2. Click your user profile in the top-right corner and select **Edit Profile**.
3. Scroll down to the **API Tokens** section and click **Create Token**.
4. Provide a token name and set an expiry date.
5. Copy both the **Token ID** and **Token Secret** (the secret is only shown once).
6. Ensure the associated BookStack user has the necessary permissions (read, create, edit, delete) for the shelves, books, chapters, and pages you want the assistant to manage.

---

## Installation & How-To Guide

You do **not** need a Rust development environment to use `mcp-bookstack`. Precompiled, standalone binaries are built and published for each release.

### Step 1: Download the Precompiled Binary

Download the archive for your operating system and architecture from the [GitHub Releases](https://github.com/RafaelQSantos-RQS/mcp-bookstack/releases/latest):

| Platform | Architecture | Archive Asset |
|:---|:---|:---|
| **Linux** | `x86_64` | `mcp-bookstack-linux-x86_64.tar.gz` |
| **macOS** | Apple Silicon (`arm64` - M1/M2/M3/M4) | `mcp-bookstack-macos-arm64.tar.gz` |
| **Windows** | `x86_64` | `mcp-bookstack-windows-x86_64.zip` |

#### Quick Install via Terminal

**Linux (`x86_64`):**
```bash
# Download and unpack
curl -sL https://github.com/RafaelQSantos-RQS/mcp-bookstack/releases/latest/download/mcp-bookstack-linux-x86_64.tar.gz | tar -xz

# Make executable and move to PATH (optional)
chmod +x mcp-bookstack
sudo mv mcp-bookstack /usr/local/bin/
```

**macOS (Apple Silicon `arm64`):**
```bash
# Download and unpack
curl -sL https://github.com/RafaelQSantos-RQS/mcp-bookstack/releases/latest/download/mcp-bookstack-macos-arm64.tar.gz | tar -xz

# Make executable and move to PATH (optional)
chmod +x mcp-bookstack
sudo mv mcp-bookstack /usr/local/bin/
```

**Windows (PowerShell):**
```powershell
# Download and unpack into your user bin folder
Invoke-WebRequest -Uri "https://github.com/RafaelQSantos-RQS/mcp-bookstack/releases/latest/download/mcp-bookstack-windows-x86_64.zip" -OutFile "$env:TEMP\mcp-bookstack.zip"
Expand-Archive -Path "$env:TEMP\mcp-bookstack.zip" -DestinationPath "$HOME\bin" -Force
```

---

### Step 2: Configure in Your AI Agent / MCP Client

Once the binary is placed on your system (e.g. `/usr/local/bin/mcp-bookstack` or `C:\Users\username\bin\mcp-bookstack.exe`), configure your agent:

#### 1. AI Agents (`.agents` configuration)

For agents using a project-level `.agents/mcp_config.json` or global `~/.agents/mcp_config.json`:

```json
{
  "mcpServers": {
    "bookstack": {
      "command": "/usr/local/bin/mcp-bookstack",
      "env": {
        "BOOKSTACK_URL": "https://wiki.example.com",
        "BOOKSTACK_TOKEN_ID": "your_token_id_here",
        "BOOKSTACK_TOKEN_SECRET": "your_token_secret_here",
        "BOOKSTACK_ALLOW_DELETE": "false"
      }
    }
  }
}
```

#### 2. Claude Desktop

Add the server to your `claude_desktop_config.json`:

- **macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
- **Linux**: `~/.config/Claude/claude_desktop_config.json`
- **Windows**: `%APPDATA%\Claude\claude_desktop_config.json`

```json
{
  "mcpServers": {
    "bookstack": {
      "command": "/usr/local/bin/mcp-bookstack",
      "env": {
        "BOOKSTACK_URL": "https://wiki.example.com",
        "BOOKSTACK_TOKEN_ID": "your_token_id_here",
        "BOOKSTACK_TOKEN_SECRET": "your_token_secret_here",
        "BOOKSTACK_ALLOW_DELETE": "false"
      }
    }
  }
}
```
*Restart Claude Desktop after updating the file.*

#### 3. Cursor

Add the server in your user settings (`~/.cursor/mcp.json`) or project-level `.cursor/mcp.json`:

```json
{
  "mcpServers": {
    "bookstack": {
      "command": "/usr/local/bin/mcp-bookstack",
      "env": {
        "BOOKSTACK_URL": "https://wiki.example.com",
        "BOOKSTACK_TOKEN_ID": "your_token_id_here",
        "BOOKSTACK_TOKEN_SECRET": "your_token_secret_here",
        "BOOKSTACK_ALLOW_DELETE": "false"
      }
    }
  }
}
```

#### 4. Antigravity / Gemini CLI

Add the server to `~/.gemini/config/mcp_config.json`:

```json
{
  "mcpServers": {
    "bookstack": {
      "command": "/usr/local/bin/mcp-bookstack",
      "env": {
        "BOOKSTACK_URL": "https://wiki.example.com",
        "BOOKSTACK_TOKEN_ID": "your_token_id_here",
        "BOOKSTACK_TOKEN_SECRET": "your_token_secret_here",
        "BOOKSTACK_ALLOW_DELETE": "false"
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
  -e BOOKSTACK_ALLOW_DELETE="true" \
  ./target/release/mcp-bookstack
```

*(Note: If `https://` is omitted from `BOOKSTACK_URL`, `mcp-bookstack` will automatically prepend `https://`)*.

### Using `.env` file:

Create a `.env` file in the project root:

```env
BOOKSTACK_URL=https://wiki.example.com
BOOKSTACK_TOKEN_ID=your_token_id
BOOKSTACK_TOKEN_SECRET=your_token_secret
BOOKSTACK_ALLOW_DELETE=false
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
