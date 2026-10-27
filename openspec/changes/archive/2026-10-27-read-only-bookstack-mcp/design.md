# Design

## Context

The repository is a Rust package (`mcp-bookstack`). BookStack exposes a comprehensive REST API documented in `.tmp/docs.json`, authenticating via `Authorization: Token <id>:<secret>`.
This design specifies the internal architecture for a read-only MCP server connecting LLM hosts to BookStack over stdio.

## Goals / Non-Goals

**Goals:**
- Provide standard MCP tools for search, reading pages, and navigating the BookStack content tree.
- Prioritize Markdown format for page content to maximize LLM readability and reduce token consumption.
- Maintain a strict boundary between stdout (JSON-RPC) and stderr (diagnostics and logs).
- Structure code modularly into `config`, `models`, `client`, and `server`.

**Non-Goals:**
- Write, update, or delete operations (deferred to avoid corrupting BookStack WYSIWYG editor formats).
- Alternative MCP transports like SSE or streamable HTTP (can be introduced in future changes).
- Background polling, caching, or webhooks.

## Decisions

### 1. Model Context Protocol Implementation: `rmcp`
- **Choice**: Use official Rust `rmcp` SDK with `macros`, `server`, and `transport-io` features.
- **Rationale**: `rmcp` provides first-class macros (`#[tool_router]`, `#[tool_handler]`, `#[tool]`) and native `stdio` support, eliminating boilerplate JSON-RPC protocol handling.
- **Alternatives Considered**: Hand-rolling JSON-RPC stdio handling or using older unmaintained crates.

### 2. HTTP Client: `reqwest`
- **Choice**: `reqwest` with async Tokio runtime and `json` feature.
- **Rationale**: De-facto standard in the Rust ecosystem with excellent connection reuse and error ergonomics.
- **Auth Strategy**: Inject `Authorization: Token {token_id}:{token_secret}` header on every request through a custom client wrapper.

### 3. Content Format: Markdown Export Endpoint
- **Choice**: Use `GET api/pages/{id}/export/markdown` as the primary mechanism for `read_page`.
- **Rationale**: BookStack provides direct markdown exports. This avoids token-heavy HTML elements while preserving headers, lists, codeblocks, and links.
- **Fallback**: If markdown export fails or returns empty, fetch `GET api/pages/{id}` and return the `raw_html` or text content.

### 4. Logging and Protocol Integrity
- **Choice**: `tracing` and `tracing-subscriber` configured with `.with_writer(std::io::stderr)`.
- **Rationale**: Any byte emitted to `stdout` that is not a valid JSON-RPC frame will crash or desynchronize MCP clients.

## Architecture

```
                   +------------------------+
                   |  MCP Host (stdio pipe) |
                   +-----------+------------+
                               |
                        [stdin / stdout]
                               v
                     +--------------------+
                     |    src/main.rs     | (Runtime, tracing -> stderr)
                     +---------+----------+
                               |
                               v
                     +--------------------+
                     |   src/server.rs    | (rmcp ToolRouter & Handlers)
                     +---------+----------+
                               |
                               v
                     +--------------------+
                     |   src/client.rs    | (BookStack REST Client)
                     +---------+----------+
                               |
                      [HTTP REST with Token]
                               v
                   +------------------------+
                   |   BookStack Instance   |
                   +------------------------+
```

## Risks / Trade-offs

- **[Risk]** Instâncias com certificados HTTPS autoassinados podem falhar na conexão.
  → **Mitigação**: O `BookStackClient` aceitará parâmetros padrão do reqwest e documentará variáveis de ambiente caso seja necessário desabilitar verificação estrita em ambientes de desenvolvimento local.
- **[Risk]** Páginas muito grandes consumindo excesso de memória no export.
  → **Mitigação**: O endpoint `/export/markdown` é enxuto, e ferramentas de listagem/busca limitam a quantidade padrão de resultados retornados (`count` configurável com teto seguro).
