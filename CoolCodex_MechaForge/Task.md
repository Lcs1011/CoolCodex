Task 0001-Fix-C：接入 CToolCommandPolicy 到 command_request.rs

根目录：  
C:\Arsenal\CoolAI\CoolCodex

只允许修改 1 个文件：  
codex-rs\utils\ctool\src\command_request.rs

禁止修改其他文件。  
禁止运行 cargo、just、git、npm、pnpm、powershell、cmd、bat、sh。  
禁止构建。  
禁止测试。  
禁止格式化整个工程。  
禁止读取 Task.md 多次。  
禁止全量读取源码文件。

目标：  
把 CToolCommandPolicy 真正接入 CToolCommandConfig 和命令分类逻辑。

目标文件：  
codex-rs\utils\ctool\src\command_request.rs

文件当前大约 1028 行。  
本任务修改 4 个位置。

# ============================================================  
修改 1：补齐 CToolCommandPolicy impl

位置：  
command_request.rs 文件开头附近，大约第 17 行。  
搜索词：  
pub enum CToolCommandPolicy

找到这个 enum 块后，把从：

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]  
pub enum CToolCommandPolicy {

一直到它后面的第一个单独的：

}

替换为下面完整代码：

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]  
pub enum CToolCommandPolicy {  
#[serde(rename = "green")]  
Green,  
#[serde(rename = "yellow")]  
Yellow,  
#[serde(rename = "red")]  
Red,  
#[serde(rename = "block", alias = "blocked")]  
Blocked,  
#[serde(rename = "block-all", alias = "block_all", alias = "blockall")]  
BlockAll,  
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

# ============================================================  
修改 2：给 CToolCommandConfig 增加 policy 字段

位置：  
大约第 70 行附近。  
搜索词：  
pub struct CToolCommandConfig

在结构体里这一段：

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

# ============================================================  
修改 3：给 Default 初始化 policy

位置：  
大约第 105 行附近。  
搜索词：  
impl Default for CToolCommandConfig

在 Default 里面这一段：

Self {  
enabled: true,

替换为：

Self {  
policy: CToolCommandPolicy::BlockAll,  
enabled: true,

# ============================================================  
修改 4：merge_command_configs 合并 policy

位置：  
大约第 280 行附近。  
搜索词：  
pub fn merge_command_configs

在 merge_command_configs 里面这一段：

let mut merged = CToolCommandConfig {  
enabled: character_config.enabled && system_config.enabled,  
green_exact_commands: Vec::new(),

替换为：

let mut merged = CToolCommandConfig {  
policy: std::cmp::max(system_config.policy, character_config.policy),  
enabled: character_config.enabled && system_config.enabled,  
green_exact_commands: Vec::new(),

# ============================================================  
修改 5：CommandPolicy::BlockAll 阻断全部命令

位置：  
大约第 360 行附近。  
搜索词：  
pub fn classify_command

在 classify_command 里面这一段：

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

# ============================================================  
修改 6：未知命令按 policy 兜底

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

然后在 classify_command_segment 函数结束后、build_command_request_preview 函数之前，插入下面这个新函数：

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
完成记录

完成后，把下面内容写入：  
C:\Arsenal\CoolAI\CoolCodex\CoolCodex_MechaForge\TaskLog.md

# Task 0001-Fix-C Result

Changed:

- codex-rs\utils\ctool\src\command_request.rs
    

Changed points:

- CToolCommandPolicy impl Default/as_str
    
- CToolCommandConfig.policy
    
- CToolCommandConfig Default policy
    
- merge_command_configs policy merge
    
- classify_command BlockAll hard block
    
- unknown command fallback uses policy
    

Notes:

- 未运行构建
    
- 未运行测试
    
- 未执行 git
    
- 未操作 Launcher 文件夹