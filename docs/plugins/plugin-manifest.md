# Plugin Manifest 参考 (`plugin.json`)

> `.claude-plugin/plugin.json` 是每个 Claude Code 插件的必需入口文件。
> 它声明插件的身份、元数据和组件路径。

## 文件位置

```
<plugin-root>/.claude-plugin/plugin.json
```

**必须**放在 `.claude-plugin/` 目录中。Claude Code 通过此文件发现和加载插件。

## 完整字段参考

### name（必需）

- **类型**：`string`
- **格式**：kebab-case（小写字母、数字、连字符）
- **正则验证**：`^[a-z][a-z0-9]*(-[a-z0-9]+)*$`

插件的唯一标识符。用于插件识别、冲突检测和命令命名空间。

```
✅ api-tester, code-review, git-workflow-automation
❌ API Tester, code_review, -git-workflow, test-
```

### version

- **类型**：`string`
- **格式**：语义版本 `MAJOR.MINOR.PATCH`
- **默认**：`"0.1.0"`

支持预发布版本：`"1.0.0-alpha.1"`, `"1.0.0-beta.2"`, `"1.0.0-rc.1"`。

### description

- **类型**：`string`
- **长度**：推荐 50-200 字符

简明描述插件功能。使用主动语态，聚焦于"做什么"而非"怎么做"。

### author

- **类型**：`object` 或 `string`

```json
// 对象格式
{ "author": { "name": "Jane Dev", "email": "jane@example.com", "url": "https://jane.dev" } }

// 字符串格式
{ "author": "Jane Dev <jane@example.com> (https://jane.dev)" }
```

仅 `name` 是必需的。

### 元数据字段

| 字段 | 类型 | 说明 |
|------|------|------|
| `homepage` | string (URL) | 插件文档/主页链接 |
| `repository` | string (URL) 或 object | 源码仓库地址，支持 `{ "type": "git", "url": "...", "directory": "..." }` |
| `license` | string | SPDX 标识符：`MIT`, `Apache-2.0`, `GPL-3.0`, `BSD-3-Clause`, `ISC`, `UNLICENSED` |
| `keywords` | array of strings | 搜索关键词，推荐 5-10 个 |

### 组件路径字段

所有路径必须：
- 使用相对路径
- 以 `./` 开头
- 不能使用 `../`（不能跳出插件根目录）
- 不能使用绝对路径

#### commands

- **类型**：`string` 或 `string[]`
- **默认**：`["./commands"]`

额外的命令文件目录。**补充**默认的 `./commands/` 目录而非替换。

```json
// 单个额外路径
{ "commands": "./custom-commands" }

// 多个额外路径
{ "commands": ["./commands", "./admin-commands", "./experimental-commands"] }
```

#### agents

- **类型**：`string` 或 `string[]`
- **默认**：`["./agents"]`

额外的子代理文件目录。格式和规则与 `commands` 相同。

#### hooks

- **类型**：`string`（文件路径） 或 `object`（内联配置）
- **默认**：`"./hooks/hooks.json"`

```json
// 文件路径（推荐复杂插件）
{ "hooks": "./config/hooks.json" }

// 内联配置（适合简单插件）
{
  "hooks": {
    "PreToolUse": [{
      "matcher": "Write",
      "hooks": [{
        "type": "command",
        "command": "bash ${CLAUDE_PLUGIN_ROOT}/scripts/validate.sh"
      }]
    }]
  }
}
```

详见 [plugin-components.md § Hooks](plugin-components.md#4-hooks)。

#### mcpServers

- **类型**：`string`（文件路径） 或 `object`（内联配置）
- **默认**：`"./.mcp.json"`

```json
// 文件路径（推荐）
{ "mcpServers": "./.mcp.json" }

// 内联配置
{
  "mcpServers": {
    "my-server": {
      "command": "node",
      "args": ["${CLAUDE_PLUGIN_ROOT}/servers/my-mcp.js"],
      "env": { "API_KEY": "${API_KEY}" }
    }
  }
}
```

详见 [plugin-components.md § MCP Servers](plugin-components.md#5-mcp-servers)。

## 路径解析规则

插件加载时的组件发现顺序：

1. **扫描默认位置**：`./commands/`、`./agents/`、`./skills/`、`./hooks/hooks.json`、`./.mcp.json`
2. **扫描声明路径**：`plugin.json` 中 `commands`、`agents`、`hooks`、`mcpServers` 字段的路径
3. **合并加载**：所有位置的组件都会被加载，不会互相覆盖。名称冲突会导致错误。

## 常见验证错误

| 错误 | 原因 | 修复 |
|------|------|------|
| `"name": "My Plugin"` | 包含空格 | 使用 kebab-case：`"my-plugin"` |
| `"commands": "/abs/path"` | 绝对路径 | 使用 `"./commands"` |
| `"hooks": "hooks/hooks.json"` | 缺少 `./` 前缀 | 使用 `"./hooks/hooks.json"` |
| `"version": "1.0"` | 非语义版本 | 使用 `"1.0.0"` |

## 示例

### 最小插件

只依赖默认目录自动发现：

```json
{
  "name": "hello-world"
}
```

有效的前提是存在 `commands/`、`skills/` 或 `agents/` 中的至少一个组件。

### 推荐配置（适合发布）

```json
{
  "name": "code-review-assistant",
  "version": "1.0.0",
  "description": "Automates code review with style checks and suggestions.",
  "author": {
    "name": "Jane Developer",
    "email": "jane@example.com"
  },
  "homepage": "https://docs.example.com/code-review",
  "repository": "https://github.com/janedev/code-review-assistant",
  "license": "MIT",
  "keywords": ["code-review", "automation", "quality", "ci-cd"]
}
```

### 完整配置（企业级）

```json
{
  "name": "enterprise-devops",
  "version": "2.3.1",
  "description": "Comprehensive DevOps automation for enterprise CI/CD pipelines.",
  "author": {
    "name": "DevOps Team",
    "email": "devops@company.com",
    "url": "https://company.com/devops"
  },
  "homepage": "https://docs.company.com/plugins/devops",
  "repository": {
    "type": "git",
    "url": "https://github.com/company/devops-plugin.git"
  },
  "license": "Apache-2.0",
  "keywords": ["devops", "ci-cd", "kubernetes", "deployment"],
  "commands": ["./commands", "./admin-commands"],
  "agents": "./specialized-agents",
  "hooks": "./config/hooks.json",
  "mcpServers": "./.mcp.json"
}
```

### GitHub 插件的实际 plugin.json

这是从官方市场安装的 GitHub 插件的真实 manifest —— 极其简洁：

```json
{
  "name": "github",
  "description": "Official GitHub MCP server for repository management. Create issues, manage pull requests, review code, search repositories, and interact with GitHub's full API directly from Claude Code.",
  "author": {
    "name": "GitHub"
  }
}
```

它只有 `name`、`description` 和 `author`。所有功能通过 `.mcp.json` 中配置的 HTTP MCP 服务器提供 —— 无需 `commands`、`agents` 或 `skills` 目录。
