# 案例研究：真实插件剖析

> 四个典型插件案例，涵盖从最简 MCP 到复杂多组件架构。
> 所有案例均来自实际安装和运行的插件。

---

## 1. GitHub — 纯 MCP 插件

**来源**：官方市场 `claude-plugins-official`
**类型**：纯 HTTP MCP 集成
**复杂度**：⭐（最简单）

### 目录结构

```
github/unknown/
├── .claude-plugin/
│   └── plugin.json
└── .mcp.json
```

只有 **2 个文件**，是所有插件中最简形式。无 `commands/`、`agents/`、`skills/` 目录。

### plugin.json

```json
{
  "name": "github",
  "description": "Official GitHub MCP server for repository management. Create issues, manage pull requests, review code, search repositories, and interact with GitHub's full API directly from Claude Code.",
  "author": {
    "name": "GitHub"
  }
}
```

极简 manifest —— 只有 `name`、`description`、`author`。所有功能通过 MCP 提供。

### .mcp.json

```json
{
  "github": {
    "type": "http",
    "url": "https://api.githubcopilot.com/mcp/",
    "headers": {
      "Authorization": "Bearer ${GITHUB_PERSONAL_ACCESS_TOKEN}"
    }
  }
}
```

关键点：
- **HTTP 类型**：连接到 GitHub 的远程 MCP 端点，无需本地进程
- **认证**：通过 `GITHUB_PERSONAL_ACCESS_TOKEN` 环境变量传递 Bearer token
- **单服务器**：只有一个 `github` 条目

### 此模式的要点

| 特性 | 说明 |
|------|------|
| 插件体积 | 极小（2 个文件） |
| 维护成本 | 极低（只需更新 MCP URL 或 schema） |
| 功能扩展 | 完全由远程 MCP 服务器决定 |
| 适用场景 | 任何可以通过 HTTP MCP 端点的 API（GitHub、Slack、Linear 等） |

---

## 2. claude-hud — 自包含市场+插件

**来源**：第三方市场 `jarrodwatts/claude-hud`
**类型**：Marketplace + Plugin 合一，命令驱动
**复杂度**：⭐⭐

### 目录结构

```
claude-hud/
├── .claude-plugin/
│   ├── marketplace.json    # 使仓库可作为市场
│   └── plugin.json         # 插件清单
└── commands/
    ├── setup.md            # /claude-hud:setup
    └── configure.md        # /claude-hud:configure
```

### marketplace.json

```json
{
  "name": "claude-hud-marketplace",
  "description": "Marketplace for the Claude HUD plugin.",
  "owner": {
    "name": "Jarrod Watts",
    "url": "https://github.com/jarrodwatts"
  },
  "plugins": [
    {
      "name": "claude-hud",
      "source": "./",
      "description": "A beautiful, customizable HUD for Claude Code.",
      "category": "productivity",
      "tags": ["hud", "statusline", "productivity"]
    }
  ]
}
```

关键点：
- **`source: "./"`**：插件在同一个仓库的根目录
- **市场即插件**：一个仓库同时扮演两个角色

### plugin.json

```json
{
  "name": "claude-hud",
  "version": "0.1.0",
  "description": "A beautiful, customizable heads-up display (HUD) for Claude Code.",
  "author": {
    "name": "Jarrod Watts",
    "url": "https://github.com/jarrodwatts"
  },
  "commands": [
    "./commands/setup.md",
    "./commands/configure.md"
  ],
  "homepage": "https://github.com/jarrodwatts/claude-hud",
  "repository": "https://github.com/jarrodwatts/claude-hud",
  "license": "MIT",
  "keywords": ["claude-code", "hud", "statusline", "plugin"]
}
```

关键点：
- **显式声明 commands**：虽然 `commands/` 目录会被自动发现，但显式声明更清晰
- **完整元数据**：homepage、repository、license、keywords 一应俱全

### 用户安装流程

```text
/plugin marketplace add jarrodwatts/claude-hud
/plugin install claude-hud
/reload-plugins
/claude-hud:setup        ← post-install 配置由命令完成
/claude-hud:configure    ← 调整 HUD 显示选项
```

### 此模式的设计哲学

1. **复杂配置后置到命令**：`plugin.json` 只负责暴露命令入口，真正的配置逻辑在 `setup.md` 中
2. **仓库自包含**：克隆一个仓库即可获得市场 + 插件
3. **README 明确安装步骤**：降低用户试错成本

---

## 3. clangd-lsp — LSP 插件

**来源**：官方市场 `claude-plugins-official`
**类型**：LSP 语言服务器
**复杂度**：⭐（代码极简，但概念独特）

### 目录结构

```
clangd-lsp/
├── LICENSE
└── README.md
```

> **惊人之处**：不存在 `.claude-plugin/plugin.json` 文件。所有配置直接在 marketplace.json 的插件条目中声明。

### marketplace.json 中的条目

```json
{
  "name": "clangd-lsp",
  "version": "1.0.0",
  "description": "C/C++ language server (clangd) for code intelligence",
  "author": {
    "name": "Anthropic",
    "email": "support@anthropic.com"
  },
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

关键点：
- **无 plugin.json**：LSP 插件的配置完全在 marketplace 条目中
- **`lspServers`**：直接声明语言服务器配置
- **`extensionToLanguage`**：将文件扩展名映射到 LSP 语言 ID
- **`strict: false`**：服务器不可用时不影响插件加载

### 其他 LSP 插件的变异

不同类型的 LSP 插件在 `lspServers` 配置上有细微差异：

```json
// Java 需要更长的启动超时
{ "command": "jdtls", "startupTimeout": 120000, "extensionToLanguage": { ".java": "java" } }

// Python 有多个扩展名
{ "command": "pyright-langserver", "args": ["--stdio"], "extensionToLanguage": { ".py": "python", ".pyi": "python" } }

// TypeScript 有 8 个扩展名映射
{ "command": "typescript-language-server", "args": ["--stdio"], "extensionToLanguage": {
  ".ts": "typescript", ".tsx": "typescriptreact", ".js": "javascript", ".jsx": "javascriptreact",
  ".mts": "typescript", ".cts": "typescript", ".mjs": "javascript", ".cjs": "javascript"
}}
```

### 此模式的要点

| 特性 | 说明 |
|------|------|
| 配置位置 | marketplace.json（而非 plugin.json） |
| 用户交互 | 无命令/技能 —— 对用户完全透明 |
| 依赖 | 需要系统已安装对应的 LSP 服务器二进制 |
| 适用场景 | 任何标准 LSP 语言服务器 |

---

## 4. security-guidance — 复杂钩子插件

**来源**：官方市场 `claude-plugins-official`
**类型**：安全审查，以钩子为主
**复杂度**：⭐⭐⭐⭐

### 目录结构

```
security-guidance/
├── .claude-plugin/
│   └── plugin.json
├── hooks/
│   ├── hooks.json              # 核心：9 个事件处理器的钩子配置
│   ├── sg-python.sh            # Python 环境启动脚本
│   ├── ensure_agent_sdk.py     # 依赖检查
│   └── security_reminder_hook.py  # 安全审查核心逻辑
```

### plugin.json

```json
{
  "name": "security-guidance",
  "version": "2.0.3",
  "description": "Security review for Claude-generated code. Pattern-based warnings on edits, LLM-powered diff review on Stop, and an agentic commit reviewer that catches injection, XSS, SSRF, hardcoded secrets, and 25+ other vulnerability classes.",
  "author": {
    "name": "David Dworken",
    "email": "dworken@anthropic.com"
  },
  "homepage": "https://github.com/anthropics/claude-plugins-official/tree/main/plugins/security-guidance"
}
```

### hooks/hooks.json（注释版）

```json
{
  "SessionStart": [{
    "hooks": [{
      "type": "command",
      "command": "bash \"${CLAUDE_PLUGIN_ROOT}/hooks/sg-python.sh\" \"${CLAUDE_PLUGIN_ROOT}/hooks/ensure_agent_sdk.py\"",
      "timeout": 180
    }]
  }],

  "UserPromptSubmit": [{
    "hooks": [{
      "type": "command",
      "command": "bash \"${CLAUDE_PLUGIN_ROOT}/hooks/sg-python.sh\" \"${CLAUDE_PLUGIN_ROOT}/hooks/security_reminder_hook.py\""
    }]
  }],

  "PostToolUse": [
    {
      "matcher": "Edit|Write|MultiEdit|NotebookEdit",
      "hooks": [{
        "type": "command",
        "command": "bash \"${CLAUDE_PLUGIN_ROOT}/hooks/sg-python.sh\" \"${CLAUDE_PLUGIN_ROOT}/hooks/security_reminder_hook.py\""
      }]
    },
    {
      "matcher": "Bash",
      "hooks": [
        {
          "type": "command",
          "command": "bash … security_reminder_hook.py",
          "if": "Bash(git commit:*)",
          "asyncRewake": true,
          "rewakeMessage": "Background security review of commit — address or acknowledge the findings below…",
          "rewakeSummary": "Commit security review found issues"
        },
        {
          "type": "command",
          "command": "bash … security_reminder_hook.py",
          "if": "Bash(git push:*)",
          "asyncRewake": true,
          "rewakeMessage": "Background security review of pushed commits not yet reviewed…",
          "rewakeSummary": "Push security review found issues"
        }
      ]
    }
  ],

  "Stop": [{
    "hooks": [{
      "type": "command",
      "command": "bash … security_reminder_hook.py",
      "asyncRewake": true,
      "rewakeMessage": "Background security review feedback…",
      "rewakeSummary": "Background security review found issues"
    }]
  }]
}
```

### 架构分析

该插件利用了四种钩子事件：

```
SessionStart ─→ 确保 Python 环境和依赖可用
     │
UserPromptSubmit ─→ 每次用户输入时注入安全提醒
     │
PostToolUse ─→ Edit|Write|MultiEdit|NotebookEdit：检查编辑内容
     │         └→ Bash(git commit:*|git push:*|gt create:*|gt modify:*|gt submit:*)
     │             异步审查 Git 操作，完成后自动唤醒
     │
Stop ─→ 会话结束时的后台安全审查
```

关键设计模式：

1. **分层防御**：在 SessionStart → 每次编辑 → Git 操作 → Stop 四个层面设置检查点
2. **异步非阻塞**：对耗时的审查操作（git commit/push）使用 `asyncRewake: true`，用户体验不被阻塞
3. **精确匹配**：使用 `if` 条件匹配具体命令模式（如 `Bash(git commit:*)`）
4. **统一入口脚本**：所有钩子通过 `sg-python.sh` 启动，确保 Python 环境一致

### 此模式的要点

| 特性 | 说明 |
|------|------|
| 核心组件 | Hooks（无 commands、无 agents、无 skills） |
| 事件覆盖 | SessionStart + UserPromptSubmit + PostToolUse + Stop |
| 异步模式 | git commit/push 审查使用 asyncRewake |
| 条件过滤 | 通过 `matcher` + `if` 精确控制触发条件 |
| 适用场景 | CI/CD 质量门禁、代码规范检查、安全审计 |

---

## 模式对比

| 维度 | GitHub | claude-hud | clangd-lsp | security-guidance |
|------|--------|------------|------------|-------------------|
| **核心组件** | MCP (HTTP) | Commands | LSP | Hooks × 4 events |
| **plugin.json** | 极简 | 完整元数据 | **无** | 精简 |
| **文件数量** | 2 | 4+ | 2 | 6+ |
| **用户交互** | 透明（工具自动出现） | 手动 `/` 命令 | 透明 | 透明 |
| **安装后步骤** | 无需 | `/setup` 命令 | 无需 | 无需 |
| **维护复杂度** | 低 | 中 | 低 | 高 |
| **适合模仿** | 任何 API 集成 | 工具型插件 | 语言支持 | 自动化审查 |
