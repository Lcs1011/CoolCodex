Task 0006：补齐 CommandPolicy 接入，并修复 ScopeConfig privileged 字段

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

本任务只做机械替换。  
不要分析。  
不要优化。  
不要改测试。  
不要改 Launcher。  
不要改文档。

# ============================================================  
文件 1：  
codex-rs\utils\ctool\src\scope_config.rs

位置：  
大约第 35 行附近。

搜索词：  
pub struct CToolScopeConfig

把这个结构体：

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]  
pub struct CToolScopeConfig {  
#[serde(default)]  
pub files: CToolScopeRuleSet,

```
#[serde(default)]
pub folders: CToolScopeRuleSet,
```

}

替换为：

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
不要修改其他位置。

# ============================================================  
文件 2：  
codex-rs\utils\ctool\src\command_request.rs

本文件做 6 个机械替换。

---

## 修改 2.1：补齐 CToolCommandPolicy impl

位置：  
文件开头附近，大约第 17 行附近。

搜索词：  
pub enum CToolCommandPolicy

把这一段：

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]  
pub enum CToolCommandPolicy {  
#[serde(rename = "green")]  
Green,

```
#[serde(rename = "yellow")]
Yellow,

#[serde(rename = "red")]
Red,

#[serde(rename = "block", alias = "blocked")]
Blocked,

#[serde(rename = "block-all", alias = "block_all", alias = "blockall")]
BlockAll,
```

}

替换为：

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]  
pub enum CToolCommandPolicy {  
#[serde(rename = "green")]  
Green,

```
#[serde(rename = "yellow")]
Yellow,

#[serde(rename = "red")]
Red,

#[serde(rename = "block", alias = "blocked")]
Blocked,

#[serde(rename = "block-all", alias = "block_all", alias = "blockall")]
BlockAll,
```

}

impl CToolCommandPolicy {  
pub fn as_str(self) -> &'static str {  
match self {  
CToolCommandPolicy::Green => "green",  
CToolCommandPolicy::Yellow => "yellow",  
CToolCommandPolicy::Red => "red",  
CToolCommandPolicy::Blocked => "block",  
CToolCommandPolicy::BlockAll => "block-all",  
}  
}  
}

impl Default for CToolCommandPolicy {  
fn default() -> Self {  
CToolCommandPolicy::BlockAll  
}  
}

---

## 修改 2.2：CToolCommandConfig 增加 policy 字段

位置：  
大约第 70 行附近。

搜索词：  
pub struct CToolCommandConfig

把这一段：

pub struct CToolCommandConfig {  
#[serde(default = "default_true")]  
pub enabled: bool,

替换为：

pub struct CToolCommandConfig {  
#[serde(default)]  
pub policy: CToolCommandPolicy,

```
#[serde(default = "default_true")]
pub enabled: bool,
```

---

## 修改 2.3：Default 初始化 policy

位置：  
大约第 105 行附近。

搜索词：  
impl Default for CToolCommandConfig

把这一段：

Self {  
enabled: true,

替换为：

Self {  
policy: CToolCommandPolicy::BlockAll,  
enabled: true,

---

## 修改 2.4：merge_command_configs 合并 policy

位置：  
大约第 280 行附近。

搜索词：  
pub fn merge_command_configs

把这一段：

let mut merged = CToolCommandConfig {  
enabled: character_config.enabled && system_config.enabled,

替换为：

let mut merged = CToolCommandConfig {  
policy: std::cmp::max(system_config.policy, character_config.policy),  
enabled: character_config.enabled && system_config.enabled,

---

## 修改 2.5：classify_command 增加 BlockAll 硬阻断

位置：  
大约第 360 行附近。

搜索词：  
pub fn classify_command

把这一段：

let raw_command = command.as_ref().trim().to_string();

if raw_command.is_empty() {

替换为：

let raw_command = command.as_ref().trim().to_string();

if config.policy == CToolCommandPolicy::BlockAll {  
return CToolCommandClassification {  
command: raw_command,  
risk: CToolCommandRisk::Blocked,  
reason: "CToolCommandPolicy is block-all".to_string(),  
};  
}

if raw_command.is_empty() {

---

## 修改 2.6：未知命令按 policy 兜底

位置：  
大约第 440 行附近。

搜索词：  
unknown command defaults to red

把 classify_command_segment 末尾这一段：

CToolCommandClassification {  
command: raw_command,  
risk: CToolCommandRisk::Red,  
reason: "unknown command defaults to red".to_string(),  
}

替换为：

let policy_risk = risk_from_command_policy(config.policy);  
CToolCommandClassification {  
command: raw_command,  
risk: policy_risk,  
reason: format!("unknown command defaults to policy: {}", config.policy.as_str()),  
}

然后在 classify_command_segment 函数结束后、build_command_request_preview 函数之前，插入这个函数：

fn risk_from_command_policy(policy: CToolCommandPolicy) -> CToolCommandRisk {  
match policy {  
CToolCommandPolicy::Green => CToolCommandRisk::Green,  
CToolCommandPolicy::Yellow => CToolCommandRisk::Yellow,  
CToolCommandPolicy::Red => CToolCommandRisk::Red,  
CToolCommandPolicy::Blocked | CToolCommandPolicy::BlockAll => {  
CToolCommandRisk::Blocked  
}  
}  
}

# ============================================================  
文件 3：  
codex-rs\utils\ctool\src\scope_context.rs

本文件只替换 path_access 区域。

位置：  
大约第 490 行附近。

搜索词：  
fn path_access

把从：

fn path_access(ctx: &CToolScopeContext, path: impl AsRef) -> PathAccess {

开始，到：

pub fn ensure_read_allowed_by_scope(

之前的全部内容替换为下面内容。

注意：  
替换后必须保留 pub fn ensure_read_allowed_by_scope( 这一行。  
不要删除 ensure_read_allowed_by_scope 函数体。

新内容：

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

pub fn ensure_read_allowed_by_scope(

# ============================================================  
完成记录

完成后，把下面内容写入：  
C:\Arsenal\CoolAI\CoolCodex\CoolCodex_MechaForge\TaskLog.md

# Task 0006 Result

Changed:

- codex-rs\utils\ctool\src\scope_config.rs
    
- codex-rs\utils\ctool\src\command_request.rs
    
- codex-rs\utils\ctool\src\scope_context.rs
    

Notes:

- CToolScopeConfig 增加 privileged_files / privileged_folders
    
- CToolCommandPolicy 增加 as_str / Default
    
- CToolCommandConfig 增加 policy 字段
    
- merge_command_configs 合并 policy
    
- classify_command 增加 BlockAll 硬阻断
    
- 未知命令改为按 policy 兜底
    
- path_access 增加 None 绝对无视野
    
- path_access 增加 privileged 规则优先级
    
- 未运行构建
    
- 未运行测试
    
- 未执行 git
    
- 未操作 Launcher 文件夹