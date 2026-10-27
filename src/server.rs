use std::sync::Arc;

use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    tool, tool_handler, tool_router, ServerHandler,
};
use schemars::{json_schema, JsonSchema, Schema, SchemaGenerator};
use serde::{Deserialize, Serialize};

use crate::client::BookStackClient;

/// Generates a schema with explicit `anyOf` branches (`[{"type": "integer"}, {"type": "null"}]`)
/// instead of `type: ["integer", "null"]` for portable compatibility with all MCP clients and LLMs.
fn anyof_nullable_u32_schema(_gen: &mut SchemaGenerator) -> Schema {
    json_schema!({
        "anyOf": [
            { "type": "integer" },
            { "type": "null" }
        ]
    })
}

/// Arguments for `search_content` tool.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SearchContentArgs {
    /// Search query string
    pub query: String,
    /// Optional page number for pagination
    #[serde(default)]
    #[schemars(schema_with = "anyof_nullable_u32_schema")]
    pub page: Option<u32>,
    /// Optional number of items per page
    #[serde(default)]
    #[schemars(schema_with = "anyof_nullable_u32_schema")]
    pub count: Option<u32>,
}

/// Arguments for `read_page` tool.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ReadPageArgs {
    /// ID of the page to read
    pub page_id: u64,
}

/// Arguments for `list_books` tool.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListBooksArgs {
    /// Optional page number for pagination
    #[serde(default)]
    #[schemars(schema_with = "anyof_nullable_u32_schema")]
    pub page: Option<u32>,
    /// Optional number of items per page
    #[serde(default)]
    #[schemars(schema_with = "anyof_nullable_u32_schema")]
    pub count: Option<u32>,
}

/// Arguments for `get_book_details` tool.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetBookDetailsArgs {
    /// ID of the book to get details for
    pub book_id: u64,
}

/// Arguments for `list_shelves` tool.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct ListShelvesArgs {
    /// Optional page number for pagination
    #[serde(default)]
    #[schemars(schema_with = "anyof_nullable_u32_schema")]
    pub page: Option<u32>,
    /// Optional number of items per page
    #[serde(default)]
    #[schemars(schema_with = "anyof_nullable_u32_schema")]
    pub count: Option<u32>,
}

/// Arguments for `read_chapter` tool.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ReadChapterArgs {
    /// ID of the chapter to read
    pub chapter_id: u64,
}

/// MCP Server implementation for BookStack.
#[derive(Clone)]
pub struct BookStackServer {
    pub client: Arc<BookStackClient>,
    pub tool_router: ToolRouter<Self>,
}

impl BookStackServer {
    /// Creates a new `BookStackServer` wrapping the given client.
    pub fn new(client: Arc<BookStackClient>) -> Self {
        Self {
            client,
            tool_router: Self::tool_router(),
        }
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for BookStackServer {}

#[tool_router(router = tool_router)]
impl BookStackServer {
    /// Search across BookStack content (books, chapters, pages, shelves).
    #[tool(
        name = "search_content",
        description = "Search across BookStack content (books, chapters, pages, shelves)"
    )]
    pub async fn search_content(
        &self,
        params: Parameters<SearchContentArgs>,
    ) -> Result<String, rmcp::ErrorData> {
        let args = params.0;
        let resp = self
            .client
            .search(&args.query, args.page, args.count)
            .await
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

        let mut out = format!("# Search Results for \"{}\"\n\n", args.query);
        if let Some(total) = resp.total {
            out.push_str(&format!("*Total matches: {}*\n\n", total));
        }

        if resp.data.is_empty() {
            out.push_str("No results found.\n");
        } else {
            for item in &resp.data {
                out.push_str(&format!(
                    "### [{}] {} (ID: {})\n",
                    item.r#type, item.name, item.id
                ));
                out.push_str(&format!("- **Slug**: `{}`\n", item.slug));
                if let Some(ref url) = item.url {
                    out.push_str(&format!("- **URL**: {}\n", url));
                }
                if let Some(ref preview) = item.preview_html
                    && let Some(ref content) = preview.content
                {
                    let cleaned = clean_html_snippet(content);
                    if !cleaned.is_empty() {
                        out.push_str(&format!("- **Snippet**: {}\n", cleaned));
                    }
                }
                out.push('\n');
            }
        }

        Ok(out)
    }

    /// Read the content of a BookStack page formatted in Markdown.
    #[tool(
        name = "read_page",
        description = "Read the content of a BookStack page formatted in Markdown"
    )]
    pub async fn read_page(
        &self,
        params: Parameters<ReadPageArgs>,
    ) -> Result<String, rmcp::ErrorData> {
        let args = params.0;
        self.client
            .get_page_markdown(args.page_id)
            .await
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
    }

    /// List books in the BookStack instance with IDs, names, and slugs.
    #[tool(
        name = "list_books",
        description = "List books in the BookStack instance with IDs, names, and slugs"
    )]
    pub async fn list_books(
        &self,
        params: Parameters<ListBooksArgs>,
    ) -> Result<String, rmcp::ErrorData> {
        let args = params.0;
        let books = self
            .client
            .list_books(args.page, args.count)
            .await
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

        let mut out = String::from("# Books\n\n");
        if books.is_empty() {
            out.push_str("No books found.\n");
        } else {
            for b in &books {
                out.push_str(&format!("- **{}** (ID: {}, Slug: `{}`)\n", b.name, b.id, b.slug));
                if let Some(ref desc) = b.description
                    && !desc.trim().is_empty()
                {
                    out.push_str(&format!("  - Description: {}\n", desc.trim()));
                }
            }
        }

        Ok(out)
    }

    /// Get detailed book information, description, chapters, and child pages.
    #[tool(
        name = "get_book_details",
        description = "Get detailed book information, description, chapters, and child pages"
    )]
    pub async fn get_book_details(
        &self,
        params: Parameters<GetBookDetailsArgs>,
    ) -> Result<String, rmcp::ErrorData> {
        let args = params.0;
        let book = self
            .client
            .get_book(args.book_id)
            .await
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

        let mut out = format!("# Book: {} (ID: {})\n\n", book.name, book.id);
        out.push_str(&format!("- **Slug**: `{}`\n", book.slug));
        if let Some(ref desc) = book.description
            && !desc.trim().is_empty()
        {
            out.push_str(&format!("- **Description**: {}\n", desc.trim()));
        }

        out.push_str("\n## Contents\n\n");
        if let Some(ref contents) = book.contents {
            if contents.is_empty() {
                out.push_str("No chapters or pages in this book.\n");
            } else {
                for item in contents {
                    let item_type = item.get("type").and_then(|v| v.as_str()).unwrap_or("");
                    let name = item.get("name").and_then(|v| v.as_str()).unwrap_or("Untitled");
                    let id = item.get("id").and_then(|v| v.as_u64()).unwrap_or(0);
                    let slug = item.get("slug").and_then(|v| v.as_str()).unwrap_or("");

                    if item_type == "chapter" {
                        out.push_str(&format!(
                            "### Chapter: {} (ID: {}, Slug: `{}`)\n",
                            name, id, slug
                        ));
                        if let Some(pages) = item.get("pages").and_then(|v| v.as_array()) {
                            for page in pages {
                                let p_name = page
                                    .get("name")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("Untitled");
                                let p_id = page.get("id").and_then(|v| v.as_u64()).unwrap_or(0);
                                let p_slug = page
                                    .get("slug")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("");
                                out.push_str(&format!(
                                    "  - Page: {} (ID: {}, Slug: `{}`)\n",
                                    p_name, p_id, p_slug
                                ));
                            }
                        }
                        out.push('\n');
                    } else if item_type == "page" {
                        out.push_str(&format!("- Page: {} (ID: {}, Slug: `{}`)\n", name, id, slug));
                    }
                }
            }
        } else {
            out.push_str("No contents available.\n");
        }

        Ok(out)
    }

    /// List shelves in the BookStack instance with IDs, names, and slugs.
    #[tool(
        name = "list_shelves",
        description = "List shelves in the BookStack instance with IDs, names, and slugs"
    )]
    pub async fn list_shelves(
        &self,
        params: Parameters<ListShelvesArgs>,
    ) -> Result<String, rmcp::ErrorData> {
        let args = params.0;
        let shelves = self
            .client
            .list_shelves(args.page, args.count)
            .await
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

        let mut out = String::from("# Shelves\n\n");
        if shelves.is_empty() {
            out.push_str("No shelves found.\n");
        } else {
            for s in &shelves {
                out.push_str(&format!("- **{}** (ID: {}, Slug: `{}`)\n", s.name, s.id, s.slug));
                if let Some(ref desc) = s.description
                    && !desc.trim().is_empty()
                {
                    out.push_str(&format!("  - Description: {}\n", desc.trim()));
                }
            }
        }

        Ok(out)
    }

    /// Get detailed chapter information and its child pages.
    #[tool(
        name = "read_chapter",
        description = "Get detailed chapter information and its child pages"
    )]
    pub async fn read_chapter(
        &self,
        params: Parameters<ReadChapterArgs>,
    ) -> Result<String, rmcp::ErrorData> {
        let args = params.0;
        let chapter = self
            .client
            .get_chapter(args.chapter_id)
            .await
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

        let mut out = format!("# Chapter: {} (ID: {})\n\n", chapter.name, chapter.id);
        out.push_str(&format!("- **Book ID**: {}\n", chapter.book_id));
        out.push_str(&format!("- **Slug**: `{}`\n", chapter.slug));
        if let Some(ref desc) = chapter.description
            && !desc.trim().is_empty()
        {
            out.push_str(&format!("- **Description**: {}\n", desc.trim()));
        }

        out.push_str("\n## Pages\n\n");
        if let Some(ref pages) = chapter.pages {
            if pages.is_empty() {
                out.push_str("No pages in this chapter.\n");
            } else {
                for page in pages {
                    out.push_str(&format!(
                        "- **{}** (ID: {}, Slug: `{}`)\n",
                        page.name, page.id, page.slug
                    ));
                }
            }
        } else {
            out.push_str("No pages found in this chapter.\n");
        }

        Ok(out)
    }
}

/// Helper to sanitize HTML snippet preview to Markdown/plain text.
pub fn clean_html_snippet(input: &str) -> String {
    let s = input
        .replace("<strong>", "**")
        .replace("</strong>", "**")
        .replace("<b>", "**")
        .replace("</b>", "**")
        .replace("<em>", "*")
        .replace("</em>", "*")
        .replace("<i>", "*")
        .replace("</i>", "*")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#039;", "'")
        .replace("&nbsp;", " ");

    let mut clean = String::new();
    let mut in_tag = false;
    for c in s.chars() {
        if c == '<' {
            in_tag = true;
        } else if c == '>' {
            in_tag = false;
        } else if !in_tag {
            clean.push(c);
        }
    }
    clean.trim().to_string()
}
