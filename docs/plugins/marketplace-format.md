# Marketplace 格式参考

> `marketplace.json` 是 Claude Code 插件市场的入口文件。
> 它声明市场中包含哪些插件，以及每个插件的来源位置。

## 文件位置

```
<marketplace-root>/.claude-plugin/marketplace.json
```

`marketplace.json` 必须放在仓库或目录的 `.claude-plugin/` 子目录中。当用户执行
`/plugin marketplace add <owner>/<repo>` 时，Claude Code 会克隆仓库并读取此文件。

## 顶层字段

```json
{
  "$schema": "https://anthropic.com/claude-code/marketplace.schema.json",
  "name": "marketplace-name",
  "description": "Marketplace description.",
  "owner": {
    "name": "Owner Name",
    "email": "owner@example.com"
  },
  "plugins": [ ... ]
}
```

| 字段 | 类型 | 必需 | 说明 |
|------|------|------|------|
| `$schema` | string (URL) | 否 | JSON Schema 引用 |
| `name` | string | 是 | 市场名称，kebab-case |
| `description` | string | 否 | 市场描述 |
| `owner` | object | 否 | `name` (必需)、`email` (可选) |
| `plugins` | array | 是 | 插件条目列表 |

## plugins[] — 插件条目字段

每个插件条目描述一个可安装的插件。

### 核心字段

| 字段 | 类型 | 必需 | 说明 |
|------|------|------|------|
| `name` | string | **是** | 插件名，kebab-case，需与 `plugin.json` 中的 `name` 一致 |
| `description` | string | 推荐 | 插件功能描述（50-200 字符） |
| `version` | string | 否 | 语义版本号，如 `"1.0.0"` |
| `author` | object | 否 | `name` (必需)、`email` (可选)、`url` (可选) |

### 分类字段

| 字段 | 类型 | 说明 |
|------|------|------|
| `category` | string | 分类：`development`, `productivity`, `database`, `security`, `monitoring`, `design`, `deployment`, `testing`, `learning`, `location`, `math` |
| `displayName` | string | 在 UI 中显示的名称（可含空格、大小写） |
| `tags` | array | 标签列表，如 `["community-managed"]` |
| `keywords` | array | 搜索关键词列表 |
| `homepage` | string (URL) | 插件主页或文档链接 |

### source — 插件来源

`source` 是最关键的字段，它告诉 Claude Code 从哪获取插件代码。有 **4 种格式**：

#### 1. 本地子目录 `source: "./path"`

插件代码在同一仓库的子目录中。这是官方市场 `claude-plugins-official` 管理自有插件的方式。

```json
{
  "name": "code-review",
  "source": "./plugins/code-review"
}
```

> **适用场景**：市场仓库和插件代码在同一个 Git 仓库中，插件放在 `plugins/` 子目录下。

#### 2. 外部 Git 仓库 `source: { source: "url", ... }`

插件在独立的外部 Git 仓库中。SHA 锁定版本。

```json
{
  "name": "stripe",
  "source": {
    "source": "url",
    "url": "https://github.com/stripe/ai.git",
    "sha": "b8f6adcb5d05f6ff01334411561ee8cb1ec014c6"
  }
}
```

> **适用场景**：插件由第三方维护在独立仓库中。`sha` 用于锁定版本。

#### 3. Git 子目录 `source: { source: "git-subdir", ... }`

插件在外部仓库的特定子目录中。适用于大型 monorepo 中只发布部分内容。

```json
{
  "name": "aws-core",
  "source": {
    "source": "git-subdir",
    "url": "https://github.com/aws/agent-toolkit-for-aws.git",
    "path": "plugins/aws-core",
    "ref": "main",
    "sha": "c0991f463b54ac94af32a730d6d13293dcff98cf"
  }
}
```

| 子字段 | 说明 |
|--------|------|
| `source` | 固定为 `"git-subdir"` |
| `url` | Git 仓库地址 |
| `path` | 仓库中插件所在的子目录路径 |
| `ref` | 分支名或 tag 名 |
| `sha` | 锁定到特定 commit |

> **适用场景**：插件在 monorepo 的子目录中（如 `plugins/aws-core`）。

#### 4. GitHub 快捷方式 `source: { source: "github", ... }`

GitHub 仓库的简写形式。

```json
{
  "name": "jfrog",
  "source": {
    "source": "github",
    "repo": "jfrog/claude-plugin",
    "commit": "259c8e718266c16e99b4f30ae9b1ed0f9f00d98d",
    "sha": "6788fe15d4a63d47f038c05e58ae533aeb2dadb6"
  }
}
```

| 子字段 | 说明 |
|--------|------|
| `source` | 固定为 `"github"` |
| `repo` | `owner/repo` 格式 |
| `commit` | Commit SHA（可选） |
| `sha` | 另一个 SHA 标识（可选） |

> **适用场景**：插件是独立的 GitHub 仓库，不需要指定子目录。

### 内联组件覆盖

在 marketplace.json 的插件条目中，可以直接声明组件配置，覆盖插件自身的默认配置：

```json
{
  "name": "box",
  "source": { "source": "url", "url": "...", "sha": "..." },
  "skills": [
    "./skills/box",
    "./skills/box-legal-workflows"
  ]
}
```

| 字段 | 说明 |
|------|------|
| `skills` | 额外加载的技能路径列表 |
| `commands` | 额外的命令目录 |
| `agents` | 额外的代理目录 |
| `hooks` | 钩子配置路径或内联对象 |
| `mcpServers` | MCP 服务器配置路径或内联对象 |

### LSP 服务器配置

LSP（Language Server Protocol）插件可以直接在 marketplace.json 中声明，无需独立的 `plugin.json`：

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
        ".hpp": "cpp"
      }
    }
  }
}
```

| 字段 | 说明 |
|------|------|
| `lspServers` | LSP 服务器定义对象，key 为服务器名 |
| `lspServers.<name>.command` | 启动命令 |
| `lspServers.<name>.args` | 命令行参数（可选） |
| `lspServers.<name>.extensionToLanguage` | 文件扩展名到语言 ID 的映射 |
| `lspServers.<name>.startupTimeout` | 启动超时毫秒数（可选，如 `120000`） |
| `strict` | 是否严格模式（`false` 表示宽松加载） |

## 完整示例

### 最小市场（单插件，自包含）

适合 `claude-hud` 模式：一个仓库既是市场又是唯一插件。

```json
{
  "name": "my-plugin-marketplace",
  "description": "Marketplace for my Claude Code plugin.",
  "owner": { "name": "My Name" },
  "plugins": [
    {
      "name": "my-plugin",
      "description": "Does something useful.",
      "source": "./"
    }
  ]
}
```

### 多插件本地市场

适合管理多个自有插件的组织。

```json
{
  "$schema": "https://anthropic.com/claude-code/marketplace.schema.json",
  "name": "my-org-plugins",
  "description": "Official plugins from My Org.",
  "owner": { "name": "My Org", "email": "plugins@myorg.com" },
  "plugins": [
    {
      "name": "plugin-alpha",
      "description": "First plugin.",
      "source": "./plugins/plugin-alpha",
      "category": "development"
    },
    {
      "name": "plugin-beta",
      "description": "Second plugin.",
      "source": "./plugins/plugin-beta",
      "category": "productivity"
    }
  ]
}
```

### 引用外部插件的市场

适合聚合第三方插件的市场。

```json
{
  "name": "third-party-marketplace",
  "description": "Curated third-party Claude Code plugins.",
  "owner": { "name": "Curator" },
  "plugins": [
    {
      "name": "external-tool",
      "description": "An external MCP-based plugin.",
      "category": "development",
      "source": {
        "source": "url",
        "url": "https://github.com/someone/external-tool.git",
        "sha": "abc123..."
      },
      "homepage": "https://github.com/someone/external-tool"
    },
    {
      "name": "monorepo-component",
      "description": "A plugin from a monorepo.",
      "category": "security",
      "source": {
        "source": "git-subdir",
        "url": "https://github.com/org/monorepo.git",
        "path": "claude-plugins/security-scanner",
        "ref": "main",
        "sha": "def456..."
      }
    }
  ]
}
```

## 验证清单

发布 marketplace 前检查：

- [ ] `.claude-plugin/marketplace.json` 路径正确
- [ ] JSON 语法有效
- [ ] `name` 字段存在且为 kebab-case
- [ ] `plugins[]` 非空
- [ ] 每个插件条目的 `name` 与对应 `plugin.json` 中的 `name` 一致
- [ ] 每个插件的 `source` 指向正确的位置（本地路径存在 / URL 可达）
- [ ] `git-subdir` 类型的 source 同时指定了 `path` 和 `ref`
- [ ] SHA 值有效（对于锁定版本的 source）
