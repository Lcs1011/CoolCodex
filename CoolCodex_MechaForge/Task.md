Task 0002-A：修正 CommandPolicy 映射，并接入 Scope privileged 基础规则

根目录：  
C:\Arsenal\CoolAI\CoolCodex

只允许修改 3 个文件：  
codex-rs\utils\ctool\src\command_request.rs  
codex-rs\utils\ctool\src\scope_config.rs  
codex-rs\utils\ctool\src\scope_context.rs

禁止修改其他文件。  
禁止运行 cargo、just、git、npm、pnpm、powershell、cmd、bat、sh。  
禁止构建。  
禁止测试。  
禁止格式化整个工程。  
禁止读取 Task.md 多次。  
禁止全量读取源码文件。

注意：  
当前 GitHub 上这些 Rust 文件几乎是单行，所以不要依赖真实行号。  
按下面给出的搜索词定位，定位后直接替换指定代码块。

# ============================================================  
修改 1：修正 CommandPolicy 到 Risk 的映射

目标文件：  
codex-rs\utils\ctool\src\command_request.rs

搜索词：  
fn risk_from_command_policy

定位到这个函数后，把整个函数替换为：

fn risk_from_command_policy(policy: &CToolCommandPolicy) -> CToolCommandRisk {  
match policy {  
CToolCommandPolicy::Green => CToolCommandRisk::Green,  
CToolCommandPolicy::Yellow => CToolCommandRisk::Yellow,  
CToolCommandPolicy::Red => CToolCommandRisk::Red,  
CToolCommandPolicy::Blocked | CToolCommandPolicy::BlockAll => {  
CToolCommandRisk::Blocked  
}  
}  
}

只替换这个函数。  
不要修改 command_request.rs 的其他位置。

# ============================================================  
修改 2：给 CToolScopeConfig 增加 privileged_files / privileged_folders

目标文件：  
codex-rs\utils\ctool\src\scope_config.rs

搜索词：  
pub struct CToolScopeConfig

找到这个结构体后，把整个结构体替换为：

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]  
pub struct CToolScopeConfig {  
#[serde(default)]  
pub privileged_files: CToolScopeRuleSet,

```
#[serde(default)]
pub privileged_folders: CToolScopeRuleSet,

#[serde(default)]
pub files: CToolScopeRuleSet,

#[serde(default)]
pub folders: CToolScopeRuleSet,
```

}

不要修改 CToolScopeRuleSet。  
不要修改 parse_scope_base。  
不要修改 command 配置解析。

# ============================================================  
修改 3：normalize_scope_config 同步 privileged 字段

目标文件：  
codex-rs\utils\ctool\src\scope_context.rs

搜索词：  
pub fn normalize_scope_config

找到这个函数后，把整个函数替换为：

pub fn normalize_scope_config(config: CToolScopeConfig, root: &Path) -> CToolScopeConfig {  
CToolScopeConfig {  
privileged_files: normalize_rule_set(config.privileged_files, root),  
privileged_folders: normalize_rule_set(config.privileged_folders, root),  
files: normalize_rule_set(config.files, root),  
folders: normalize_rule_set(config.folders, root),  
}  
}

只替换这个函数。  
不要修改 normalize_rule_set。  
不要修改 normalize_scope_paths。

# ============================================================  
修改 4：替换 path_access，实现 None 绝对无视野 + privileged 优先级

目标文件：  
codex-rs\utils\ctool\src\scope_context.rs

搜索词：  
fn path_access

把从：

fn path_access(ctx: &CToolScopeContext, path: impl AsRef) -> PathAccess {

开始，到：

pub fn ensure_read_allowed_by_scope

之前的全部内容替换为下面这段：

fn path_access(ctx: &CToolScopeContext, path: impl AsRef) -> PathAccess {  
let path = lexical_normalize_path(path.as_ref());

```
if ctx.base_scope == CToolScopeBase::None {
    return PathAccess::Unspecified;
}

if is_web_search_cache_path(ctx, &path) {
    return PathAccess::Readonly;
}

if is_hard_protected_config_path(ctx, &path) {
    return PathAccess::Hidden;
}

if let Some(access) = path_access_from_rule_sets(
    &path,
    &ctx.system_config.privileged_files,
    &ctx.system_config.privileged_folders,
) {
    return access;
}

if let Some(access) = path_access_from_rule_sets(
    &path,
    &ctx.user_config.privileged_files,
    &ctx.user_config.privileged_folders,
) {
    return access;
}

if let Some(access) =
    path_access_from_rule_sets(&path, &ctx.system_config.files, &ctx.system_config.folders)
{
    return access;
}

if let Some(access) =
    path_access_from_rule_sets(&path, &ctx.user_config.files, &ctx.user_config.folders)
{
    return access;
}

if is_visible_by_base_scope(ctx, &path) {
    return PathAccess::Readwrite;
}

PathAccess::Unspecified
```

}

fn path_access_from_rule_sets(  
path: &Path,  
file_rules: &CToolScopeRuleSet,  
folder_rules: &CToolScopeRuleSet,  
) -> Option {  
if matches_any_exact_path(path, &file_rules.hidden) {  
return Some(PathAccess::Hidden);  
}

```
if matches_any_exact_path(path, &file_rules.readonly) {
    return Some(PathAccess::Readonly);
}

if matches_any_exact_path(path, &file_rules.readwrite) {
    return Some(PathAccess::Readwrite);
}

if matches_any_path(path, &folder_rules.hidden) {
    return Some(PathAccess::Hidden);
}

if matches_any_path(path, &folder_rules.readonly) {
    return Some(PathAccess::Readonly);
}

if matches_any_path(path, &folder_rules.readwrite) {
    return Some(PathAccess::Readwrite);
}

None
```

}

pub fn ensure_read_allowed_by_scope

注意：  
替换时必须保留最后这一行：  
pub fn ensure_read_allowed_by_scope

也就是说，新内容最后一行就是函数声明开头，不要删掉它后面的函数体。

# ============================================================  
完成记录

完成后，把下面内容写入：  
C:\Arsenal\CoolAI\CoolCodex\CoolCodex_MechaForge\TaskLog.md

# Task 0002-A Result

Changed:

- codex-rs\utils\ctool\src\command_request.rs
    
- codex-rs\utils\ctool\src\scope_config.rs
    
- codex-rs\utils\ctool\src\scope_context.rs
    

Changed points:

- 修正 risk_from_command_policy：Blocked / BlockAll 都映射为 Blocked
    
- CToolScopeConfig 增加 privileged_files
    
- CToolScopeConfig 增加 privileged_folders
    
- normalize_scope_config 同步 privileged 字段
    
- path_access 实现 CToolScopeBase::None 绝对无视野
    
- path_access 实现 privileged 优先于普通 System / Character 规则
    

Notes:

- 未运行构建
    
- 未运行测试
    
- 未执行 git
    
- 未操作 Launcher 文件夹