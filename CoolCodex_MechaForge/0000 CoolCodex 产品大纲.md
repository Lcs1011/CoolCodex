

原版OpenAI 官方CodexCLI开源仓库
https://github.com/openai/codex

我自己改良版的CoolCodex仓库
https://github.com/Lcs1011/CoolCodex

我的CoolCodex 位于本地的文件夹路径
```
C:\Arsenal\CoolAI\CoolCodex
```

我的Launcher 位于本地的文件夹路径
```
C:\Arsenal\CoolAI\Launchers
```


"C:\Arsenal\CoolAI\CoolCodex\CoolCodex_MechaForge\CoolBot\00 CoolOperation.md" 你看一下这个协议，之后的话，我会给你。C
oolOperation 指令 你根据这个协议和指令语法，然后帮我执行，可以吗？
我会在C:\Arsenal\CoolAI\CoolCodex\CoolCodex_MechaForge\Task.md 给你指定相关内容 然后你有什么执行中的困难输出到 C:\Arsena
l\CoolAI\CoolCodex\CoolCodex_MechaForge\TaskLog.md 能明白吗？

```
执行新的Task 文件中的任务, 文件已更新！ 执行任务前，先清空"C:\Arsenal\CoolAI\CoolCodex\CoolCodex_MechaForge\TaskLog.md"在"C:\Arsenal\CoolAI\CoolCodex\CoolCodex_MechaForge\Task.md" 中，请傻瓜执行。为彻底杜绝死循环与Token浪费 按照"C:\Arsenal\CoolAI\CoolCodex\CoolCodex_MechaForge\CoolBot\00 CoolOperation.md" 执行其中的CoolOperation 指令 如果执行中有任何困难不要思考！不要自己尝试解决 直接输出到C:\Arsenal\CoolAI\CoolCodex\CoolCodex_MechaForge\TaskLog.md
```


```
执行新的Task 文件中的任务, 文件已更新！ 执行任务前，先清空"C:\Arsenal\CoolAI\CoolCodex\CoolCodex_MechaForge\TaskLog.md"。任务更新在"C:\Arsenal\CoolAI\CoolCodex\CoolCodex_MechaForge\Task.md" 中，请傻瓜执行。为彻底杜绝死循环与Token浪费，强制开启【精准模式】：绝对禁止全量读取文件！每个文件的修改，**最多只允许调用 1 次 workspace_search_text 和 1 次 read_file（前后20行）！** 【绝对红线】：1. 拿到 1 次搜索和读取结果后，必须立刻执行 edit_file，或立刻放弃。2. 如果仅有的 1 次搜索没找到，或读取没对上，**绝对禁止换关键字重试！绝对禁止扩大读取范围！** 立刻放弃该文件，将失败直接记录到 TaskLog.md，并强制开启下一任务。3. 若发现任务代码已存在，立刻跳过。 单文件修改完毕或放弃后，必须主动声明清空历史上下文，严禁全文复读。
```




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
/
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


