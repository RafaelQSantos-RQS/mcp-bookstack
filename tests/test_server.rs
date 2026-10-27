use std::sync::Arc;
use mcp_bookstack::server::clean_html_snippet;
use mcp_bookstack::{BookStackClient, BookStackServer, Config};
use rmcp::handler::server::wrapper::Parameters;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

/// Helper to run a mock HTTP server on a random local port
async fn spawn_mock_bookstack() -> (String, tokio::task::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let base_url = format!("http://{}", addr);

    let handle = tokio::spawn(async move {
        loop {
            let (mut socket, _) = match listener.accept().await {
                Ok(conn) => conn,
                Err(_) => break,
            };

            tokio::spawn(async move {
                let mut buf = [0u8; 4096];
                let n = socket.read(&mut buf).await.unwrap_or(0);
                if n == 0 {
                    return;
                }
                let req_str = String::from_utf8_lossy(&buf[..n]);
                let first_line = req_str.lines().next().unwrap_or("");

                let (status, content_type, body) = if first_line.contains("/api/search") {
                    (
                        "200 OK",
                        "application/json",
                        r#"{"data":[{"id":42,"name":"Rust Guide","slug":"rust-guide","type":"page","url":"https://example.com/books/rust/page/rust-guide","preview_html":{"name":"Rust Guide","content":"Learn <strong>Rust</strong> fast"},"book_id":1,"tags":[]}],"total":1}"#,
                    )
                } else if first_line.contains("/api/pages/42/export/markdown") {
                    (
                        "200 OK",
                        "text/plain; charset=UTF-8",
                        "# Rust Guide\n\nWelcome to BookStack documentation written in Rust.",
                    )
                } else if first_line.contains("/api/books/10") {
                    (
                        "200 OK",
                        "application/json",
                        r#"{"id":10,"name":"Advanced Systems","slug":"advanced-systems","description":"Deep systems guide","contents":[{"id":20,"name":"Concurrency","slug":"concurrency","type":"chapter","pages":[{"id":42,"name":"Rust Guide","slug":"rust-guide"}]}]}"#,
                    )
                } else if first_line.contains("/api/books") {
                    (
                        "200 OK",
                        "application/json",
                        r#"{"data":[{"id":10,"name":"Advanced Systems","slug":"advanced-systems","description":"Deep systems guide"}],"total":1}"#,
                    )
                } else if first_line.contains("/api/shelves") {
                    (
                        "200 OK",
                        "application/json",
                        r#"{"data":[{"id":5,"name":"Engineering Shelf","slug":"engineering-shelf","description":"All engineering books"}],"total":1}"#,
                    )
                } else if first_line.contains("/api/chapters/20") {
                    (
                        "200 OK",
                        "application/json",
                        r#"{"id":20,"book_id":10,"name":"Concurrency","slug":"concurrency","description":"Threads and channels","pages":[{"id":42,"name":"Rust Guide","slug":"rust-guide"}]}"#,
                    )
                } else {
                    ("404 Not Found", "text/plain", "Not Found")
                };

                let response = format!(
                    "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    status,
                    content_type,
                    body.len(),
                    body
                );
                let _ = socket.write_all(response.as_bytes()).await;
            });
        }
    });

    (base_url, handle)
}

#[test]
fn test_clean_html_snippet() {
    let snippet = "This is a <strong>highlighted</strong> snippet with &amp; entity.";
    let cleaned = clean_html_snippet(snippet);
    assert_eq!(cleaned, "This is a **highlighted** snippet with & entity.");
}

#[tokio::test]
async fn test_server_tool_definitions() {
    let config = Config::new("https://bookstack.example.com", "id", "secret").unwrap();
    let client = Arc::new(BookStackClient::new(&config));
    let server = BookStackServer::new(client);

    let tools = server.tool_router.list_all();
    assert_eq!(tools.len(), 6);

    let tool_names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();
    assert!(tool_names.contains(&"search_content"));
    assert!(tool_names.contains(&"read_page"));
    assert!(tool_names.contains(&"list_books"));
    assert!(tool_names.contains(&"get_book_details"));
    assert!(tool_names.contains(&"list_shelves"));
    assert!(tool_names.contains(&"read_chapter"));

    for tool in &tools {
        assert!(
            tool.description.is_some(),
            "Tool {} missing description",
            tool.name
        );
    }

    // Verify list_books, list_shelves and search_content use anyOf for optional integer parameters (portable JSON Schema)
    for tool_name in ["list_books", "list_shelves", "search_content"] {
        let tool = tools.iter().find(|t| t.name == tool_name).unwrap();
        let schema_json = serde_json::to_value(&tool.input_schema).unwrap();
        let page_prop = &schema_json["properties"]["page"];
        assert!(
            page_prop.get("anyOf").is_some(),
            "Expected anyOf in {}.page schema, got: {:?}",
            tool_name,
            page_prop
        );
        let count_prop = &schema_json["properties"]["count"];
        assert!(
            count_prop.get("anyOf").is_some(),
            "Expected anyOf in {}.count schema, got: {:?}",
            tool_name,
            count_prop
        );
    }
}

#[tokio::test]
async fn test_tools_execution_with_mock_bookstack() {
    let (base_url, _handle) = spawn_mock_bookstack().await;
    let config = Config::new(base_url, "test_token_id", "test_token_secret").unwrap();
    let client = Arc::new(BookStackClient::new(&config));
    let server = BookStackServer::new(client);

    // 1. search_content
    use mcp_bookstack::server::{
        GetBookDetailsArgs, ListBooksArgs, ListShelvesArgs, ReadChapterArgs, ReadPageArgs,
        SearchContentArgs,
    };

    let search_res = server
        .search_content(Parameters(SearchContentArgs {
            query: "Rust".to_string(),
            page: None,
            count: None,
        }))
        .await
        .expect("search_content failed");
    assert!(search_res.contains("Rust Guide"));
    assert!(search_res.contains("42"));
    assert!(search_res.contains("**Rust**"));

    // 2. read_page
    let page_res = server
        .read_page(Parameters(ReadPageArgs { page_id: 42 }))
        .await
        .expect("read_page failed");
    assert!(page_res.contains("# Rust Guide"));
    assert!(page_res.contains("Welcome to BookStack documentation"));

    // 3. list_books
    let books_res = server
        .list_books(Parameters(ListBooksArgs {
            page: None,
            count: None,
        }))
        .await
        .expect("list_books failed");
    assert!(books_res.contains("Advanced Systems"));
    assert!(books_res.contains("ID: 10"));

    // 4. get_book_details
    let book_res = server
        .get_book_details(Parameters(GetBookDetailsArgs { book_id: 10 }))
        .await
        .expect("get_book_details failed");
    assert!(book_res.contains("Advanced Systems"));
    assert!(book_res.contains("Concurrency"));
    assert!(book_res.contains("Rust Guide"));

    // 5. list_shelves
    let shelves_res = server
        .list_shelves(Parameters(ListShelvesArgs::default()))
        .await
        .expect("list_shelves failed");
    assert!(shelves_res.contains("Engineering Shelf"));
    assert!(shelves_res.contains("ID: 5"));

    // 6. read_chapter
    let chapter_res = server
        .read_chapter(Parameters(ReadChapterArgs { chapter_id: 20 }))
        .await
        .expect("read_chapter failed");
    assert!(chapter_res.contains("Concurrency"));
    assert!(chapter_res.contains("Rust Guide"));
}

#[tokio::test]
async fn test_main_binary_stdio_initialize() -> anyhow::Result<()> {
    // Build binary first if not built
    let (base_url, _handle) = spawn_mock_bookstack().await;

    let mut child = tokio::process::Command::new(env!("CARGO_BIN_EXE_mcp-bookstack"))
        .env("BOOKSTACK_URL", base_url)
        .env("BOOKSTACK_TOKEN_ID", "dummy_id")
        .env("BOOKSTACK_TOKEN_SECRET", "dummy_secret")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    let mut stdout = tokio::io::BufReader::new(child.stdout.take().expect("Failed to open stdout"));

    // Send JSON-RPC initialize
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {
                "name": "test-runner",
                "version": "1.0.0"
            }
        }
    });

    let init_line = format!("{}\n", init_req);
    stdin.write_all(init_line.as_bytes()).await?;
    stdin.flush().await?;

    use tokio::io::AsyncBufReadExt;
    let mut resp_line = String::new();
    stdout.read_line(&mut resp_line).await?;

    let init_resp: serde_json::Value = serde_json::from_str(&resp_line)
        .unwrap_or_else(|e| panic!("Failed to parse initialize response: {}\nLine was: {}", e, resp_line));

    assert_eq!(init_resp["id"], 1);
    assert!(init_resp["result"]["capabilities"]["tools"].is_object());

    // Send initialized notification
    let initialized_notif = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "notifications/initialized"
    });
    let notif_line = format!("{}\n", initialized_notif);
    stdin.write_all(notif_line.as_bytes()).await?;
    stdin.flush().await?;

    // Send tools/list request
    let list_tools_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/list",
        "params": {}
    });
    let list_line = format!("{}\n", list_tools_req);
    stdin.write_all(list_line.as_bytes()).await?;
    stdin.flush().await?;

    let mut list_resp_line = String::new();
    stdout.read_line(&mut list_resp_line).await?;

    let list_resp: serde_json::Value = serde_json::from_str(&list_resp_line)
        .unwrap_or_else(|e| panic!("Failed to parse tools/list response: {}\nLine was: {}", e, list_resp_line));

    assert_eq!(list_resp["id"], 2);
    let tools = list_resp["result"]["tools"].as_array().expect("tools array");
    assert_eq!(tools.len(), 6);

    let tool_names: Vec<&str> = tools
        .iter()
        .filter_map(|t| t["name"].as_str())
        .collect();
    assert!(tool_names.contains(&"search_content"));
    assert!(tool_names.contains(&"read_page"));
    assert!(tool_names.contains(&"list_books"));
    assert!(tool_names.contains(&"get_book_details"));
    assert!(tool_names.contains(&"list_shelves"));
    assert!(tool_names.contains(&"read_chapter"));

    // Send tools/call request for read_page
    let call_tool_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "tools/call",
        "params": {
            "name": "read_page",
            "arguments": {
                "page_id": 42
            }
        }
    });
    let call_line = format!("{}\n", call_tool_req);
    stdin.write_all(call_line.as_bytes()).await?;
    stdin.flush().await?;

    let mut call_resp_line = String::new();
    stdout.read_line(&mut call_resp_line).await?;

    let call_resp: serde_json::Value = serde_json::from_str(&call_resp_line)
        .unwrap_or_else(|e| panic!("Failed to parse tools/call response: {}\nLine was: {}", e, call_resp_line));

    assert_eq!(call_resp["id"], 3);
    let content = call_resp["result"]["content"].as_array().expect("content array");
    assert!(!content.is_empty());
    let text = content[0]["text"].as_str().expect("text content");
    assert!(text.contains("# Rust Guide"));
    assert!(text.contains("Welcome to BookStack documentation written in Rust."));

    // Close stdin and terminate child
    drop(stdin);
    let _ = child.kill().await;

    Ok(())
}
