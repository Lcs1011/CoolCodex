Task 0001-Fix-A：只修 scope.rs

根目录：  
C:\Arsenal\CoolAI\CoolCodex

只允许修改 1 个文件：  
codex-rs\utils\ctool\src\scope.rs

禁止修改其他文件。  
禁止运行 cargo、just、git、npm、pnpm、powershell、cmd、bat、sh。  
禁止构建。  
禁止测试。  
禁止格式化整个工程。  
禁止读取 Task.md 多次。  
禁止全量读取其他源码文件。

本任务目标：  
把 CToolScopeBase 修成权威口径。

权威口径：  
枚举顺序必须是：  
None  
SelectedOnly  
CoolWorkspace  
TheEyeOfProvidence

as_str 输出必须是：  
None -> none  
SelectedOnly -> selected-only  
CoolWorkspace -> cool-workspace  
TheEyeOfProvidence -> the-eye-of-providence

Default 必须是：  
CToolScopeBase::None

Display 必须写出 as_str。

执行规则：

1. 只搜索一次。
    
2. 只读取一次命中附近内容。
    
3. 命中后立刻 edit_file。
    
4. 如果没命中，立刻放弃并记录失败。
    
5. 不允许换关键词重试。
    
6. 不允许扩大读取范围。
    
7. 不允许读取其他源码文件。
    

目标文件：  
codex-rs\utils\ctool\src\scope.rs

搜索词：  
pub enum CToolScopeBase

如果文件内容已经完全等于下面内容，则跳过：

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]  
pub enum CToolScopeBase {  
None,  
SelectedOnly,  
CoolWorkspace,  
TheEyeOfProvidence,  
}

impl CToolScopeBase {  
pub fn as_str(self) -> &'static str {  
match self {  
CToolScopeBase::None => "none",  
CToolScopeBase::SelectedOnly => "selected-only",  
CToolScopeBase::CoolWorkspace => "cool-workspace",  
CToolScopeBase::TheEyeOfProvidence => "the-eye-of-providence",  
}  
}  
}

impl Default for CToolScopeBase {  
fn default() -> Self {  
CToolScopeBase::None  
}  
}

impl fmt::Display for CToolScopeBase {  
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {  
f.write_str(self.as_str())  
}  
}

如果不完全等于，则把 codex-rs\utils\ctool\src\scope.rs 的全部内容替换为上面这段内容。

完成后，在这个文件追加记录：  
C:\Arsenal\CoolAI\CoolCodex\CoolCodex_MechaForge\TempLog.md

记录内容：

# Task 0001-Fix-A Result

Changed:

- codex-rs\utils\ctool\src\scope.rs
    

Skipped:

- 如果已经符合要求，在这里写 skipped
    

Failed:

- 如果失败，写失败原因
    

Notes:

- 未运行构建
    
- 未运行测试
    
- 未执行 git
    
- 未操作 Launcher 文件夹