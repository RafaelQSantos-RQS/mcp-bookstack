use mcp_bookstack::models::*;
use mcp_bookstack::{BookStackClient, Config};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

fn test_config() -> Config {
    Config::new("https://bookstack.test", "token-id", "token-secret").unwrap()
}

#[test]
fn test_client_header_and_url_construction() {
    let config = test_config();
    let client = BookStackClient::new(&config);

    assert_eq!(client.base_url, "https://bookstack.test");
    assert_eq!(client.auth_header, "Token token-id:token-secret");

    assert_eq!(client.search_url(), "https://bookstack.test/api/search");
    assert_eq!(client.books_url(), "https://bookstack.test/api/books");
    assert_eq!(client.book_url(42), "https://bookstack.test/api/books/42");
    assert_eq!(client.chapters_url(), "https://bookstack.test/api/chapters");
    assert_eq!(
        client.chapter_url(10),
        "https://bookstack.test/api/chapters/10"
    );
    assert_eq!(client.pages_url(), "https://bookstack.test/api/pages");
    assert_eq!(client.page_url(99), "https://bookstack.test/api/pages/99");
    assert_eq!(
        client.page_markdown_url(99),
        "https://bookstack.test/api/pages/99/export/markdown"
    );
    assert_eq!(client.shelves_url(), "https://bookstack.test/api/shelves");
    assert_eq!(client.shelf_url(7), "https://bookstack.test/api/shelves/7");
}

#[tokio::test]
async fn test_get_page_markdown_fallback_to_page_markdown_field() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let server_url = format!("http://127.0.0.1:{}", port);

    let config = Config::new(&server_url, "test_id", "test_sec").unwrap();
    let client = BookStackClient::new(&config);

    tokio::spawn(async move {
        // First request: GET /api/pages/1/export/markdown -> returns 404
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut buf = [0u8; 1024];
        let _ = socket.read(&mut buf).await.unwrap();

        let response = "HTTP/1.1 404 Not Found\r\nConnection: close\r\nContent-Length: 0\r\n\r\n";
        socket.write_all(response.as_bytes()).await.unwrap();
        let _ = socket.shutdown().await;

        // Second request: GET /api/pages/1 -> returns JSON with markdown
        let (mut socket, _) = listener.accept().await.unwrap();
        let _ = socket.read(&mut buf).await.unwrap();

        let page_json =
            r##"{"id":1,"name":"Test","slug":"test","markdown":"# Fallback Markdown"}"##;
        let response = format!(
            "HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            page_json.len(),
            page_json
        );
        socket.write_all(response.as_bytes()).await.unwrap();
        let _ = socket.shutdown().await;
    });

    let md = client.get_page_markdown(1).await.unwrap();
    assert_eq!(md, "# Fallback Markdown");
}

#[tokio::test]
async fn test_get_page_markdown_direct_success() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let server_url = format!("http://127.0.0.1:{}", port);

    let config = Config::new(&server_url, "test_id", "test_sec").unwrap();
    let client = BookStackClient::new(&config);

    tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut buf = [0u8; 1024];
        let _ = socket.read(&mut buf).await.unwrap();

        let content = "# Direct Export Markdown";
        let response = format!(
            "HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            content.len(),
            content
        );
        socket.write_all(response.as_bytes()).await.unwrap();
        let _ = socket.shutdown().await;
    });

    let md = client.get_page_markdown(1).await.unwrap();
    assert_eq!(md, "# Direct Export Markdown");
}

#[tokio::test]
async fn test_get_page_markdown_fallback_to_raw_html() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let server_url = format!("http://127.0.0.1:{}", port);

    let config = Config::new(&server_url, "test_id", "test_sec").unwrap();
    let client = BookStackClient::new(&config);

    tokio::spawn(async move {
        // First request: export returns empty text
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut buf = [0u8; 1024];
        let _ = socket.read(&mut buf).await.unwrap();

        let response = "HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Length: 0\r\n\r\n";
        socket.write_all(response.as_bytes()).await.unwrap();
        let _ = socket.shutdown().await;

        // Second request: get_page returns page with raw_html
        let (mut socket, _) = listener.accept().await.unwrap();
        let _ = socket.read(&mut buf).await.unwrap();

        let page_json = r##"{"id":1,"name":"Test","slug":"test","raw_html":"<p>Raw HTML</p>"}"##;
        let response = format!(
            "HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            page_json.len(),
            page_json
        );
        socket.write_all(response.as_bytes()).await.unwrap();
        let _ = socket.shutdown().await;
    });

    let content = client.get_page_markdown(1).await.unwrap();
    assert_eq!(content, "<p>Raw HTML</p>");
}

#[tokio::test]
async fn test_get_page_markdown_empty_page_placeholder() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let server_url = format!("http://127.0.0.1:{}", port);

    let config = Config::new(&server_url, "test_id", "test_sec").unwrap();
    let client = BookStackClient::new(&config);

    tokio::spawn(async move {
        // First request: export returns empty text
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut buf = [0u8; 1024];
        let _ = socket.read(&mut buf).await.unwrap();

        let response = "HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Length: 0\r\n\r\n";
        socket.write_all(response.as_bytes()).await.unwrap();
        let _ = socket.shutdown().await;

        // Second request: get_page returns page with empty markdown and html
        let (mut socket, _) = listener.accept().await.unwrap();
        let _ = socket.read(&mut buf).await.unwrap();

        let page_json = r##"{"id":1,"name":"Blank Page","slug":"blank-page"}"##;
        let response = format!(
            "HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            page_json.len(),
            page_json
        );
        socket.write_all(response.as_bytes()).await.unwrap();
        let _ = socket.shutdown().await;
    });

    let content = client.get_page_markdown(1).await.unwrap();
    assert_eq!(content, "# Blank Page\n\n*(This page is empty)*");
}

#[tokio::test]
async fn test_search_and_list_endpoints() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let server_url = format!("http://127.0.0.1:{}", port);

    let config = Config::new(&server_url, "test_id", "test_sec").unwrap();
    let client = BookStackClient::new(&config);

    tokio::spawn(async move {
        // 1. search
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut buf = [0u8; 2048];
        let n = socket.read(&mut buf).await.unwrap();
        let req_str = String::from_utf8_lossy(&buf[..n]);
        assert!(
            req_str
                .to_lowercase()
                .contains("authorization: token test_id:test_sec")
        );
        assert!(req_str.contains("query=rust"));

        let search_json = r##"{"data":[{"id":10,"name":"Rust Guide","slug":"rust-guide","type":"book"}],"total":1}"##;
        let response = format!(
            "HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            search_json.len(),
            search_json
        );
        socket.write_all(response.as_bytes()).await.unwrap();
        let _ = socket.shutdown().await;

        // 2. list_books
        let (mut socket, _) = listener.accept().await.unwrap();
        let _ = socket.read(&mut buf).await.unwrap();
        let books_json =
            r##"{"data":[{"id":10,"name":"Rust Guide","slug":"rust-guide"}],"total":1}"##;
        let response = format!(
            "HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            books_json.len(),
            books_json
        );
        socket.write_all(response.as_bytes()).await.unwrap();
        let _ = socket.shutdown().await;

        // 3. get_book
        let (mut socket, _) = listener.accept().await.unwrap();
        let _ = socket.read(&mut buf).await.unwrap();
        let book_json = r##"{"id":10,"name":"Rust Guide","slug":"rust-guide","contents":[]}"##;
        let response = format!(
            "HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            book_json.len(),
            book_json
        );
        socket.write_all(response.as_bytes()).await.unwrap();
        let _ = socket.shutdown().await;

        // 4. list_chapters with filter
        let (mut socket, _) = listener.accept().await.unwrap();
        let n = socket.read(&mut buf).await.unwrap();
        let req_str = String::from_utf8_lossy(&buf[..n]);
        assert!(
            req_str.contains("filter%5Bbook_id%5D=10") || req_str.contains("filter[book_id]=10")
        );
        let chapters_json =
            r##"{"data":[{"id":20,"book_id":10,"name":"Intro","slug":"intro"}],"total":1}"##;
        let response = format!(
            "HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            chapters_json.len(),
            chapters_json
        );
        socket.write_all(response.as_bytes()).await.unwrap();
        let _ = socket.shutdown().await;

        // 5. get_chapter
        let (mut socket, _) = listener.accept().await.unwrap();
        let _ = socket.read(&mut buf).await.unwrap();
        let chapter_json = r##"{"id":20,"book_id":10,"name":"Intro","slug":"intro","pages":[]}"##;
        let response = format!(
            "HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            chapter_json.len(),
            chapter_json
        );
        socket.write_all(response.as_bytes()).await.unwrap();
        let _ = socket.shutdown().await;

        // 6. list_shelves
        let (mut socket, _) = listener.accept().await.unwrap();
        let _ = socket.read(&mut buf).await.unwrap();
        let shelves_json =
            r##"{"data":[{"id":30,"name":"Dev Shelf","slug":"dev-shelf"}],"total":1}"##;
        let response = format!(
            "HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            shelves_json.len(),
            shelves_json
        );
        socket.write_all(response.as_bytes()).await.unwrap();
        let _ = socket.shutdown().await;

        // 7. get_shelf
        let (mut socket, _) = listener.accept().await.unwrap();
        let _ = socket.read(&mut buf).await.unwrap();
        let shelf_json = r##"{"id":30,"name":"Dev Shelf","slug":"dev-shelf","books":[]}"##;
        let response = format!(
            "HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            shelf_json.len(),
            shelf_json
        );
        socket.write_all(response.as_bytes()).await.unwrap();
        let _ = socket.shutdown().await;
    });

    let search_res = client.search("rust", Some(1), Some(10)).await.unwrap();
    assert_eq!(search_res.data.len(), 1);
    assert_eq!(search_res.data[0].name, "Rust Guide");

    let books = client.list_books(Some(1), Some(10)).await.unwrap();
    assert_eq!(books.len(), 1);
    assert_eq!(books[0].id, 10);

    let book = client.get_book(10).await.unwrap();
    assert_eq!(book.id, 10);

    let chapters = client.list_chapters(Some(10)).await.unwrap();
    assert_eq!(chapters.len(), 1);
    assert_eq!(chapters[0].id, 20);

    let chapter = client.get_chapter(20).await.unwrap();
    assert_eq!(chapter.id, 20);

    let shelves = client.list_shelves(None, None).await.unwrap();
    assert_eq!(shelves.len(), 1);
    assert_eq!(shelves[0].id, 30);

    let shelf = client.get_shelf(30).await.unwrap();
    assert_eq!(shelf.id, 30);
}

#[tokio::test]
async fn test_client_mutation_methods() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let server_url = format!("http://127.0.0.1:{}", port);

    let config = Config::new(&server_url, "test_id", "test_sec").unwrap();
    let client = BookStackClient::new(&config);

    tokio::spawn(async move {
        // Helper to serve responses
        async fn handle_req(socket: &mut tokio::net::TcpStream, status: &str, body: &str) {
            let mut buf = [0u8; 4096];
            let _ = socket.read(&mut buf).await.unwrap();
            let response = format!(
                "HTTP/1.1 {}\r\nConnection: close\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                status,
                body.len(),
                body
            );
            socket.write_all(response.as_bytes()).await.unwrap();
            let _ = socket.shutdown().await;
        }

        // 1. POST /api/pages
        let (mut socket, _) = listener.accept().await.unwrap();
        handle_req(
            &mut socket,
            "200 OK",
            r#"{"id":42,"name":"New Page","slug":"new-page","book_id":10}"#,
        )
        .await;

        // 2. PUT /api/pages/42
        let (mut socket, _) = listener.accept().await.unwrap();
        handle_req(
            &mut socket,
            "200 OK",
            r#"{"id":42,"name":"Updated Page","slug":"updated-page","book_id":10}"#,
        )
        .await;

        // 3. DELETE /api/pages/42
        let (mut socket, _) = listener.accept().await.unwrap();
        handle_req(&mut socket, "204 No Content", "").await;

        // 4. POST /api/chapters
        let (mut socket, _) = listener.accept().await.unwrap();
        handle_req(
            &mut socket,
            "200 OK",
            r#"{"id":20,"book_id":10,"name":"New Chapter","slug":"new-chapter"}"#,
        )
        .await;

        // 5. PUT /api/chapters/20
        let (mut socket, _) = listener.accept().await.unwrap();
        handle_req(
            &mut socket,
            "200 OK",
            r#"{"id":20,"book_id":10,"name":"Updated Chapter","slug":"updated-chapter"}"#,
        )
        .await;

        // 6. DELETE /api/chapters/20
        let (mut socket, _) = listener.accept().await.unwrap();
        handle_req(&mut socket, "204 No Content", "").await;

        // 7. POST /api/books
        let (mut socket, _) = listener.accept().await.unwrap();
        handle_req(
            &mut socket,
            "200 OK",
            r#"{"id":10,"name":"New Book","slug":"new-book"}"#,
        )
        .await;

        // 8. PUT /api/books/10
        let (mut socket, _) = listener.accept().await.unwrap();
        handle_req(
            &mut socket,
            "200 OK",
            r#"{"id":10,"name":"Updated Book","slug":"updated-book"}"#,
        )
        .await;

        // 9. DELETE /api/books/10
        let (mut socket, _) = listener.accept().await.unwrap();
        handle_req(&mut socket, "204 No Content", "").await;

        // 10. POST /api/shelves
        let (mut socket, _) = listener.accept().await.unwrap();
        handle_req(
            &mut socket,
            "200 OK",
            r#"{"id":30,"name":"New Shelf","slug":"new-shelf"}"#,
        )
        .await;

        // 11. PUT /api/shelves/30
        let (mut socket, _) = listener.accept().await.unwrap();
        handle_req(
            &mut socket,
            "200 OK",
            r#"{"id":30,"name":"Updated Shelf","slug":"updated-shelf"}"#,
        )
        .await;

        // 12. DELETE /api/shelves/30
        let (mut socket, _) = listener.accept().await.unwrap();
        handle_req(&mut socket, "204 No Content", "").await;
    });

    // 1. create_page
    let new_page = client
        .create_page(&CreatePagePayload {
            name: "New Page".to_string(),
            book_id: Some(10),
            chapter_id: None,
            markdown: Some("# Hello".to_string()),
            html: None,
            priority: None,
            tags: None,
        })
        .await
        .unwrap();
    assert_eq!(new_page.id, 42);
    assert_eq!(new_page.name, "New Page");

    // 2. update_page
    let updated_page = client
        .update_page(
            42,
            &UpdatePagePayload {
                name: Some("Updated Page".to_string()),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(updated_page.id, 42);
    assert_eq!(updated_page.name, "Updated Page");

    // 3. delete_page
    client.delete_page(42).await.unwrap();

    // 4. create_chapter
    let new_chapter = client
        .create_chapter(&CreateChapterPayload {
            book_id: 10,
            name: "New Chapter".to_string(),
            description: None,
            description_html: None,
            priority: None,
            tags: None,
            default_template_id: None,
        })
        .await
        .unwrap();
    assert_eq!(new_chapter.id, 20);

    // 5. update_chapter
    let updated_chapter = client
        .update_chapter(
            20,
            &UpdateChapterPayload {
                name: Some("Updated Chapter".to_string()),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(updated_chapter.id, 20);

    // 6. delete_chapter
    client.delete_chapter(20).await.unwrap();

    // 7. create_book
    let new_book = client
        .create_book(&CreateBookPayload {
            name: "New Book".to_string(),
            description: None,
            description_html: None,
            tags: None,
            default_template_id: None,
        })
        .await
        .unwrap();
    assert_eq!(new_book.id, 10);

    // 8. update_book
    let updated_book = client
        .update_book(
            10,
            &UpdateBookPayload {
                name: Some("Updated Book".to_string()),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(updated_book.id, 10);

    // 9. delete_book
    client.delete_book(10).await.unwrap();

    // 10. create_shelf
    let new_shelf = client
        .create_shelf(&CreateShelfPayload {
            name: "New Shelf".to_string(),
            description: None,
            description_html: None,
            books: Some(vec![10]),
            tags: None,
        })
        .await
        .unwrap();
    assert_eq!(new_shelf.id, 30);

    // 11. update_shelf
    let updated_shelf = client
        .update_shelf(
            30,
            &UpdateShelfPayload {
                name: Some("Updated Shelf".to_string()),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(updated_shelf.id, 30);

    // 12. delete_shelf
    client.delete_shelf(30).await.unwrap();
}
