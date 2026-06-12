


## 基础视野

### CToolScopeBase
基础视野 由   枚举变量 决定

CToolScopeBase 的设置位于
```text
CharacterRoot\.cool\config.toml
```



示范：
```toml
ctool_scope_base = "CoolWorkspace"
```

如果没有这一项，则按默认值处理：
```text
CToolScopeBase = None
```



//补一下设置示范
没有找到配置 则  将 CToolScopeBase 设置为 None


### None 为无基础视野。
CTool 即没有任何视野 既不能看到  CoolWorkspace 更不能看到 CharacterRoot

仅位于  CharacterRoot   的 .cool 中 config.toml

### CoolWorkspace 为 
使用 CoolWorkspace 文件夹为基础视野

### SelectedOnly
不开放基础视野，只依赖显式文件/文件夹规则。

### TheEyeOfProvidence

全局视野。

#### CoolWorkspace

默认设置位于 ：
```text
CharacterRoot\.cool\config.toml
```

Cool workspace 的设置可以是绝对路径或相对路径。

绝对路径示范：
```toml
cool_workspace = "C:\\CodexLab\\codex"
```
// 在 TOML 语法中，反斜杠在双引号中有转义含义。就是双反斜杠让反斜杠转义反斜杠，从而避免了反斜杠转义别的东西。


TOML 单引号写法，可以避免 Windows 路径中的反斜杠转义：
```toml
cool_workspace = 'C:\CodexLab\codex'
```


相对路径示范：（ 保留相对路径的话，有利于后续的产品移植。）
采用根据 CharacterRoot 的相对位置进行解析。

```toml
cool_workspace = "..\codex"
```
// ..\codex 这个表示上一级文件夹存在一个叫 codex 的文件夹


## 视野配置文件

### 视野详细配置分为 CoolSystem 级 和 Character 级。

CoolSystem级 配置位于：

```text
LauncherDir\.cool-system\scope.toml
```

Character 配置位于：

```text
CharacterRoot\.cool\scope.toml
```


### 配置文件内部结构

```toml
[files]
readwrite = []
readonly = []
hidden = []

[folders]
readwrite = []
readonly = []
hidden = []
```



#### 推荐系统配置文件

```toml
[files]
readwrite = []
readonly = []
hidden = []

[folders]
readwrite = []
readonly = []
hidden = [
  ".codex",
  ".cool"
]
```


## 视野配置权重

#### 原则概述 （上面的原则 大于  下面的原则）


1/ CoolSystem 大于 Character级别
2/ 文件大于文件夹 
这意味着可以通过文件级规则对隐藏文件夹里的单个文件做例外放行
但是 Character 级别 无法越过 CoolSystem 级别

3/ 禁止大于开放  （hidden > readonly > readwrite）

4/ 都大于 CToolScopeBase


#### 详细判定路径：
System 和 Character 表示两个权限级别

System.filehidden>System.fileReadOnly>System.filereadwrite >
System.folderhidden>System.folderReadOnly>System.folderreadwrite >
Character.filehidden>Character.fileReadOnly>Character.filereadwrite >
Character.folderhidden>Character.folderReadOnly>Character.folderreadwrite >
CToolScopeBase




## 启动加载顺序




```text
1. 初始化 SafeMode
2. 解析 PermissionProfile
3. 捕获 LauncherDir
4. 捕获 CharacterRoot
5. 定位 CoolSystemDir = LauncherDir\.cool-system
6. 定位 CoolDir = CharacterRoot\.cool

7. 读取 CoolSystemDir\config.toml
   找不到：按空配置
   格式错误：CTool 不启用，并报错

8. 读取 CoolDir\config.toml
   找不到：按空配置
   格式错误：CTool 不启用，并报错

9. 根据 CoolDir\config.toml 确定：
   CToolScopeBase
   CoolWorkspace

10. 读取 CoolSystemDir\scope.toml
    找不到：按空配置
    格式错误：CTool 不启用，并报错

11. 读取 CoolDir\scope.toml
    找不到：按空配置
    格式错误：CTool 不启用，并报错

12. 合并系统级和 `CharacterRoot` 级 scope 配置

13. 构造 CToolScopeContext

14. 读取 CoolSystemDir\command.toml
    找不到：按空配置
    格式错误：ctool_command_request 不启用，并报错

15. 读取 CoolDir\command.toml
    找不到：按空配置
    格式错误：ctool_command_request 不启用，并报错

16. 合并系统级和 `CharacterRoot` 级 command 配置

17. 构造 CToolContext

18. 注册 CTool
```





## 视野操作 相关命令

/cs  为 CToolScope 简写
f = file
不带 f = folder

可以操作配置的命令，都是操作 Character 级别的。



```
//向 Character 设置 读写 只读 隐藏 添加 文件 路径
  /cs f <path>
  /cs f ro <path>
  /cs f hidden <path>
  
//从 Character 设置 读写 只读 隐藏 移除 文件 路径
  /cs f - <path>
  /cs f ro - <path>
  /cs f hidden - <path>

//向 Character 设置 读写 只读 隐藏 添加 文件夹 路径
  /cs <path>
  /cs ro <path>

  /cs hidden <path>
  
//Character 设置 读写 只读 隐藏 移除 文件夹 路径
  /cs - <path>
  /cs ro - <path>
  /cs hidden - <path>
  
//设置基础视野枚举
  /cs base none
  /cs base coolworkspace
  
//显示视野配置相关信息: 枚举、Character 设置、系统视野设置
/cs show

```

###### <font color="#ff0000">TODO</font>

`.cool`、`.cool-system` 呃，这2个文件夹直接在 所有CTool 代码层写死，开头是这2个的文件夹直接读不了，那现在先不做，因为暂时这么干会耽误干活。





