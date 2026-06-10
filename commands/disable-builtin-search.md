---
description: Disable Claude Code's built-in WebSearch and WebFetch tools so grok-search-rs is used instead
allowed-tools: [Read, Write, Bash]
---

# Disable Built-in Search

Disable Claude Code's native WebSearch and WebFetch tools by adding them to `permissions.deny` in `.claude/settings.json`. After this, Claude will use grok-search-rs for all web search and fetch operations.

## Instructions

### Step 1: Read current settings

Read `.claude/settings.json`. If the file doesn't exist, create a minimal one with `{}`.

### Step 2: Check current state

Check if `WebSearch` and `WebFetch` are already in `permissions.deny`:
- If both are already present, tell the user "Built-in WebSearch and WebFetch are already disabled. grok-search-rs is in control."
- Otherwise, proceed to Step 3.

### Step 3: Add deny rules

Merge `WebSearch` and `WebFetch` into `permissions.deny`, preserving any existing deny entries. The result should look like:

```json
{
  "permissions": {
    "deny": [
      "WebSearch",
      "WebFetch"
    ]
  }
}
```

If the file already has other settings (hooks, env, other permissions), keep them all — only add or merge the `permissions.deny` array.

### Step 4: Confirm

Tell the user:

> "Claude Code's built-in WebSearch and WebFetch are now disabled. All web search and fetch requests will go through grok-search-rs."
>
> "To re-enable the built-in tools later, run `/grok-search-rs:enable-builtin-search`."
