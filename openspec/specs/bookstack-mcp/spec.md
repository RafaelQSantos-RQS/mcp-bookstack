# bookstack-mcp Specification

## Purpose
Provides a read-only Model Context Protocol (MCP) server for querying, reading, and exploring documentation stored in BookStack instances.

## Requirements

### Requirement: BookStack Authentication and Configuration
The server SHALL read instance credentials from environment variables and authenticate requests with the BookStack API token format.

#### Scenario: Valid credentials configured
- **WHEN** the server starts with `BOOKSTACK_URL`, `BOOKSTACK_TOKEN_ID`, and `BOOKSTACK_TOKEN_SECRET` set
- **THEN** HTTP requests to BookStack include the header `Authorization: Token <token_id>:<token_secret>`

#### Scenario: Missing credentials
- **WHEN** any required environment variable is missing
- **THEN** the server halts immediately with an informative error message on stderr

### Requirement: Content Search Tool
The server SHALL expose a `search_content` tool that executes full-text searches across shelves, books, chapters, and pages.

#### Scenario: Searching content
- **WHEN** the client invokes `search_content` with a query string and optional count and page
- **THEN** the server queries `/api/search` and returns the matching results with title, type, id, slug, and preview text

### Requirement: Page Reading Tool
The server SHALL expose a `read_page` tool that returns page content formatted in Markdown.

#### Scenario: Read page in markdown
- **WHEN** the client invokes `read_page` with a valid page ID
- **THEN** the server fetches content via `/api/pages/{id}/export/markdown` and returns the markdown text

#### Scenario: Page not found
- **WHEN** the client invokes `read_page` with a non-existent page ID
- **THEN** the server returns an error response indicating the page was not found

### Requirement: Books and Shelves Listing Tools
The server SHALL expose tools to list bookshelves, list books, and inspect book details.

#### Scenario: Listing books
- **WHEN** the client invokes `list_books`
- **THEN** the server queries `/api/books` and returns an array of books with IDs, slugs, and names

#### Scenario: Getting book details
- **WHEN** the client invokes `get_book_details` with a book ID
- **THEN** the server queries `/api/books/{id}` and returns the book metadata along with its chapters and child pages

#### Scenario: Listing shelves
- **WHEN** the client invokes `list_shelves`
- **THEN** the server queries `/api/shelves` and returns an array of shelves with IDs, slugs, and names

### Requirement: Chapter Reading Tool
The server SHALL expose a `read_chapter` tool that retrieves chapter metadata and child pages.

#### Scenario: Reading chapter details
- **WHEN** the client invokes `read_chapter` with a chapter ID
- **THEN** the server queries `/api/chapters/{id}` and returns chapter metadata and its list of pages

### Requirement: Safe Transport and Logging Isolation
The server SHALL communicate JSON-RPC messages exclusively over standard input/output (stdio) and direct all diagnostic logs to stderr.

#### Scenario: Server initialization and logging
- **WHEN** an MCP client sends an initialize request via stdin
- **THEN** the server responds on stdout with valid JSON-RPC initialization and outputs diagnostic logs solely to stderr

### Requirement: Page Management Tools
The server SHALL expose `create_page` and `update_page` tools to create new pages and update existing pages in BookStack, accepting Markdown or HTML content.

#### Scenario: Creating a page with markdown
- **WHEN** the client invokes `create_page` with a name, a target book_id or chapter_id, and markdown content
- **THEN** the server sends `POST /api/pages` and returns the created page's ID, slug, and name

#### Scenario: Updating an existing page
- **WHEN** the client invokes `update_page` with a valid page_id and updated markdown or metadata
- **THEN** the server sends `PUT /api/pages/{id}` and returns the updated page summary

### Requirement: Chapter Management Tools
The server SHALL expose `create_chapter` and `update_chapter` tools to manage chapters within a book.

#### Scenario: Creating a chapter
- **WHEN** the client invokes `create_chapter` with a target book_id and chapter name
- **THEN** the server sends `POST /api/chapters` and returns the created chapter's ID and slug

#### Scenario: Updating a chapter
- **WHEN** the client invokes `update_chapter` with a valid chapter_id and updated attributes
- **THEN** the server sends `PUT /api/chapters/{id}` and returns the updated chapter details

### Requirement: Book Management Tools
The server SHALL expose `create_book` and `update_book` tools to create and update books.

#### Scenario: Creating a book
- **WHEN** the client invokes `create_book` with a book name and optional description
- **THEN** the server sends `POST /api/books` and returns the created book's ID, slug, and details

#### Scenario: Updating a book
- **WHEN** the client invokes `update_book` with a valid book_id and updated fields
- **THEN** the server sends `PUT /api/books/{id}` and returns the updated book details

### Requirement: Shelf Management Tools
The server SHALL expose `create_shelf` and `update_shelf` tools to create and organize shelves of books.

#### Scenario: Creating a shelf
- **WHEN** the client invokes `create_shelf` with a name and optional list of book IDs
- **THEN** the server sends `POST /api/shelves` and returns the created shelf details

#### Scenario: Updating a shelf
- **WHEN** the client invokes `update_shelf` with a valid shelf_id and updated book IDs or description
- **THEN** the server sends `PUT /api/shelves/{id}` and returns the updated shelf details

### Requirement: Guarded Deletion Tools
The server SHALL expose `delete_page`, `delete_chapter`, `delete_book`, and `delete_shelf` tools guarded by both a server configuration flag and explicit confirmation.

#### Scenario: Deletion attempted when disabled on server
- **WHEN** any deletion tool is called and `BOOKSTACK_ALLOW_DELETE` is not set to `true`
- **THEN** the server aborts the request immediately with an error indicating deletions are disabled

#### Scenario: Deletion attempted without confirmation flag
- **WHEN** any deletion tool is called with `confirm: false` or missing
- **THEN** the server aborts the request immediately with an error stating confirm must be true

#### Scenario: Successful confirmed deletion
- **WHEN** a deletion tool is called with `BOOKSTACK_ALLOW_DELETE=true`, `confirm: true`, and a valid entity ID
- **THEN** the server sends `DELETE` to the corresponding BookStack API endpoint and returns a confirmation message
