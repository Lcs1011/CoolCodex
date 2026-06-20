
## 设计哲学
CoolOperationDollarSyntax 是被设计用来修改代码的代码 下文简称 DollarSyntax
我设计的脚本会根据 CoolOperationDollarSyntax语法 精确的修改代码。

语法设计高度类似于 Markdown ，`$` 起到类似 `#` 的标题作用，每多一个就表示小一个层级，让结构清晰易懂

因为CoolOperationDollarSyntax 是为了修改代码的代码，因而采用`$` 这个代码中中的冷门符号，极大程度避免了语义冲突



## 外层围栏

采用`$$$$$$$$$$ CoolOperation`
和 `$$$$$$$$$$` 作为外层围栏，是DollarSyntax的标识 

例如：

```text
$$$$$$$$$$ CoolOperation
$ OperationID <本次操作名称>
Workspace <工作区绝对路径>

<具体指令>

$$ End
$$$$$$$$$$
```


## 指令语法

下文中出现 `<>` 包围的内容是为了辅助讲解，请勿将 尖括号视为实际语法。

### 1. 总标题

#### OperationID
第一行 命名本次操作的 **OperationID** 
```
$ OperationID <给ID起的名字>
```
OperationID 也是之后自动推送到 github上的 commit 备注的名字。

#### Workspace
第二行要填写 **Workspace**
```
Workspace <Workspace的绝对路径>
```

Workspace 是代码执行完毕后自动 commit 和 push的 目录
日志存放的文件夹也是根据 Workspace 路径设计，是非常重要的

并且，下文中介绍涉及到路径，如果是出现在 Workspace范围内的路径都要求用
`.` 作为开头使用相对路径表示
例如，加入Workspace为 `C:\Arsenal\CoolAI\CoolBot\`
```
.\src\main.rs
```
表示
```
C:\Arsenal\CoolAI\CoolBot\src\main.rs
```

指令中填写路径 不允许使用 `..` 表示上级路径
因而工作区外的指令 必须是绝对路径，包含使用绝对路径指令，脚本会给用户额外警告。
路径中的 / 和 \ 视为等价。

#### EndAction
第三行要填写 **EndAction** 类型
EndAction 枚举类型 决定了指令结束之后脚本自动做什么
`none`  表示什么都不做
`git-commit-push` 成功执行完所有指令后推送
`always-git-commit-push` 如果有任何对项目的修改，都强行推送代码，哪怕是错误的修改，甚至没有执行完毕所有的指令都推送

AI 网页端编程强烈推荐使用 `always-git-commit-push` ！
因为 网页端 AI了解当下情况的唯一方式就是看代码仓库，如果不推送，网页端AI是完全失明的状态。

#### 总标题部分完整示范：
ID为  `0021 重构RunCmd人类可读显示`
Workspace 为 `C:\Arsenal\CoolAI\CoolBot`
EndAction 为 `always-git-commit-push`

```text
$$$$$$$$$$ CoolOperation
$ OperationID 0021 重构RunCmd人类可读显示
Workspace C:\Arsenal\CoolAI\CoolBot
EndAction always-git-commit-push

<具体指令>

$$ End
$$$$$$$$$$
```


### 2. 修改文件指令 ModifyFile

ModifyFile 用来修改一个已有文件。 是最重要的核心指令之一。
ModifyFile 可以包含多个子指令，通过子指令完成具体的修改

```text
$$ ModifyFile
$$$ Path <要修改的文件的路径>

<修改子命令>
```

示例：

```text
$$ ModifyFile
$$$ Path ./src/main.rs

$$$ ReplaceExact
<...省略具体内容>
<...省略其他子指令>
```

#### 2.1 修改文件子指令 Exact 整行 /整代码块指令
这一部分介绍的都是ModifyFile 的子指令。
这些指令 表示的都是整行，或者整代码块为单位的操作，不支持行内局部操作。
其中的 Target 和 Content包含的内容，必须是整行的内容，不可以是行中一部分内容。
这部分指令都要求Target必须命中且只精准命中1次，否则Preflight 失败。

**注意** Target 填入使用指令之前的代码内容即可，既不需要、也不允许 通过计算前面指令发生之后的代码来决定 Target是什么

##### 2.1.1 ReplaceExact
是 ModifyFile 的子指令，用于精确的替换文本块

```
$$$ ReplaceExact

$$$$ Target
<旧内容>
$$$$ Content
<新内容>
$$$$ EndReplaceExact
```

##### 2.1.2 InsertBeforeExact
在 Target文本 前插入内容。

```
$$$ InsertBeforeExact

$$$$ Target
<锚点内容>
$$$$ Content
<插入的内容>
$$$$ EndInsertBeforeExact
```

##### 2.1.3 InsertAfterExact
在 Target文本 后插入内容。

```
$$$ InsertAfterExact

$$$$ Target
<锚点内容>
$$$$ Content
<插入内容>
$$$$ EndInsertAfterExact
```

##### 2.1.4 RemoveExact
删除 Target内容

```
$$$ RemoveExact

$$$$ Target
<要删除的内容>
$$$$ EndRemoveExact
```



#### 2.2. 修改文件子指令 LineRange 整行 / 整代码块指令

这一部分介绍的都是 ModifyFile 的子指令。

这些指令表示的都是以整行，或者整代码块为单位的操作，不支持行内局部操作。

其中的 Line、StartLine、EndLine 表示的都是使用指令之前的文件行号。

其中的 Content 包含的内容，必须是整行的内容，不可以是行中一部分内容。

这部分指令都要求行号必须存在，且行号范围必须合法，否则 Preflight 失败。

**注意** Line、StartLine、EndLine 填入使用指令之前的文件行号即可，既不需要、也不允许通过计算前面指令发生之后的代码来决定行号是什么。

##### 2.2.1 ReplaceLineRange

是 ModifyFile 的子指令，用于用 Content 替换指定行号范围内的内容。

~~~text
$$$ ReplaceLineRange

StartLine <开始行号>
EndLine <结束行号>

$$$$ Content
<新内容>
$$$$ EndReplaceLineRange
~~~

##### 2.2.2 InsertBeforeLine

在 Line 指向的行前插入内容。

~~~text
$$$ InsertBeforeLine

Line <行号>

$$$$ Content
<插入的内容>
$$$$ EndInsertBeforeLine
~~~

##### 2.2.3 InsertAfterLine

在 Line 指向的行后插入内容。

~~~text
$$$ InsertAfterLine

Line <行号>

$$$$ Content
<插入内容>
$$$$ EndInsertAfterLine
~~~

##### 2.2.4 RemoveLineRange

删除指定行号范围内的内容。

~~~text
$$$ RemoveLineRange

StartLine <开始行号>
EndLine <结束行号>

$$$$ EndRemoveLineRange
~~~


#### 2.3 ReplaceAll 替换全部内容指令

ReplaceAll 是 ModifyFile 的子指令，用于将当前文件内所有 Target 内容替换成 Content 内容。  该指令允许行内操作
  
```text  
$$$ ReplaceAll  
  
$$$$ Target  
<要被替换的内容>  
$$$$ Content  
<替换成的新内容>  
$$$$ EndReplaceAll  
```


### 3. 创建文件指令 CreateFile

`CreateFile` 用于创建新文件。
目标文件已存在时失败，避免覆盖重要文件。

```text
$$ CreateFile
$$$ Path <文件路径>
$$$$ Content
<文件内容>
$$$$ EndCreateFile
```

### 4. 创建文件夹 CreateFolder

如果已存在该文件夹，则直接视为创建成功
```
$$ CreateFolder  
  
$$$ Path <要创建的文件夹路径>  
$$$ EndCreateFolder
```

### 5. 删除文件 DeleteFile


```
$$ DeleteFile

$$$ Path <要删除的文件路径>
$$$ EndDeleteFile
```

### 6. 删除文件夹 DeleteFolder

会删除整个文件夹，包括其中的全部内容
```
$$ DeleteFolder

$$$ Path <要删除的文件夹路径>
$$$ EndDeleteFolder
```

DeleteFolder 不允许删除 Workspace 根目录、磁盘根目录、用户主目录等高风险目录。


### 7. RunCmd

RunCmd 用于导航到指定位置执行命令行
其中 WorkDir 即为要导航的位置
LogMode 表示输出执行日志的方式 
  Verbose 输出命令行执行的所有内容到日志
  ErrorOnly 仅将出现错误的位置前后文记录到日志

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


### 8. End

`End` 必须存在，并且必须是最后一个指令。

```text
$$ End
```

`End` 后不应再有任何业务指令。

### 9. 注释语法

#### 单行注释语法： 
```
$//$
```

当行注释 表示这一行是注释内容，之后不会对操作有任何影响

#### 范围注释:
开头
```
$/*$
```
结尾
```
$*/$
```

被范围注释包围的全部内容都会被自动踢出 实际命令，不会对命令有任何影响
范围注释必须成对出现，否则视为语法错误。