use serde::{Deserialize, Serialize};

/// Generic list response envelope used by BookStack API endpoints.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(bound(deserialize = "T: serde::de::DeserializeOwned"))]
pub struct ListResponse<T> {
    #[serde(default)]
    pub data: Vec<T>,
    #[serde(default)]
    pub total: Option<u64>,
}

/// Search response returned by `GET /api/search`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchResponse {
    #[serde(default)]
    pub data: Vec<SearchResult>,
    #[serde(default)]
    pub total: Option<u64>,
}

/// Individual item in a search result.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchResult {
    pub id: u64,
    pub name: String,
    pub slug: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub preview_html: Option<SearchPreviewHtml>,
    #[serde(default)]
    pub book_id: Option<u64>,
    #[serde(default)]
    pub chapter_id: Option<u64>,
    #[serde(default)]
    pub tags: Option<Vec<Tag>>,
}

/// HTML preview snippets within a search result.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchPreviewHtml {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub content: Option<String>,
}

/// Tag applied to BookStack entities.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Tag {
    pub name: String,
    pub value: String,
    #[serde(default)]
    pub order: Option<u64>,
}

/// Book summary item returned in book listings.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct BookListItem {
    pub id: u64,
    pub name: String,
    pub slug: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub created_by: Option<serde_json::Value>,
    #[serde(default)]
    pub updated_by: Option<serde_json::Value>,
    #[serde(default)]
    pub owned_by: Option<serde_json::Value>,
    #[serde(default)]
    pub cover: Option<serde_json::Value>,
}

/// Detailed book information returned by `GET /api/books/{id}`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct BookDetails {
    pub id: u64,
    pub name: String,
    pub slug: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub description_html: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub created_by: Option<serde_json::Value>,
    #[serde(default)]
    pub updated_by: Option<serde_json::Value>,
    #[serde(default)]
    pub owned_by: Option<serde_json::Value>,
    #[serde(default)]
    pub default_template_id: Option<serde_json::Value>,
    #[serde(default)]
    pub contents: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub tags: Option<Vec<Tag>>,
    #[serde(default)]
    pub cover: Option<serde_json::Value>,
    #[serde(default)]
    pub shelves: Option<Vec<serde_json::Value>>,
}

/// Chapter summary item returned in chapter listings.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ChapterListItem {
    pub id: u64,
    pub book_id: u64,
    pub name: String,
    pub slug: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub priority: Option<i64>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub created_by: Option<serde_json::Value>,
    #[serde(default)]
    pub updated_by: Option<serde_json::Value>,
    #[serde(default)]
    pub owned_by: Option<serde_json::Value>,
    #[serde(default)]
    pub book_slug: Option<String>,
}

/// Detailed chapter information returned by `GET /api/chapters/{id}`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ChapterDetails {
    pub id: u64,
    pub book_id: u64,
    pub name: String,
    pub slug: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub description_html: Option<String>,
    #[serde(default)]
    pub default_template_id: Option<serde_json::Value>,
    #[serde(default)]
    pub priority: Option<i64>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub created_by: Option<serde_json::Value>,
    #[serde(default)]
    pub updated_by: Option<serde_json::Value>,
    #[serde(default)]
    pub owned_by: Option<serde_json::Value>,
    #[serde(default)]
    pub book_slug: Option<String>,
    #[serde(default)]
    pub tags: Option<Vec<Tag>>,
    #[serde(default)]
    pub pages: Option<Vec<PageListItem>>,
}

/// Page summary item returned in page listings or chapter details.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct PageListItem {
    pub id: u64,
    pub name: String,
    pub slug: String,
    #[serde(default)]
    pub book_id: Option<u64>,
    #[serde(default)]
    pub chapter_id: Option<u64>,
    #[serde(default)]
    pub priority: Option<i64>,
    #[serde(default)]
    pub draft: Option<bool>,
    #[serde(default)]
    pub revision_count: Option<u64>,
    #[serde(default)]
    pub template: Option<bool>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub created_by: Option<serde_json::Value>,
    #[serde(default)]
    pub updated_by: Option<serde_json::Value>,
    #[serde(default)]
    pub owned_by: Option<serde_json::Value>,
    #[serde(default)]
    pub editor: Option<String>,
    #[serde(default)]
    pub book_slug: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
}

/// Detailed page information returned by `GET /api/pages/{id}`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct PageDetails {
    pub id: u64,
    pub name: String,
    pub slug: String,
    #[serde(default)]
    pub book_id: Option<u64>,
    #[serde(default)]
    pub chapter_id: Option<u64>,
    #[serde(default)]
    pub html: Option<String>,
    #[serde(default)]
    pub raw_html: Option<String>,
    #[serde(default)]
    pub markdown: Option<String>,
    #[serde(default)]
    pub priority: Option<i64>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub created_by: Option<serde_json::Value>,
    #[serde(default)]
    pub updated_by: Option<serde_json::Value>,
    #[serde(default)]
    pub owned_by: Option<serde_json::Value>,
    #[serde(default)]
    pub draft: Option<bool>,
    #[serde(default)]
    pub revision_count: Option<u64>,
    #[serde(default)]
    pub template: Option<bool>,
    #[serde(default)]
    pub editor: Option<String>,
    #[serde(default)]
    pub tags: Option<Vec<Tag>>,
    #[serde(default)]
    pub comments: Option<serde_json::Value>,
}

/// Shelf summary item returned in shelf listings.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ShelfListItem {
    pub id: u64,
    pub name: String,
    pub slug: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub created_by: Option<serde_json::Value>,
    #[serde(default)]
    pub updated_by: Option<serde_json::Value>,
    #[serde(default)]
    pub owned_by: Option<serde_json::Value>,
    #[serde(default)]
    pub cover: Option<serde_json::Value>,
}

/// Detailed shelf information returned by `GET /api/shelves/{id}`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ShelfDetails {
    pub id: u64,
    pub name: String,
    pub slug: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub description_html: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub created_by: Option<serde_json::Value>,
    #[serde(default)]
    pub updated_by: Option<serde_json::Value>,
    #[serde(default)]
    pub owned_by: Option<serde_json::Value>,
    #[serde(default)]
    pub tags: Option<Vec<Tag>>,
    #[serde(default)]
    pub cover: Option<serde_json::Value>,
    #[serde(default)]
    pub books: Option<Vec<BookListItem>>,
}
