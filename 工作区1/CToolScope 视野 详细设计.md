


# 基础视野


基础视野 由  CToolScopeBase 枚举变量 决定

CToolScopeBase 的默认值位于
仅位于  SessionRoot   的 .cool 中 config.toml


如果没有 找到默认值 则默认为 none

 None 为基础视野
 即使 CoolWorkspace 默认等于 SessionRoot，
 CTool 也仍然没有视野；
 只有 CToolScopeBase = CoolWorkspace 时，才使用 CoolWorkspace 路径作为基础视野

# 视野设置

CToolScopeBase枚举 位于 SessionRoot  .cool 中的  config.toml  设置
CoolWorkspace  路径 也位于 SessionRoot  .cool 中的  config.toml  设置



## 视野详细设置
LauncherDir  .cool-system  中 scope.toml
和 SessionRoot  .cool 中 scope.toml


其中有 以下分段 分别的决定 System级别 的 和SessionRoot 级别 的视野设置。

```
[files]
readwrite = []
readonly = []
hide = []


[folders]
readwrite = []
readonly = []
hide = [
  ".codex",
  ".cool"
]
```





## 视野设置权限

### 原则概述

系统 大于 Session

文件大于文件夹 

注：“文件规则优先于文件夹规则”是否符合预期。这意味着可以通过文件级规则对隐藏文件夹里的单个文件做例外放行
但是SessionRoot 级别 无法越过 System级别

禁止大于开放  （hide > readonly > readwrite）

都大于 CToolScopeBase


### 详细判定路径：
System 和 SessionRoot 表示两个权限级别

System.filehide>System.fileReadOnly>System.filereadwrite >
System.folderhide>System.folderReadOnly>System.folderreadwrite >
SessionRoot.filehide>SessionRoot.fileReadOnly>SessionRoot.filereadwrite >
SessionRoot.folderhide>SessionRoot.folderReadOnly>SessionRoot.folderreadwrite >
CToolScopeBase






#### 加载顺序

1. main 初始化 SafeMode
2. 解析 PermissionProfile
3. 解析 CToolScopeBase
4. 获取 SessionRoot，也就是当前 CMD 调用 codex 的文件夹

5. 加载 LauncherDir\.cool-system\config.toml //暂时位于启动 bat相同文件夹  找不到所有内容按默认空算

	- 找不到：按空配置  
	- 格式错误：CTool 不启用，报错  
6. 加载 SessionRoot\.cool\config.toml
	- 找不到：按空配置  
	- 格式错误：CTool 不启用，报错  
7. 构造 CToolScopeContext  
8. 构造 CToolContext  
9. 注册 CTool




#### 视野操作 相关命令
命令正式为 
/CToolScope 简写 
/cs 
整个命令无视大小写
//CLI 本身是区分命令大小写的

```
/cs
/cs show
显示当前视野配置

/cs <path>
添加到 visible_paths

/cs - <path>
从 visible_paths 移除

/cs hide <path>
添加到 hide_paths

/cs hide - <path>
从 hide_paths 移除

/cs base none
/cs base workspace

/cs protect <path>  //表示只读
/cs protect - <path>

```




