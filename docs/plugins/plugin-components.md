# 插件组件类型详解

> Claude Code 插件支持六大组件类型：Commands、Agents、Skills、Hooks、MCP Servers、LSP Servers。
> 所有组件都遵循约定优于配置的自动发现机制。

## 目录结构总览

```
plugin-root/
├── .claude-plugin/
│   └── plugin.json          # 必需：插件清单
├── commands/                 # Slash 命令
│   └── *.md
├── agents/                   # 子代理
│   └── *.md
├── skills/                   # 自动激活技能
│   └── <skill-name>/
│       └── SKILL.md          # 必需
├── hooks/
│   └── hooks.json           # 事件钩子配置
├── .mcp.json                # MCP 服务器定义
└── scripts/                 # 辅助脚本（可选）
```

---

## 1. Commands（命令）

**位置**：`commands/*.md`
**自动发现**：所有 `.md` 文件自动注册为 `/` 斜杠命令
**调用方式**：用户手动调用 `/plugin-name:command-name`

### 文件格式

Markdown 文件 + YAML 前置元数据（frontmatter）：

```markdown
---
description: 命令的简短描述，显示在 /help 中
argument-hint: <required-arg> [optional-arg]
allowed-tools: [Read, Glob, Grep, Bash]
model: sonnet
---

# 命令标题

当用户调用此命令时，Claude 将执行以下指令…

## Arguments

用户传入的参数：$ARGUMENTS

## Instructions

1. 解析用户提供的参数
2. 使用允许的工具执行操作
3. 将结果返回给用户
```

### Frontmatter 字段

| 字段 | 说明 |
|------|------|
| `description` | 命令描述，显示在 `/help` 中 |
| `argument-hint` | 参数提示，如 `<file> [options]` |
| `allowed-tools` | 预授权工具列表，减少权限提示。格式：`[Read, Glob, Grep, Bash]` |
| `model` | 覆盖模型：`haiku`、`sonnet`、`opus` |

### 命令命名

- 文件名决定命令名：`hello.md` → `/hello`
- 调用时使用插件命名空间：`/plugin-name:hello`
- 如果命令名唯一，可以省略命名空间

### 子目录组织

命令可以按功能分组到子目录：

```
commands/
├── ci/
│   ├── build.md      # /ci:build
│   ├── test.md       # /ci:test
│   └── deploy.md     # /ci:deploy
├── admin/
│   ├── configure.md  # /admin:configure
│   └── manage.md     # /admin:manage
```

---

## 2. Agents（子代理）

**位置**：`agents/*.md`
**自动发现**：所有 `.md` 文件自动注册为子代理
**调用方式**：用户可以手动调用，或 Claude Code 根据任务上下文自动选择

### 文件格式

```markdown
---
description: 代理的角色和专业领域
capabilities:
  - 具体任务能力 1
  - 具体任务能力 2
model: sonnet
---

# Agent Name

详细的代理指令和领域知识…

## Expertise

- 专业领域 1
- 专业领域 2

## Process

1. 步骤 1
2. 步骤 2

## MCP Integration

可以引用插件提供的 MCP 工具：
`const result = await tools.server_name_tool_name({ param: value })`
```

### Frontmatter 字段

| 字段 | 说明 |
|------|------|
| `description` | 代理的描述，Claude 用它来决定何时自动调用此代理 |
| `capabilities` | 此代理能完成的具体任务列表 |
| `model` | 可选：覆盖模型 |

### 子代理组织

```
agents/
├── orchestration/
│   ├── deployment-orchestrator.md
│   └── rollback-manager.md
└── specialized/
    ├── kubernetes-expert.md
    └── security-auditor.md
```

---

## 3. Skills（技能）

**位置**：`skills/<skill-name>/SKILL.md`
**自动发现**：所有包含 `SKILL.md` 的子目录自动注册
**调用方式**：Claude 根据任务上下文自动激活 —— 用户无需手动调用

### 目录结构

```
skills/
└── <skill-name>/
    ├── SKILL.md          # 必需：技能定义
    ├── references/       # 参考文档（可选）
    │   └── *.md
    ├── examples/         # 示例文件（可选）
    │   └── *
    ├── scripts/          # 辅助脚本（可选）
    │   └── *
    └── evals/            # 评估数据（可选）
        └── *.json
```

### SKILL.md 格式

```markdown
---
name: Skill Display Name
description: 描述何时应使用此技能。Claude 根据此描述自动匹配。写得越精确越好。
version: 1.0.0
---

# Skill Title

技能的详细指令和指导…

## Overview

技能的总体说明…

## Core Concepts

### Concept 1
…

### Concept 2
…

## Best Practices
…

## References

- `references/detail.md` — 深入细节
- `examples/template.yaml` — 可复制的模板
```

### 技能 vs 命令

| 特性 | Skills | Commands |
|------|--------|----------|
| 激活方式 | Claude 自动判断 | 用户手动 `/` 调用 |
| 适用场景 | 领域知识注入、工作流指导 | 特定操作的快捷入口 |
| 文件组织 | 目录 + `SKILL.md` + 支持文件 | 单个 `.md` 文件 |
| 用户体验 | 对用户透明 | 用户主动触发 |

---

## 4. Hooks（钩子）

**位置**：`hooks/hooks.json`（默认）或 `plugin.json` 中内联
**自动发现**：插件启用时自动注册
**调用方式**：Claude Code 在特定事件发生时自动执行

### 事件类型

| 事件 | 触发时机 |
|------|----------|
| `PreToolUse` | 工具调用**之前** |
| `PostToolUse` | 工具调用**之后** |
| `Stop` | Claude 响应完成**之后** |
| `SubagentStop` | 子代理完成**之后** |
| `SessionStart` | 会话**开始时** |
| `SessionEnd` | 会话**结束时** |
| `UserPromptSubmit` | 用户提交提示时 |
| `PreCompact` | 上下文压缩**之前** |
| `Notification` | 收到通知时 |

### 两种钩子类型

#### command 钩子

执行外部命令/脚本：

```json
{
  "type": "command",
  "command": "bash ${CLAUDE_PLUGIN_ROOT}/hooks/scripts/validate.sh",
  "timeout": 30
}
```

#### prompt 钩子

向 Claude 注入额外的提示指令：

```json
{
  "type": "prompt",
  "prompt": "在写入文件前，请确认代码符合项目的代码规范：使用 2 空格缩进，单引号，无尾随分号。",
  "timeout": 20
}
```

### Matcher（匹配器）

`matcher` 用于过滤钩子触发条件：

| Matcher | 匹配范围 |
|---------|----------|
| `""` 或 `".*"` | 所有工具 |
| `"Write"` | 仅 `Write` 工具 |
| `"Write|Edit"` | `Write` 或 `Edit` 工具 |
| `"Bash"` | 仅 `Bash` 工具 |
| `"Edit|Write|MultiEdit|NotebookEdit"` | 所有编辑类工具 |

### 条件触发 (`if`)

`if` 字段基于工具调用内容进一步过滤：

```json
{
  "type": "command",
  "command": "bash ${CLAUDE_PLUGIN_ROOT}/hooks/scripts/review-commit.sh",
  "if": "Bash(git commit:*)",
  "asyncRewake": true,
  "rewakeMessage": "后台安全审查发现以下问题，请处理或确认后继续：",
  "rewakeSummary": "提交安全审查发现问题"
}
```

### 异步钩子

设置 `asyncRewake: true` 让钩子在后台执行，完成后通过 `rewakeMessage` 将结果注入会话：

| 字段 | 说明 |
|------|------|
| `asyncRewake` | `true` 时异步执行，完成后唤醒 Claude |
| `rewakeMessage` | 唤醒时显示给 Claude 的消息 |
| `rewakeSummary` | 在状态栏中显示的简短摘要 |

### 完整配置示例

```json
{
  "PreToolUse": [
    {
      "matcher": "Write|Edit",
      "hooks": [
        {
          "type": "command",
          "command": "bash ${CLAUDE_PLUGIN_ROOT}/hooks/scripts/scan-secrets.sh",
          "timeout": 30
        }
      ]
    },
    {
      "matcher": "Bash",
      "hooks": [
        {
          "type": "prompt",
          "prompt": "评估此 bash 命令是否安全。检查是否有破坏性操作、缺少安全防护措施。命令应该是幂等和可逆的。",
          "timeout": 20
        }
      ]
    }
  ],
  "PostToolUse": [
    {
      "matcher": "Bash",
      "hooks": [
        {
          "type": "command",
          "command": "bash ${CLAUDE_PLUGIN_ROOT}/hooks/scripts/update-status.sh",
          "timeout": 15
        }
      ]
    }
  ],
  "Stop": [
    {
      "hooks": [
        {
          "type": "command",
          "command": "bash ${CLAUDE_PLUGIN_ROOT}/hooks/scripts/audit-changes.sh",
          "timeout": 45
        }
      ]
    }
  ],
  "SessionStart": [
    {
      "hooks": [
        {
          "type": "command",
          "command": "bash ${CLAUDE_PLUGIN_ROOT}/hooks/scripts/setup-env.sh",
          "timeout": 20
        }
      ]
    }
  ]
}
```

### `${CLAUDE_PLUGIN_ROOT}` 环境变量

**关键规则**：在钩子脚本和 MCP 服务器配置中引用插件内文件时，必须使用 `${CLAUDE_PLUGIN_ROOT}` 而非硬编码路径。

```bash
# ✅ 正确
bash ${CLAUDE_PLUGIN_ROOT}/scripts/validate.sh

# ❌ 错误 —— 插件安装位置因用户和环境而异
bash /Users/alice/.claude/plugins/cache/marketplace/plugin/scripts/validate.sh
bash ~/my-plugin/scripts/validate.sh
```

---

## 5. MCP Servers（MCP 服务器）

**位置**：`.mcp.json`（默认）或 `plugin.json` 中内联
**自动发现**：插件启用时自动启动
**调用方式**：Claude 自动将 MCP 工具暴露给 AI

### 三种服务器类型

#### stdio（本地进程）

通过标准输入/输出与本地进程通信：

```json
{
  "mcpServers": {
    "filesystem": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-filesystem", "${CLAUDE_PROJECT_DIR}"],
      "env": {
        "LOG_LEVEL": "info"
      }
    },
    "python-server": {
      "command": "python",
      "args": ["-m", "my_mcp_server", "--port", "8080"],
      "env": {
        "API_KEY": "${CUSTOM_API_KEY}",
        "DEBUG": "false"
      }
    }
  }
}
```

| 字段 | 说明 |
|------|------|
| `command` | 启动命令 |
| `args` | 命令行参数数组 |
| `env` | 环境变量（支持 `${VAR}` 和 `${VAR:-default}`） |

#### http（HTTP REST API）

连接远程 HTTP MCP 端点：

```json
{
  "mcpServers": {
    "github": {
      "type": "http",
      "url": "https://api.githubcopilot.com/mcp/",
      "headers": {
        "Authorization": "Bearer ${GITHUB_PERSONAL_ACCESS_TOKEN}"
      }
    }
  }
}
```

| 字段 | 说明 |
|------|------|
| `type` | `"http"` |
| `url` | MCP 端点 URL |
| `headers` | HTTP 请求头（支持 `${VAR}` 环境变量） |

#### sse（Server-Sent Events）

连接 SSE 流式 MCP 端点：

```json
{
  "mcpServers": {
    "asana": {
      "type": "sse",
      "url": "https://mcp.asana.com/sse"
    },
    "custom-service": {
      "type": "sse",
      "url": "https://mcp.example.com/sse",
      "headers": {
        "X-API-Version": "v1",
        "X-Client-ID": "${CLIENT_ID}"
      }
    }
  }
}
```

| 字段 | 说明 |
|------|------|
| `type` | `"sse"` |
| `url` | SSE 端点 URL |
| `headers` | 可选 HTTP 请求头 |

### 环境变量替换

在 `.mcp.json` 中可以使用环境变量：

| 语法 | 说明 |
|------|------|
| `${VAR}` | 引用环境变量，不存在时为空 |
| `${VAR:-default}` | 引用环境变量，不存在时使用默认值 |
| `${CLAUDE_PLUGIN_ROOT}` | 插件根目录的绝对路径（自动设置） |
| `${CLAUDE_PROJECT_DIR}` | 当前项目目录（由 Claude Code 设置） |

### 内联 vs 文件

```json
// plugin.json 中内联（适合单个简单服务器）
{
  "name": "my-plugin",
  "mcpServers": {
    "my-server": {
      "command": "python",
      "args": ["${CLAUDE_PLUGIN_ROOT}/server.py"]
    }
  }
}

// 引用外部文件（适合多个复杂服务器）
{
  "name": "my-plugin",
  "mcpServers": "./.mcp.json"
}
```

---

## 6. LSP Servers（语言服务器）

**位置**：在 `marketplace.json` 的插件条目中声明（不用 `plugin.json`）
**特殊之处**：LSP 插件通常不需要独立的 `plugin.json`，所有配置都在 marketplace 条目中

### 配置格式

```json
{
  "name": "clangd-lsp",
  "version": "1.0.0",
  "source": "./plugins/clangd-lsp",
  "category": "development",
  "strict": false,
  "lspServers": {
    "clangd": {
      "command": "clangd",
      "args": ["--background-index"],
      "extensionToLanguage": {
        ".c": "c",
        ".h": "c",
        ".cpp": "cpp",
        ".cc": "cpp",
        ".cxx": "cpp",
        ".hpp": "cpp",
        ".C": "cpp",
        ".H": "cpp"
      }
    }
  }
}
```

| 字段 | 说明 |
|------|------|
| `lspServers` | 对象，key 为服务器标识名 |
| `lspServers.<name>.command` | LSP 服务器启动命令 |
| `lspServers.<name>.args` | 命令行参数（可选） |
| `lspServers.<name>.extensionToLanguage` | **必需**：文件扩展名到语言 ID 的映射 |
| `lspServers.<name>.startupTimeout` | 启动超时毫秒数（可选，如 `120000` = 2 分钟） |
| `strict` | `false` 表示宽松加载模式（服务器不可用时不影响插件加载） |

### 官方 LSP 插件示例

| 插件名 | 语言 | 命令 |
|--------|------|------|
| `pyright-lsp` | Python | `pyright-langserver --stdio` |
| `typescript-lsp` | TypeScript/JavaScript | `typescript-language-server --stdio` |
| `rust-analyzer-lsp` | Rust | `rust-analyzer` |
| `gopls-lsp` | Go | `gopls` |
| `clangd-lsp` | C/C++ | `clangd --background-index` |
| `jdtls-lsp` | Java | `jdtls`（超时 120s） |
| `ruby-lsp` | Ruby | `ruby-lsp` |
| `swift-lsp` | Swift | `sourcekit-lsp` |
| `csharp-lsp` | C# | `csharp-ls` |
| `kotlin-lsp` | Kotlin | `kotlin-lsp --stdio` |
| `php-lsp` | PHP | `intelephense --stdio` |
| `lua-lsp` | Lua | `lua-language-server` |

---

## 组件组合模式

### 最简：纯 MCP

```
plugin/
├── .claude-plugin/plugin.json   # name + description
└── .mcp.json                    # MCP 服务器定义
```

代表：GitHub 插件

### 标准：命令 + 技能

```
plugin/
├── .claude-plugin/plugin.json
├── commands/
│   └── setup.md                 # 安装后配置入口
└── skills/
    └── my-skill/
        └── SKILL.md
```

代表：claude-hud

### 完整：全部组件

```
plugin/
├── .claude-plugin/plugin.json
├── commands/                    # 用户命令
├── agents/                      # 子代理
├── skills/                      # 自动技能
├── hooks/
│   ├── hooks.json              # 事件处理
│   └── scripts/
├── .mcp.json                   # MCP 集成
├── servers/                    # MCP 服务器源码
└── scripts/                    # 共享工具
```

代表：security-guidance（钩子为主）、code-modernization（代理+命令为主）
