
# CoolOperation

下文中出现 尖括号包围的内容  连同尖括号在内 都是表示注解。
请勿将注解本身当作实际代码。

## 指令内容
用十个 \` 框住的代码块 + CoolOperation 修饰 
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

```````````
`````````` CoolOperation 

# OperationID <起的名字>


``````````
``````````` 


### 2. 具体指令类型

具体指令类型 用 二级标题表示
具体指令包括以下类型：

| **中文名**      | **指令名字**        | 注释                                                                      |
| ------------ | --------------- | ----------------------------------------------------------------------- |
| 修改文件         | `ModifyFile`    |                                                                         |
| 修改文件子命令：删除行  | `DeleteLine`    |                                                                         |
| 修改文件子命令：插入内容 | `InsertContent` |                                                                         |
| 添加文件         | `CreateFile`    |                                                                         |
| 删除文件         | `RemoveFile`    |                                                                         |
| 清空文件         | `ClearFile`     | 删除文件内的全部内容                                                              |
| 添加文件夹        | `CreateFolder`  |                                                                         |
| 删除文件夹        | `RemoveFolder`  |                                                                         |
| 清空文件夹        | `ClearFolder`   | 删除文件夹中的全部内容                                                             |
| 运行CMD指令      | `RunCmd`        |                                                                         |
| 延迟           | `Delay`         | 上一个**指令完成后**等待<br>注意 是上一个命令 报告完成后 <br>开始等待，并非上一个命令刚执行<br>专门用于应付莫名其妙的Bug |
| 截图           | `Screenshot`    |                                                                         |
| 结束指令         | `End`           | 无需提供git相关指令<br>结束时使用End指令<br>会自动完成git提交等其他相关工作                          |

### 以下是具体指令讲解

所有指令的执行结果都会记录在
名为 CoolOperationReport.md 的 Markdown文件中
会以二级标题 表示指令 (子命令用3级标题)
下面表示指令执行的情况
例如

```
## ModifyFile Path:<文件的绝对路径>
Succeed

### DeleteLine LineNum：5
Succeed

## Delay Duration：3
Succeed

```


### 修改文件 ModifyFile

要求 二级标题写 ModifyFile
下一行写 三级标题 Path 空格 接要修改的文件的 **绝对路径**

后续三级标题 表示 修改文件的子命令

例如：
```
## ModifyFile
### Path <文件的绝对路径>
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
具体的内容以五个\` 包裹

例如 表示在第6行的紧下方插入

```
     ABC
DFG
```
：

````````
### InsertContent
6
#### Content
`````
     ABC
DFG
`````
````````


### 注意修改文件子命令的使用纪律

#### 1/ 命令应当自上而下依次发布 不得错乱顺序
#### 2/ 不得互相交叉
删除接 删除 上面的末尾 必须小于 下面的开头
删除接插入 上面的末尾 必须小于等于 下面的开头
插入接删除  上面的末尾 必须小于 下面的开头
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
接下来三级标题 以Path命名 接空格 然后写 执行的 **绝对路径**

注意：添加文件、文件夹时，绝对路径直接将 添加的文件、文件夹名字包含在内

以 添加文件夹 为例

```
## CreateFolder
### Path <文件夹的绝对路径>
```


### 运行CMD指令 RunCmd
二级标题   RunCmd 开头 
下一行接 三级标题 LogMode 表示执行结果的记录方式 
记录方式分别是
Verbose 全记录
ErrorOnly 只记录错误
记录方式下方紧接着写要执行的命令，或一组命令
使用五个反引号包起来


例如
```````
## RunCmd
### LogMode ErrorOnly
`````
cargo check
`````
```````

再例如 执行命令组：
````````
## RunCmd
### LogMode ErrorOnly
`````
cargo fmt
cargo check
`````
````````

RunCmd 的结果 
在记录到 CoolOperationReport.md 文件中时 
会根据 LogMode 记录执行的结果以 五个 \`   包围的代码块记录结果
例如：

``````
## RunCmd cargo check
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
例如：
```
## End
```


