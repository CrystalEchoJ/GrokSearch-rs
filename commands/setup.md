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
> 3. Verify the environment is ready
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

> "GrokSearch-rs reads all configuration from environment variables. The easiest way is to add them to your shell profile (`~/.bashrc`, `~/.zshrc`, etc.).
>
> **Required keys:**
>
> | Key | Where to get it |
> |-----|----------------|
> | `GROK_SEARCH_API_KEY` | https://x.ai/api |
> | `TAVILY_API_KEY` | https://tavily.com |
>
> **Optional keys:**
>
> | Key | Purpose |
> |-----|---------|
> | `FIRECRAWL_API_KEY` | Fallback when Tavily fails |
> | `GITHUB_TOKEN` | Higher GitHub API rate limits |
>
> Add them to your shell profile:
> ```bash
> echo 'export GROK_SEARCH_API_KEY="xai-..."' >> ~/.bashrc
> echo 'export TAVILY_API_KEY="tvly-..."' >> ~/.bashrc
> source ~/.bashrc
> ```
>
> The plugin's `.mcp.json` already has sensible defaults for all other settings (model, URLs, timeouts, etc.). The full list is in the `.mcp.json` file — you can see it at `${CLAUDE_PLUGIN_ROOT}/.mcp.json`.
>
> **OAuth alternative:**
> If you prefer not to manage a static API key:
> ```
> grok-search-rs login
> ```
> Then set `GROK_SEARCH_AUTH_MODE=oauth` instead of `GROK_SEARCH_API_KEY`.

### Step 4: Verify Config

Check that the env vars are visible to the current shell:

```bash
echo "GROK_SEARCH_API_KEY is ${GROK_SEARCH_API_KEY:+set} ${GROK_SEARCH_API_KEY:-unset}"
echo "TAVILY_API_KEY is ${TAVILY_API_KEY:+set} ${TAVILY_API_KEY:-unset}"
```

If any required key is missing, go back to Step 3.

> **Note:** There's also a global config file option (`grok-search-rs --init` → `~/.config/grok-search-rs/config.toml`) for users who use multiple MCP clients and don't want to duplicate env vars. For Claude Code plugin users, shell environment variables are simpler — one place, works everywhere.

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
> - **Reconfigure**: Edit env vars in `~/.bashrc` and restart the shell, or set new values in your MCP client config (env takes priority over defaults)
>
> If you're switching between transports (Responses ↔ ChatCompletions), see the [Configuration docs](https://github.com/CrystalEchoJ/GrokSearch-rs#configuration)."
