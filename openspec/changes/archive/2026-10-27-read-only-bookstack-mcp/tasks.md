# Tasks

## 1. Setup & Client Core

- [x] 1.1 Add dependencies via cargo CLI (`rmcp`, `tokio`, `reqwest`, `serde`, `serde_json`, `schemars`, `tracing`, `tracing-subscriber`, `anyhow`) and verify `cargo check` passes
- [x] 1.2 Implement `src/config.rs` for environment variable parsing (`BOOKSTACK_URL`, `BOOKSTACK_TOKEN_ID`, `BOOKSTACK_TOKEN_SECRET`) and verify unit tests pass
- [x] 1.3 Implement `src/models.rs` with typed data structures for BookStack API responses and verify deserialization unit tests pass
- [x] 1.4 Implement `src/client.rs` with `BookStackClient` supporting search and content read methods, and verify client unit tests pass

## 2. MCP Server & stdio Transport

- [x] 2.1 Implement `src/server.rs` with `BookStackServer` exposing read-only MCP tools (`search_content`, `read_page`, `list_books`, `get_book_details`, `list_shelves`, `read_chapter`) and verify tool routing compiles
- [x] 2.2 Implement `src/main.rs` with safe stderr logging and stdio server loop, and verify server starts and answers initialize requests

## 3. Integration Verification & Documentation

- [x] 3.1 Add integration tests in `tests/test_server.rs` validating tool declarations and execution against mock BookStack responses, and verify `cargo test` passes
- [x] 3.2 Add `README.md` with client configuration instructions (Claude Desktop, Cursor, Antigravity) and verify examples run as documented
