# ctool_tavily_search_request 产品方案

## 1. 产品定位

`ctool_tavily_search_request` 是 CoolCode / CoolTool 体系中的受控联网搜索工具。

它不是 Codex 原生联网搜索，也不是默认自由联网能力。它属于 `ctool_` 工具体系，必须受到 SafeMode、系统配置、项目配置和日志审计的管制。

核心目标：

```text
让 Codex 在需要外部资料时，可以通过 Tavily 搜索 / 抽取 / 放大网页内容；
但所有搜索行为必须可记录、可复查、可限制、可缓存。
```

---

## 2. 总体设计原则

### 2.1 仍然受 SafeMode 管制

`ctool_tavily_search_request` 不能绕过 SafeMode。

执行链路：

```text
SafeMode ON
=> 允许 CTool
=> ctool_tavily_search_request
=> 配置检查
=> 风险判断
=> 必要时确认
=> 写入缓存与日志
=> 返回缓存文件路径
```

### 2.2 搜索结果必须落盘

搜索结果不直接大段塞回 Codex 上下文。

工具执行后，只返回：

```text
1. 生成的 Markdown 文件路径
2. 简短摘要
3. 下一步建议
```

完整搜索结果写入 `.coolcache/web_search/YYYY-MM-DD/`。

### 2.3 Codex 只读搜索缓存

搜索工具可以写入搜索缓存目录。

普通 CTool 文件工具对搜索缓存目录只读：

```text
ctool_tavily_search_request：可写 .coolcache/web_search
Codex 普通 CTool：可读 .coolcache/web_search
Codex 普通 CTool：不可改、不可删、不可移动 .coolcache/web_search
```

推荐把搜索缓存目录加入项目配置：

```toml
[ctool_scope]
visible_paths = [
  ".coolcache\\web_search"
]

protected_paths = [
  ".coolcache\\web_search"
]
```

### 2.4 每次请求都生成新文件

搜索、抽取、放大都生成新文件，不覆盖旧文件。

---

## 3. 配置文件

沿用 Cool 配置体系：

```text
.coolsystemconfig.toml  系统级，优先级更高
.coolconfig.toml        项目级，优先级更低
```

系统配置权限更大。

系统配置可以强制禁止图片、强制限制 provider、强制限制文件类型、强制升级风险。

---

## 4. 配置段设计

建议配置段名：

```toml
[ctool_tavily_search]
```

示例：

```toml
[ctool_tavily_search]
enabled = true

provider = "tavily"

cache_root = ".coolcache\\web_search"

default_text_search_risk = "green"
default_image_search_risk = "red"

allow_text_search_without_confirmation = true

allow_image_search = false
allowed_image_extensions = [
  ".jpg",
  ".jpeg",
  ".png",
  ".webp"
]

max_search_results = 8
max_extract_chars = 12000
max_zoom_chars = 6000

write_request_log = true
write_result_markdown = true

red_keywords = [
  "token",
  "api key",
  "apikey",
  "password",
  "secret",
  "bearer",
  "sk-",
  "tvly-"
]
```

系统级配置可以更严格，例如：

```toml
[ctool_tavily_search]
allow_image_search = false

red_keywords = [
  "token",
  "api key",
  "apikey",
  "password",
  "secret",
  "bearer",
  "sk-",
  "tvly-",
  "private key",
  "ssh key"
]
```

---

## 5. 工具拆分

建议先做两个工具。

### 5.1 纯文本搜索工具

工具名：

```text
ctool_tavily_text_search_request
```

能力：

```text
search
extract
zoom
research
```

特征：

```text
只返回文字 / Markdown
不返回图片
不下载文件
不打开浏览器
不执行网页脚本
普通搜索默认放行
敏感搜索升级确认
```

### 5.2 带图片搜索工具

工具名：

```text
ctool_tavily_image_search_request
```

能力：

```text
search_with_images
extract_with_images
```

特征：

```text
可以返回配置允许的图片类型
必须二次确认
只能保存到搜索缓存目录
不自动打开图片
不自动嵌入网页
不允许 SVG
不允许可执行文件 / 压缩包 / 脚本
```

第一版建议只实现 `ctool_tavily_text_search_request`。图片能力先设计好，但默认关闭。

---

## 6. 风险等级规则

### 6.1 文本搜索

普通文本搜索默认可以放行：

```text
普通技术搜索：Green / Auto Approved
包含敏感信息：Red
上传本地源码大段内容：Red
要求搜索 token、key、secret：Red
```

普通搜索示例：

```text
rust cargo workspace dependency
ratatui textarea enter handler
windows rust path canonicalize
```

这种可以默认放行，并写入日志。

### 6.2 图片搜索

图片搜索默认 Red，必须二次确认。

原因：

```text
图片通常不是可执行程序，但图片解析器历史上存在过漏洞；
图片也可能携带误导性视觉内容；
远程图片可能暴露访问行为或引入追踪资源。
```

允许的图片类型第一版只建议：

```text
.jpg
.jpeg
.png
.webp
```

不允许：

```text
.svg
.gif
.bmp
.ico
.tif
.tiff
.heic
.avif
```

SVG 按网页/脚本风险处理，不按普通图片处理。

---

## 7. 请求展示格式

所有 Tavily 搜索请求都必须明显展示，方便回看日志。

### 7.1 文本搜索自动放行展示

```text
==============================
🟢 TAVILY SEARCH REQUEST: GREEN
Provider: Tavily
Mode: Text
Action: search
CurrentDir: C:\CodexLab\codex\codex-rs
Auto Approved: text search allowed by config

Query:
rust cargo workspace dependency

Will write:
.coolcache\web_search\2026-06-09\00000_search_rust_cargo_workspace_dependency.md
==============================
```

### 7.2 图片搜索二次确认展示

```text
==============================
🔴 TAVILY SEARCH REQUEST: RED
Provider: Tavily
Mode: Image
Action: search_with_images
CurrentDir: C:\CodexLab\codex\codex-rs

Query:
unreal engine material graph example

Allowed Image Extensions:
.jpg, .jpeg, .png, .webp

Will write:
.coolcache\web_search\2026-06-09\00001_image_search_unreal_material_graph.md

First confirm? Type Y:
Second confirm? Type RUN RED:
==============================
```

---

## 8. 确认规则

### 8.1 文本搜索

普通文本搜索可以配置为自动放行。

如果被升级为 Red，则需要二次确认。

### 8.2 图片搜索

图片搜索必须二次确认：

```text
第一次：首字母 Y / y 才进入第二步
第二次：必须输入 RUN RED
其他任何输入都拒绝
```

### 8.3 拒绝时允许带反馈

用户可以输入：

```text
N 不要联网，先看本地 Cargo.toml
```

处理方式：

```text
首字母 N => 拒绝执行
N 后面的内容 => 作为用户反馈返回给 Codex
```

如果输入不是 Y/N，也按拒绝处理，并把完整输入作为反馈。

---

## 9. 缓存目录结构

搜索缓存目录：

```text
.coolcache\web_search\YYYY-MM-DD\
```

示例：

```text
.coolcache\web_search\2026-06-09\
  request_log.md
  00000_search_rust_cargo_workspace_dependency.md
  00001_extract_cargo_docs_workspace_dependencies.md
  00002_zoom_workspace_dependencies_section.md
  00003_research_ratatui_textarea_enter.md
```

编号规则：

```text
当天从 00000 开始
每新增一个结果文件编号 +1
超过 99999 后继续 100000、100001
不截断
不覆盖
```

---

## 10. 文件命名规则

格式：

```text
00000_<action>_<slug>.md
```

示例：

```text
00000_search_rust_cargo_workspace_dependency.md
00001_extract_cargo_docs_workspace_dependencies.md
00002_zoom_workspace_dependencies_section.md
00003_research_ratatui_textarea_enter.md
```

`slug` 来源：

```text
优先使用 file_name_hint
否则根据 query / title / URL 自动生成
非法字符替换为 _
长度限制 80 字符左右
```

---

## 11. request_log.md 设计

每次搜索申请都追加到当天日志：

```markdown
# Tavily Search Request Log

Date: 2026-06-09

## 00000

Time: 2026-06-09 15:30:12
Provider: Tavily
Tool: ctool_tavily_text_search_request
Mode: Text
Action: search
Risk: Green
Approved: Auto
CurrentDir: C:\CodexLab\codex\codex-rs

Query:
rust cargo workspace dependency

Output:
00000_search_rust_cargo_workspace_dependency.md

---

## 00001

Time: 2026-06-09 15:32:44
Provider: Tavily
Tool: ctool_tavily_text_search_request
Mode: Text
Action: extract
Risk: Green
Approved: Auto
CurrentDir: C:\CodexLab\codex\codex-rs

URL:
https://doc.rust-lang.org/cargo/reference/workspaces.html

Output:
00001_extract_cargo_docs_workspace_dependencies.md
```

日志文件也属于 protected 搜索缓存，Codex 只能读，不能改。

---

## 12. 搜索结果文件格式

### 12.1 Search 文件

```markdown
# Tavily Search Result

Provider: Tavily
Kind: Search
Time: 2026-06-09 15:30:12
CurrentDir: C:\CodexLab\codex\codex-rs
Risk: Green
Approved: Auto

## Query

rust cargo workspace dependency

## Short Summary

Found official Cargo documentation and examples related to workspace dependencies.

## Results

### 1. Cargo Workspaces - The Cargo Book

URL:
https://doc.rust-lang.org/cargo/reference/workspaces.html

Summary:
...
```

### 12.2 Extract 文件

```markdown
# Tavily Extract Result

Provider: Tavily
Kind: Extract
Time: 2026-06-09 15:32:44
Source Search:
00000_search_rust_cargo_workspace_dependency.md

## URL

https://doc.rust-lang.org/cargo/reference/workspaces.html

## Short Summary

This page explains Cargo workspaces, workspace dependencies, and shared package configuration.

## Outline

- Workspaces
- The members and exclude fields
- The workspace.dependencies table

## Content

...
```

### 12.3 Zoom 文件

```markdown
# Tavily Zoom Result

Provider: Tavily
Kind: Zoom
Time: 2026-06-09 15:35:10
Source:
00001_extract_cargo_docs_workspace_dependencies.md
Target:
workspace.dependencies

## Short Summary

This section explains how workspace.dependencies are inherited by workspace members.

## Content

...
```

---

## 13. 工具返回给 Codex 的内容

不要返回完整正文。

只返回：

```text
Search completed.

Output:
.coolcache\web_search\2026-06-09\00000_search_rust_cargo_workspace_dependency.md

Summary:
Found official Cargo documentation and examples related to workspace dependencies.

Suggested next step:
Read the output file. If result 1 is useful, request extract for its URL.
```

这样不会撑爆上下文。

---

## 14. Action 设计

### 14.1 search

输入：

```json
{
  "action": "search",
  "query": "rust cargo workspace dependency",
  "file_name_hint": "rust_cargo_workspace_dependency"
}
```

输出：

```text
生成 search MD 文件
返回文件路径和摘要
```

### 14.2 extract

输入：

```json
{
  "action": "extract",
  "url": "https://doc.rust-lang.org/cargo/reference/workspaces.html",
  "source_file": ".coolcache\\web_search\\2026-06-09\\00000_search_rust_cargo_workspace_dependency.md",
  "file_name_hint": "cargo_docs_workspace_dependencies"
}
```

输出：

```text
生成 extract MD 文件
返回文件路径和摘要
```

### 14.3 zoom

输入：

```json
{
  "action": "zoom",
  "source_file": ".coolcache\\web_search\\2026-06-09\\00001_extract_cargo_docs_workspace_dependencies.md",
  "target": "workspace.dependencies",
  "file_name_hint": "workspace_dependencies_section"
}
```

输出：

```text
生成 zoom MD 文件
返回文件路径和摘要
```

### 14.4 research

输入：

```json
{
  "action": "research",
  "query": "ratatui textarea enter submit handler",
  "file_name_hint": "ratatui_textarea_enter"
}
```

输出：

```text
生成 research MD 文件
返回文件路径和摘要
```

---

## 15. 网络安全规则

### 15.1 默认禁止的行为

```text
不打开浏览器
不执行网页 JavaScript
不下载二进制文件
不下载压缩包
不下载脚本
不保存 HTML 为可执行网页
不自动嵌入远程图片
不自动打开搜索结果文件
```

### 15.2 下载相关统一高危

以下行为永远 Red：

```text
下载 exe / msi / dll
下载 bat / cmd / ps1 / sh
下载 zip / rar / 7z / tar / gz
下载未知二进制
打开网站
调用浏览器
git clone 外部仓库
curl / wget / Invoke-WebRequest 下载文件
```

### 15.3 普通搜索可默认放行

普通技术查询可以默认放行：

```text
query 仅为普通技术问题
不包含 token / key / secret
不包含大段私有代码
不要求上传本地文件
不要求下载内容
```

---

## 16. 图片搜索规则

### 16.1 默认关闭

图片搜索默认关闭或 Red。

```toml
[ctool_tavily_search]
allow_image_search = false
```

### 16.2 开启后仍需二次确认

即使配置允许图片搜索，使用时也要二次确认。

```text
图片搜索 = Red
第一次确认 Y
第二次确认 RUN RED
```

### 16.3 允许类型

第一版建议只允许：

```text
.jpg
.jpeg
.png
.webp
```

### 16.4 禁止类型

第一版禁止：

```text
.svg
.gif
.bmp
.ico
.tif
.tiff
.heic
.avif
```

SVG 按网页/脚本风险处理，不按普通图片处理。

### 16.5 图片文件保存规则

```text
只保存到 .coolcache\web_search\YYYY-MM-DD\
不自动打开
不自动预览
不自动嵌入 HTML
不允许远程 URL 图片直接进入网页
不允许 data:image
需要校验扩展名和文件头
限制文件大小
```

---

## 17. Provider 设计

第一版只实现：

```text
provider = "tavily"
```

未来可以扩展：

```text
provider = "openai_web"
provider = "bocha"
provider = "querit"
provider = "bing"
```

第一版不要同时做太多 provider。

---

## 18. 与原生搜索的关系

OpenAI 原生 web_search 更像云端托管搜索：

```text
模型自己搜索
模型自己打开页面
模型自己页内查找
模型自己组织引用
```

Tavily 方案更像安全工程化搜索：

```text
Codex 申请
工具搜索
写入缓存
写入日志
Codex 读取缓存
Codex 决定下一步
```

最终体验接近原生搜索，但多了：

```text
缓存
日志
权限
可审计
可复查
不会直接撑爆上下文
```

---

## 19. 推荐开发顺序

第一阶段：

```text
ctool_tavily_text_search_request
只支持 search
写 MD
写 request_log
普通搜索默认放行
敏感搜索 Red
```

第二阶段：

```text
支持 extract
```

第三阶段：

```text
支持 zoom
```

第四阶段：

```text
支持 research
```

第五阶段：

```text
ctool_tavily_image_search_request
默认关闭
二次确认
只允许 jpg/jpeg/png/webp
```

---

## 20. 最终结论

本方案正式采用以下原则：

```text
ctool_tavily_text_search_request 是默认主力搜索工具。
普通文本搜索默认放行，但必须记录日志。
搜索结果必须写入 Markdown 缓存文件。
工具只把缓存文件路径和简短摘要返回给 Codex。
Codex 通过普通 CTool 只读缓存文件。
每一次搜索、抽取、放大都生成新文件。
图片搜索独立成 ctool_tavily_image_search_request，默认关闭，使用时必须二次确认。
下载内容、打开网站、执行浏览器，一律 Red。
```