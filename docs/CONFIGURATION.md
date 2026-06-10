# Configuration

All configuration lives in the `.mcp.json` file's `env` block. The MCP client reads this file at startup, passes the env vars to the binary, and the binary uses built-in defaults for any unset keys.

There are no other config files. No TOML, no `.env`, no shell profile env vars needed. Everything is in one place: `.mcp.json`.

## Grok Responses

| Variable | Default | Description |
|---|---|---|
| `GROK_SEARCH_AUTH_MODE` | `api_key` | `api_key` uses `GROK_SEARCH_API_KEY`; `oauth` uses the local token file created by `grok-search-rs login`. |
| `GROK_SEARCH_API_KEY` | required in `api_key` mode | Bearer token for the configured Grok-compatible gateway. |
| `GROK_SEARCH_AUTH_FILE` | `<home>/.config/grok-search-rs/auth.json` | Optional OAuth token file override. |
| `GROK_SEARCH_URL` | `https://api.x.ai` | Root URL, `/v1` base URL, or endpoint-like URL. The service normalizes it to a `/v1` base. |
| `GROK_SEARCH_MODEL` | `grok-4-1-fast-reasoning` | Model sent in the Responses payload. |
| `GROK_SEARCH_WEB_SEARCH` | `true` | Sends Responses `{"type":"web_search"}`. |
| `GROK_SEARCH_X_SEARCH` | `false` | Sends Responses `{"type":"x_search"}` only when enabled. |

Boolean values accept `1`, `true`, or `yes` as enabled. Any other value is treated as disabled.

### OAuth mode

OAuth mode keeps the normal Responses payload and only changes where the Bearer token comes from. The binary handles login and MCP stdio; it does not start a background HTTP proxy.

```bash
grok-search-rs login
grok-search-rs status
grok-search-rs logout
```

`login` opens xAI OAuth in a browser, listens once on `http://127.0.0.1:56121/callback`, and writes `access_token`, `refresh_token`, `id_token`, `token_endpoint`, `base_url`, and `last_refresh` to `auth.json`. `status` prints token presence, expiry, and the auth file path without printing the token. `logout` removes the local auth file.

OAuth mode reuses Hermes' xAI OAuth client id. This may violate xAI terms or create account risk, and Windows stores the token as a normal local file. Do not share the token file.

Minimal `.mcp.json` config for OAuth:

```json
{
  "mcpServers": {
    "grok-search-rs": {
      "command": "grok-search-rs",
      "env": {
        "GROK_SEARCH_AUTH_MODE": "oauth",
        "GROK_SEARCH_MODEL": "grok-4.3",
        "GROK_SEARCH_WEB_SEARCH": "true"
      }
    }
  }
}
```

## OpenAI-compatible transport

When `GROK_SEARCH_API_KEY` is unset and these three are set, the service talks to `/v1/chat/completions` instead of `/v1/responses`. Useful for OpenAI-compatible gateways that do not implement `/responses`.

| Variable | Default | Description |
|---|---|---|
| `OPENAI_COMPATIBLE_API_URL` | unset | Base URL for the chat-completions gateway. |
| `OPENAI_COMPATIBLE_API_KEY` | unset | Bearer token for the gateway. |
| `OPENAI_COMPATIBLE_MODEL` | unset | Model name sent in the chat-completions payload. |

## Tavily

| Variable | Default | Description |
|---|---|---|
| `TAVILY_API_KEY` | unset | Enables Tavily-backed source enrichment, fallback, fetch, and map. |
| `TAVILY_API_URL` | `https://api.tavily.com` | Tavily API base URL. |
| `TAVILY_ENABLED` | `true` | Optional override. Set to `false` only when you want to disable Tavily even if `TAVILY_API_KEY` is configured. |
| `GROK_SEARCH_EXTRA_SOURCES` | `3` | Adds Tavily enrichment sources after a verifiable Grok result; Firecrawl can fallback if Tavily returns none. Set `0` to disable enrichment. |
| `GROK_SEARCH_FALLBACK_SOURCES` | `5` | Number of fallback sources to cache when Grok is unverifiable. |

## Firecrawl

| Variable | Default | Description |
|---|---|---|
| `FIRECRAWL_API_KEY` | unset | Enables Firecrawl fallback for `web_fetch` and supplemental fallback sources. |
| `FIRECRAWL_API_URL` | `https://api.firecrawl.dev` | Firecrawl API base URL, normalized to `/v1`. |
| `FIRECRAWL_ENABLED` | `true` | Optional override. Set to `false` to disable Firecrawl even if a key is configured. |

## Cache

| Variable | Default | Description |
|---|---|---|
| `GROK_SEARCH_CACHE_SIZE` | `256` | Maximum cached search sessions for `get_sources`. |
| `GROK_SEARCH_TIMEOUT_SECONDS` | `60` | HTTP timeout for Grok, Tavily, and Firecrawl requests. |
| `GROK_SEARCH_FETCH_MAX_CHARS` | unset | Default character cap on `web_fetch` content. Overridden per call by `max_chars`. Unset means no truncation. |

## Source extraction

Specialist `web_fetch` extractors (GitHub, StackExchange, arXiv, Wikipedia) and
`web_search` inline enrichment. The specialists call public APIs directly — no
Tavily/Firecrawl key required.

| Variable | Default | Description |
|---|---|---|
| `GITHUB_TOKEN` | unset | GitHub token for issue/PR fetches. Anonymous works but is capped at ~60 req/hr; a token raises the limit and allows private repos. |
| `GROK_SEARCH_SOURCE_MAX_ANSWERS` | `5` | StackExchange answers rendered before the "more answers" fold. |
| `GROK_SEARCH_SOURCE_MAX_COMMENTS` | `30` | GitHub / StackExchange comments rendered before folding. |
| `GROK_SEARCH_ENRICH_CONCURRENCY` | `3` | Parallel source enrichments when `web_search` is called with `include_content: true`. Clamped to `1..=5`. |
| `GROK_SEARCH_ENRICH_MAX_CHARS` | `15000` | Character cap per enriched source body. |
