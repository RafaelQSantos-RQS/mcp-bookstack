use anyhow::{Context, Result, bail};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub url: String,
    pub token_id: String,
    pub token_secret: String,
    pub allow_delete: bool,
}

impl Config {
    /// Loads configuration from the environment variables:
    /// - `BOOKSTACK_URL`
    /// - `BOOKSTACK_TOKEN_ID`
    /// - `BOOKSTACK_TOKEN_SECRET`
    /// - `BOOKSTACK_ALLOW_DELETE` (optional, defaults to false)
    ///
    /// Automatically reads a `.env` file if present in the current working directory.
    pub fn from_env() -> Result<Self> {
        let dotenv = parse_dotenv();

        let url = get_var("BOOKSTACK_URL", &dotenv)
            .context("BOOKSTACK_URL environment variable is required")?;
        let token_id = get_var("BOOKSTACK_TOKEN_ID", &dotenv)
            .context("BOOKSTACK_TOKEN_ID environment variable is required")?;
        let token_secret = get_var("BOOKSTACK_TOKEN_SECRET", &dotenv)
            .context("BOOKSTACK_TOKEN_SECRET environment variable is required")?;

        let allow_delete = get_var("BOOKSTACK_ALLOW_DELETE", &dotenv)
            .map(|v| {
                let v = v.trim().to_lowercase();
                v == "true" || v == "1" || v == "yes"
            })
            .unwrap_or(false);

        let mut config = Self::new(url, token_id, token_secret)?;
        config.allow_delete = allow_delete;
        Ok(config)
    }

    /// Creates a new `Config`, validating that fields are non-empty,
    /// auto-prepending `https://` if protocol scheme is omitted,
    /// and stripping trailing slashes from the URL.
    /// `allow_delete` defaults to `false`.
    pub fn new(
        url: impl Into<String>,
        token_id: impl Into<String>,
        token_secret: impl Into<String>,
    ) -> Result<Self> {
        let mut url = url.into().trim().to_string();
        let token_id = token_id.into().trim().to_string();
        let token_secret = token_secret.into().trim().to_string();

        if url.is_empty() {
            bail!("BOOKSTACK_URL cannot be empty");
        }
        if token_id.is_empty() {
            bail!("BOOKSTACK_TOKEN_ID cannot be empty");
        }
        if token_secret.is_empty() {
            bail!("BOOKSTACK_TOKEN_SECRET cannot be empty");
        }

        // Auto-prepend https:// if scheme is missing
        if !url.starts_with("http://") && !url.starts_with("https://") {
            url = format!("https://{}", url);
        }

        // Strip trailing slash(es)
        while url.ends_with('/') {
            url.pop();
        }

        Ok(Self {
            url,
            token_id,
            token_secret,
            allow_delete: false,
        })
    }

    /// Builder method to specify whether delete operations are permitted.
    pub fn with_allow_delete(mut self, allow_delete: bool) -> Self {
        self.allow_delete = allow_delete;
        self
    }

    /// Formats the Authorization header value for this configuration.
    pub fn auth_header(&self) -> String {
        format_auth_header(&self.token_id, &self.token_secret)
    }
}

/// Helper function to format the BookStack Authorization header.
pub fn format_auth_header(token_id: &str, token_secret: &str) -> String {
    format!("Token {}:{}", token_id, token_secret)
}

/// Helper to look up an environment variable with fallback to a parsed `.env` map.
fn get_var(key: &str, dotenv: &std::collections::HashMap<String, String>) -> Option<String> {
    std::env::var(key).ok().or_else(|| dotenv.get(key).cloned())
}

/// Simple built-in reader for `.env` file in the current working directory,
/// safely parsing key-value pairs without mutating the global process environment.
fn parse_dotenv() -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();
    if let Ok(contents) = std::fs::read_to_string(".env") {
        for line in contents.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim().trim_matches('"').trim_matches('\'');
                if !key.is_empty() {
                    map.insert(key.to_string(), value.to_string());
                }
            }
        }
    }
    map
}
