# CoolOperation 语法手册

本文档记录当前 CBot / CoolOperation 的机械指令语法。写法参考“新标准”的清晰分层方式，但内容以当前正在使用的美元语法为准。

尖括号包围的内容，例如 `<OperationID>`，表示注解。请勿把尖括号和其中内容当作实际指令。

---

## 1. 基本原则

CoolOperation 是给 CBot 执行的机械修改指令。

核心原则：

1. 指令必须明确，不让执行器自由搜索、猜测、判断。
2. 文件路径推荐相对 Workspace。
3. 修改已有文件使用 `ModifyFile`。
4. 创建新文件使用 `CreateFile`。
5. `CreateFile` 不覆盖已有文件。
6. 执行前会进行 Preflight 预检。
7. `End` 必须存在，并且必须是最后一个指令。
8. `$$$$ Content`、`$$$$ Old`、`$$$$ New`、`$$$$ Command` 必须单独一行。

---

## 2. 外层围栏

CoolOperation 使用十个 `$` 作为外层围栏。

```text
$$$$$$$$$$ CoolOperation
$ OperationID <本次操作名称>
Workspace = <工作区绝对路径>

<具体指令>

$$ End
$$$$$$$$$$
```

示例：

```text
$$$$$$$$$$ CoolOperation
$ OperationID 0021 重构RunCmd人类可读显示
Workspace = C:\Arsenal\CoolAI\CoolBot

$$ RunCmd
$$$ WorkDir .
$$$ LogMode Verbose
$$$$ Command
cargo check
$$$$ EndCommand

$$ End
$$$$$$$$$$
```

---

## 3. 语法层级

| 层级       | 语法           | 含义                                      |
| -------- | ------------ | --------------------------------------- |
| 外层围栏     | `$$$$$$$$$$` | 整个 CoolOperation 的开始 / 结束               |
| 元信息      | `$`          | OperationID 等元信息                        |
| 一级指令     | `$$`         | ModifyFile / CreateFile / RunCmd / End  |
| 子指令 / 参数 | `$$$`        | Path / WorkDir / LogMode / ReplaceExact |
| 内容块      | `$$$$`       | Old / New / Content / Command           |

---

## 4. OperationID

`OperationID` 表示本次操作名称。当前配置下，它通常也会作为 git commit message。

```text
$ OperationID <本次操作名称>
```

示例：

```text
$ OperationID 0021 重构RunCmd人类可读显示
```

---

## 5. Workspace

`Workspace` 表示本次 CoolOperation 所属工作区根目录。Workspace 必须是绝对路径。

```text
Workspace = C:\Arsenal\CoolAI\CoolBot
```

相对路径均从 Workspace 解析。

例如：

```text
src\main.rs
```

表示：

```text
C:\Arsenal\CoolAI\CoolBot\src\main.rs
```

---

## 6. ModifyFile

`ModifyFile` 用来修改已有文件。

```text
$$ ModifyFile
$$$ Path <文件路径>

<修改子命令>
```

示例：

```text
$$ ModifyFile
$$$ Path src/main.rs
```

---
Preflight 预检验
## 7. ReplaceExact

`ReplaceExact` 用于精确替换文本块。

```text
$$$ ReplaceExact
expect: 1

$$$$ Old
<旧内容>
$$$$ New
<新内容>
$$$$ EndReplaceExact
```

规则：

1. `expect: 1` 表示旧内容必须精确命中一次。
2. 命中 0 次或多次，Preflight 失败。
3. 适合稳定替换函数、代码块、配置片段。
4. 不需要计算行号。
5. 旧内容必须来自执行前的原始文件。

---

## 8. ReplaceLineRange

`ReplaceLineRange` 用于按行号替换。

```text
$$$ ReplaceLineRange
start: <开始行号>
end: <结束行号>

$$$$ Content
<新内容>
$$$$ EndReplaceLineRange
```

规则：

1. 行号以执行前文件为准。
2. 适合用户已经明确提供精确行号的情况。
3. 大文件中若行号可能漂移，优先用 `ReplaceExact`。

---

## 9. InsertBeforeExact

在锚点文本前插入内容。

```text
$$$ InsertBeforeExact
expect: 1

$$$$ Anchor
<锚点内容>
$$$$ Content
<插入内容>
$$$$ EndInsertBeforeExact
```

规则：

1. Anchor 必须精确命中。
2. Content 必须单独起行。
3. Anchor 必须来自执行前的原始文件。

---

## 10. InsertAfterExact

在锚点文本后插入内容。

```text
$$$ InsertAfterExact
expect: 1

$$$$ Anchor
<锚点内容>
$$$$ Content
<插入内容>
$$$$ EndInsertAfterExact
```

重要规则：

Preflight 会在执行前统一检查所有锚点。不能用“本次操作前面刚替换出来的新内容”作为后续锚点。

错误模式：

```text
先 ReplaceExact 生成新函数
再 InsertAfterExact 锚定这个新函数
```

这会在 Preflight 阶段失败，因为新函数在原始文件中不存在。

---

## 11. CreateFile

`CreateFile` 用于创建新文件。

```text
$$ CreateFile
$$$ Path <文件路径>
$$$$ Content
<文件内容>
$$$$ EndContent
```

规则：

1. 目标文件已存在时失败。
2. 已有文件必须用 `ModifyFile`。
3. 这是为了避免误覆盖重要文件。

---

## 12. RunCmd

`RunCmd` 用于在指定目录执行命令。

```text
$$ RunCmd
$$$ WorkDir <工作目录>
$$$ LogMode <Verbose 或 ErrorOnly>
$$$$ Command
<命令>
$$$$ EndCommand
```

示例：

```text
$$ RunCmd
$$$ WorkDir .
$$$ LogMode Verbose
$$$$ Command
cargo fmt
cargo check
git diff --check
$$$$ EndCommand
```

### WorkDir

`WorkDir` 可以是相对路径，也可以是绝对路径。相对路径从 Workspace 解析。

推荐：

```text
$$$ WorkDir .
```

### LogMode

支持：

```text
Verbose
ErrorOnly
```

当前推荐开发时使用 `Verbose`，因为它会显示 stdout、stderr、exit code 和执行状态。

---

## 13. End

`End` 必须存在，并且必须是最后一个指令。

```text
$$ End
```

`End` 后不应再有任何业务指令。

---

## 14. EndAction

CBot 配置中存在 `end_action`。

常见值：

```text
none
git-commit
git-commit-push
```

`git-commit-push` 表示：

```text
git add -A
git commit -m "<OperationID>"
git push
```

注意：当前版本仍存在“纯报告也可能被提交推送”的问题，后续需要修复。

---

## 15. Preflight 规则

执行前，CBot 会先检查全部指令是否可执行。

重点规则：

1. `ReplaceExact expect: 1` 必须命中一次。
2. `InsertBeforeExact expect: 1` 必须命中一次。
3. `InsertAfterExact expect: 1` 必须命中一次。
4. `CreateFile` 目标不能已存在。
5. 所有锚点都基于执行前原始文件。
6. Preflight 失败时，不应执行业务修改。

---

## 16. 关键纪律

1. 不要使用 CBot 不支持的子命令，例如旧错误中的 `$$$ InsertBefore`。
2. 已有文件不要用 `CreateFile`。
3. 大改优先 `ReplaceExact`。
4. 不要用本轮新生成内容作为本轮后续锚点。
5. `$$$$ Content` 必须单独一行。
6. `RunCmd` 建议总是写 `WorkDir`。
7. 每次执行后查看 Summary 和 Git 结果。
8. 失败报告自动提交问题尚未修复时，要留意是否污染仓库。

---

## 17. Display Protocol Draft

后续 CBot 输出显示计划采用钱袋和星星作为视觉锚点。

大分类使用：

```text
⭐⭐⭐⭐⭐💰💰💰💰💰
```

语法标题仍使用美元层级，方便 AI 和机器定位：

```text
$$      一级标题 / 大阶段
$$$$    二级标题 / 指令阶段
$$$$$$  三级标题 / 输出细节
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

最终格式仍待实现。
