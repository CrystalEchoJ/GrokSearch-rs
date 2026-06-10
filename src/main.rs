use std::io::{IsTerminal, Write};

use grok_search_rs::config::{self, AuthMode, Config};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    // CLI shim: handle --version before MCP server mode.
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args
        .iter()
        .any(|a| a == "--version" || a == "-V" || a == "-v")
    {
        println!("grok-search-rs {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    if args.first().map(String::as_str) == Some("login") {
        let cfg = Config::load();
        return run_login(&cfg).await;
    }

    if args.first().map(String::as_str) == Some("status") {
        let cfg = Config::load();
        return run_status(&cfg);
    }

    if args.first().map(String::as_str) == Some("logout") {
        let cfg = Config::load();
        return run_logout(&cfg);
    }

    let cfg = Config::load();

    // Detect interactive run with missing credentials and print a friendly
    // onboarding guide instead of a cryptic error. MCP clients always pipe
    // stdio, so a TTY here means the user ran the binary directly.
    if cfg.grok_auth_mode == AuthMode::ApiKey
        && cfg.grok_api_key.is_none()
        && std::io::stdin().is_terminal()
    {
        print_setup_guide();
        return Ok(());
    }

    let service = grok_search_rs::service::SearchService::new(cfg)?;
    grok_search_rs::mcp::run_stdio(service).await?;
    Ok(())
}

async fn run_login(cfg: &Config) -> anyhow::Result<()> {
    let path = resolve_auth_path(cfg)?;
    let store = grok_search_rs::oauth::login::login(&path, true).await?;
    println!("Login successful.");
    println!("Auth file: {}", path.display());
    if let Some(exp) = grok_search_rs::oauth::token_store::jwt_exp(&store.access_token) {
        println!("Access token expires at unix time: {exp}");
    }
    Ok(())
}

fn run_status(cfg: &Config) -> anyhow::Result<()> {
    let path = resolve_auth_path(cfg)?;
    let status = grok_search_rs::oauth::token_store::auth_status(&path);
    println!("grok-search-rs OAuth status");
    println!("  Auth file: {}", status.path.display());
    println!(
        "  Authenticated: {}",
        if status.authenticated { "yes" } else { "no" }
    );
    println!(
        "  Refresh token: {}",
        if status.refresh_token_present {
            "present"
        } else {
            "missing"
        }
    );
    println!(
        "  Access expires at: {}",
        status
            .access_expires_at
            .map(|value| value.to_string())
            .unwrap_or_else(|| "unknown".to_string())
    );
    println!(
        "  Base URL: {}",
        status.base_url.unwrap_or_else(|| "unknown".to_string())
    );
    Ok(())
}

fn run_logout(cfg: &Config) -> anyhow::Result<()> {
    let path = resolve_auth_path(cfg)?;
    let removed = grok_search_rs::oauth::token_store::delete_token_store(&path)?;
    if removed {
        println!("Removed OAuth token file: {}", path.display());
    } else {
        println!("No OAuth token file found: {}", path.display());
    }
    Ok(())
}

fn resolve_auth_path(cfg: &Config) -> anyhow::Result<std::path::PathBuf> {
    cfg.grok_auth_file
        .clone()
        .or_else(config::auth_path)
        .ok_or_else(|| anyhow::anyhow!("cannot resolve OAuth auth path; set GROK_SEARCH_AUTH_FILE"))
}

fn print_setup_guide() {
    let guide = r#"grok-search-rs is an MCP server. It speaks JSON-RPC over stdio and
should be launched by an MCP client (Claude Code, Codex CLI, Gemini CLI,
Cursor, VS Code, Windsurf, ...), not run directly.

All configuration lives in the .mcp.json file's env block. Edit it to set
your keys and preferences — no other config files or shell env vars needed.

Required keys
  GROK_SEARCH_API_KEY   xAI / Grok-compatible key   (https://x.ai/api)
  TAVILY_API_KEY        Tavily fetch + map          (https://tavily.com)
  FIRECRAWL_API_KEY     optional fetch fallback     (https://firecrawl.dev)

OAuth alternative
  grok-search-rs login
  Set GROK_SEARCH_AUTH_MODE=oauth in .mcp.json.
  OAuth mode reuses Hermes' xAI client_id and may carry account / terms risk.

One-line install (Claude Code)
  claude mcp add-json grok-search-rs --scope user '{
    "type": "stdio",
    "command": "grok-search-rs",
    "env": {
      "GROK_SEARCH_API_KEY": "xai-...",
      "TAVILY_API_KEY": "tvly-..."
    }
  }'

Docs:    https://github.com/CrystalEchoJ/GrokSearch-rs#readme
Issues:  https://github.com/CrystalEchoJ/GrokSearch-rs/issues
"#;

    let stdout = std::io::stdout();
    let _ = stdout.lock().write_all(guide.as_bytes());
}
