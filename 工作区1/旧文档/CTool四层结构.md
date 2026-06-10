
# 四层结构


## 第 1 层：SafeMode  

SafeMode AboveAll 原则

一旦开启，会强制关闭所有的 Codex 原生工具
只有CTool 可以正常使用

## 第 2 层：PermissionProfile
加入 Cool开头的模式。 
核心是 CoolReadWrite

Cool开头的模式
只能使用 TCool工具


## 第 3 层：CToolScope  

CToolScope 并非是来限制 PermissionProfile 的视野范围
而是 直接限制 CTool的视野范围 从而达到 绝对控制

由 CToolScopeBase来设定基础范围 配合配置文件


None
Workspace
SelectedOnly
TheEyeofProvidence


## 第4层 CTool

### 读取工具

| 功能                  | Rust 类型名             | 文件名                        | 工具名                     |
| ------------------- | -------------------- | -------------------------- | ----------------------- |
| 看项目结构               | `CToolListDirectory` | `ctool_list_directory.rs`  | `ctool_list_directory`  |
| 搜关键词 / 类型 / 函数 / 报错 | `CToolRgSearch`      | `ctool_rg_search.rs`       | `ctool_rg_search`       |
| 读取命中位置附近代码          | `CToolReadCodeRange` | `ctool_read_code_range.rs` | `ctool_read_code_range` |
| 读取完整小文件或配置文件        | `CToolReadFile`      | `ctool_read_file.rs`       | `ctool_read_file`       |


### 修改工具

| 功能            | Rust 类型名           | 文件名                     | 工具名                  |
| ------------- | ------------------ | ----------------------- | -------------------- |
| 单点精确替换        | `CToolEditReplace` | `ctool_edit_replace.rs` | `ctool_edit_replace` |
| 单点插入          | `CToolEditInsert`  | `ctool_edit_insert.rs`  | `ctool_edit_insert`  |
| 预览修改结果        | `CToolPreviewDiff` | `ctool_preview_diff.rs` | `ctool_preview_diff` |
| 多个替换 / 插入打包执行 | `CToolEditBatch`   | `ctool_edit_batch.rs`   | `ctool_edit_batch`   |

### 文件操作工具

| 功能          | Rust 类型名               | 文件名                         | 工具名                      |
| ----------- | ---------------------- | --------------------------- | ------------------------ |
| 创建文件        | `CToolCreateFile`      | `ctool_create_file.rs`      | `ctool_create_file`      |
| 删除文件        | `CToolDeleteFile`      | `ctool_delete_file.rs`      | `ctool_delete_file`      |
| 移动 / 重命名文件  | `CToolMoveFile`        | `ctool_move_file.rs`        | `ctool_move_file`        |
| 创建文件夹       | `CToolCreateDirectory` | `ctool_create_directory.rs` | `ctool_create_directory` |
| 删除文件夹       | `CToolDeleteDirectory` | `ctool_delete_directory.rs` | `ctool_delete_directory` |
| 移动 / 重命名文件夹 | `CToolMoveDirectory`   | `ctool_move_directory.rs`   | `ctool_move_directory`   |
### Telegram 对话工具

| 功能     | Rust 类型名                   | 文件名                              | 工具名                           |
| ------ | -------------------------- | -------------------------------- | ----------------------------- |
| TG 发消息 | `CToolTelegramSendMessage` | `ctool_telegram_send_message.rs` | `ctool_telegram_send_message` |
| TG 收指令 | `CToolTelegramPollCommand` | `ctool_telegram_poll_command.rs` | `ctool_telegram_poll_command` |

已经推送成功 commit1 
我们根据这个视野设计。 开始制定函数 你认为都需要哪些函数？ 把这些函数 一一列出来。

## 视野设计 详细
 

两个配置 Cool配置文件
一个位于cmd调用 codex的文件夹 名为 .coolconfig.toml
另一个位于 启动bat相同文件夹 名为 .coolsystemconfig.toml

CTool不能修改  .coolconfig.toml 和 .coolsystemconfig.toml 直接在视野判定函数里 写死

两个配置 Cool配置文件 中 都包含ctool_scope 配置视野分段

```
[ctool_scope] 
visible_paths = [] 
hide_paths = [] 
protected_paths = []
```


当下的权限设计
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


System.filehide>System.fileReadOnly>System.filereadwrite >
System.folderhide>System.folderReadOnly>System.folderreadwrite >
LaunchDir.filehide>LaunchDir.fileReadOnly>LaunchDir.filereadwrite >
LaunchDir.folderhide>LaunchDir.folderReadOnly>LaunchDir.folderreadwrite 

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







### Codex 自带命令参考

我现在能用的工具主要是： - 
shell：在当前工作区运行命令，比如 rg、git、just、cargo、pwsh 等。 
apply_patch：按补丁方式修改文件。 

update_plan：维护任务计划和进度。
view_image：查看本地图片。 -

multi_tool_use.parallel：并行运行多个读取/搜索类工具调用，适合同时看多个文件或命令输出。


### CTool 文件夹设计结构

```
codex-rs/   //Codex自带
  utils/    //Codex自带
    ctool/
      Cargo.toml
      src/
        lib.rs

        scope.rs
        context.rs
        error.rs
        gate.rs

        tool.rs
        registry.rs

        tools/
          mod.rs

          read/
            mod.rs
            ctool_list_directory.rs
            ctool_rg_search.rs
            ctool_read_code_range.rs
            ctool_read_file.rs

          edit/
            mod.rs
            ctool_edit_replace.rs
            ctool_edit_insert.rs
            ctool_edit_batch.rs
            ctool_preview_diff.rs

          file_ops/
            mod.rs
            ctool_create_file.rs
            ctool_delete_file.rs
            ctool_move_file.rs

          tg/
            mod.rs
            ctool_telegram_send_message.rs
            ctool_telegram_poll_command.rs
            
```


# 制作规划



## 第一阶段

是这样子的 我们先做这个 CToolScope 枚举

包含
None
Workspace
SelectedOnly
TheEyeofProvidence

前期做 CTool 的时候，只考虑 workspace 这一种情况。 
其他三种我们先暂时不考虑。
然后我们第一步是把这个枚举给创建出来。 
然后保证在启动 codex 的时候，
显示第一行是 SafeMode 状态
显示第二行是 PermissionProfile 状态
显示第三行是 CToolScopeBase 状态

## 第二阶段

 permission profile 添加 CoolReadWrite 
这个模式，先不给任何工具，它不能调用任何Codex工具。 
并且让CoolReadWrite  作为默认的 permission profile 。



## 第三步
陆续制作CTool 一个一个送给 
CoolReadWrite