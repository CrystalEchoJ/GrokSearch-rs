# Claude Code 插件系统架构

> 本文档从实际安装的官方市场和第三方插件中提炼，覆盖从最简 MCP 插件到复杂多组件插件的完整知识。

## 两层架构

Claude Code 的插件系统由两层组成：

```text
Marketplace（市场层）
├── marketplace.json          # 声明"这个市场里有哪些插件"
│   └── plugins[]
│       ├── 插件 A  → source: "./plugins/a"          (本地子目录)
│       ├── 插件 B  → source: { url + sha }          (外部 Git 仓库)
│       └── 插件 C  → source: { git-subdir + ref }   (Git 子目录)
│
Plugin（插件层）
└── .claude-plugin/plugin.json   # 声明"这个插件具体包含什么"
    ├── commands/                # Slash 命令
    ├── agents/                  # 子代理
    ├── skills/                  # 自动激活技能
    ├── hooks/hooks.json         # 事件钩子
    ├── .mcp.json                # MCP 服务器配置
    └── (scripts/, lib/, …)      # 辅助脚本
```

### 关键概念

| 概念 | 作用 | 入口文件 |
|------|------|----------|
| **Marketplace** | 插件目录，告诉 Claude Code 有哪些插件可用 | `.claude-plugin/marketplace.json` |
| **Plugin** | 插件本身，包含命令、技能、钩子等组件 | `.claude-plugin/plugin.json` |
| **Component** | 插件的功能单元：命令、代理、技能、钩子、MCP 服务器、LSP 服务器 | 自动发现 + 可选声明 |

### 自动发现机制

Claude Code 遵循**约定优于配置**原则。只要把文件放在正确的目录，它们就会被自动加载：

| 目录/文件 | 自动发现规则 |
|-----------|-------------|
| `commands/*.md` | 所有 `.md` 文件自动注册为 `/` 命令 |
| `agents/*.md` | 所有 `.md` 文件自动注册为子代理 |
| `skills/*/SKILL.md` | 所有包含 `SKILL.md` 的子目录自动注册为技能 |
| `hooks/hooks.json` | 默认钩子配置文件 |
| `.mcp.json` | 默认 MCP 服务器配置文件 |

在 `plugin.json` 中声明自定义路径会**补充**而非替代默认路径。

## 插件安装流程

```text
用户操作                          Claude Code 内部行为
───────────────────────────────  ─────────────────────────────
/plugin marketplace add <repo>   克隆仓库到 ~/.claude/plugins/marketplaces/
                                 读取 marketplace.json
                                 缓存插件目录

/plugin install <name>           从 marketplace 找到插件 source
                                 克隆/复制插件到 ~/.claude/plugins/cache/
                                 读取 .claude-plugin/plugin.json
                                 扫描并注册所有组件

/reload-plugins                  重新加载所有已安装插件
```

## 插件类型速查

根据你想构建的插件类型，选择对应的参考文档：

| 我想做… | 关键文件 | 参考案例 |
|---------|---------|---------|
| 纯 MCP 工具集成（如连接 API） | `plugin.json` + `.mcp.json` | [GitHub 案例](case-studies.md#1-github---纯-mcp-插件) |
| Slash 命令集合 | `plugin.json` + `commands/` | [claude-hud 案例](case-studies.md#2-claude-hud---自包含市场插件) |
| 自动激活的技能 | `plugin.json` + `skills/` | 无案例（见[组件文档](plugin-components.md#3-skills)） |
| LSP 语言服务器 | `marketplace.json` 中的 `lspServers` | [clangd-lsp 案例](case-studies.md#3-clangd-lsp---lsp-插件) |
| 安全/质量钩子 | `plugin.json` + `hooks/hooks.json` | [security-guidance 案例](case-studies.md#4-security-guidance---复杂钩子插件) |
| 自建第三方市场 | `marketplace.json` + `plugin.json` | [claude-hud 案例](case-studies.md#2-claude-hud---自包含市场插件) |

## 文档导航

| 文档 | 内容 | 适合场景 |
|------|------|----------|
| [marketplace-format.md](marketplace-format.md) | `marketplace.json` 完整字段参考、source 类型详解 | 创建/维护插件市场 |
| [plugin-manifest.md](plugin-manifest.md) | `plugin.json` 完整字段参考、路径规则、验证 | 创建任何类型的插件 |
| [plugin-components.md](plugin-components.md) | 六大组件类型详解：命令、代理、技能、钩子、MCP、LSP | 为插件添加特定功能 |
| [case-studies.md](case-studies.md) | 4 个真实案例的目录结构和配置文件剖析 | 参考成熟插件的实现模式 |

## 关键环境变量

| 变量 | 用途 |
|------|------|
| `${CLAUDE_PLUGIN_ROOT}` | 插件安装根目录的绝对路径，用于钩子脚本和 MCP 服务器引用插件内文件 |

> **重要**：永远不要在插件配置中硬编码绝对路径。使用 `${CLAUDE_PLUGIN_ROOT}` 确保插件在不同安装位置下都能正常工作。

## 参考资料

- 官方市场仓库：<https://github.com/anthropics/claude-plugins-official>
- `plugin-dev` 插件（通过官方市场安装）：包含完整的插件开发技能
- `claude-hud` 第三方市场案例：<https://github.com/jarrodwatts/claude-hud>
