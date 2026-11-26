use mcp_bookstack::models::*;

#[test]
fn test_deserialize_sample_search_response() {
    let json_data = r##"{
      "data": [
        {
          "id": 84,
          "book_id": 1,
          "slug": "a-chapter-for-cats",
          "name": "A chapter for cats",
          "type": "chapter",
          "url": "https://example.com/books/cats/chapter/a-chapter-for-cats",
          "preview_html": {
            "name": "A chapter for <strong>cats</strong>",
            "content": "...once a bunch of cats..."
          },
          "tags": []
        },
        {
          "name": "The hows and whys of cats",
          "id": 396,
          "slug": "the-hows-and-whys-of-cats",
          "book_id": 1,
          "chapter_id": 75,
          "type": "page",
          "url": "https://example.com/books/cats/page/the-hows-and-whys-of-cats",
          "preview_html": {
            "name": "The hows and whys of cats",
            "content": "...why cats?..."
          },
          "tags": [
            {
              "name": "Animal",
              "value": "Cat",
              "order": 0
            }
          ]
        }
      ],
      "total": 2
    }"##;

    let resp: SearchResponse = serde_json::from_str(json_data).unwrap();
    assert_eq!(resp.total, Some(2));
    assert_eq!(resp.data.len(), 2);

    let item0 = &resp.data[0];
    assert_eq!(item0.id, 84);
    assert_eq!(item0.slug, "a-chapter-for-cats");
    assert_eq!(item0.r#type, "chapter");
    assert_eq!(item0.book_id, Some(1));
    assert_eq!(item0.chapter_id, None);
    assert!(item0.preview_html.is_some());
    assert_eq!(
        item0.preview_html.as_ref().unwrap().name,
        Some("A chapter for <strong>cats</strong>".to_string())
    );

    let item1 = &resp.data[1];
    assert_eq!(item1.id, 396);
    assert_eq!(item1.r#type, "page");
    assert_eq!(item1.chapter_id, Some(75));
    let tags = item1.tags.as_ref().unwrap();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].name, "Animal");
    assert_eq!(tags[0].value, "Cat");
}

#[test]
fn test_deserialize_sample_book_list_and_details() {
    let list_json = r##"{
      "data": [
        {
          "id": 1,
          "name": "BookStack User Guide",
          "slug": "bookstack-user-guide",
          "description": "General guide",
          "created_at": "2019-05-05T21:48:46.000000Z",
          "updated_at": "2019-12-11T20:57:31.000000Z",
          "created_by": 1,
          "updated_by": 1,
          "owned_by": 1,
          "cover": null
        }
      ],
      "total": 1
    }"##;

    let list: ListResponse<BookListItem> = serde_json::from_str(list_json).unwrap();
    assert_eq!(list.data.len(), 1);
    assert_eq!(list.data[0].id, 1);
    assert_eq!(list.data[0].name, "BookStack User Guide");

    let details_json = r##"{
      "id": 16,
      "name": "My own book",
      "slug": "my-own-book",
      "description": "This is my own little book",
      "description_html": "<p>This is my own <em>little</em> book</p>",
      "contents": [
        {
          "id": 50,
          "name": "Bridge Structures",
          "type": "chapter"
        }
      ],
      "tags": [
        {
          "name": "Category",
          "value": "Guide",
          "order": 0
        }
      ]
    }"##;

    let details: BookDetails = serde_json::from_str(details_json).unwrap();
    assert_eq!(details.id, 16);
    assert_eq!(details.name, "My own book");
    assert_eq!(details.contents.as_ref().unwrap().len(), 1);
    assert_eq!(details.tags.as_ref().unwrap().len(), 1);
}

#[test]
fn test_deserialize_sample_chapter_and_page() {
    let chapter_details_json = r##"{
      "id": 1,
      "book_id": 1,
      "slug": "content-creation",
      "name": "Content Creation",
      "description": "How to create documentation",
      "pages": [
        {
          "id": 1,
          "book_id": 1,
          "chapter_id": 1,
          "name": "How to create page content",
          "slug": "how-to-create-page-content"
        }
      ]
    }"##;

    let chapter: ChapterDetails = serde_json::from_str(chapter_details_json).unwrap();
    assert_eq!(chapter.id, 1);
    assert_eq!(chapter.pages.as_ref().unwrap().len(), 1);
    assert_eq!(chapter.pages.as_ref().unwrap()[0].id, 1);

    let page_details_json = r##"{
      "id": 306,
      "book_id": 1,
      "chapter_id": null,
      "name": "A page written in markdown",
      "slug": "a-page-written-in-markdown",
      "html": "<h1>This is my cool page!</h1>",
      "raw_html": "<h1>This is my cool page!</h1>",
      "markdown": "# How this is built\r\n\r\nContent here"
    }"##;

    let page: PageDetails = serde_json::from_str(page_details_json).unwrap();
    assert_eq!(page.id, 306);
    assert_eq!(
        page.markdown.as_deref(),
        Some("# How this is built\r\n\r\nContent here")
    );
    assert_eq!(page.chapter_id, None);
}

#[test]
fn test_deserialize_sample_shelf() {
    let shelf_details_json = r##"{
      "id": 14,
      "name": "My shelf",
      "slug": "my-shelf",
      "description": "This is my shelf with some books",
      "books": [
        {
          "id": 5,
          "name": "Sint explicabo alias sunt.",
          "slug": "jbsQrzuaXe"
        }
      ]
    }"##;

    let shelf: ShelfDetails = serde_json::from_str(shelf_details_json).unwrap();
    assert_eq!(shelf.id, 14);
    assert_eq!(shelf.books.as_ref().unwrap().len(), 1);
    assert_eq!(shelf.books.as_ref().unwrap()[0].id, 5);
}

#[test]
fn test_deserialize_all_docs_json_examples() {
    let docs_path = std::path::Path::new(".tmp/docs.json");
    if !docs_path.exists() {
        return;
    }

    let content = std::fs::read_to_string(docs_path).expect("Failed to read .tmp/docs.json");
    let docs: serde_json::Value =
        serde_json::from_str(&content).expect("Failed to parse .tmp/docs.json");

    // Test search
    if let Some(eps) = docs.get("search").and_then(|v| v.as_array()) {
        for ep in eps {
            if ep.get("name").and_then(|v| v.as_str()) == Some("search-all")
                && let Some(resp_str) = ep.get("example_response").and_then(|v| v.as_str())
            {
                let parsed: SearchResponse = serde_json::from_str(resp_str)
                    .expect("Failed to deserialize search-all example response");
                assert!(!parsed.data.is_empty());
            }
        }
    }

    // Test books
    if let Some(eps) = docs.get("books").and_then(|v| v.as_array()) {
        for ep in eps {
            let name = ep.get("name").and_then(|v| v.as_str()).unwrap_or_default();
            if name == "books-list" {
                if let Some(resp_str) = ep.get("example_response").and_then(|v| v.as_str()) {
                    let parsed: ListResponse<BookListItem> = serde_json::from_str(resp_str)
                        .expect("Failed to deserialize books-list example response");
                    assert!(!parsed.data.is_empty());
                }
            } else if name == "books-read"
                && let Some(resp_str) = ep.get("example_response").and_then(|v| v.as_str())
            {
                let parsed: BookDetails = serde_json::from_str(resp_str)
                    .expect("Failed to deserialize books-read example response");
                assert_eq!(parsed.id, 16);
            }
        }
    }

    // Test chapters
    if let Some(eps) = docs.get("chapters").and_then(|v| v.as_array()) {
        for ep in eps {
            let name = ep.get("name").and_then(|v| v.as_str()).unwrap_or_default();
            if name == "chapters-list" {
                if let Some(resp_str) = ep.get("example_response").and_then(|v| v.as_str()) {
                    let parsed: ListResponse<ChapterListItem> = serde_json::from_str(resp_str)
                        .expect("Failed to deserialize chapters-list example response");
                    assert!(!parsed.data.is_empty());
                }
            } else if name == "chapters-read"
                && let Some(resp_str) = ep.get("example_response").and_then(|v| v.as_str())
            {
                let parsed: ChapterDetails = serde_json::from_str(resp_str)
                    .expect("Failed to deserialize chapters-read example response");
                assert_eq!(parsed.id, 1);
            }
        }
    }

    // Test pages
    if let Some(eps) = docs.get("pages").and_then(|v| v.as_array()) {
        for ep in eps {
            let name = ep.get("name").and_then(|v| v.as_str()).unwrap_or_default();
            if name == "pages-list" {
                if let Some(resp_str) = ep.get("example_response").and_then(|v| v.as_str()) {
                    let parsed: ListResponse<PageListItem> = serde_json::from_str(resp_str)
                        .expect("Failed to deserialize pages-list example response");
                    assert!(!parsed.data.is_empty());
                }
            } else if name == "pages-read"
                && let Some(resp_str) = ep.get("example_response").and_then(|v| v.as_str())
            {
                let parsed: PageDetails = serde_json::from_str(resp_str)
                    .expect("Failed to deserialize pages-read example response");
                assert_eq!(parsed.id, 306);
            }
        }
    }

    // Test shelves
    if let Some(eps) = docs.get("shelves").and_then(|v| v.as_array()) {
        for ep in eps {
            let name = ep.get("name").and_then(|v| v.as_str()).unwrap_or_default();
            if name == "shelves-list" {
                if let Some(resp_str) = ep.get("example_response").and_then(|v| v.as_str()) {
                    let parsed: ListResponse<ShelfListItem> = serde_json::from_str(resp_str)
                        .expect("Failed to deserialize shelves-list example response");
                    assert!(!parsed.data.is_empty());
                }
            } else if name == "shelves-read"
                && let Some(resp_str) = ep.get("example_response").and_then(|v| v.as_str())
            {
                let parsed: ShelfDetails = serde_json::from_str(resp_str)
                    .expect("Failed to deserialize shelves-read example response");
                assert_eq!(parsed.id, 14);
            }
        }
    }
}

#[test]
fn test_mutation_payloads_serialization() {
    let tag = TagInput::new("Category", "Dev").with_order(1);
    assert_eq!(tag.name, "Category");
    assert_eq!(tag.value, "Dev");
    assert_eq!(tag.order, Some(1));

    // Page creation payload
    let page_payload = CreatePagePayload {
        name: "New Page".to_string(),
        book_id: Some(10),
        chapter_id: None,
        markdown: Some("# Header\n\nContent".to_string()),
        html: None,
        priority: Some(5),
        tags: Some(vec![tag.clone()]),
    };
    let json_val = serde_json::to_value(&page_payload).unwrap();
    assert_eq!(json_val["name"], "New Page");
    assert_eq!(json_val["book_id"], 10);
    assert!(json_val.get("chapter_id").is_none());
    assert_eq!(json_val["markdown"], "# Header\n\nContent");
    assert_eq!(json_val["tags"][0]["name"], "Category");

    // Page update payload
    let page_update = UpdatePagePayload {
        name: Some("Updated Name".to_string()),
        markdown: Some("Updated Content".to_string()),
        ..Default::default()
    };
    let json_update = serde_json::to_value(&page_update).unwrap();
    assert_eq!(json_update["name"], "Updated Name");
    assert_eq!(json_update["markdown"], "Updated Content");
    assert!(json_update.get("book_id").is_none());

    // Chapter payloads
    let chapter_payload = CreateChapterPayload {
        book_id: 10,
        name: "Chapter 1".to_string(),
        description: Some("Description".to_string()),
        description_html: None,
        priority: Some(1),
        tags: Some(vec![tag.clone()]),
        default_template_id: None,
    };
    let json_ch = serde_json::to_value(&chapter_payload).unwrap();
    assert_eq!(json_ch["book_id"], 10);
    assert_eq!(json_ch["name"], "Chapter 1");

    // Book payloads
    let book_payload = CreateBookPayload {
        name: "Rust In Action".to_string(),
        description: Some("A book about Rust".to_string()),
        description_html: None,
        tags: None,
        default_template_id: None,
    };
    let json_b = serde_json::to_value(&book_payload).unwrap();
    assert_eq!(json_b["name"], "Rust In Action");

    // Shelf payloads
    let shelf_payload = CreateShelfPayload {
        name: "Backend Shelf".to_string(),
        description: Some("Books for backend engineering".to_string()),
        description_html: None,
        books: Some(vec![1, 2, 3]),
        tags: None,
    };
    let json_s = serde_json::to_value(&shelf_payload).unwrap();
    assert_eq!(json_s["name"], "Backend Shelf");
    assert_eq!(json_s["books"], serde_json::json!([1, 2, 3]));
}

#[test]
fn test_deserialize_audit_log_and_search_timestamps() {
    let search_json = r##"{
      "data": [
        {
          "id": 84,
          "book_id": 1,
          "slug": "a-chapter-for-cats",
          "name": "A chapter for cats",
          "created_at": "2021-11-14T15:57:35.000000Z",
          "updated_at": "2021-11-15T10:20:30.000000Z",
          "type": "chapter"
        }
      ],
      "total": 1
    }"##;

    let search: SearchResponse = serde_json::from_str(search_json).unwrap();
    assert_eq!(
        search.data[0].created_at.as_deref(),
        Some("2021-11-14T15:57:35.000000Z")
    );
    assert_eq!(
        search.data[0].updated_at.as_deref(),
        Some("2021-11-15T10:20:30.000000Z")
    );

    let audit_json = r##"{
      "data": [
        {
          "id": 5,
          "type": "page_update",
          "detail": "Updated content",
          "user_id": 1,
          "loggable_id": 42,
          "loggable_type": "page",
          "ip": "127.0.0.1",
          "created_at": "2021-09-29T12:37:27.000000Z",
          "user": {
            "id": 1,
            "name": "Admin",
            "slug": "admin"
          }
        }
      ],
      "total": 6088
    }"##;

    let audit: ListResponse<AuditLogItem> = serde_json::from_str(audit_json).unwrap();
    assert_eq!(audit.total, Some(6088));
    assert_eq!(audit.data.len(), 1);
    let item = &audit.data[0];
    assert_eq!(item.r#type, "page_update");
    assert_eq!(item.loggable_type.as_deref(), Some("page"));
    assert_eq!(item.loggable_id, Some(42));
    assert_eq!(item.user.as_ref().unwrap().name, "Admin");

    assert_eq!(
        format_user_summary(Some(&serde_json::json!(1))),
        Some("User #1".to_string())
    );
    assert_eq!(
        format_user_summary(Some(&serde_json::json!({"id": 2, "name": "Alice"}))),
        Some("Alice (ID: 2)".to_string())
    );
    assert_eq!(format_user_summary(None), None);
}
