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
