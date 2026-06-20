# CoolOperation

下文中出现 尖括号包围的内容 连同尖括号在内 都是表示注解。  
请勿将注解本身当作实际代码。

## 指令内容

用十个 ` 框住的代码块 + CoolOperation 修饰  
表示我们的操作指令

例如

```````````
`````````` CoolOperation 

// 具体的操作指令

``````````
```````````

## CoolOperation 指令语法

### 1. 指令标题

一级标题 表示指令ID 以 `OperationID` 为开头 空格 接给指令起的名字  
这个名字也是完成指令后 push到 github上 之前commit的名字

一级标题下方可以写 Workspace。  
Workspace 表示本次 CoolOperation 所属的工作区根目录。  
Workspace 必须是绝对路径。

```````````
`````````` CoolOperation 

# OperationID <起的名字>

Workspace = <工作区绝对路径>


``````````
```````````

例如：

```````````
`````````` CoolOperation 

# OperationID 0013 测试CoolAIMessenger

Workspace = C:\Arsenal\CoolAI\CoolBot


``````````
```````````

### 2. 默认文件位置

CoolOperation 默认放在 Workspace 下方的 `.coolbot` 文件夹中。

默认指令文件：

```
<Workspace>\.cool\CoolOperation.toml
```

默认报告文件：

```
<Workspace>\.cool\CoolOperationReport.md
```

例如：

```
C:\Arsenal\CoolAI\CoolBot\.coolbot\CoolOperation.md
C:\Arsenal\CoolAI\CoolBot\.coolbot\CoolOperationReport.md
```

如果使用 cbot d，则默认读取：

```
<default_workspace>\.coolbot\CoolOperation.md
```

其中 default_workspace 来自 coolbot-config.toml。

### 3. 路径规则

CoolOperation 中涉及文件、文件夹、命令工作目录时，推荐使用相对路径。

相对路径从 Workspace 开始解析。

例如：

```
Workspace = C:\Arsenal\CoolAI\CoolBot
```

那么：

```
src\main.rs
```

表示：

```
C:\Arsenal\CoolAI\CoolBot\src\main.rs
```

如果路径写成绝对路径，则直接使用绝对路径。

例如：

```
C:\Arsenal\CoolAI\CoolBot\src\main.rs
```

注意：

1. Workspace 必须是绝对路径。
    
2. WorkDir 可以是相对路径，也可以是绝对路径。
    
3. 文件、文件夹路径推荐使用相对 Workspace 的路径。
    
4. 跨出 Workspace 的绝对路径应谨慎使用。
    

### 4. 具体指令类型

具体指令类型 用 二级标题表示  
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
|延迟|`Delay`|上一个**指令完成后**等待注意 是上一个命令 报告完成后 开始等待，并非上一个命令刚执行专门用于应付莫名其妙的Bug|
|截图|`Screenshot`||
|结束指令|`End`||

### 以下是具体指令讲解

所有指令的执行结果都会记录在  
名为 CoolOperationReport.md 的 Markdown文件中

默认位置是：

```
<Workspace>\.coolbot\CoolOperationReport.md
```

会以二级标题 表示指令 (子命令用3级标题)  
下面表示指令执行的情况  
例如

```
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

要求 二级标题写 ModifyFile  
下一行写 三级标题 Path 空格 接要修改的文件路径

文件路径可以是相对路径，也可以是绝对路径。  
推荐使用相对 Workspace 的路径。

后续三级标题 表示 修改文件的子命令

例如：

```
## ModifyFile
### Path src\main.rs
### <后续子命令>
```

也可以写绝对路径：

```
## ModifyFile
### Path C:\Arsenal\CoolAI\CoolBot\src\main.rs
### <后续子命令>
```

### 接下来两节讲解修改文件子命令

一个 ModifyFile命令 可以报很多子命令

注意里面操作的行数，指向的都是修改前的行数  
而非修改后的行数。所以考虑这些命令的时候，  
针对当前代码的位置即可无需计算递归

### 修改文件子命令： 删除行 DeleteLine

要求 三级标题开头 以 DeleteLine 命名  
下一行 表示 删除的行数  
可以是 单个数字接 - 再接数字 表示删除一段

子命令示范 ： 删除第5行

```
### DeleteLine
5
```

子命令示范 ： 删除第5-10行

```
### DeleteLine
5-10
```

### 修改文件子命令： 插入内容 InsertContent

要求 三级标题开头 以 InsertContent 命名  
下一行 写一个数字 表示插入的行号。表示插入该行的紧下方。

再下一行 四级标题 以 Content 命名  
再接下来写具体的内容  
具体的内容以五个` 包裹

例如 表示在第6行的紧下方插入

```
     ABC
DFG
```

：

``````
### InsertContent
6
#### Content
`````
     ABC
DFG
`````
``````

### 注意修改文件子命令的使用纪律

#### 1/ 命令应当自上而下依次发布 不得错乱顺序

#### 2/ 不得互相交叉

删除接 删除 上面的末尾 必须小于 下面的开头  
删除接插入 上面的末尾 必须小于等于 下面的开头  
插入接删除 上面的末尾 必须小于 下面的开头  
插入没必要 接插入

### 添加、删除、清空 文件、文件夹

分别以  
CreateFile  
RemoveFile  
ClearFile  
CreateFolder  
RemoveFolder  
ClearFolder  
为二级标题  
接下来三级标题 以Path命名 接空格 然后写 执行的路径

路径可以是相对路径，也可以是绝对路径。  
推荐使用相对 Workspace 的路径。

注意：添加文件、文件夹时，路径直接将 添加的文件、文件夹名字包含在内

以 添加文件夹 为例

```
## CreateFolder
### Path .coolbot
```

以 添加文件 为例

```
## CreateFile
### Path src\new_file.rs
```

以 删除文件 为例

```
## RemoveFile
### Path src\old_file.rs
```

### 运行CMD指令 RunCmd

二级标题 RunCmd 开头

下一行接 三级标题 WorkDir 表示命令执行目录。  
WorkDir 可以是相对路径，也可以是绝对路径。  
推荐使用相对 Workspace 的路径。

再下一行接 三级标题 LogMode 表示执行结果的记录方式  
记录方式分别是  
Verbose 全记录  
ErrorOnly 只记录错误

LogMode 下方紧接着写要执行的命令，或一组命令  
使用五个反引号包起来

例如

``````
## RunCmd
### WorkDir .
### LogMode ErrorOnly
`````
cargo check
`````
``````

再例如 执行命令组：

``````
## RunCmd
### WorkDir .
### LogMode ErrorOnly
`````
cargo fmt
cargo check
`````
``````

再例如 在子目录执行命令：

``````
## RunCmd
### WorkDir codex-rs
### LogMode ErrorOnly
`````
cargo check
`````
``````

RunCmd 的结果  
在记录到 CoolOperationReport.md 文件中时  
会根据 LogMode 记录执行的结果以 五个 ` 包围的代码块记录结果  
例如：

``````
## RunCmd
WorkDir: .
LogMode: ErrorOnly
Status: Failed
ExitCode: 101

`````
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
`````

## <Report中的其他指令记录..>
``````

### 延迟 Delay

二级标题 Delay 开头  
下一行数字表示延迟时间 以秒为单位

例如 等待5秒

```
## Delay
5
```

### 截图 Screenshot

二级标题 Screenshot 为开头

暂时执行全屏截图

### 结束指令 End

使用 二级标题 End 即可 无内容  
End指令必须存在且必须是最后一个指令  
如下：

```
## End
```

coolbot-config.toml 中

```
end_action = 'none'
```

则 End 只结束本次 CoolOperation，不额外执行 Git 操作。

coolbot-config.toml 中

```
end_action = 'git-commit'
```

则结束时会自动执行：

```
git add -A
git commit -m "<OperationID>"
```

coolbot-config.toml 中

```
end_action = 'git-commit-push'
```

则结束时会自动执行：

```
git add -A
git commit -m "<OperationID>"
git push
```

Git 命令执行位置是 Workspace。

EndAction 的结果必须记录在 CoolOperationReport.md 中。  
记录内容包括：

```
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

```
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

```````````
`````````` CoolOperation

# OperationID 0013 测试CoolAIMessenger

Workspace = C:\Arsenal\CoolAI\CoolBot

## RunCmd
### WorkDir .
### LogMode ErrorOnly
`````
cargo check
`````

## End

``````````
```````````

## CoolAIMessenger 传递规则

CoolAIMessenger 用于通过 Telegram 接收 CoolOperation，并调用 CBot 执行。

### 启动反馈

CoolAIMessenger 启动成功后，应当向主人账号发送启动信息。

启动信息应当至少包括：

```
CBotNetLinker is ready.
Waiting for CoolOperation.
CBot: <cbot命令或路径>
Poll Seconds: <轮询间隔>
```

### 收到普通消息

CoolAIMessenger 收到主人账号发来的任何普通消息，都应当回复收到的信息。

如果信息很长，只回复前20字左右。

例如：

```
Message received: <消息前20字>
```

### 收到 CoolOperation

CoolAIMessenger 收到 CoolOperation 后，应当立即回复：

```
CoolOperation received. Executing now.

OperationID: <OperationID>
```

然后将 CoolOperation 保存为本地文件，并调用 CBot 执行。

### 执行完成反馈

执行完成后，CoolAIMessenger 应当先发送英文执行结果。

成功示范：

```
CoolOperation finished.

Status: Succeed
OperationID: <OperationID>
ExitCode: 0
Duration: <耗时> ms
```

失败示范：

```
CoolOperation finished.

Status: Failed
OperationID: <OperationID>
ExitCode: <退出码>
Duration: <耗时> ms

Error:
<错误信息>
```

### 中文总结

英文执行结果之后，CoolAIMessenger 应当发送一段中文总结。

中文总结至少包括：

1. 执行的 CoolOperation ID
    
2. 是否执行成功
    
3. 如果失败，报错信息
    
4. Git commit 是否成功
    
5. Git push 是否成功
    
6. Git commit 备注信息是什么
    
7. 最后用中文总结一小段
    

成功示范：

```
执行总结：

CoolOperation ID：0013 测试CoolAIMessenger
执行结果：成功

Git 提交：成功
Git 推送：成功
Commit 备注：0013 测试CoolAIMessenger

本次操作已成功完成，代码已经提交并推送到远程仓库。
```

失败示范：

```
执行总结：

CoolOperation ID：0013 测试CoolAIMessenger
执行结果：失败

报错信息：
cargo check failed

Git 提交：跳过
Git 推送：跳过
Commit 备注：0013 测试CoolAIMessenger

本次操作没有完整完成，需要根据上方报错修复后重新执行。
```

## coolbot-config.toml 推荐配置

```
default_workspace = 'C:\Arsenal\CoolAI\CoolBot'
cool_operation_file_name = 'CoolOperation.md'
cool_operation_report_file_name = 'CoolOperationReport.md'

end_action = 'git-commit-push'
```

默认文件位置：

```
C:\Arsenal\CoolAI\CoolBot\.coolbot\CoolOperation.md
C:\Arsenal\CoolAI\CoolBot\.coolbot\CoolOperationReport.md
```

## 关键纪律

### 1/ Workspace 必须明确

每个 CoolOperation 推荐写 Workspace。  
如果不写 Workspace，则只能依赖 CBot 的默认 Workspace 配置。

### 2/ End 必须存在

End 指令必须存在，而且必须是最后一个指令。

### 3/ RunCmd 必须写 WorkDir

RunCmd 推荐总是写 WorkDir。  
即使在 Workspace 根目录执行，也写：

```
### WorkDir .
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