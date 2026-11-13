use std::sync::Arc;

use rmcp::{
    ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    tool, tool_handler, tool_router,
};
use schemars::{JsonSchema, Schema, SchemaGenerator, json_schema};
use serde::{Deserialize, Serialize};

use crate::client::BookStackClient;
use crate::models::{
    CreateBookPayload, CreateChapterPayload, CreatePagePayload, CreateShelfPayload, TagInput,
    UpdateBookPayload, UpdateChapterPayload, UpdatePagePayload, UpdateShelfPayload,
    nullable_integer_schema,
};

fn anyof_nullable_string_schema(_gen: &mut SchemaGenerator) -> Schema {
    json_schema!({
        "anyOf": [
            { "type": "string" },
            { "type": "null" }
        ]
    })
}

fn anyof_nullable_tags_schema(generator: &mut SchemaGenerator) -> Schema {
    let tag_schema = generator.subschema_for::<TagInput>();
    json_schema!({
        "anyOf": [
            {
                "type": "array",
                "items": tag_schema
            },
            { "type": "null" }
        ]
    })
}

fn anyof_nullable_u64_vec_schema(_gen: &mut SchemaGenerator) -> Schema {
    json_schema!({
        "anyOf": [
            {
                "type": "array",
                "items": { "type": "integer" }
            },
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
    #[schemars(schema_with = "nullable_integer_schema")]
    pub page: Option<u32>,
    /// Optional number of items per page
    #[serde(default)]
    #[schemars(schema_with = "nullable_integer_schema")]
    pub count: Option<u32>,
}

/// Arguments for `read_page` tool.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ReadPageArgs {
    /// ID of the page to read
    pub page_id: u64,
}

/// Arguments for `create_page` tool.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreatePageArgs {
    /// Name / title of the new page
    pub name: String,
    /// ID of the parent book (required if chapter_id is not specified)
    #[serde(default)]
    #[schemars(schema_with = "nullable_integer_schema")]
    pub book_id: Option<u64>,
    /// ID of the parent chapter (required if book_id is not specified)
    #[serde(default)]
    #[schemars(schema_with = "nullable_integer_schema")]
    pub chapter_id: Option<u64>,
    /// Page content formatted in Markdown (preferred)
    #[serde(default)]
    #[schemars(schema_with = "anyof_nullable_string_schema")]
    pub markdown: Option<String>,
    /// Page content formatted in HTML
    #[serde(default)]
    #[schemars(schema_with = "anyof_nullable_string_schema")]
    pub html: Option<String>,
    /// Optional priority order for sorting
    #[serde(default)]
    #[schemars(schema_with = "nullable_integer_schema")]
    pub priority: Option<i64>,
    /// Optional tags to associate with the page
    #[serde(default)]
    #[schemars(schema_with = "anyof_nullable_tags_schema")]
    pub tags: Option<Vec<TagInput>>,
}

/// Arguments for `update_page` tool.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UpdatePageArgs {
    /// ID of the page to update
    pub page_id: u64,
    /// Optional new name / title for the page
    #[serde(default)]
    #[schemars(schema_with = "anyof_nullable_string_schema")]
    pub name: Option<String>,
    /// Optional new parent book ID
    #[serde(default)]
    #[schemars(schema_with = "nullable_integer_schema")]
    pub book_id: Option<u64>,
    /// Optional new parent chapter ID
    #[serde(default)]
    #[schemars(schema_with = "nullable_integer_schema")]
    pub chapter_id: Option<u64>,
    /// Updated page content formatted in Markdown
    #[serde(default)]
    #[schemars(schema_with = "anyof_nullable_string_schema")]
    pub markdown: Option<String>,
    /// Updated page content formatted in HTML
    #[serde(default)]
    #[schemars(schema_with = "anyof_nullable_string_schema")]
    pub html: Option<String>,
    /// Optional priority order for sorting
    #[serde(default)]
    #[schemars(schema_with = "nullable_integer_schema")]
    pub priority: Option<i64>,
    /// Optional tags to associate with the page
    #[serde(default)]
    #[schemars(schema_with = "anyof_nullable_tags_schema")]
    pub tags: Option<Vec<TagInput>>,
}

/// Arguments for `delete_page` tool.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DeletePageArgs {
    /// ID of the page to delete
    pub page_id: u64,
    /// Must be explicitly set to true to confirm deletion
    pub confirm: bool,
}

/// Arguments for `list_books` tool.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListBooksArgs {
    /// Optional page number for pagination
    #[serde(default)]
    #[schemars(schema_with = "nullable_integer_schema")]
    pub page: Option<u32>,
    /// Optional number of items per page
    #[serde(default)]
    #[schemars(schema_with = "nullable_integer_schema")]
    pub count: Option<u32>,
}

/// Arguments for `get_book_details` tool.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetBookDetailsArgs {
    /// ID of the book to get details for
    pub book_id: u64,
}

/// Arguments for `create_book` tool.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateBookArgs {
    /// Name / title of the book
    pub name: String,
    /// Optional plain text description
    #[serde(default)]
    #[schemars(schema_with = "anyof_nullable_string_schema")]
    pub description: Option<String>,
    /// Optional HTML description
    #[serde(default)]
    #[schemars(schema_with = "anyof_nullable_string_schema")]
    pub description_html: Option<String>,
    /// Optional tags to associate with the book
    #[serde(default)]
    #[schemars(schema_with = "anyof_nullable_tags_schema")]
    pub tags: Option<Vec<TagInput>>,
    /// Optional ID of a template page to use as default
    #[serde(default)]
    #[schemars(schema_with = "nullable_integer_schema")]
    pub default_template_id: Option<u64>,
}

/// Arguments for `update_book` tool.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UpdateBookArgs {
    /// ID of the book to update
    pub book_id: u64,
    /// Optional new name / title for the book
    #[serde(default)]
    #[schemars(schema_with = "anyof_nullable_string_schema")]
    pub name: Option<String>,
    /// Optional plain text description
    #[serde(default)]
    #[schemars(schema_with = "anyof_nullable_string_schema")]
    pub description: Option<String>,
    /// Optional HTML description
    #[serde(default)]
    #[schemars(schema_with = "anyof_nullable_string_schema")]
    pub description_html: Option<String>,
    /// Optional tags to associate with the book
    #[serde(default)]
    #[schemars(schema_with = "anyof_nullable_tags_schema")]
    pub tags: Option<Vec<TagInput>>,
    /// Optional ID of a template page to use as default
    #[serde(default)]
    #[schemars(schema_with = "nullable_integer_schema")]
    pub default_template_id: Option<u64>,
}

/// Arguments for `delete_book` tool.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DeleteBookArgs {
    /// ID of the book to delete
    pub book_id: u64,
    /// Must be explicitly set to true to confirm deletion
    pub confirm: bool,
}

/// Arguments for `list_shelves` tool.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct ListShelvesArgs {
    /// Optional page number for pagination
    #[serde(default)]
    #[schemars(schema_with = "nullable_integer_schema")]
    pub page: Option<u32>,
    /// Optional number of items per page
    #[serde(default)]
    #[schemars(schema_with = "nullable_integer_schema")]
    pub count: Option<u32>,
}

/// Arguments for `create_shelf` tool.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateShelfArgs {
    /// Name / title of the shelf
    pub name: String,
    /// Optional plain text description
    #[serde(default)]
    #[schemars(schema_with = "anyof_nullable_string_schema")]
    pub description: Option<String>,
    /// Optional HTML description
    #[serde(default)]
    #[schemars(schema_with = "anyof_nullable_string_schema")]
    pub description_html: Option<String>,
    /// Optional list of book IDs to include on this shelf
    #[serde(default)]
    #[schemars(schema_with = "anyof_nullable_u64_vec_schema")]
    pub books: Option<Vec<u64>>,
    /// Optional tags to associate with the shelf
    #[serde(default)]
    #[schemars(schema_with = "anyof_nullable_tags_schema")]
    pub tags: Option<Vec<TagInput>>,
}

/// Arguments for `update_shelf` tool.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UpdateShelfArgs {
    /// ID of the shelf to update
    pub shelf_id: u64,
    /// Optional new name / title for the shelf
    #[serde(default)]
    #[schemars(schema_with = "anyof_nullable_string_schema")]
    pub name: Option<String>,
    /// Optional plain text description
    #[serde(default)]
    #[schemars(schema_with = "anyof_nullable_string_schema")]
    pub description: Option<String>,
    /// Optional HTML description
    #[serde(default)]
    #[schemars(schema_with = "anyof_nullable_string_schema")]
    pub description_html: Option<String>,
    /// Optional list of book IDs to set on this shelf
    #[serde(default)]
    #[schemars(schema_with = "anyof_nullable_u64_vec_schema")]
    pub books: Option<Vec<u64>>,
    /// Optional tags to associate with the shelf
    #[serde(default)]
    #[schemars(schema_with = "anyof_nullable_tags_schema")]
    pub tags: Option<Vec<TagInput>>,
}

/// Arguments for `delete_shelf` tool.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DeleteShelfArgs {
    /// ID of the shelf to delete
    pub shelf_id: u64,
    /// Must be explicitly set to true to confirm deletion
    pub confirm: bool,
}

/// Arguments for `list_chapters` tool.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct ListChaptersArgs {
    /// Optional book ID to filter chapters by parent book
    #[serde(default)]
    #[schemars(schema_with = "nullable_integer_schema")]
    pub book_id: Option<u64>,
}

/// Arguments for `read_chapter` tool.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ReadChapterArgs {
    /// ID of the chapter to read
    pub chapter_id: u64,
}

/// Arguments for `create_chapter` tool.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateChapterArgs {
    /// ID of the parent book
    pub book_id: u64,
    /// Name of the chapter
    pub name: String,
    /// Optional plain text description
    #[serde(default)]
    #[schemars(schema_with = "anyof_nullable_string_schema")]
    pub description: Option<String>,
    /// Optional HTML description
    #[serde(default)]
    #[schemars(schema_with = "anyof_nullable_string_schema")]
    pub description_html: Option<String>,
    /// Optional priority order for sorting
    #[serde(default)]
    #[schemars(schema_with = "nullable_integer_schema")]
    pub priority: Option<i64>,
    /// Optional tags to associate with the chapter
    #[serde(default)]
    #[schemars(schema_with = "anyof_nullable_tags_schema")]
    pub tags: Option<Vec<TagInput>>,
    /// Optional ID of a template page to use as default for pages in this chapter
    #[serde(default)]
    #[schemars(schema_with = "nullable_integer_schema")]
    pub default_template_id: Option<u64>,
}

/// Arguments for `update_chapter` tool.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UpdateChapterArgs {
    /// ID of the chapter to update
    pub chapter_id: u64,
    /// Optional new parent book ID
    #[serde(default)]
    #[schemars(schema_with = "nullable_integer_schema")]
    pub book_id: Option<u64>,
    /// Optional new name for the chapter
    #[serde(default)]
    #[schemars(schema_with = "anyof_nullable_string_schema")]
    pub name: Option<String>,
    /// Optional plain text description
    #[serde(default)]
    #[schemars(schema_with = "anyof_nullable_string_schema")]
    pub description: Option<String>,
    /// Optional HTML description
    #[serde(default)]
    #[schemars(schema_with = "anyof_nullable_string_schema")]
    pub description_html: Option<String>,
    /// Optional priority order for sorting
    #[serde(default)]
    #[schemars(schema_with = "nullable_integer_schema")]
    pub priority: Option<i64>,
    /// Optional tags to associate with the chapter
    #[serde(default)]
    #[schemars(schema_with = "anyof_nullable_tags_schema")]
    pub tags: Option<Vec<TagInput>>,
    /// Optional ID of a template page to use as default
    #[serde(default)]
    #[schemars(schema_with = "nullable_integer_schema")]
    pub default_template_id: Option<u64>,
}

/// Arguments for `delete_chapter` tool.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DeleteChapterArgs {
    /// ID of the chapter to delete
    pub chapter_id: u64,
    /// Must be explicitly set to true to confirm deletion
    pub confirm: bool,
}

/// MCP Server implementation for BookStack.
#[derive(Clone)]
pub struct BookStackServer {
    pub client: Arc<BookStackClient>,
    pub tool_router: ToolRouter<Self>,
    pub allow_delete: bool,
}

impl BookStackServer {
    /// Creates a new `BookStackServer` wrapping the given client.
    /// Deletions default to disabled; use `.with_allow_delete(bool)` to configure.
    pub fn new(client: Arc<BookStackClient>) -> Self {
        Self {
            client,
            tool_router: Self::tool_router(),
            allow_delete: false,
        }
    }

    /// Builder method to override deletion permission.
    pub fn with_allow_delete(mut self, allow_delete: bool) -> Self {
        self.allow_delete = allow_delete;
        self
    }

    /// Validates deletion preconditions (both global server permission and explicit confirm flag).
    fn check_deletion_permission(
        &self,
        entity_type: &str,
        id: u64,
        confirm: bool,
    ) -> Result<(), rmcp::ErrorData> {
        if !self.allow_delete {
            return Err(rmcp::ErrorData::invalid_params(
                "Deletions are disabled on this server. Set BOOKSTACK_ALLOW_DELETE=true in the server environment to enable deletion tools.".to_string(),
                None,
            ));
        }

        if !confirm {
            return Err(rmcp::ErrorData::invalid_params(
                format!(
                    "Deletion of {} {} aborted: 'confirm' must be explicitly set to true.",
                    entity_type, id
                ),
                None,
            ));
        }

        Ok(())
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
                out.push_str(&format!(
                    "- **{}** (ID: {}, Slug: `{}`)\n",
                    b.name, b.id, b.slug
                ));
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
                    let name = item
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Untitled");
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
                                let p_slug =
                                    page.get("slug").and_then(|v| v.as_str()).unwrap_or("");
                                out.push_str(&format!(
                                    "  - Page: {} (ID: {}, Slug: `{}`)\n",
                                    p_name, p_id, p_slug
                                ));
                            }
                        }
                        out.push('\n');
                    } else if item_type == "page" {
                        out.push_str(&format!(
                            "- Page: {} (ID: {}, Slug: `{}`)\n",
                            name, id, slug
                        ));
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
                out.push_str(&format!(
                    "- **{}** (ID: {}, Slug: `{}`)\n",
                    s.name, s.id, s.slug
                ));
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

    /// List chapters in the BookStack instance, optionally filtered by book ID.
    #[tool(
        name = "list_chapters",
        description = "List chapters in the BookStack instance, optionally filtered by book ID"
    )]
    pub async fn list_chapters(
        &self,
        params: Parameters<ListChaptersArgs>,
    ) -> Result<String, rmcp::ErrorData> {
        let args = params.0;
        let chapters = self
            .client
            .list_chapters(args.book_id)
            .await
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

        let mut out = String::from("# Chapters\n\n");
        if chapters.is_empty() {
            out.push_str("No chapters found.\n");
        } else {
            for c in &chapters {
                out.push_str(&format!(
                    "- **{}** (ID: {}, Book ID: {}, Slug: `{}`)\n",
                    c.name, c.id, c.book_id, c.slug
                ));
                if let Some(ref desc) = c.description
                    && !desc.trim().is_empty()
                {
                    out.push_str(&format!("  - Description: {}\n", desc.trim()));
                }
            }
        }

        Ok(out)
    }

    /// Create a new page in a book or chapter with Markdown or HTML content.
    #[tool(
        name = "create_page",
        description = "Create a new page in BookStack within a book or chapter"
    )]
    pub async fn create_page(
        &self,
        params: Parameters<CreatePageArgs>,
    ) -> Result<String, rmcp::ErrorData> {
        let args = params.0;
        if args.book_id.is_none() && args.chapter_id.is_none() {
            return Err(rmcp::ErrorData::invalid_params(
                "Either book_id or chapter_id must be provided to create a page.",
                None,
            ));
        }

        let payload = CreatePagePayload {
            name: args.name,
            book_id: args.book_id,
            chapter_id: args.chapter_id,
            markdown: args.markdown,
            html: args.html,
            priority: args.priority,
            tags: args.tags,
        };

        let page = self
            .client
            .create_page(&payload)
            .await
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

        let mut out = format!(
            "# Page Created Successfully\n\n- **Name**: {}\n- **ID**: {}\n- **Slug**: `{}`\n",
            page.name, page.id, page.slug
        );
        if let Some(bid) = page.book_id {
            out.push_str(&format!("- **Book ID**: {}\n", bid));
        }
        if let Some(cid) = page.chapter_id {
            out.push_str(&format!("- **Chapter ID**: {}\n", cid));
        }

        Ok(out)
    }

    /// Update an existing page's name, content, or metadata.
    #[tool(
        name = "update_page",
        description = "Update an existing page's name, Markdown/HTML content, or location in BookStack"
    )]
    pub async fn update_page(
        &self,
        params: Parameters<UpdatePageArgs>,
    ) -> Result<String, rmcp::ErrorData> {
        let args = params.0;
        let payload = UpdatePagePayload {
            name: args.name,
            book_id: args.book_id,
            chapter_id: args.chapter_id,
            markdown: args.markdown,
            html: args.html,
            priority: args.priority,
            tags: args.tags,
        };

        let page = self
            .client
            .update_page(args.page_id, &payload)
            .await
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

        let mut out = format!(
            "# Page Updated Successfully\n\n- **Name**: {}\n- **ID**: {}\n- **Slug**: `{}`\n",
            page.name, page.id, page.slug
        );
        if let Some(bid) = page.book_id {
            out.push_str(&format!("- **Book ID**: {}\n", bid));
        }
        if let Some(cid) = page.chapter_id {
            out.push_str(&format!("- **Chapter ID**: {}\n", cid));
        }

        Ok(out)
    }

    /// Delete a page (moves it to the BookStack recycle bin). Requires confirmation.
    #[tool(
        name = "delete_page",
        description = "Delete a page and send it to the BookStack recycle bin (requires confirm: true)"
    )]
    pub async fn delete_page(
        &self,
        params: Parameters<DeletePageArgs>,
    ) -> Result<String, rmcp::ErrorData> {
        let args = params.0;
        self.check_deletion_permission("page", args.page_id, args.confirm)?;

        self.client
            .delete_page(args.page_id)
            .await
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

        Ok(format!(
            "# Page Deleted Successfully\n\nPage ID {} has been deleted and moved to the recycle bin.\n",
            args.page_id
        ))
    }

    /// Create a new chapter within a book.
    #[tool(
        name = "create_chapter",
        description = "Create a new chapter within a book in BookStack"
    )]
    pub async fn create_chapter(
        &self,
        params: Parameters<CreateChapterArgs>,
    ) -> Result<String, rmcp::ErrorData> {
        let args = params.0;
        let payload = CreateChapterPayload {
            book_id: args.book_id,
            name: args.name,
            description: args.description,
            description_html: args.description_html,
            priority: args.priority,
            tags: args.tags,
            default_template_id: args.default_template_id,
        };

        let chapter = self
            .client
            .create_chapter(&payload)
            .await
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

        Ok(format!(
            "# Chapter Created Successfully\n\n- **Name**: {}\n- **ID**: {}\n- **Book ID**: {}\n- **Slug**: `{}`\n",
            chapter.name, chapter.id, chapter.book_id, chapter.slug
        ))
    }

    /// Update an existing chapter's details.
    #[tool(
        name = "update_chapter",
        description = "Update an existing chapter's name, description, or parent book in BookStack"
    )]
    pub async fn update_chapter(
        &self,
        params: Parameters<UpdateChapterArgs>,
    ) -> Result<String, rmcp::ErrorData> {
        let args = params.0;
        let payload = UpdateChapterPayload {
            book_id: args.book_id,
            name: args.name,
            description: args.description,
            description_html: args.description_html,
            priority: args.priority,
            tags: args.tags,
            default_template_id: args.default_template_id,
        };

        let chapter = self
            .client
            .update_chapter(args.chapter_id, &payload)
            .await
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

        Ok(format!(
            "# Chapter Updated Successfully\n\n- **Name**: {}\n- **ID**: {}\n- **Book ID**: {}\n- **Slug**: `{}`\n",
            chapter.name, chapter.id, chapter.book_id, chapter.slug
        ))
    }

    /// Delete a chapter (moves it to the BookStack recycle bin). Requires confirmation.
    #[tool(
        name = "delete_chapter",
        description = "Delete a chapter and send it to the BookStack recycle bin (requires confirm: true)"
    )]
    pub async fn delete_chapter(
        &self,
        params: Parameters<DeleteChapterArgs>,
    ) -> Result<String, rmcp::ErrorData> {
        let args = params.0;
        self.check_deletion_permission("chapter", args.chapter_id, args.confirm)?;

        self.client
            .delete_chapter(args.chapter_id)
            .await
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

        Ok(format!(
            "# Chapter Deleted Successfully\n\nChapter ID {} has been deleted and moved to the recycle bin.\n",
            args.chapter_id
        ))
    }

    /// Create a new book.
    #[tool(name = "create_book", description = "Create a new book in BookStack")]
    pub async fn create_book(
        &self,
        params: Parameters<CreateBookArgs>,
    ) -> Result<String, rmcp::ErrorData> {
        let args = params.0;
        let payload = CreateBookPayload {
            name: args.name,
            description: args.description,
            description_html: args.description_html,
            tags: args.tags,
            default_template_id: args.default_template_id,
        };

        let book = self
            .client
            .create_book(&payload)
            .await
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

        Ok(format!(
            "# Book Created Successfully\n\n- **Name**: {}\n- **ID**: {}\n- **Slug**: `{}`\n",
            book.name, book.id, book.slug
        ))
    }

    /// Update an existing book's details.
    #[tool(
        name = "update_book",
        description = "Update an existing book's name or description in BookStack"
    )]
    pub async fn update_book(
        &self,
        params: Parameters<UpdateBookArgs>,
    ) -> Result<String, rmcp::ErrorData> {
        let args = params.0;
        let payload = UpdateBookPayload {
            name: args.name,
            description: args.description,
            description_html: args.description_html,
            tags: args.tags,
            default_template_id: args.default_template_id,
        };

        let book = self
            .client
            .update_book(args.book_id, &payload)
            .await
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

        Ok(format!(
            "# Book Updated Successfully\n\n- **Name**: {}\n- **ID**: {}\n- **Slug**: `{}`\n",
            book.name, book.id, book.slug
        ))
    }

    /// Delete a book (moves it to the BookStack recycle bin). Requires confirmation.
    #[tool(
        name = "delete_book",
        description = "Delete a book and send it to the BookStack recycle bin (requires confirm: true)"
    )]
    pub async fn delete_book(
        &self,
        params: Parameters<DeleteBookArgs>,
    ) -> Result<String, rmcp::ErrorData> {
        let args = params.0;
        self.check_deletion_permission("book", args.book_id, args.confirm)?;

        self.client
            .delete_book(args.book_id)
            .await
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

        Ok(format!(
            "# Book Deleted Successfully\n\nBook ID {} has been deleted and moved to the recycle bin.\n",
            args.book_id
        ))
    }

    /// Create a new bookshelf.
    #[tool(
        name = "create_shelf",
        description = "Create a new bookshelf in BookStack"
    )]
    pub async fn create_shelf(
        &self,
        params: Parameters<CreateShelfArgs>,
    ) -> Result<String, rmcp::ErrorData> {
        let args = params.0;
        let payload = CreateShelfPayload {
            name: args.name,
            description: args.description,
            description_html: args.description_html,
            books: args.books,
            tags: args.tags,
        };

        let shelf = self
            .client
            .create_shelf(&payload)
            .await
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

        Ok(format!(
            "# Shelf Created Successfully\n\n- **Name**: {}\n- **ID**: {}\n- **Slug**: `{}`\n",
            shelf.name, shelf.id, shelf.slug
        ))
    }

    /// Update an existing bookshelf.
    #[tool(
        name = "update_shelf",
        description = "Update an existing bookshelf's name, description, or assigned books in BookStack"
    )]
    pub async fn update_shelf(
        &self,
        params: Parameters<UpdateShelfArgs>,
    ) -> Result<String, rmcp::ErrorData> {
        let args = params.0;
        let payload = UpdateShelfPayload {
            name: args.name,
            description: args.description,
            description_html: args.description_html,
            books: args.books,
            tags: args.tags,
        };

        let shelf = self
            .client
            .update_shelf(args.shelf_id, &payload)
            .await
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

        Ok(format!(
            "# Shelf Updated Successfully\n\n- **Name**: {}\n- **ID**: {}\n- **Slug**: `{}`\n",
            shelf.name, shelf.id, shelf.slug
        ))
    }

    /// Delete a bookshelf (moves it to the BookStack recycle bin). Requires confirmation.
    #[tool(
        name = "delete_shelf",
        description = "Delete a bookshelf and send it to the BookStack recycle bin (requires confirm: true)"
    )]
    pub async fn delete_shelf(
        &self,
        params: Parameters<DeleteShelfArgs>,
    ) -> Result<String, rmcp::ErrorData> {
        let args = params.0;
        self.check_deletion_permission("shelf", args.shelf_id, args.confirm)?;

        self.client
            .delete_shelf(args.shelf_id)
            .await
            .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

        Ok(format!(
            "# Shelf Deleted Successfully\n\nShelf ID {} has been deleted and moved to the recycle bin.\n",
            args.shelf_id
        ))
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
        .replace("</i>", "*");

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

    clean
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#039;", "'")
        .replace("&nbsp;", " ")
        .replace("&amp;", "&")
        .trim()
        .to_string()
}
