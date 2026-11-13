use mcp_bookstack::Config;
use mcp_bookstack::config::format_auth_header;
use std::sync::Mutex;

static ENV_MUTEX: Mutex<()> = Mutex::new(());

#[test]
fn test_format_auth_header() {
    let header = format_auth_header("my-id", "my-secret");
    assert_eq!(header, "Token my-id:my-secret");
}

#[test]
fn test_valid_config_and_strip_trailing_slash() {
    let config = Config::new("https://bookstack.example.com/", "my_id", "my_secret").unwrap();
    assert_eq!(config.url, "https://bookstack.example.com");
    assert_eq!(config.token_id, "my_id");
    assert_eq!(config.token_secret, "my_secret");
    assert_eq!(config.auth_header(), "Token my_id:my_secret");

    // Multiple slashes
    let config2 = Config::new("https://bookstack.example.com///", "my_id", "my_secret").unwrap();
    assert_eq!(config2.url, "https://bookstack.example.com");

    // Domain without scheme auto-prepends https://
    let config3 = Config::new("ndsi-docs.senaicimatec.com.br", "my_id", "my_secret").unwrap();
    assert_eq!(config3.url, "https://ndsi-docs.senaicimatec.com.br");
}

#[test]
fn test_config_empty_url() {
    let err = Config::new("", "my_id", "my_secret").unwrap_err();
    assert!(err.to_string().contains("BOOKSTACK_URL cannot be empty"));

    let err_ws = Config::new("   ", "my_id", "my_secret").unwrap_err();
    assert!(err_ws.to_string().contains("BOOKSTACK_URL cannot be empty"));
}

#[test]
fn test_config_empty_token_id() {
    let err = Config::new("https://example.com", "", "my_secret").unwrap_err();
    assert!(
        err.to_string()
            .contains("BOOKSTACK_TOKEN_ID cannot be empty")
    );

    let err_ws = Config::new("https://example.com", "  ", "my_secret").unwrap_err();
    assert!(
        err_ws
            .to_string()
            .contains("BOOKSTACK_TOKEN_ID cannot be empty")
    );
}

#[test]
fn test_config_empty_token_secret() {
    let err = Config::new("https://example.com", "my_id", "").unwrap_err();
    assert!(
        err.to_string()
            .contains("BOOKSTACK_TOKEN_SECRET cannot be empty")
    );

    let err_ws = Config::new("https://example.com", "my_id", "  ").unwrap_err();
    assert!(
        err_ws
            .to_string()
            .contains("BOOKSTACK_TOKEN_SECRET cannot be empty")
    );
}

#[test]
fn test_from_env_success_and_failure() {
    let _guard = ENV_MUTEX.lock().unwrap();

    unsafe {
        std::env::set_var("BOOKSTACK_URL", "https://docs.local/api-test/");
        std::env::set_var("BOOKSTACK_TOKEN_ID", "token_123");
        std::env::set_var("BOOKSTACK_TOKEN_SECRET", "secret_456");
    }

    let config = Config::from_env().unwrap();
    assert_eq!(config.url, "https://docs.local/api-test");
    assert_eq!(config.token_id, "token_123");
    assert_eq!(config.token_secret, "secret_456");
    assert_eq!(config.auth_header(), "Token token_123:secret_456");
    assert!(!config.allow_delete);

    // Test with BOOKSTACK_ALLOW_DELETE=true
    unsafe {
        std::env::set_var("BOOKSTACK_ALLOW_DELETE", "true");
    }
    let config_with_delete = Config::from_env().unwrap();
    assert!(config_with_delete.allow_delete);

    unsafe {
        std::env::remove_var("BOOKSTACK_ALLOW_DELETE");
        std::env::remove_var("BOOKSTACK_URL");
        std::env::remove_var("BOOKSTACK_TOKEN_ID");
        std::env::remove_var("BOOKSTACK_TOKEN_SECRET");
    }

    let err = Config::from_env().unwrap_err();
    assert!(err.to_string().contains("BOOKSTACK_URL"));
}

#[test]
fn test_config_with_allow_delete_builder() {
    let config = Config::new("https://example.com", "id", "secret")
        .unwrap()
        .with_allow_delete(true);
    assert!(config.allow_delete);

    let config_default = Config::new("https://example.com", "id", "secret").unwrap();
    assert!(!config_default.allow_delete);
}
