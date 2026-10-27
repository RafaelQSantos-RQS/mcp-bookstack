# Proposal

## Why

BookStack is a widely used knowledge management and documentation platform. Integrating BookStack with AI assistants (LLMs) requires a standard interface conforming to the Model Context Protocol (MCP).
Currently, this repository has no MCP implementation. This change introduces an initial, robust, read-only MCP server in Rust, enabling AI agents to search, explore, and read content stored in BookStack instances efficiently without risking unintended modifications.

## What Changes

- Add a typed HTTP client for BookStack's REST API targeting search and read endpoints.
- Support BookStack authentication via API token (`Authorization: Token <token_id>:<token_secret>`).
- Implement an MCP server using `rmcp` communicating over `stdio`.
- Expose read-only MCP tools:
  - `search_content`: Full-text search across shelves, books, chapters, and pages.
  - `read_page`: Read page contents in Markdown format (via `/api/pages/{id}/export/markdown` with fallback).
  - `list_books`: List books with pagination.
  - `get_book_details`: Retrieve table of contents and chapter/page hierarchy for a book.
  - `list_shelves`: List bookshelves.
  - `read_chapter`: Retrieve chapter details and list of pages in the chapter.
- Configure safe logging to `stderr` to preserve stdout for JSON-RPC transport.

## Capabilities

### New Capabilities
- `bookstack-mcp`: Exposes BookStack search and reading tools to MCP clients over stdio transport.

### Modified Capabilities
<!-- None -->

## Impact

- Affected dependencies: adds `rmcp`, `tokio`, `reqwest`, `serde`, `serde_json`, `schemars`, `tracing`, `tracing-subscriber`, `anyhow`.
- Affected files: `Cargo.toml`, `src/main.rs`, `src/config.rs`, `src/client.rs`, `src/models.rs`, `src/server.rs`.
- System requirements: requires environment variables `BOOKSTACK_URL`, `BOOKSTACK_TOKEN_ID`, `BOOKSTACK_TOKEN_SECRET`.
