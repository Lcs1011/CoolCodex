# CoolBot 使用手册

本文档记录 CoolBot / CBot 的日常使用方式和注意事项。

---

## 1. 常用路径

CoolBot 仓库：

```text
C:\Arsenal\CoolAI\CoolBot
```

启动器：

```text
C:\Arsenal\CoolAI\Launchers\cbot.bat
```

常用命令目录：

```bat
cd /d C:\Arsenal\CoolAI\CoolBot
```

---

## 2. 常用启动方式

执行剪贴板 CoolOperation：

```bat
cbot c
```

直接输入 cbot 后回车确认：

```bat
cbot
```

执行默认指令文件：

```bat
cbot d
```

---

## 3. 基本执行流程

推荐流程：

```bat
cd /d C:\Arsenal\CoolAI\CoolBot
git status --short
cbot c
```

执行后检查：

```bat
git status --short
```

如果需要检查代码：

```bat
cargo check
```

如果修改的是 CoolBot 自己源码，执行成功后还需要：

```bat
cargo build
```

原因：`cargo check` 只检查，不会更新正在使用的 exe。

---

## 4. CoolBot 修改自己时的规则

当 CoolOperation 修改 CoolBot 自己源码时：

1. CoolOperation 里可以跑 `cargo fmt`。
2. CoolOperation 里可以跑 `cargo check`。
3. CoolOperation 里通常不要跑 `cargo build`。
4. 执行结束后，手动运行 `cargo build`。

原因：`cargo build` 可能尝试覆盖当前正在运行的 `target\debug\coolbot.exe`。Windows 可能拒绝覆盖运行中的 exe。

---

## 5. cbot back

`cbot back` 用于回滚。

脏工作区模式：

```bat
target\debug\coolbot.exe back --local
```

会：

1. 创建 backup branch。
2. 将 dirty 文件保存到 git stash。
3. reset 回当前 HEAD。
4. 不推送远端。

提交回退模式：

```bat
target\debug\coolbot.exe back --local
```

如果当前工作区干净，会：

1. 创建 backup branch。
2. reset 到 HEAD~1。
3. 因为有 `--local`，不推送远端。

真实同步远端：

```bat
target\debug\coolbot.exe back
```

会尝试：

```bat
git push --force-with-lease
```

---

## 6. RunCmd 使用

RunCmd 应显示：

1. 工作目录。
2. 命令列表。
3. STDOUT。
4. STDERR。
5. ExitCode。
6. 成功 / 失败状态。

测试指令：

```text
$$$$$$$$$$ CoolOperation
$ OperationID 测试RunCmd
Workspace = C:\Arsenal\CoolAI\CoolBot

$$ RunCmd
$$$ WorkDir .
$$$ LogMode Verbose
$$$$ Command
echo Hello
cargo --version
cargo check
$$$$ EndCommand

$$ End
$$$$$$$$$$
```

---

## 7. 报告文件

当前默认报告路径：

```text
C:\Arsenal\CoolAI\CoolBot\.cool\CoolOperationReport.md
```

如果只是报告文件脏了：

```bat
git restore .cool/CoolOperationReport.md
```

---

## 8. Git 拉取问题

如果执行：

```bat
git pull --rebase
```

失败：

```text
error: cannot pull with rebase: You have unstaged changes.
```

先看：

```bat
git status --short
```

如果只是报告文件脏了，可以：

```bat
git restore .cool/CoolOperationReport.md
git pull --rebase
```

如果有代码文件脏了，不要乱 reset，先确认内容。

---

## 9. 常用检查命令

```bat
git status --short
git diff --stat
git log --oneline -5
cargo check
cargo build
```

---

## 10. 当前已知问题

1. 纯报告可能被 EndAction 自动提交推送。
2. Git 原始中文输出在 RunCmd 捕获后可能乱码。
3. `run_cmd_timeout_seconds` 字段暂时未使用，会有 warning。
4. `.cool` 输出与业务仓库混在一起，后续应考虑迁移到 Debug 仓库或从业务提交中排除。
