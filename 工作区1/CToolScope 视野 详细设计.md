
# 视野设置

# 基础视野设置
由  CToolBaseScope
决定

仅位于  LaunchDir  .cool 中 config.toml

如果没有




##

SystemDir  .cool-system  中 config.toml
和 LaunchDir  .cool 中 config.toml
scope.toml

其中有 以下分段 分别的决定 System级别的 和 的视野设置。

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



## 视野权限

System.filehide>System.fileReadOnly>System.filereadwrite >
System.folderhide>System.folderReadOnly>System.folderreadwrite >
LaunchDir.filehide>LaunchDir.fileReadOnly>LaunchDir.filereadwrite >
LaunchDir.folderhide>LaunchDir.folderReadOnly>LaunchDir.folderreadwrite >
CToolBaseScope




#### 权限设计

就叫 LaunchDir 如何，这个简洁易懂   你觉得合适吗？
然后我们的 .coolcache  和 .coolconfig.toml 完全跟随 LaunchDir 文件夹 而不是 Coolworkspace

Coolworkspace 应当再 .coolconfig.toml 中有设置。 没有设置 默认为 LaunchDir
Coolworkspace 开头应该显示 具体目录 作为第四项


readwrite  
readonly  
hide


CToolBaseScope 对应的也应该是 Coolworkspace
并且 Coolworkspace 在开头应该也标注出来具体是哪个文件夹


我的意思是问，Codex 有没有系统性提示词？ 这个提示词决定了当前整个的绘画基调。




另外，你得告诉我 Codex 是否有天然的，就是系统提示此文件夹？ 这个提示词是要贯穿始末的，它不是说某一次加载的 test，它决定了整个这一次对话这一个启动出来的 context 的整体会话气质。

#### 加载顺序
```
1. main 初始化 SafeMode
2. 解析 PermissionProfile
3. 解析 CToolBaseScope
4. 获取 CurrentDir，也就是当前 CMD 调用 codex 的文件夹
5. 加载 .coolsystemconfig.toml  //暂时位于启动 bat相同文件夹  找不到所有内容按默认空算
- 找不到：按空配置  
- 格式错误：CTool 不启用，报错  
6. 加载 WorkspaceRoot\.coolconfig.toml  
- 找不到：按空配置  
- 格式错误：CTool 不启用，报错  
7. 构造 CToolScopeContext  
8. 构造 CToolContext  
9. 注册 CTool
```



#### 视野操作 相关命令
命令正式为 /CToolScope 简写 /cs 
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




