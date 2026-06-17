Task 0007：修复 CToolScopeConfig 缺失 privileged 字段

根目录：  
C:\Arsenal\CoolAI\CoolCodex

只允许修改 1 个文件：  
codex-rs\utils\ctool\src\scope_config.rs

禁止修改其他文件。  
禁止运行 cargo、just、git、npm、pnpm、powershell、cmd、bat、sh。  
禁止构建。  
禁止测试。  
禁止格式化整个工程。  
禁止读取 Task.md 多次。  
禁止全量读取源码文件。

本任务只做 1 次机械替换。  
不要分析。  
不要检查其他文件。  
不要搜索其他关键词。  
不要使用 grep_search。  
不要使用 glob_search。  
不要读取第二次 Task.md。

目标文件：  
codex-rs\utils\ctool\src\scope_config.rs

搜索词：  
pub struct CToolScopeConfig

读取命中点前后 20 行。

把下面这段旧代码：

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]  
pub struct CToolScopeConfig {  
#[serde(default)]  
pub files: CToolScopeRuleSet,

```
#[serde(default)]
pub folders: CToolScopeRuleSet,
```

}

替换为下面这段新代码：

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

如果旧代码没有完全命中，立刻停止本任务，并把失败原因写入 TaskLog.md。  
不要换关键词。  
不要扩大读取范围。  
不要修改其他位置。

完成后，把下面内容写入：  
C:\Arsenal\CoolAI\CoolCodex\CoolCodex_MechaForge\TaskLog.md

# Task 0007 Result

Changed:

- codex-rs\utils\ctool\src\scope_config.rs
    

Notes:

- 只修复 CToolScopeConfig 缺失 privileged_files / privileged_folders
    
- 未运行构建
    
- 未运行测试
    
- 未执行 git
    
- 未操作 Launcher 文件夹