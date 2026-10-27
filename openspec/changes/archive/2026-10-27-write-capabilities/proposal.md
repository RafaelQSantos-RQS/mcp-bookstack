# Proposal: Write Capabilities & Content Management for BookStack MCP

## Why

The current `mcp-bookstack` server only supports read-only operations (searching, reading pages/chapters, and listing books/shelves). MCP-enabled AI assistants (Claude, Cursor, etc.) cannot currently draft documentation, update stale pages, reorganize chapters, or manage shelves directly in BookStack. Adding guarded write tools allows AI assistants to become active documentation partners while preventing accidental data loss through explicit safety controls.

## What Changes

- Add tools to create, update, and safely delete BookStack pages (`create_page`, `update_page`, `delete_page`).
- Add tools to create, update, and safely delete BookStack chapters (`create_chapter`, `update_chapter`, `delete_chapter`).
- Add tools to create, update, and safely delete BookStack books (`create_book`, `update_book`, `delete_book`).
- Add tools to create, update, and safely delete BookStack shelves (`create_shelf`, `update_shelf`, `delete_shelf`).
- Introduce a dual-layer deletion safety mechanism:
  - Global server flag `BOOKSTACK_ALLOW_DELETE` (defaults to `false`, rejecting deletion requests unless set to `true`).
  - Mandatory `confirm: bool` argument on every deletion tool.
- Add payload models and API client mutation methods supporting both Markdown (preferred) and HTML content formats.

## Capabilities

### New Capabilities
<!-- None: Extending existing bookstack-mcp capability -->

### Modified Capabilities
- `bookstack-mcp`: Expands the MCP server specification to include write tools for pages, chapters, books, and shelves, alongside deletion safety guardrails and configuration.

## Impact

- **Configuration (`src/config.rs`)**: Adds `allow_delete` field populated from `BOOKSTACK_ALLOW_DELETE`.
- **Data Models (`src/models.rs`)**: Adds request payload structs and schema definitions for mutation operations.
- **Client (`src/client.rs`)**: Adds HTTP POST, PUT, and DELETE methods for pages, chapters, books, and shelves.
- **Server (`src/server.rs`)**: Registers 12 new MCP tools, bringing the total exposed tools from 6 to 18.
- **Tests (`tests/`)**: Updates integration and unit tests to verify mutation handlers, safety guards, and portable JSON schema generation.
