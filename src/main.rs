use std::sync::Arc;

use anyhow::Context;
use mcp_bookstack::{BookStackClient, BookStackServer, Config};
use rmcp::ServiceExt;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Starting BookStack MCP server...");

    let config = Config::from_env().context("Failed to load BookStack configuration from environment")?;
    let client = Arc::new(BookStackClient::new(&config));
    let server = BookStackServer::new(client);

    info!("Serving BookStack MCP over stdio...");
    let running_server = server.serve(rmcp::transport::stdio()).await?;
    running_server.waiting().await?;

    info!("BookStack MCP server shut down cleanly.");
    Ok(())
}
