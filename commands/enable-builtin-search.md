---
description: Re-enable Claude Code's built-in WebSearch and WebFetch tools (undo disable-builtin-search)
allowed-tools: [Read, Write, Bash]
---

# Enable Built-in Search

Re-enable Claude Code's native WebSearch and WebFetch tools by removing them from `permissions.deny` in `.claude/settings.json`.

## Instructions

### Step 1: Read current settings

Read `.claude/settings.json`. If the file doesn't exist, there's nothing to do — tell the user "Built-in WebSearch and WebFetch are already enabled (no settings file found)."

### Step 2: Check current state

Check if `WebSearch` and `WebFetch` are in `permissions.deny`:
- If neither is present, tell the user "Built-in WebSearch and WebFetch are already enabled. grok-search-rs is disabled."
- Otherwise, proceed to Step 3.

### Step 3: Remove deny rules

Remove `WebSearch` and `WebFetch` from `permissions.deny`, preserving any other deny entries.

- If `permissions.deny` becomes empty after removal, you can either remove the array or leave it empty — both are valid.
- If the file only had `permissions.deny` and nothing else, feel free to simplify it.

### Step 4: Clean up (optional)

If `permissions.deny` is now empty and `permissions` has no other keys, you can remove the `permissions` object entirely to keep the file clean.

### Step 5: Confirm

Tell the user:

> "Claude Code's built-in WebSearch and WebFetch are now re-enabled."
>
> "To disable them again (and use grok-search-rs instead), run `/grok-search-rs:disable-builtin-search`."
