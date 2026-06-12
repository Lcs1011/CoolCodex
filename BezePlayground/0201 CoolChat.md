
通知在线
有一个注册信息

CoolChat.toml
ID 名字
显示名字

本地聊天记录
本地压缩
打开网页会看到
在线cli列表

本地聊天备份



ID 
时间
类型

根据时间找上下文工具
rg可能已经具备

结尾阅读工具 
可能也已经具备

上下文压缩机制

AIPlayerground
CoolChat
ChatContext
Charcter命名的 chat

bat启动

给CLI增加指令
注册到 CoolChat
带上自己名字



有约定对话的发送块的约定分割符。



几乎不会因为路由器增加风险
但是有链接风险

# CoolCode Web 会话镜像基础规划

## 1. 目标

把 Codex / CoolCode CLI 当前会话的可见内容同步写入本地文件，再由一个轻量 Web Viewer 渲染出来。

核心目标：

```text
CLI 继续负责模型调用、CTool 调度、安全策略和执行。
Web 只负责展示当前会话进度。
```

第一版不做网页输入，不做网页命令执行，不做网页确认按钮。

第一版只做：

```text
只读
实时刷新
局域网可查看
Markdown 渲染
基础安全保护
```

---

## 2. 产品定位

这不是把 Codex TUI 硬改成网页应用。

更合理的定位是：

```text
CoolCode CLI = 后端执行器
CoolCode Web Viewer = 会话进度看板
```

CLI 持续写会话日志：

```text
SessionRoot\.cool\logs\session_live.md
SessionRoot\.cool\logs\session_events.jsonl
```

Web Viewer 读取这些日志并渲染。

---

## 3. 第一阶段：只读 Web Viewer

### 3.1 功能

第一阶段只做只读展示。

支持展示：

```text
用户消息
Assistant 可见回复
Assistant 可见进度说明
工具调用摘要
工具结果摘要
命令申请摘要
Tavily 搜索申请摘要
测试结果摘要
错误信息
SafeMode / PermissionProfile / CToolScope / CoolWorkspace 状态
```

不支持：

```text
网页输入消息
网页执行命令
网页确认 Yellow / Red 请求
网页编辑文件
网页读取任意文件
```

---

## 4. 日志文件设计

### 4.1 Markdown 日志

路径：

```text
SessionRoot\.cool\logs\session_live.md
```

用途：

```text
给人直接阅读
方便用 Markdown 渲染
适合第一版 Web Viewer
```

示例：

```markdown
# CoolCode Session

## Status

- SafeMode: ON
- PermissionProfile: CoolReadWrite
- CToolScope: CoolWorkspace
- CoolWorkspace: C:\CodexLab\codex

---

## User

把 Tavily 工具彻底做完。

## Assistant Progress

我会先检查 Tavily 工具实现和 Scope 逻辑。

## Tool Call: ctool_read_file

Input:
```json
{
  "path": "codex-rs/utils/ctool/src/tools/special/ctool_tavily_search_request.rs"
}
```

Result:
```text
Read 42134 bytes.
```

## Assistant

已完成 Tavily 工具收口：...
```

### 4.2 JSONL 事件日志

路径：

```text
SessionRoot\.cool\logs\session_events.jsonl
```

用途：

```text
给 Web UI 做结构化渲染
后续支持折叠工具调用、artifact 卡片、状态栏
```

示例：

```jsonl
{"type":"status","safeMode":"ON","permissionProfile":"CoolReadWrite","scope":"CoolWorkspace","workspace":"C:\\CodexLab\\codex"}
{"type":"user","text":"把 Tavily 工具彻底做完。"}
{"type":"assistant_progress","text":"我会先检查 Tavily 工具实现和 Scope 逻辑。"}
{"type":"tool_call","name":"ctool_read_file","input":{"path":"codex-rs/utils/ctool/src/..."}}
{"type":"tool_result","name":"ctool_read_file","summary":"Read 42134 bytes."}
{"type":"assistant","text":"已完成 Tavily 工具收口：..."}
```

---

## 5. 记录内容边界

### 5.1 可以记录

```text
TUI/CLI 已经显示给用户看的内容
用户输入
Assistant 最终回复
Assistant 可见进度说明
工具调用名称
工具调用参数摘要
工具返回摘要
缓存文件路径
错误信息
测试结果摘要
```

### 5.2 不记录

```text
隐藏 system prompt
隐藏 developer prompt
不可见 chain-of-thought
API key
Tavily token
ChatGPT token
完整认证信息
.cool-system 内容
超长 stdout / stderr 全文
超长 Tavily 正文
```

### 5.3 大内容处理

大内容不直接写入主会话 Markdown。

应该写入 artifact 文件，再在会话中记录路径。

例如：

```text
.cool\cache\command_request\YYYY-MM-DD\00000_yellow_cargo_check.md
.cool\cache\web_search\YYYY-MM-DD\00000_search_rust_cargo.md
```

会话日志只写：

```text
Output: <path>
Summary: <short summary>
Suggested next step: <next step>
```

---

## 6. Web Viewer 设计

### 6.1 默认只监听本机

默认：

```text
127.0.0.1:8765
```

访问：

```text
http://127.0.0.1:8765
```

### 6.2 局域网模式

需要用户显式开启：

```text
cool-viewer --host 0.0.0.0 --port 8765 --token 123456
```

局域网访问：

```text
http://192.168.x.x:8765/?token=123456
```

### 6.3 页面组成

第一版页面：

```text
顶部状态栏
主会话 Markdown 渲染区
自动刷新开关
手动刷新按钮
artifact 链接列表
```

顶部状态栏显示：

```text
SafeMode
PermissionProfile
CToolScope
CoolWorkspace
SessionRoot
最后更新时间
```

---

## 7. 安全边界

第一版 Web Viewer 必须是只读。

禁止：

```text
执行命令
调用 CTool
读取任意路径
修改文件
删除文件
移动文件
写 .cool 配置
访问 .cool-system
显示 token / key / secret
```

允许读取：

```text
.cool\logs\session_live.md
.cool\logs\session_events.jsonl
.cool\cache\web_search\...
.cool\cache\command_request\...
```

但 artifact 读取也要限制在允许目录内。

---

## 8. 局域网安全策略

局域网模式必须显式开启。

建议规则：

```text
默认只绑定 127.0.0.1
绑定 0.0.0.0 必须带 token
token 只用于 viewer 访问
token 不写入 session_live.md
不提供目录遍历
不提供任意文件下载
不提供命令执行 API
```

第一版可以使用简单访问码。

后续再考虑更完整的认证。

---

## 9. 实现阶段

### 阶段一：CLI 写 Markdown 日志

目标：

```text
每条可见消息同步 append 到 session_live.md
```

最小事件：

```text
User
Assistant
AssistantProgress
ToolCall
ToolResult
Error
Status
```

### 阶段二：CLI 写 JSONL 事件

目标：

```text
每条事件同时写入 session_events.jsonl
```

为后续 Web UI 结构化渲染做准备。

### 阶段三：只读 Web Viewer

目标：

```text
读取 session_live.md
渲染 Markdown
定时刷新
默认监听 127.0.0.1
```

### 阶段四：局域网查看

目标：

```text
支持 --host 0.0.0.0
支持 --token
手机 / 平板 / 其他电脑可查看
```

### 阶段五：artifact 列表

目标：

```text
展示 web_search 缓存
展示 command_request 缓存
点击查看 Markdown artifact
```

### 阶段六：网页输入

暂不做。

后续如果需要，再引入：

```text
HTTP / WebSocket 输入
确认按钮
命令请求确认
Tavily 请求确认
```

---

## 10. 推荐第一版原则

第一版只做只读镜像。

不要一开始就做完整 Web CLI。

原因：

```text
只读镜像风险低
不影响现有 CLI 工作流
便于调试日志格式
局域网看进度需求可以立刻满足
后续可以自然升级为 Web UI
```

---

## 11. 与 CTool 的关系

CTool 仍然是执行能力边界。

Web Viewer 不直接调用 CTool。

Web Viewer 只看 CTool 产生的日志和 artifact。

未来如果支持网页输入，也应该走：

```text
Web UI
=> CoolCode 后端
=> SafeMode / PermissionProfile
=> CTool registry
=> CToolScope / CommandPolicy
```

不能让 Web UI 绕过 CToolScope。

---

## 12. 最小可用形态

最小可用形态：

```text
Codex/CoolCode CLI 正常运行
同时写 session_live.md
cool-viewer 渲染 session_live.md
浏览器自动刷新
```

这已经可以实现：

```text
同一局域网内查看当前对话和工作进度
手机查看编译/测试进度
其他终端查看工具调用和结果
不干扰 CLI 主流程
```
