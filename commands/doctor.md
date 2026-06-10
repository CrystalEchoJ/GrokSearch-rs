---
description: Run a connectivity health check on all configured GrokSearch-rs backends
allowed-tools: [Bash]
---

# GrokSearch-rs Doctor

Call the `doctor` MCP tool to probe connectivity to all configured backends (Grok/xAI, Tavily, Firecrawl) and present the results in a readable format.

## Instructions

1. Call the `doctor` tool (no arguments needed).
2. Parse the JSON response and present a clear summary to the user.

### Interpreting Results

For each backend, check the `reachable` and `detail` fields:

| Backend | Field | Meaning |
|---------|-------|---------|
| `grok` | `reachable` | Whether the AI search endpoint responded successfully |
| `grok` | `model` | Current model being used |
| `grok` | `transport` | `grok_responses` or `openai_compatible` |
| `grok` | `auth_mode` | `api_key` or `oauth` |
| `grok` | `web_search_enabled` | Whether `web_search` tool is offered to the AI |
| `grok` | `x_search_enabled` | Whether X/Twitter search is enabled (Responses only) |
| `tavily` | `reachable` | Whether the source retrieval API responded |
| `firecrawl` | `reachable` | Whether the fallback source API responded |
| `github_token` | — | `set` or `unset` (affects GitHub API rate limits) |

### Presenting Results

Format the output as:

```
GrokSearch-rs Health Check
==========================
Transport:     grok_responses
Model:         grok-4.20-fast
Auth mode:     api_key

Backend        Status      Detail
───────        ──────      ──────
Grok           ✅ / ❌      ...
Tavily         ✅ / ❌      ...
Firecrawl      ✅ / ⚠️      ...

GitHub token:  set / unset
Config file:   (from redacted diagnostics, note the path)
```

If any backend is unreachable, suggest common fixes:
- **Grok unreachable**: check `GROK_SEARCH_API_KEY` or OAuth token (`grok-search-rs status`)
- **Tavily unreachable**: check `TAVILY_API_KEY`; `web_search` still works without it, but `web_fetch` for generic URLs won't
- **Firecrawl unreachable**: check `FIRECRAWL_API_KEY`; only impacts fallback when Tavily also fails

If everything is reachable, congratulate the user — GrokSearch-rs is fully operational.
