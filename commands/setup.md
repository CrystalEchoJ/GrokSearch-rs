---
description: Interactive guided setup for GrokSearch-rs — check binary, configure API keys, scaffold config, verify connectivity
argument-hint: "[--skip-binary-check]"
allowed-tools: [Bash, Read, Write]
---

# GrokSearch-rs Setup

Guide the user through setting up GrokSearch-rs for the first time. This command handles binary installation check, API key configuration, config file scaffolding, and connectivity verification.

## Arguments

$ARGUMENTS — if `--skip-binary-check` is passed, skip the binary-installed check (useful when the user has already verified it).

## Instructions

### Step 1: Welcome

Start by explaining what this setup will do:

> "I'll walk you through setting up GrokSearch-rs. Here's what we'll do:
> 1. Verify the `grok-search-rs` binary is installed
> 2. Help you configure your API keys (Grok/xAI, Tavily, Firecrawl)
> 3. Scaffold a global config file so any MCP client can use the same keys
> 4. Run a connectivity check to confirm everything works"

### Step 2: Check Binary

Unless `--skip-binary-check` was passed, run:

```bash
which grok-search-rs || command -v grok-search-rs || echo "NOT_FOUND"
```

If the binary is found, report the path and version (`grok-search-rs --version`). Then proceed to Step 3.

If NOT found, tell the user:

> "The `grok-search-rs` binary isn't on your PATH yet. This plugin provides the MCP configuration and tool guidance, but you still need the Rust binary itself. Choose one:
>
> **Option A — npm (recommended for most users):**
> ```
> npm install -g grok-search-rs
> ```
>
> **Option B — Cargo (if you have Rust installed):**
> ```
> cargo install grok-search-rs
> ```
>
> **Option C — Build from source:**
> ```
> git clone https://github.com/CrystalEchoJ/GrokSearch-rs.git
> cd GrokSearch-rs
> cargo build --release
> # then add target/release/ to your PATH or use the absolute path
> ```
>
> After installing, run `/grok-search-rs:setup --skip-binary-check` to continue."

If the user asks you to install it for them, run the npm or cargo command (whichever they prefer). Then proceed.

### Step 3: API Keys

Explain the key requirements:

> "GrokSearch-rs needs API keys to work. You can configure them in two ways:
>
> **Option A — Global config file (recommended):**
> Run `grok-search-rs --init` and edit the resulting file. I'll help with that in a moment.
>
> **Option B — Environment variables:**
> Set them in your shell profile or MCP client config.
>
> Here are the keys you need:
>
> | Key | Required? | Where to get it |
> |-----|-----------|----------------|
> | `GROK_SEARCH_API_KEY` | **Yes** (or use OAuth) | https://x.ai/api |
> | `TAVILY_API_KEY` | Recommended | https://tavily.com |
> | `FIRECRAWL_API_KEY` | Optional (fallback) | https://firecrawl.dev |
> | `GITHUB_TOKEN` | Optional (higher rate limits) | https://github.com/settings/tokens |
>
> **OAuth alternative to API key:**
> If you prefer not to manage a static API key, you can use:
> ```
> grok-search-rs login
> ```
> Then set `GROK_SEARCH_AUTH_MODE=oauth` in your config. Note: OAuth mode reuses Hermes' xAI OAuth client and may carry account/terms risk."

### Step 4: Scaffold Config

Run `grok-search-rs --init`. If the file already exists, say so and ask if they want to see its current contents. If it was just created, show the path and offer to open it.

If the user wants to edit the config file now, read it and help them fill in their keys. The config file uses `snake_case` keys (not the `SCREAMING_SNAKE_CASE` of env vars):

| File key | Env var equivalent |
|----------|-------------------|
| `grok_api_key` | `GROK_SEARCH_API_KEY` |
| `grok_model` | `GROK_SEARCH_MODEL` |
| `tavily_api_key` | `TAVILY_API_KEY` |
| `tavily_api_url` | `TAVILY_API_URL` |
| `firecrawl_api_key` | `FIRECRAWL_API_KEY` |
| `firecrawl_api_url` | `FIRECRAWL_API_URL` |
| `github_token` | `GITHUB_TOKEN` |

Help the user uncomment and fill in the keys they have. Don't ask them to paste keys into the chat — ask them to edit the file directly and tell you when they're done.

### Step 5: Verify

Once keys are configured, tell the user to ask their assistant to run the doctor check:

> "Now let's verify everything works. I'll ask the assistant to call the `doctor` tool. You can also run `/grok-search-rs:doctor` separately."

Then instruct them: "Call doctor" — or if you're in a context where you can call MCP tools, call the `doctor` tool directly and present the results.

If `doctor` reports `reachable: true` for Grok (and Tavily if configured), the setup is complete. If anything is unreachable, help debug:
- **Grok unreachable**: Check the API key and URL
- **Tavily unreachable**: Check the Tavily key (still usable without it, but `web_fetch` for non-specialist URLs won't work)
- **Firecrawl unreachable**: Optional — only matters when Tavily also fails

### Step 6: Wrap Up

Summarize what was configured:

> "GrokSearch-rs is ready. Here's what you can do:
>
> - **Search the web**: Just ask me to search for something — I'll use `web_search` automatically
> - **Fetch a specific page**: Ask me to fetch a URL — I'll use `web_fetch` with the best extractor
> - **Health check**: Run `/grok-search-rs:doctor` anytime
> - **Reconfigure**: Edit `~/.config/grok-search-rs/config.toml` and restart
>
> If you're switching between transports (Responses ↔ ChatCompletions), see the [Configuration docs](https://github.com/CrystalEchoJ/GrokSearch-rs#configuration)."
