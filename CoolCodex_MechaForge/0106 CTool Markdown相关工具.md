

## ctool_read_markdown

核心能力：

- 读取 Markdown 正文。
- 默认忽略 `scratch` 标题及其子标题内容。
- 支持忽略以下标题层级：`# scratch`、`## scratch`、`### scratch`、`#### scratch`、`##### scratch`、`###### scratch`。
- 可选参数允许显式包含 `scratch` 内容。
- 可选只读取某个标题 section。
- 可选返回目录结构 TOC。
- 可选限制最大字数 / 最大标题数。


## CToolAnnotateMarkdown

Rust 类型名：

- `CToolAnnotateMarkdown`

文件名：

- `ctool_annotate_markdown.rs`

工具名：

- `ctool_annotate_markdown`

### 功能介绍

- 给 Markdown 文档添加批注块。
- 只用于 `.md` / `.markdown` 文档。
- 使用 Obsidian callout 语法插入批注，不使用 HTML `<mark>` 高亮。
- AI 只需要决定：批注插入哪一行之后、批注级别、方向、标题、内容。
- 工具负责把结构化参数翻译成最终 annotation 格式。

对于插入哪里，函数只提供插入能力。插入得对不对，应该由上游决定。这个工具是一个纯粹的 Markdown 批注插入工具。

### 权限

- 只能操作 Markdown 文件。
- 可以对 CToolScope 中 ReadWrite 和 ReadOnly 级别的 Markdown 文件进行批注。
- 不可以批注 Hidden 级别文件。

### 保护规则

- 批注不可以插入到代码块内部。
- 如果 `insert_after_line` 位于代码块内部，则批注失败。
- 工具只新增批注块，不改写、不删除、不重排原文。

### 函数参数

```json
{
  "path": "docs/design.md",
  "insert_after_line": 42,
  "annotation_kind": "normal",
  "direction": "down",
  "title": "标题",
  "content": "批注内容",
  "dry_run": false
}
```

参数说明：

- `path`
  - Markdown 文件路径。

- `insert_after_line`
  - 批注插入在哪一行之后。
  - 1-based 行号。
  - 可选允许 `0` 表示插入到文件开头。

- `annotation_kind`
  - `normal`：普通批注。
  - `critical`：警告型 / 严重问题批注。

- `direction`
  - `up`：生成 `↑`。
  - `down`：生成 `↓`。

- `title`
  - 批注标题。
  - 可为空。

- `content`
  - 批注正文。
  - 支持多行。

- `dry_run`
  - `true`：只预览，不写入。
  - `false`：实际写入。

### 返回结果

成功时返回：

```json
{
  "success": true,
  "path": "docs/design.md",
  "insert_after_line": 42,
  "annotation_kind": "normal",
  "direction": "down",
  "preview": "修改后的局部预览"
}
```

失败时返回：

```json
{
  "success": false,
  "path": "docs/design.md",
  "reason": "失败原因"
}
```

### 生成格式

普通批注：

```markdown
> [!annotation] ↓ 标题
批注内容
```

警告型批注：

```markdown
> [!critical-annotation] ↑ 标题
批注内容
```

多行内容：

```markdown
> [!annotation] ↓ 标题
第一行批注
第二行批注
第三行批注
```

### 生成规则

- 只有第一行使用 Obsidian callout 标记。
- 除第一行之外，批注正文不添加 `>` 前缀。
- 工具会自动在批注块上方添加一个空行。
- 前后空行由工具函数生成，AI 不需要、也不应该在 `content` 中手动补空行。
- 工具会自动在批注正文最后一行下方添加一个空行。
- 前后空行用于隔离上下文，防止 Obsidian 把相邻正文误渲染进批注块。

最终插入效果：

```markdown
原文上一段。

> [!annotation] ↓ 标题
第一行批注
第二行批注

原文下一段。
```
