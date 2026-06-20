# CoolBot 开发摘要

本文档记录 CoolBot 当前开发状态、设计方向和后续规划。

---

## 1. 项目定位

CoolBot 是本地机械修改器。

目标：

1. 执行 CoolOperation。
2. 避免 AI 在本地自由搜索、自由判断、自由乱改。
3. 通过明确语法执行文件修改、命令运行、报告输出、Git 提交推送。
4. 未来作为 CoolHeroTarven 的本地执行核心。

---

## 2. 当前核心链路

当前已经跑通：

```text
ChatGPT 生成 CoolOperation
↓
用户复制到剪贴板
↓
cbot c 执行
↓
CBot 预检
↓
CBot 修改文件 / 执行命令
↓
输出 Summary
↓
EndAction git commit / push
```

CoolHeroTarven 也已能通过网页 / 手机端调用 CBot。

---

## 3. 二段人格法

当前生成 CBot 指令时采用二段人格法：

```text
第一段：爆炸头天才程序员
第二段：犀利眼严格审计员
只执行第二段最终版
```

爆炸头负责完整实现，犀利眼负责审计语法、锚点、Preflight、CreateFile 风险、仓库污染风险，并重写最终版。

---

## 4. 已完成能力

### 4.1 CoolOperation 执行

已支持：

1. 修改文件。
2. 创建文件。
3. RunCmd。
4. EndAction。
5. 报告输出。
6. Preflight。

### 4.2 cbot back

已实现并测试：

1. Dirty Workspace Reset。
2. Commit Step Back。
3. backup branch。
4. stash 备份。
5. `--local` 跳过远端推送。
6. 人类可读输出。

### 4.3 RunCmd 输出

已实现：

1. 显示 WorkDir。
2. 显示 Command。
3. 显示 STDOUT。
4. 显示 STDERR。
5. 显示 ExitCode。
6. 显示 Succeed / Failed。
7. 保留 cargo warning/error 颜色。

---

## 5. 当前正在设计：显示协议

显示协议目标：

1. 人类能快速扫到标题。
2. AI 能快速锚定结构。
3. 输出越来越长时仍可读。
4. stdout/stderr 保留原始颜色和内容。
5. Git 推送结果必须清楚。

当前倾向：

大分类使用：

```text
⭐⭐⭐⭐⭐💰💰💰💰💰
```

语法标题使用：

```text
$$
$$$$
$$$$$$
```

示例：

```text
⭐⭐⭐⭐⭐💰💰💰💰💰
$$ CBot Operation

💰💰💰💰
$$$$ RunCmd

💰💰
$$$$$$ STDOUT
```

---

## 6. 当前已知问题

### 6.1 纯报告自动提交推送

某些测试指令只改变 `.cool/CoolOperationReport.md`，但 EndAction 仍然提交并推送。  
后续需要让 EndAction 区分业务文件和报告文件。

### 6.2 Git 中文原始输出乱码

RunCmd 捕获 git commit 输出时，中文 commit message 可能乱码。  
后续应改为 CBot 自己输出 Git Sync 摘要，而不是直接依赖 git 中文原始行。

### 6.3 .cool 与业务仓库混合

`.cool/CoolOperationReport.md` 经常使仓库 dirty。  
后续应考虑 Debug 仓库、.gitignore 或 EndAction 排除报告文件。

---

## 7. 近期优先级

```text
1. 定稿 CBot Display Protocol v1
2. 重构 Summary / EndAction / Git Sync 输出
3. 修复纯报告自动提交推送污染
4. 修复 git 中文输出乱码
5. 规范 .cool / Debug 仓库策略
6. 添加 CI：cargo check
7. 清理 run_cmd_timeout_seconds warning
```

---

## 8. CI 规划

建议加入：

```text
.github/workflows/rust-check.yml
```

初版 CI：

```yaml
name: Rust Check

on:
  push:
  pull_request:

permissions:
  contents: read

jobs:
  check:
    runs-on: windows-latest

    steps:
      - uses: actions/checkout@v4
      - name: Check
        run: cargo check
```

---

## 9. Debug 仓库规划

长期建议将调试输出从业务仓库分离：

```text
C:\Arsenal\CoolAI\CoolBot
  正式源码仓库

C:\Arsenal\CoolAI\CoolBotDebug
  操作记录、报告、日志、RunCmd 输出
```

业务仓库只保留源码、文档、配置和 CI。

---

## 10. 近期关键提交

```text
0010 添加CBotBack命令
0013 清理CBotBack插入格式
0014 优化CBotBack人类可读输出
0017 修复RunCmd完整输出显示
0019 修复RunCmd保留Cargo颜色
0021 重构RunCmd人类可读显示
```

`0015`、`0018`、`0020` 等纯测试提交可能是报告污染，需要后续清理。
