
产品诉求


在使用我们纯净的 修改后的 Codex
不安装任何Skill 任何 Plugin的情况下 

## 网络方面
假设我们使用的是 纯净的 改良后的 CodexCLI，
不安装任何 skill 和插件。 
假设账号登录和大模型推理 API 的使用出口和入口是绝对安全的。 
基于以上假设，
SafeMode 开启时 
可以认为 CLI 网络入口和出口是绝对安全。

## 防止创建有威胁文件
必须由用户主动发起才能创建的能力，不算  我们可以直接禁用用户权限来解决。
必须连网，被远程激活的也不算。 我们会掐死所有的网络链接 只放行必要的

我们只排查，当我们运行了这个程序，它自己就能莫名其妙主动创建文件的地方。
如果创建的文件是绝对安全的文本文件不算。 
如果是基于开源代码创建，本身绝对安全，又不具备联网或者再次创建文件的能力，也不算。


除以上之外  SafeMode 开启时 不能创建任何文件

## 防止进程残留

保证退出之后没有 残留进程
可以监视电脑

## 防止扫描

没有上传，都掐死了。 也不能创建文件 也没有退出残留 对于扫描根本就没有必要焦虑

保证 SafeMode开启时，运行CLI的时候 没有任何 CTool以外的工具 可以自己启动的 进程有权限扫描 电脑上的文件
扫描指定文件不会扫描我们创建的文件不算（假设我们没有创建Skill或Plugin）
通过非 Cool PermissionProfile 才能启用的也不算。Safemode模式会强制不允许切换 非Cool PermissionProfile

## 一键清空所有记录的 bat

# SafeMode

safe-mode 设计原则 Above All 
启动main前设置 ，staitc 只读变量
//Rust 进入main之前 没有 C++自动跑一堆全局对象构造函数
main 进入后，
第一件事解析 --safe-mode  没有参数默认 ON
初始化 safe-mode

# 网络的解决方式

pub enum NetworkPurpose {  
    ChatGPTAuth, // ChatGPT 账号相关：登录 / token 刷新 / 账号验证
    ModelApi,    // 模型推理相关：流式回复 / Responses API
    Other,       // 其他全部网络
}



网络出入口 申请函数全部用 携带NetworkPurpose的自创函数包装起来
整个程序不存在任何对网络的裸访问

SafeMode开启时，所有的访问都需要 审核 NetworkPurpose

扫描所有出口裸函数 和 入口裸函数 全部替换成我们包装函数



# 默认启动
我们要保证
SafeModeOn
默认
PermissionProfile 为 CoolReadWrite
CToolScope 为 Workspace


 点亮前只做两个实测： 
1/  启动退出后检查有没有 codex/app-server/exec-server 残留进程。
2/  启动后确认前三行显示：SafeMode ON / PermissionProfile CoolReadWrite / CToolScope Workspace。


# 常用提示词

##
傻瓜指令 意思就是不懂代码，照着做就能操作的指令。 描述的非常具体，直接说哪一个文件在哪个位置。 一种机械替换，照着做就行的指令
## 让CLI以最少Token执行傻瓜指令

```
执行新的Task 文件已更新！ 执行任务前，先清空"C:\CodexLab\TempLog.md"。任务更新在 "C:\CodexLab\Task.md" 中，请傻瓜执行。为彻底杜绝死循环与Token浪费，强制开启【精准模式】：绝对禁止全量读取文件！每个文件的修改，**最多只允许调用 1 次 workspace_search_text 和 1 次 read_file（前后20行）！** 【绝对红线】：1. 拿到 1 次搜索和读取结果后，必须立刻执行 edit_file，或立刻放弃。2. 如果仅有的 1 次搜索没找到，或读取没对上，**绝对禁止换关键字重试！绝对禁止扩大读取范围！** 立刻放弃该文件，将失败直接记录到 TempLog.md，并强制开启下一任务。3. 若发现任务代码已存在，立刻跳过。 单文件修改完毕或放弃后，必须主动声明清空历史上下文，严禁全文复读。
```



网络出入口 等大量需要授权的内容 统一读取这个全局 SafeMode



# scratch

你确认当前的codex  当位于 CoolReadWrite时
能清楚的知道自己当前可以使用的工具是什么吗？ 
已经知道这些工具怎么用了吗？




假设 账号登陆

我们通过修改来做到以下内容。

我能接受的是，API 或者账号为了运算所必然产生的上传。
除此之外，我希望 CLI不要有任何上传。 

运行CLI时 ，
在默认权限下，严格限制能力。
保证读写范围（尤其是读的范围，因为读很多时候是隐形的） 被牢牢限制在工作区以内，绝对不会绝对不会读写工作区以外的文件。 

我的策略是 进入 CLI 的默认权限时，它只能用我给的4个工具。 代码搜索工具 文件读取功 文件写工具 和批注工具 除此之外，一切工具都不给。我认为这4个工具足够了。

关掉 CLI之后，保证没有任何进程 ，绝对不能有任何可能在后台偷窥电脑文件的进程。



开辟 SafeMode 
Safe mode 是 CLI中的全局最高权限。 


开启SafeMode 必须保证

账号登陆必然产生的确认上传
以及 API推理必要产生的上传之外 无任何上传 遥测 数据传输

关掉 CLI之后，保证没有任何进程
绝对不能有任何可能在后台偷窥电脑文件的进程。

必须保证 CLI全程 没有偷偷下载残留的内容


```
模型列表刷新 / model manager
版本检查 / update check
反馈上报 / feedback
OTel / tracing
analytics client
auth env telemetry
prewarm / warmup request
崩溃报告
```



现在我们正在做的 `skills / plugins / connectors` 正好能堵住最大风险源。

尤其是 plugins，因为它已经有：

```
远程下载 bundle解压安装写入 plugin store
```

所以这一步优先级很高。

你现在改 `skills / plugins / connectors` 的时候，目标不是只让模型看不到它们，而是要做到：

```
SafeMode 下这些系统根本不启动：- 不同步- 不下载- 不解压- 不安装- 不扫描外部插件- 不启动 watcher- 不注入 prompt
```



# 创建
官方仓库的主页  https://github.com/openai/codex
右上角的 **Fork** 按钮
克隆出自己的分支叫 Coolcode

将官方原本地址命名为 upstream
自己的独立分支设置为 origin是社区与欸的那个俗成的做法

```
git remote rename origin upstream
```



```
git remote add origin https://github.com/你的用户名/codex.git
```


```

git add .
git commit -m "1"

git push -u origin main
```


```
git diff > C:\CodexLab\Diff.md
```
# 1 

第一步 

创建一个 Mode 叫做 BaseMode 要保证这个 mode 进去之后，什么功能都没有。 除了可以登录 GPT 账号之外， 没有任何读写功能。

这一步做完了，绝对不能急着编译登录。 还要把摇测和关闭残留都给去掉了之后，才能登录。


我们约定一件事 你一定要牢记这件事。 任何时候你需要我 更新代码才能做出好的决策的时候。 你一定要提醒我更新代码，然后你再告诉我怎么修改。


第三刀：  
cli/main.rs 和 tui/app.rs 里 BaseMode 禁止后台、遥测、update、file_search、watcher。



AGENTS.md、project docs、history、memory
存在上下文注入
后台做一个一键清空功能即可
```
1. <CODEX_HOME>\AGENTS.override.md
2. <CODEX_HOME>\AGENTS.md
3. <Workspace>\**\AGENTS.override.md
4. <Workspace>\**\AGENTS.md
5. config.toml 里的 project_doc_fallback_filenames 对应文件
6. config.toml 里的 instructions
7. config.toml 里的 developer_instructions
8. config.toml 里的 model_instructions_file 指向文件
9. config.toml 里的 compact_prompt
10. <CODEX_HOME>\history.jsonl
11. <CODEX_HOME 或 CODEX_SQLITE_HOME>\logs_2.sqlite
12. <CODEX_HOME 或 CODEX_SQLITE_HOME>\goals_1.sqlite
13. <CODEX_HOME 或 CODEX_SQLITE_HOME>\memories_1.sqlite
14. <CODEX_HOME 或 CODEX_SQLITE_HOME>\state_5.sqlite
15. config.toml 里的 memories 配置
```


第 1 层：SafeMode  
一旦开启，会强制关闭所有的 Codex 原生工作。
只有Cool工具才好用。


第 2 层：CoolScope  
Cool scope 并非是来限制PermissionProfile的权限
而是直接限制 CTool的权限来达到绝对控制
通过前置Cool工具的视野 来达到前置 API 视野的目的。

None
Workspace
SelectedOnly
TheEyeofProvidence//先不做

第 3 层：PermissionProfile
加入Cool开头的模式。 他们只能被允许使用Cool工具。

CoolReadWrite

然后我们再做 CTool 的时候，我们只考虑 workspace 这一种情况。 其他三种我们先暂时不考虑。
然后我们第一步是把这个枚举给创建出来。 然后保证在启动 codex 的时候，要在第二行显示这个枚举。
我们要让显示第一行是 sleep mode，第二行是 cool scope。 第三行是 permission profile
我们第一个任务先完成这个。

CToolRead
CWriteTool
CoolRG搜索代码行
AnnoteMD工具
联网搜索工具
Telegram 发送工具


cool文档系统
就是记录所有库和工具的使用。






读取文件工具
edit工具
基于rg搜索代码工具
MD批注工具
联网工具
Telegram 发送工具
创建文件 工具 //限制文件类型
移动文件工具 
删除文件工具

NetworkPurpose
Auth 
ModelApi 
Blocked


我们统计所有的网络上传和网络入。 然后，我们给每一个网络请求都打上那么我们是不是就没必要再做什么之前的呢？一个标签儿。 之后，我们看着标签决定放不放心。

第二步：SafeMode 下审计并关闭所有非 API 必要联网。  
包括 telemetry、feedback、otel、analytics、update check、prewarm。





不创建有害文件
退出无残留进程
网络出入口全部守住

命令行 启动前设置。不设置则自动为on
codex --safe-mode on  
codex --safe-mode off


将所有的裸网络请求 例如 client.get(url).send().await?;
全部包装成 我们的网络函数 例如：

safe_network::send(  
config.safe_mode,  
NetworkPurpose::Other,  
client.get(url),  
).await?;

这样我们就可以把关是否放行了

先切断发送 再切断接受





先切断

```
skills
plugins
connectors

telemetry
analytics
remote
daemon
update check
```



在制作过程当中  Cargo 下载一直极其困难 
但是发现它低于某个阈值的时候，被认为是 time out。 我们让这个寓意值变得极其夸张来解决。
```
error: failed to get `aws-runtime` as a dependency of package `aws-config v1.8.12`
    ... which satisfies dependency `aws-config = "^1"` (locked to 1.8.12) of package `codex-aws-auth v0.0.0 (C:\CodexLab\codex\codex-rs\aws-auth)`

Caused by:
  download of aw/s-/aws-runtime failed

Caused by:
  failed to download from `https://index.crates.io/aw/s-/aws-runtime`

Caused by:
  [28] Timeout was reached (Operation too slow. Less than 10 bytes/sec transferred the last 30 seconds)

C:\CodexLab\codex\codex-rs>set CARGO_HTTP_LOW_SPEED_LIMIT=0

C:\CodexLab\codex\codex-rs>set CARGO_HTTP_TIMEOUT=86400

C:\CodexLab\codex\codex-rs>set CARGO_NET_RETRY=10
```


读取工具
1. list_directory 看项目结构  
2. rg_search 搜关键词 / 类型 / 函数 / 报错  
3. read_code_range 读取命中位置附近代码  
4. read_file 读取完整小文件或配置文件

edit_replace      单点精确替换
edit_insert       单点插入
edit_batch        多个 replace / insert 打包执行
preview_diff      看结果

创建




第一步
是这样子的 我们先做这个 CoolScope 枚举

包含
None
Workspace
SelectedOnly
TheEyeofProvidence

然后我们再做 CTool 的时候，我们只考虑 workspace 这一种情况。 其他三种我们先暂时不考虑。
然后我们第一步是把这个枚举给创建出来。 然后保证在启动 codex 的时候，要在第二行显示这个枚举。
我们要让显示第一行是 sleep mode，第二行是 cool scope。 第三行是 permission profile
我们第一个任务先完成这个。


第二步 
我们先来聊聊下一步的任务，我们先聊聊思路。 下一个任务是我们给 permission profile 添加 CoolReadWrite 然后要记得这个模式，先不给任何工具，它不能调用任何工具。 并且让CoolReadWrite  作为默认的 permission profile 。

然后再接下来就是我们一个一个工具送给他。