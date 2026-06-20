# CoolOperation

下文中出现 尖括号包围的内容 连同尖括号在内 都是表示注解。  
请勿将注解本身当作实际代码。

## 指令内容

CoolOperation 全部采用 TOML 语法。

CoolOperation 文件本身就是 TOML 文件，不再使用 Markdown 标题表示指令，不再使用大量反引号包裹内容。

默认指令文件名为：

```text
CoolOperation.toml
```

例如：

```toml
format = "CoolOperation"
operation_id = "0013 测试CoolAIMessenger"
workspace = 'C:\Arsenal\CoolAI\CoolBot'

[[operations]]
type = "RunCmd"
work_dir = "."
log_mode = "ErrorOnly"
command = '''
cargo check
'''

[[operations]]
type = "End"
```

## CoolOperation 指令语法

### 1. 指令标题

TOML 顶部字段 `operation_id` 表示指令ID。

`operation_id` 的内容也是完成指令后 push 到 github 之前 commit 的名字。

```toml
operation_id = "<起的名字>"
```

TOML 顶部字段 `workspace` 表示本次 CoolOperation 所属的工作区根目录。

Workspace 必须是绝对路径。

Windows 路径推荐使用 TOML 单引号字符串，因为单引号字符串不会把反斜杠当作转义符。

```toml
workspace = '<工作区绝对路径>'
```

完整顶部示范：

```toml
format = "CoolOperation"
operation_id = "0013 测试CoolAIMessenger"
workspace = 'C:\Arsenal\CoolAI\CoolBot'
```

其中：

```toml
format = "CoolOperation"
```

用于明确表示这是 CoolOperation TOML 文件。

### 2. 默认文件位置

CoolOperation 默认放在 Workspace 下方的 `.cool` 文件夹中。

默认指令文件：

```text
<Workspace>\.cool\CoolOperation.toml
```

默认报告文件：

```text
<Workspace>\.cool\CoolOperationReport.md
```

例如：

```text
C:\Arsenal\CoolAI\CoolBot\.cool\CoolOperation.toml
C:\Arsenal\CoolAI\CoolBot\.cool\CoolOperationReport.md
```

如果使用 cbot d，则默认读取：

```text
<default_workspace>\.cool\CoolOperation.toml
```

其中 default_workspace 来自 coolbot-config.toml。

### 3. 路径规则

CoolOperation 中涉及文件、文件夹、命令工作目录时，推荐使用相对路径。

相对路径从 Workspace 开始解析。

例如：

```toml
workspace = 'C:\Arsenal\CoolAI\CoolBot'
```

那么：

```toml
path = 'src\main.rs'
```

表示：

```text
C:\Arsenal\CoolAI\CoolBot\src\main.rs
```

如果路径写成绝对路径，则直接使用绝对路径。

例如：

```toml
path = 'C:\Arsenal\CoolAI\CoolBot\src\main.rs'
```

注意：

1. Workspace 必须是绝对路径。
    
2. work_dir 可以是相对路径，也可以是绝对路径。
    
3. 文件、文件夹路径推荐使用相对 Workspace 的路径。
    
4. 跨出 Workspace 的绝对路径应谨慎使用。
    
5. Windows 路径推荐使用 TOML 单引号字符串。
    

例如：

```toml
workspace = 'C:\Arsenal\CoolAI\CoolBot'
path = 'src\main.rs'
work_dir = 'codex-rs'
```

### 4. 具体指令类型

具体指令类型 用 TOML 数组表 `[[operations]]` 表示。

每一个 `[[operations]]` 表示一个具体指令。

具体指令包括以下类型：

|**中文名**|**指令名字**|注释|
|---|---|---|
|修改文件|`ModifyFile`||
|修改文件子命令：删除行|`DeleteLine`||
|修改文件子命令：插入内容|`InsertContent`||
|添加文件|`CreateFile`||
|删除文件|`RemoveFile`||
|清空文件|`ClearFile`|删除文件内的全部内容|
|添加文件夹|`CreateFolder`||
|删除文件夹|`RemoveFolder`||
|清空文件夹|`ClearFolder`|删除文件夹中的全部内容|
|运行CMD指令|`RunCmd`||
|延迟|`Delay`|上一个**指令完成后**等待。注意 是上一个命令 报告完成后 开始等待，并非上一个命令刚执行。专门用于应付莫名其妙的Bug|
|截图|`Screenshot`||
|结束指令|`End`||

### 以下是具体指令讲解

所有指令的执行结果都会记录在名为 CoolOperationReport.md 的 Markdown 文件中。

默认位置是：

```text
<Workspace>\.cool\CoolOperationReport.md
```

报告文件仍然使用 Markdown，方便人类阅读。

CoolOperation 指令本身使用 TOML。

报告示范：

```md
# CoolOperationReport

OperationID: 0013 测试CoolAIMessenger
Workspace: C:\Arsenal\CoolAI\CoolBot
Status: Succeed

## ModifyFile Path:src\main.rs
Succeed

### DeleteLine LineNum：5
Succeed

## Delay Duration：3
Succeed

## EndAction
Action: git-commit-push
Workspace: C:\Arsenal\CoolAI\CoolBot
CommitMessage: 0013 测试CoolAIMessenger
GitAdd: Succeed
GitCommit: Succeed
GitPush: Succeed
```

### 修改文件 ModifyFile

要求使用一个 `[[operations]]` 表示 ModifyFile。

`type` 写 `ModifyFile`。

`path` 写要修改的文件路径。

文件路径可以是相对路径，也可以是绝对路径。  
推荐使用相对 Workspace 的路径。

ModifyFile 的子命令用 `[[operations.commands]]` 表示。

例如：

```toml
[[operations]]
type = "ModifyFile"
path = 'src\main.rs'

[[operations.commands]]
type = "DeleteLine"
line = 5
```

也可以写绝对路径：

```toml
[[operations]]
type = "ModifyFile"
path = 'C:\Arsenal\CoolAI\CoolBot\src\main.rs'

[[operations.commands]]
type = "DeleteLine"
line = 5
```

### 接下来两节讲解修改文件子命令

一个 ModifyFile 命令可以包含很多子命令。

注意里面操作的行数，指向的都是修改前的行数，  
而非修改后的行数。所以考虑这些命令的时候，  
针对当前代码的位置即可，无需计算递归。

### 修改文件子命令： 删除行 DeleteLine

删除行使用 `[[operations.commands]]`。

`type` 写 `DeleteLine`。

删除单行时使用：

```toml
line = 5
```

子命令示范 ： 删除第5行

```toml
[[operations]]
type = "ModifyFile"
path = 'src\main.rs'

[[operations.commands]]
type = "DeleteLine"
line = 5
```

删除一段时使用：

```toml
start_line = 5
end_line = 10
```

子命令示范 ： 删除第5-10行

```toml
[[operations]]
type = "ModifyFile"
path = 'src\main.rs'

[[operations.commands]]
type = "DeleteLine"
start_line = 5
end_line = 10
```

### 修改文件子命令： 插入内容 InsertContent

插入内容使用 `[[operations.commands]]`。

`type` 写 `InsertContent`。

`after_line` 表示插入该行的紧下方。

`content` 写具体要插入的内容。

具体内容推荐使用 TOML 多行单引号字符串：

```toml
content = '''
具体内容
'''
```

例如 表示在第6行的紧下方插入：

```text
     ABC
DFG
```

TOML 写法：

```toml
[[operations]]
type = "ModifyFile"
path = 'src\main.rs'

[[operations.commands]]
type = "InsertContent"
after_line = 6
content = '''
     ABC
DFG
'''
```

复杂代码插入示范：

```toml
[[operations]]
type = "ModifyFile"
path = 'codex-rs\tui\src\lib.rs'

[[operations.commands]]
type = "InsertContent"
after_line = 1742
content = '''
    let permission_profile = config
        .permissions
        .active_permission_profile()
        .map(|profile| format!("{profile:?}"))
        .unwrap_or_else(|| format!("{:?}", config.permissions.permission_profile()));
    let ctool_scope_base = ctool::CToolScopeBase::default();

    config.startup_warnings.push(format!(
        "CoolStatus: SafeMode={}, CToolScopeBase={}, PermissionProfile={}",
        if config.safe_mode { "on" } else { "off" },
        ctool_scope_base,
        permission_profile
    ));

'''
```

### 注意修改文件子命令的使用纪律

#### 1/ 命令应当自上而下依次发布 不得错乱顺序

#### 2/ 不得互相交叉

删除接 删除 上面的末尾 必须小于 下面的开头  
删除接插入 上面的末尾 必须小于等于 下面的开头  
插入接删除 上面的末尾 必须小于 下面的开头  
插入没必要 接插入

TOML 中子命令的顺序就是 `[[operations.commands]]` 出现的顺序。

例如：

```toml
[[operations]]
type = "ModifyFile"
path = 'src\main.rs'

[[operations.commands]]
type = "DeleteLine"
start_line = 10
end_line = 20

[[operations.commands]]
type = "InsertContent"
after_line = 30
content = '''
新内容
'''
```

### 添加、删除、清空 文件、文件夹

分别以以下 `type` 表示：

CreateFile  
RemoveFile  
ClearFile  
CreateFolder  
RemoveFolder  
ClearFolder

路径写在 `path` 中。

路径可以是相对路径，也可以是绝对路径。  
推荐使用相对 Workspace 的路径。

注意：添加文件、文件夹时，路径直接将 添加的文件、文件夹名字包含在内。

以 添加文件夹 为例：

```toml
[[operations]]
type = "CreateFolder"
path = '.cool'
```

以 添加文件 为例：

```toml
[[operations]]
type = "CreateFile"
path = 'src\new_file.rs'
```

以 删除文件 为例：

```toml
[[operations]]
type = "RemoveFile"
path = 'src\old_file.rs'
```

以 清空文件 为例：

```toml
[[operations]]
type = "ClearFile"
path = 'src\main.rs'
```

以 删除文件夹 为例：

```toml
[[operations]]
type = "RemoveFolder"
path = 'old_folder'
```

以 清空文件夹 为例：

```toml
[[operations]]
type = "ClearFolder"
path = 'temp'
```

### 运行CMD指令 RunCmd

运行 CMD 指令使用 `[[operations]]`。

`type` 写 `RunCmd`。

`work_dir` 表示命令执行目录。  
work_dir 可以是相对路径，也可以是绝对路径。  
推荐使用相对 Workspace 的路径。

`log_mode` 表示执行结果的记录方式。  
记录方式分别是：

Verbose 全记录  
ErrorOnly 只记录错误

`command` 写要执行的命令，或一组命令。

命令推荐使用 TOML 多行单引号字符串：

```toml
command = '''
cargo check
'''
```

例如：

```toml
[[operations]]
type = "RunCmd"
work_dir = "."
log_mode = "ErrorOnly"
command = '''
cargo check
'''
```

再例如 执行命令组：

```toml
[[operations]]
type = "RunCmd"
work_dir = "."
log_mode = "ErrorOnly"
command = '''
cargo fmt
cargo check
'''
```

再例如 在子目录执行命令：

```toml
[[operations]]
type = "RunCmd"
work_dir = 'codex-rs'
log_mode = "ErrorOnly"
command = '''
cargo check
'''
```

RunCmd 的结果在记录到 CoolOperationReport.md 文件中时，  
会根据 LogMode 记录执行的结果。

例如：

```md
## RunCmd
WorkDir: .
LogMode: ErrorOnly
Status: Failed
ExitCode: 101

For more information about this error, try `rustc --explain E0061`.
error: could not compile `codex-cli` (bin "codex") due to 1 previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `codex-cli` (bin "codex" test) due to 1 previous error
warning: unused import: `codex_model_provider_info::ModelProviderInfo`
  --> tui\src\status\tests.rs:30:5
   |
30 | use codex_model_provider_info::ModelProviderInfo;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

## <Report中的其他指令记录..>
```

### 延迟 Delay

延迟使用 `[[operations]]`。

`type` 写 `Delay`。

`seconds` 表示延迟时间，以秒为单位。

例如 等待5秒：

```toml
[[operations]]
type = "Delay"
seconds = 5
```

### 截图 Screenshot

截图使用 `[[operations]]`。

`type` 写 `Screenshot`。

暂时执行全屏截图。

```toml
[[operations]]
type = "Screenshot"
```

### 结束指令 End

结束指令使用 `[[operations]]`。

`type` 写 `End`。

End 指令必须存在且必须是最后一个指令。

如下：

```toml
[[operations]]
type = "End"
```

coolbot-config.toml 中：

```toml
end_action = 'none'
```

则 End 只结束本次 CoolOperation，不额外执行 Git 操作。

coolbot-config.toml 中：

```toml
end_action = 'git-commit'
```

则结束时会自动执行：

```text
git add -A
git commit -m "<OperationID>"
```

coolbot-config.toml 中：

```toml
end_action = 'git-commit-push'
```

则结束时会自动执行：

```text
git add -A
git commit -m "<OperationID>"
git push
```

Git 命令执行位置是 Workspace。

EndAction 的结果必须记录在 CoolOperationReport.md 中。  
记录内容包括：

```md
## EndAction
Action: git-commit-push
Workspace: <Workspace>
CommitMessage: <OperationID>
GitAdd: Succeed / Failed / Skipped
GitCommit: Succeed / Failed / Skipped
GitPush: Succeed / Failed / Skipped
```

如果 Git 操作失败，需要记录失败原因。

例如：

```md
## EndAction
Action: git-commit-push
Workspace: C:\Arsenal\CoolAI\CoolBot
CommitMessage: 0013 测试CoolAIMessenger
GitAdd: Succeed
GitCommit: Failed
GitPush: Skipped
Reason: git commit failed
```

## CoolOperation 完整示范

```toml
format = "CoolOperation"
operation_id = "0013 测试CoolAIMessenger"
workspace = 'C:\Arsenal\CoolAI\CoolBot'

[[operations]]
type = "RunCmd"
work_dir = "."
log_mode = "ErrorOnly"
command = '''
cargo check
'''

[[operations]]
type = "End"
```

复杂完整示范：

```toml
format = "CoolOperation"
operation_id = "0012 阶段一收尾启动显示Cool状态"
workspace = 'C:\Arsenal\CoolAI\CoolCodex'

[[operations]]
type = "ModifyFile"
path = 'codex-rs\tui\src\lib.rs'

[[operations.commands]]
type = "InsertContent"
after_line = 1742
content = '''
    let permission_profile = config
        .permissions
        .active_permission_profile()
        .map(|profile| format!("{profile:?}"))
        .unwrap_or_else(|| format!("{:?}", config.permissions.permission_profile()));
    let ctool_scope_base = ctool::CToolScopeBase::default();

    config.startup_warnings.push(format!(
        "CoolStatus: SafeMode={}, CToolScopeBase={}, PermissionProfile={}",
        if config.safe_mode { "on" } else { "off" },
        ctool_scope_base,
        permission_profile
    ));

'''

[[operations]]
type = "RunCmd"
work_dir = 'codex-rs'
log_mode = "ErrorOnly"
command = '''
cargo check
'''

[[operations]]
type = "End"
```

## CoolAIMessenger 传递规则

CoolAIMessenger 用于通过 Telegram 接收 CoolOperation，并调用 CBot 执行。

CoolAIMessenger 传递 CoolOperation 时，应当传递 TOML 格式。

CoolAIMessenger 收到 CoolOperation 后，应当解析：

```toml
format = "CoolOperation"
operation_id = "<OperationID>"
workspace = '<Workspace>'
```

然后将收到的 TOML 原文保存到：

```text
<Workspace>\.cool\CoolOperation.toml
```

再调用：

```text
cbot <Workspace>\.cool\CoolOperation.toml
```

执行完成后读取：

```text
<Workspace>\.cool\CoolOperationReport.md
```

并向主人账号汇报。

### 启动反馈

CoolAIMessenger 启动成功后，应当向主人账号发送启动信息。

启动信息应当至少包括：

```text
CoolAIMessenger is ready.
Waiting for CoolOperation.
CBot: <cbot命令或路径>
Poll Seconds: <轮询间隔>
```

启动时应当丢弃 Telegram 历史 pending 消息。  
只处理 CoolAIMessenger 启动之后新收到的消息。

### 收到普通消息

CoolAIMessenger 收到主人账号发来的任何普通消息，都应当回复收到的信息。

如果信息很长，只回复前20字左右。

例如：

```text
Message received: <消息前20字>
```

普通消息只需要确认收到，不需要回复 Invalid command。

### 收到 CoolOperation

CoolAIMessenger 收到 CoolOperation TOML 后，应当立即回复：

```text
CoolOperation received. Executing now.

OperationID: <OperationID>
```

然后将 CoolOperation 保存为：

```text
<Workspace>\.cool\CoolOperation.toml
```

并调用 CBot 执行。

如果 CoolOperation 缺少 workspace，或者 workspace 不是绝对路径，应当立即向主人报错，不执行 CBot。

### 执行完成反馈

执行完成后，CoolAIMessenger 应当先发送英文执行结果。

成功示范：

```text
CoolOperation finished.

Status: Succeed
OperationID: <OperationID>
ExitCode: 0
Duration: <耗时> ms
ReportFound: Yes
ReportPath: <Workspace>\.cool\CoolOperationReport.md
```

失败示范：

```text
CoolOperation finished.

Status: Failed
OperationID: <OperationID>
ExitCode: <退出码>
Duration: <耗时> ms
ReportFound: Yes / No
ReportPath: <Workspace>\.cool\CoolOperationReport.md

Error:
<错误信息>
```

### 中文总结

英文执行结果之后，CoolAIMessenger 应当发送一段中文总结。

中文总结至少包括：

1. 执行的 CoolOperation ID
    
2. 是否执行成功
    
3. 如果失败，报错信息
    
4. Git Add 是否成功
    
5. Git commit 是否成功
    
6. Git push 是否成功
    
7. Git commit 备注信息是什么
    
8. 报告路径
    
9. 最后用中文总结一小段
    

成功示范：

```text
执行总结：

CoolOperation ID：0013 测试CoolAIMessenger
执行结果：成功

Git Add：成功
Git 提交：成功
Git 推送：成功
Commit 备注：0013 测试CoolAIMessenger
报告路径：C:\Arsenal\CoolAI\CoolBot\.cool\CoolOperationReport.md

本次操作已成功完成，代码已经提交并推送到远程仓库。
```

失败示范：

```text
执行总结：

CoolOperation ID：0013 测试CoolAIMessenger
执行结果：失败

报错信息：
cargo check failed

Git Add：跳过
Git 提交：跳过
Git 推送：跳过
Commit 备注：0013 测试CoolAIMessenger
报告路径：C:\Arsenal\CoolAI\CoolBot\.cool\CoolOperationReport.md

本次操作没有完整完成，需要根据上方报错修复后重新执行。
```

如果没有生成 CoolOperationReport.md，则 Git Add、Git 提交、Git 推送应当显示为：

```text
未执行
```

而不是：

```text
未知
```

### Windows 窗口输出

CoolAIMessenger 发送给 Telegram 的英文结果和中文总结，也应当完整打印到 Windows CMD 窗口。

这样主人可以从 Windows 窗口直接复制错误信息。

## coolbot-config.toml 推荐配置

```toml
default_workspace = 'C:\Arsenal\CoolAI\CoolBot'
cool_operation_file_name = 'CoolOperation.toml'
cool_operation_report_file_name = 'CoolOperationReport.md'

end_action = 'git-commit-push'
```

默认文件位置：

```text
C:\Arsenal\CoolAI\CoolBot\.cool\CoolOperation.toml
C:\Arsenal\CoolAI\CoolBot\.cool\CoolOperationReport.md
```

## 关键纪律

### 1/ Workspace 必须明确

每个 CoolOperation 推荐写 workspace。  
如果不写 workspace，则只能依赖 CBot 的默认 Workspace 配置。

用于 CoolAIMessenger 的 CoolOperation 必须写 workspace。

### 2/ End 必须存在

End 指令必须存在，而且必须是最后一个指令。

TOML 写法：

```toml
[[operations]]
type = "End"
```

### 3/ RunCmd 必须写 work_dir

RunCmd 推荐总是写 work_dir。  
即使在 Workspace 根目录执行，也写：

```toml
work_dir = "."
```

### 4/ 路径推荐使用相对路径

文件、文件夹、命令工作目录，推荐全部写相对 Workspace 的路径。  
只有确实需要跨 Workspace 时，才写绝对路径。

### 5/ 执行前后必须看报告

执行结果以 CoolOperationReport.md 为准。  
如果 Telegram 消息与 CoolOperationReport.md 不一致，以 CoolOperationReport.md 为准。

### 6/ Git 结果必须看 EndAction

Git add、commit、push 的结果必须查看 EndAction 区块。  
不能只看 RunCmd 是否成功。

### 7/ TOML 字符串纪律

Windows 路径推荐使用单引号字符串：

```toml
workspace = 'C:\Arsenal\CoolAI\CoolBot'
path = 'src\main.rs'
```

多行插入内容推荐使用多行单引号字符串：

```toml
content = '''
多行内容
'''
```

多行 CMD 命令推荐使用多行单引号字符串：

```toml
command = '''
cargo fmt
cargo check
'''
```

### 8/ 不再使用 Markdown 反引号指令块

CoolOperation TOML 本体不需要外层 Markdown 代码围栏。

也不需要：

``````text
# OperationID
## ModifyFile
### InsertContent
#### Content
`````
``````

这些旧 Markdown 指令语法全部废弃。

新的 CoolOperation 第一行推荐直接是：

```toml
format = "CoolOperation"
```

### 9/ Telegram 传输纪律

Telegram 文本传输时，直接发送 TOML 原文。

不要在 TOML 外面额外包 Markdown 代码块。

不要在 TOML 前面写说明文字。

不要在 TOML 后面写说明文字。

正确：

```toml
format = "CoolOperation"
operation_id = "0013 测试"
workspace = 'C:\Arsenal\CoolAI\CoolBot'

[[operations]]
type = "End"
```

错误：

```text
下面是 CoolOperation：

format = "CoolOperation"
operation_id = "0013 测试"
workspace = 'C:\Arsenal\CoolAI\CoolBot'

[[operations]]
type = "End"
```

错误：

````text
```toml
format = "CoolOperation"
operation_id = "0013 测试"
workspace = 'C:\Arsenal\CoolAI\CoolBot'

[[operations]]
type = "End"
```
````