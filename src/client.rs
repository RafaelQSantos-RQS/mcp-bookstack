use crate::config::Config;
use crate::models::{
    BookDetails, BookListItem, ChapterDetails, ChapterListItem, CreateBookPayload,
    CreateChapterPayload, CreatePagePayload, CreateShelfPayload, ListResponse, PageDetails,
    SearchResponse, ShelfDetails, ShelfListItem, UpdateBookPayload, UpdateChapterPayload,
    UpdatePagePayload, UpdateShelfPayload,
};
use anyhow::{bail, Context, Result};
use reqwest::Client;

/// HTTP client wrapper for interacting with the BookStack REST API.
#[derive(Debug, Clone)]
pub struct BookStackClient {
    pub client: Client,
    pub base_url: String,
    pub auth_header: String,
    pub allow_delete: bool,
}

impl BookStackClient {
    /// Creates a new `BookStackClient` from the given configuration.
    pub fn new(config: &Config) -> Self {
        Self {
            client: Client::new(),
            base_url: config.url.clone(),
            auth_header: config.auth_header(),
            allow_delete: config.allow_delete,
        }
    }

    /// Creates a new `BookStackClient` with a customized `reqwest::Client`.
    pub fn with_client(config: &Config, client: Client) -> Self {
        Self {
            client,
            base_url: config.url.clone(),
            auth_header: config.auth_header(),
            allow_delete: config.allow_delete,
        }
    }

    /// URL helper for the search endpoint.
    pub fn search_url(&self) -> String {
        format!("{}/api/search", self.base_url)
    }

    /// URL helper for the books listing endpoint.
    pub fn books_url(&self) -> String {
        format!("{}/api/books", self.base_url)
    }

    /// URL helper for an individual book endpoint.
    pub fn book_url(&self, id: u64) -> String {
        format!("{}/api/books/{}", self.base_url, id)
    }

    /// URL helper for the chapters listing endpoint.
    pub fn chapters_url(&self) -> String {
        format!("{}/api/chapters", self.base_url)
    }

    /// URL helper for an individual chapter endpoint.
    pub fn chapter_url(&self, id: u64) -> String {
        format!("{}/api/chapters/{}", self.base_url, id)
    }

    /// URL helper for the pages listing endpoint.
    pub fn pages_url(&self) -> String {
        format!("{}/api/pages", self.base_url)
    }

    /// URL helper for an individual page endpoint.
    pub fn page_url(&self, id: u64) -> String {
        format!("{}/api/pages/{}", self.base_url, id)
    }

    /// URL helper for exporting a page in Markdown.
    pub fn page_markdown_url(&self, id: u64) -> String {
        format!("{}/api/pages/{}/export/markdown", self.base_url, id)
    }

    /// URL helper for the shelves listing endpoint.
    pub fn shelves_url(&self) -> String {
        format!("{}/api/shelves", self.base_url)
    }

    /// URL helper for an individual shelf endpoint.
    pub fn shelf_url(&self, id: u64) -> String {
        format!("{}/api/shelves/{}", self.base_url, id)
    }

    /// Search across BookStack content (books, chapters, pages, shelves).
    /// Calls `GET {base_url}/api/search`.
    pub async fn search(
        &self,
        query: &str,
        page: Option<u32>,
        count: Option<u32>,
    ) -> Result<SearchResponse> {
        let mut req = self
            .client
            .get(self.search_url())
            .header(reqwest::header::AUTHORIZATION, &self.auth_header)
            .query(&[("query", query)]);

        if let Some(p) = page {
            req = req.query(&[("page", p.to_string())]);
        }
        if let Some(c) = count {
            req = req.query(&[("count", c.to_string())]);
        }

        let resp = req.send().await.context("Failed to send search request")?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            bail!("Search request failed with status {}: {}", status, body);
        }

        let result = resp
            .json::<SearchResponse>()
            .await
            .context("Failed to deserialize search response")?;
        Ok(result)
    }

    /// Fetch page markdown.
    /// Calls `GET {base_url}/api/pages/{id}/export/markdown`.
    /// If markdown endpoint returns empty or non-200, fallbacks to `get_page(id)` and returns
    /// `page.markdown`, `page.raw_html`, or `page.html`.
    pub async fn get_page_markdown(&self, id: u64) -> Result<String> {
        let export_url = self.page_markdown_url(id);
        let resp = self
            .client
            .get(&export_url)
            .header(reqwest::header::AUTHORIZATION, &self.auth_header)
            .send()
            .await;

        if let Ok(res) = resp
            && res.status().is_success()
        {
            let text = res.text().await.unwrap_or_default();
            if !text.trim().is_empty() {
                return Ok(text);
            }
        }

        // Fallback to get_page(id)
        let page = self.get_page(id).await?;
        if let Some(md) = page.markdown
            && !md.trim().is_empty()
        {
            return Ok(md);
        }
        if let Some(raw) = page.raw_html
            && !raw.trim().is_empty()
        {
            return Ok(raw);
        }
        if let Some(html) = page.html
            && !html.trim().is_empty()
        {
            return Ok(html);
        }

        bail!("Page {} has no markdown or HTML content available", id)
    }

    /// Fetch page details.
    /// Calls `GET {base_url}/api/pages/{id}`.
    pub async fn get_page(&self, id: u64) -> Result<PageDetails> {
        let url = self.page_url(id);
        let resp = self
            .client
            .get(&url)
            .header(reqwest::header::AUTHORIZATION, &self.auth_header)
            .send()
            .await
            .with_context(|| format!("Failed to request page {}", id))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            bail!("Get page {} failed with status {}: {}", id, status, body);
        }

        let page = resp
            .json::<PageDetails>()
            .await
            .with_context(|| format!("Failed to deserialize page {} details", id))?;
        Ok(page)
    }

    /// List books.
    /// Calls `GET {base_url}/api/books` (unwrapping the `data` array).
    pub async fn list_books(
        &self,
        page: Option<u32>,
        count: Option<u32>,
    ) -> Result<Vec<BookListItem>> {
        let mut req = self
            .client
            .get(self.books_url())
            .header(reqwest::header::AUTHORIZATION, &self.auth_header);

        if let Some(p) = page {
            req = req.query(&[("page", p.to_string())]);
        }
        if let Some(c) = count {
            req = req.query(&[("count", c.to_string())]);
        }

        let resp = req.send().await.context("Failed to send list_books request")?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            bail!("List books failed with status {}: {}", status, body);
        }

        let list_resp = resp
            .json::<ListResponse<BookListItem>>()
            .await
            .context("Failed to deserialize books list response")?;
        Ok(list_resp.data)
    }

    /// Get book details.
    /// Calls `GET {base_url}/api/books/{id}`.
    pub async fn get_book(&self, id: u64) -> Result<BookDetails> {
        let url = self.book_url(id);
        let resp = self
            .client
            .get(&url)
            .header(reqwest::header::AUTHORIZATION, &self.auth_header)
            .send()
            .await
            .with_context(|| format!("Failed to request book {}", id))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            bail!("Get book {} failed with status {}: {}", id, status, body);
        }

        let book = resp
            .json::<BookDetails>()
            .await
            .with_context(|| format!("Failed to deserialize book {} details", id))?;
        Ok(book)
    }

    /// List chapters.
    /// Calls `GET {base_url}/api/chapters`.
    pub async fn list_chapters(&self, book_id: Option<u64>) -> Result<Vec<ChapterListItem>> {
        let mut req = self
            .client
            .get(self.chapters_url())
            .header(reqwest::header::AUTHORIZATION, &self.auth_header);

        if let Some(bid) = book_id {
            req = req.query(&[("filter[book_id]", bid.to_string())]);
        }

        let resp = req.send().await.context("Failed to send list_chapters request")?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            bail!("List chapters failed with status {}: {}", status, body);
        }

        let list_resp = resp
            .json::<ListResponse<ChapterListItem>>()
            .await
            .context("Failed to deserialize chapters list response")?;
        Ok(list_resp.data)
    }

    /// Get chapter details.
    /// Calls `GET {base_url}/api/chapters/{id}`.
    pub async fn get_chapter(&self, id: u64) -> Result<ChapterDetails> {
        let url = self.chapter_url(id);
        let resp = self
            .client
            .get(&url)
            .header(reqwest::header::AUTHORIZATION, &self.auth_header)
            .send()
            .await
            .with_context(|| format!("Failed to request chapter {}", id))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            bail!("Get chapter {} failed with status {}: {}", id, status, body);
        }

        let chapter = resp
            .json::<ChapterDetails>()
            .await
            .with_context(|| format!("Failed to deserialize chapter {} details", id))?;
        Ok(chapter)
    }

    /// List shelves with optional pagination.
    /// Calls `GET {base_url}/api/shelves`.
    pub async fn list_shelves(
        &self,
        page: Option<u32>,
        count: Option<u32>,
    ) -> Result<Vec<ShelfListItem>> {
        let mut req = self
            .client
            .get(self.shelves_url())
            .header(reqwest::header::AUTHORIZATION, &self.auth_header);

        if let Some(p) = page {
            req = req.query(&[("page", p.to_string())]);
        }
        if let Some(c) = count {
            req = req.query(&[("count", c.to_string())]);
        }

        let resp = req
            .send()
            .await
            .context("Failed to send list_shelves request")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            bail!("List shelves failed with status {}: {}", status, body);
        }

        let list_resp = resp
            .json::<ListResponse<ShelfListItem>>()
            .await
            .context("Failed to deserialize shelves list response")?;
        Ok(list_resp.data)
    }

    /// Get shelf details.
    /// Calls `GET {base_url}/api/shelves/{id}`.
    pub async fn get_shelf(&self, id: u64) -> Result<ShelfDetails> {
        let url = self.shelf_url(id);
        let resp = self
            .client
            .get(&url)
            .header(reqwest::header::AUTHORIZATION, &self.auth_header)
            .send()
            .await
            .with_context(|| format!("Failed to request shelf {}", id))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            bail!("Get shelf {} failed with status {}: {}", id, status, body);
        }

        let shelf = resp
            .json::<ShelfDetails>()
            .await
            .with_context(|| format!("Failed to deserialize shelf {} details", id))?;
        Ok(shelf)
    }

    /// Create a new page.
    /// Calls `POST {base_url}/api/pages`.
    pub async fn create_page(&self, payload: &CreatePagePayload) -> Result<PageDetails> {
        let resp = self
            .client
            .post(self.pages_url())
            .header(reqwest::header::AUTHORIZATION, &self.auth_header)
            .json(payload)
            .send()
            .await
            .context("Failed to send create_page request")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            bail!("Create page failed with status {}: {}", status, body);
        }

        let page = resp
            .json::<PageDetails>()
            .await
            .context("Failed to deserialize created page details")?;
        Ok(page)
    }

    /// Update an existing page.
    /// Calls `PUT {base_url}/api/pages/{id}`.
    pub async fn update_page(&self, id: u64, payload: &UpdatePagePayload) -> Result<PageDetails> {
        let resp = self
            .client
            .put(self.page_url(id))
            .header(reqwest::header::AUTHORIZATION, &self.auth_header)
            .json(payload)
            .send()
            .await
            .with_context(|| format!("Failed to send update_page request for page {}", id))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            bail!("Update page {} failed with status {}: {}", id, status, body);
        }

        let page = resp
            .json::<PageDetails>()
            .await
            .with_context(|| format!("Failed to deserialize updated page {} details", id))?;
        Ok(page)
    }

    /// Delete a page.
    /// Calls `DELETE {base_url}/api/pages/{id}`.
    pub async fn delete_page(&self, id: u64) -> Result<()> {
        let resp = self
            .client
            .delete(self.page_url(id))
            .header(reqwest::header::AUTHORIZATION, &self.auth_header)
            .send()
            .await
            .with_context(|| format!("Failed to send delete_page request for page {}", id))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            bail!("Delete page {} failed with status {}: {}", id, status, body);
        }

        Ok(())
    }

    /// Create a new chapter.
    /// Calls `POST {base_url}/api/chapters`.
    pub async fn create_chapter(&self, payload: &CreateChapterPayload) -> Result<ChapterDetails> {
        let resp = self
            .client
            .post(self.chapters_url())
            .header(reqwest::header::AUTHORIZATION, &self.auth_header)
            .json(payload)
            .send()
            .await
            .context("Failed to send create_chapter request")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            bail!("Create chapter failed with status {}: {}", status, body);
        }

        let chapter = resp
            .json::<ChapterDetails>()
            .await
            .context("Failed to deserialize created chapter details")?;
        Ok(chapter)
    }

    /// Update an existing chapter.
    /// Calls `PUT {base_url}/api/chapters/{id}`.
    pub async fn update_chapter(
        &self,
        id: u64,
        payload: &UpdateChapterPayload,
    ) -> Result<ChapterDetails> {
        let resp = self
            .client
            .put(self.chapter_url(id))
            .header(reqwest::header::AUTHORIZATION, &self.auth_header)
            .json(payload)
            .send()
            .await
            .with_context(|| format!("Failed to send update_chapter request for chapter {}", id))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            bail!(
                "Update chapter {} failed with status {}: {}",
                id,
                status,
                body
            );
        }

        let chapter = resp
            .json::<ChapterDetails>()
            .await
            .with_context(|| format!("Failed to deserialize updated chapter {} details", id))?;
        Ok(chapter)
    }

    /// Delete a chapter.
    /// Calls `DELETE {base_url}/api/chapters/{id}`.
    pub async fn delete_chapter(&self, id: u64) -> Result<()> {
        let resp = self
            .client
            .delete(self.chapter_url(id))
            .header(reqwest::header::AUTHORIZATION, &self.auth_header)
            .send()
            .await
            .with_context(|| format!("Failed to send delete_chapter request for chapter {}", id))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            bail!(
                "Delete chapter {} failed with status {}: {}",
                id,
                status,
                body
            );
        }

        Ok(())
    }

    /// Create a new book.
    /// Calls `POST {base_url}/api/books`.
    pub async fn create_book(&self, payload: &CreateBookPayload) -> Result<BookDetails> {
        let resp = self
            .client
            .post(self.books_url())
            .header(reqwest::header::AUTHORIZATION, &self.auth_header)
            .json(payload)
            .send()
            .await
            .context("Failed to send create_book request")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            bail!("Create book failed with status {}: {}", status, body);
        }

        let book = resp
            .json::<BookDetails>()
            .await
            .context("Failed to deserialize created book details")?;
        Ok(book)
    }

    /// Update an existing book.
    /// Calls `PUT {base_url}/api/books/{id}`.
    pub async fn update_book(&self, id: u64, payload: &UpdateBookPayload) -> Result<BookDetails> {
        let resp = self
            .client
            .put(self.book_url(id))
            .header(reqwest::header::AUTHORIZATION, &self.auth_header)
            .json(payload)
            .send()
            .await
            .with_context(|| format!("Failed to send update_book request for book {}", id))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            bail!("Update book {} failed with status {}: {}", id, status, body);
        }

        let book = resp
            .json::<BookDetails>()
            .await
            .with_context(|| format!("Failed to deserialize updated book {} details", id))?;
        Ok(book)
    }

    /// Delete a book.
    /// Calls `DELETE {base_url}/api/books/{id}`.
    pub async fn delete_book(&self, id: u64) -> Result<()> {
        let resp = self
            .client
            .delete(self.book_url(id))
            .header(reqwest::header::AUTHORIZATION, &self.auth_header)
            .send()
            .await
            .with_context(|| format!("Failed to send delete_book request for book {}", id))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            bail!("Delete book {} failed with status {}: {}", id, status, body);
        }

        Ok(())
    }

    /// Create a new shelf.
    /// Calls `POST {base_url}/api/shelves`.
    pub async fn create_shelf(&self, payload: &CreateShelfPayload) -> Result<ShelfDetails> {
        let resp = self
            .client
            .post(self.shelves_url())
            .header(reqwest::header::AUTHORIZATION, &self.auth_header)
            .json(payload)
            .send()
            .await
            .context("Failed to send create_shelf request")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            bail!("Create shelf failed with status {}: {}", status, body);
        }

        let shelf = resp
            .json::<ShelfDetails>()
            .await
            .context("Failed to deserialize created shelf details")?;
        Ok(shelf)
    }

    /// Update an existing shelf.
    /// Calls `PUT {base_url}/api/shelves/{id}`.
    pub async fn update_shelf(&self, id: u64, payload: &UpdateShelfPayload) -> Result<ShelfDetails> {
        let resp = self
            .client
            .put(self.shelf_url(id))
            .header(reqwest::header::AUTHORIZATION, &self.auth_header)
            .json(payload)
            .send()
            .await
            .with_context(|| format!("Failed to send update_shelf request for shelf {}", id))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            bail!("Update shelf {} failed with status {}: {}", id, status, body);
        }

        let shelf = resp
            .json::<ShelfDetails>()
            .await
            .with_context(|| format!("Failed to deserialize updated shelf {} details", id))?;
        Ok(shelf)
    }

    /// Delete a shelf.
    /// Calls `DELETE {base_url}/api/shelves/{id}`.
    pub async fn delete_shelf(&self, id: u64) -> Result<()> {
        let resp = self
            .client
            .delete(self.shelf_url(id))
            .header(reqwest::header::AUTHORIZATION, &self.auth_header)
            .send()
            .await
            .with_context(|| format!("Failed to send delete_shelf request for shelf {}", id))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            bail!("Delete shelf {} failed with status {}: {}", id, status, body);
        }

        Ok(())
    }
}
