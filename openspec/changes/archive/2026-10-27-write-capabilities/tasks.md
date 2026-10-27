# Tasks

## 1. Configuration & Safety Guard

- [x] 1.1 Add `allow_delete: bool` to `Config` in `src/config.rs`, parse `BOOKSTACK_ALLOW_DELETE` env variable, and verify with `cargo test --test test_config`

## 2. Data Models

- [x] 2.1 Add `TagInput` and mutation payloads (`CreatePagePayload`, `UpdatePagePayload`, `CreateChapterPayload`, `UpdateChapterPayload`, `CreateBookPayload`, `UpdateBookPayload`, `CreateShelfPayload`, `UpdateShelfPayload`) to `src/models.rs` and verify with `cargo test --test test_models`

## 3. Client API Mutation Methods

- [x] 3.1 Implement page mutation methods (`create_page`, `update_page`, `delete_page`) in `src/client.rs` and verify with mock server tests in `tests/test_client.rs`
- [x] 3.2 Implement chapter mutation methods (`create_chapter`, `update_chapter`, `delete_chapter`) in `src/client.rs` and verify in `tests/test_client.rs`
- [x] 3.3 Implement book mutation methods (`create_book`, `update_book`, `delete_book`) in `src/client.rs` and verify in `tests/test_client.rs`
- [x] 3.4 Implement shelf mutation methods (`create_shelf`, `update_shelf`, `delete_shelf`) in `src/client.rs` and verify in `tests/test_client.rs`

## 4. MCP Server Tools & Guardrails

- [x] 4.1 Implement portable schema generator helpers (`anyof_nullable_*`) in `src/server.rs`
- [x] 4.2 Implement and register page tools (`create_page`, `update_page`, `delete_page`) in `src/server.rs` with dual deletion guard and verify in `tests/test_server.rs`
- [x] 4.3 Implement and register chapter tools (`create_chapter`, `update_chapter`, `delete_chapter`) in `src/server.rs` with dual deletion guard and verify in `tests/test_server.rs`
- [x] 4.4 Implement and register book tools (`create_book`, `update_book`, `delete_book`) in `src/server.rs` with dual deletion guard and verify in `tests/test_server.rs`
- [x] 4.5 Implement and register shelf tools (`create_shelf`, `update_shelf`, `delete_shelf`) in `src/server.rs` with dual deletion guard and verify in `tests/test_server.rs`

## 5. Integration Verification

- [x] 5.1 Run full test suite and clippy checks (`cargo test && cargo clippy --all-targets -- -D warnings`) to verify 18 tools and clean builds
- [x] 5.2 Validate OpenSpec change integrity with `openspec validate write-capabilities`
