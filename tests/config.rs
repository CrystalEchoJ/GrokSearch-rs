use grok_search_rs::config::{self, AuthMode, Config, Transport};

#[test]
fn config_reads_grok_search_responses_defaults() {
    let cfg = Config::from_env_map([
        ("GROK_SEARCH_API_KEY", "grok-test-key"),
        ("TAVILY_API_KEY", "tvly-test-key"),
    ]);

    assert_eq!(cfg.grok_api_url, "https://api.x.ai/v1");
    assert_eq!(cfg.grok_model, "grok-4-1-fast-reasoning");
    assert!(cfg.web_search_enabled);
    assert!(!cfg.x_search_enabled);
    assert_eq!(cfg.tavily_api_url, "https://api.tavily.com");
    assert!(cfg.tavily_enabled);
    assert_eq!(cfg.default_extra_sources, 3);
    assert_eq!(cfg.fallback_sources, 5);
    assert_eq!(cfg.timeout.as_secs(), 60);
    assert_eq!(cfg.grok_auth_mode, AuthMode::ApiKey);
}

#[test]
fn config_reads_oauth_auth_mode_from_env() {
    let cfg = Config::from_env_map([("GROK_SEARCH_AUTH_MODE", "oauth")]);

    assert_eq!(cfg.grok_auth_mode, AuthMode::OAuth);
    assert_eq!(cfg.transport, Transport::Responses);
}

#[test]
fn unknown_auth_mode_falls_back_to_api_key() {
    let cfg = Config::from_env_map([("GROK_SEARCH_AUTH_MODE", "something_else")]);

    assert_eq!(cfg.grok_auth_mode, AuthMode::ApiKey);
}

#[test]
fn oauth_transport_wins_over_openai_compatible_config() {
    let cfg = Config::from_env_map([
        ("GROK_SEARCH_AUTH_MODE", "oauth"),
        ("OPENAI_COMPATIBLE_API_URL", "https://example.com/v1"),
        ("OPENAI_COMPATIBLE_API_KEY", "sk-fake"),
    ]);

    assert_eq!(cfg.grok_auth_mode, AuthMode::OAuth);
    assert_eq!(cfg.transport, Transport::Responses);
}

#[test]
fn config_reads_auth_file_override() {
    let cfg = Config::from_env_map([
        ("GROK_SEARCH_AUTH_MODE", "oauth"),
        (
            "GROK_SEARCH_AUTH_FILE",
            "C:\\Users\\chen\\.config\\grok-search-rs\\auth.json",
        ),
    ]);

    assert_eq!(cfg.grok_auth_mode, AuthMode::OAuth);
    assert_eq!(
        cfg.grok_auth_file,
        Some(std::path::PathBuf::from(
            "C:\\Users\\chen\\.config\\grok-search-rs\\auth.json"
        ))
    );
}

#[test]
fn config_normalizes_grok_search_url_to_v1_base() {
    let cases = [
        ("https://api.modelverse.cn", "https://api.modelverse.cn/v1"),
        ("https://api.modelverse.cn/", "https://api.modelverse.cn/v1"),
        (
            "https://api.modelverse.cn/v1",
            "https://api.modelverse.cn/v1",
        ),
        (
            "https://api.modelverse.cn/v1/responses",
            "https://api.modelverse.cn/v1",
        ),
    ];

    for (input, expected) in cases {
        let cfg = Config::from_env_map([
            ("GROK_SEARCH_API_KEY", "grok-test-key"),
            ("GROK_SEARCH_URL", input),
        ]);
        assert_eq!(cfg.grok_api_url, expected);
    }
}

#[test]
fn config_enables_x_search_only_when_configured() {
    let cfg = Config::from_env_map([
        ("GROK_SEARCH_API_KEY", "grok-test-key"),
        ("GROK_SEARCH_X_SEARCH", "true"),
    ]);

    assert!(cfg.x_search_enabled);
}

#[test]
fn config_reads_firecrawl_settings() {
    let cfg = Config::from_env_map([
        ("GROK_SEARCH_API_KEY", "grok-test-key"),
        ("FIRECRAWL_API_KEY", "fc-test-key"),
        ("FIRECRAWL_API_URL", "https://firecrawl.example/v1"),
        ("FIRECRAWL_ENABLED", "true"),
    ]);

    assert_eq!(cfg.firecrawl_api_url, "https://firecrawl.example/v1");
    assert_eq!(cfg.firecrawl_api_key.as_deref(), Some("fc-test-key"));
    assert!(cfg.firecrawl_enabled);
}

#[test]
fn config_redacts_grok_tavily_and_firecrawl_keys() {
    let cfg = Config::from_env_map([
        ("GROK_SEARCH_API_KEY", "grok-1234567890"),
        ("TAVILY_API_KEY", "tvly-abcdefghi"),
        ("FIRECRAWL_API_KEY", "fc-abcdefghi"),
    ]);

    let info = cfg.redacted_diagnostics();
    assert!(info.contains("grok"));
    assert!(info.contains("tvly"));
    assert!(info.contains("fc-a"));
    assert!(!info.contains("1234567890"));
    assert!(!info.contains("abcdefghi"));
}

#[test]
fn config_reads_extra_sources_and_fallback_sources_from_env() {
    let cfg = Config::from_env_map([
        ("GROK_SEARCH_API_KEY", "grok-test-key"),
        ("GROK_SEARCH_EXTRA_SOURCES", "3"),
        ("GROK_SEARCH_FALLBACK_SOURCES", "7"),
    ]);

    assert_eq!(cfg.default_extra_sources, 3);
    assert_eq!(cfg.fallback_sources, 7);
}

#[test]
fn config_reads_timeout_seconds() {
    let cfg = Config::from_env_map([
        ("GROK_SEARCH_API_KEY", "grok-test-key"),
        ("GROK_SEARCH_TIMEOUT_SECONDS", "90"),
    ]);

    assert_eq!(cfg.timeout.as_secs(), 90);
}

#[test]
fn invalid_source_counts_fall_back_to_safe_defaults() {
    let cfg = Config::from_env_map([
        ("GROK_SEARCH_API_KEY", "grok-test-key"),
        ("GROK_SEARCH_EXTRA_SOURCES", "not-a-number"),
        ("GROK_SEARCH_FALLBACK_SOURCES", "not-a-number"),
    ]);

    assert_eq!(cfg.default_extra_sources, 3);
    assert_eq!(cfg.fallback_sources, 5);
    assert_eq!(cfg.timeout.as_secs(), 60);
}

#[test]
fn load_from_uses_provided_env_vars_directly() {
    let cfg = Config::load_from([
        ("GROK_SEARCH_API_KEY", "grok-env-key"),
        ("GROK_SEARCH_MODEL", "model-from-env"),
        ("GROK_SEARCH_EXTRA_SOURCES", "2"),
    ]);

    assert_eq!(cfg.grok_model, "model-from-env");
    assert_eq!(cfg.default_extra_sources, 2);
    assert_eq!(cfg.grok_api_key.as_deref(), Some("grok-env-key"));
}

#[test]
fn missing_env_vars_fall_back_to_defaults() {
    let cfg = Config::load_from([
        ("GROK_SEARCH_API_KEY", "grok-env-key"),
    ]);

    assert_eq!(cfg.grok_api_key.as_deref(), Some("grok-env-key"));
    assert_eq!(cfg.grok_model, "grok-4-1-fast-reasoning");
    assert_eq!(cfg.default_extra_sources, 3);
}

// ── auth_path tests ──────────────────────────────────────────────

#[test]
fn auth_path_uses_default_location_with_home() {
    let path =
        config::auth_path_for([("HOME", "/home/alice")]).expect("HOME must produce auth path");
    let expected = std::path::PathBuf::from("/home/alice")
        .join(".config")
        .join("grok-search-rs")
        .join("auth.json");
    assert_eq!(path, expected);
}

#[test]
fn auth_path_falls_back_to_userprofile_when_home_missing() {
    let path = config::auth_path_for([("USERPROFILE", "C:\\Users\\chen")])
        .expect("USERPROFILE must produce auth path");
    let expected = std::path::PathBuf::from("C:\\Users\\chen")
        .join(".config")
        .join("grok-search-rs")
        .join("auth.json");
    assert_eq!(path, expected);
}

#[test]
fn auth_path_honors_explicit_override() {
    let path = config::auth_path_for([
        ("GROK_SEARCH_AUTH_FILE", "/tmp/auth.json"),
        ("HOME", "/home/ignored"),
    ])
    .expect("explicit override must resolve");
    assert_eq!(path, std::path::PathBuf::from("/tmp/auth.json"));
}
