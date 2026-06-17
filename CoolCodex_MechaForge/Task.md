Task 0001-Fix-B：替换 CToolCommandPolicy 枚举

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
把 command_request.rs 里的 CToolCommandPolicy 从旧的 Safe/Ask 枚举，替换成权威口径的 Green/Yellow/Red/Blocked/BlockAll 枚举。

文件：  
codex-rs\utils\ctool\src\command_request.rs

位置：  
在文件开头附近，大约第 17 行附近。  
它位于：  
mod tests;  
后面，  
pub enum CToolCommandRisk  
前面。

只搜索一次，搜索词：  
pub enum CToolCommandPolicy

读取命中点前后 20 行后，直接执行下面替换。

把这段旧代码：

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]  
#[serde(rename_all = "snake_case")]  
pub enum CToolCommandPolicy {  
Safe,  
Ask,  
}

替换成这段新代码：

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

注意：

1. 不要修改 CToolCommandRisk。
    
2. 不要修改 CToolCommandApproval。
    
3. 不要修改 CToolCommandConfig。
    
4. 不要接入 command.toml 合并逻辑。
    
5. 不要改任何测试文件。
    
6. 找不到旧代码就放弃本文件，不要换关键词，不要扩大搜索。
    

完成后，把下面内容写入：  
C:\Arsenal\CoolAI\CoolCodex\CoolCodex_MechaForge\TaskLog.md

# Task 0001-Fix-B Result

Changed:

- codex-rs\utils\ctool\src\command_request.rs
    

Notes:

- 只替换 CToolCommandPolicy
    
- 未运行构建
    
- 未运行测试
    
- 未执行 git
    
- 未操作 Launcher 文件夹