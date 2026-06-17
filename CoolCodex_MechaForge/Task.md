# Task 0001：修复启动门禁与基础枚举口径

## 根目录

只操作这个工程：

```text
C:\Arsenal\CoolAI\CoolCodex
```

禁止操作 Launcher 文件夹。  
禁止修改文档。  
禁止运行任何构建、测试、格式化、提交、推送类命令。  
禁止执行 cargo、just、git、npm、pnpm、powershell、cmd、bat、sh。  
只允许用源码搜索、局部读取、精确编辑完成任务。

## 权威口径

本次只实现以下源码口径：

### 1. CToolScopeBase 标准顺序

Rust 枚举顺序必须统一为：

```rust
None,
SelectedOnly,
CoolWorkspace,
TheEyeOfProvidence,
```

配置字符串标准写法必须统一为：

```text
none
selected-only
cool-workspace
the-eye-of-providence
```

缺配置时默认值：

```text
none
```

旧写法可以兼容解析，但保存、显示、as_str 必须输出标准写法。

### 2. SafeMode 启动口径

SafeMode 默认开启。

命令行参数：

```text
--safe-mode on
--safe-mode off
```

Launcher 环境变量：

```text
COOL_SAFE_MODE=on
COOL_SAFE_MODE=off
```

如果存在 `COOL_SAFE_MODE`，源码启动时以它为准覆盖 CLI 默认值。

SafeMode 必须在 CLI 解析之后、其他逻辑之前尽早初始化，并且只初始化一次。

### 3. BAT 启动门禁

Windows 下只允许通过 LauncherBat 启动。

源码启动时检查：

```text
COOL_LAUNCHED_BY_BAT=1
```

如果 Windows 下没有这个环境变量，或者值不是 `1`，直接报错退出，提示用户必须使用 LauncherBat 启动 CoolCodex。

非 Windows 平台暂时不强制。

### 4. CommandPolicy 枚举先只补口径，不实现完整 command 规则

本次只补基础枚举，不做 command.toml 合并逻辑。

CommandPolicy 枚举顺序：

```rust
Green,
Yellow,
Red,
Blocked,
BlockAll,
```

配置字符串标准写法：

```text
green
yellow
red
block
block-all
```

缺配置默认：

```text
block-all
```

如果现有代码还没有 `CToolCommandPolicy`，就在 command 相关源码中新增。  
如果已经存在，就只校正顺序、默认值、字符串。

---

## 修改 1：修正 CToolScopeBase 枚举

目标文件优先：

```text
codex-rs\utils\ctool\src\scope.rs
```

搜索一次：

```text
pub enum CToolScopeBase
```

读取命中点前后 20 行。

如果已经完全符合下面口径，则跳过本文件。

需要达到的效果：

```rust
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
```

如果原文件还有其他内容，不要误删，只替换 `CToolScopeBase` 枚举和它的 `as_str` / `Default` / `Display` 实现。

---

## 修改 2：修正 config.toml 中 CToolScopeBase 的解析顺序

目标文件：

```text
codex-rs\utils\ctool\src\scope_config.rs
```

搜索一次：

```text
fn parse_scope_base
```

读取命中点前后 20 行。

如果已经完全符合下面口径，则跳过本文件。

将 `parse_scope_base` 调整为以下语义：

```rust
fn parse_scope_base(value: &str) -> CToolResult<CToolScopeBase> {
    match value.to_ascii_lowercase().as_str() {
        "none" => Ok(CToolScopeBase::None),

        "selected-only" | "selectedonly" | "selected_only" => {
            Ok(CToolScopeBase::SelectedOnly)
        }

        "cool-workspace" | "coolworkspace" | "cool_workspace" | "workspace" => {
            Ok(CToolScopeBase::CoolWorkspace)
        }

        "the-eye-of-providence"
        | "theeyeofprovidence"
        | "the_eye_of_providence" => Ok(CToolScopeBase::TheEyeOfProvidence),

        _ => Err(CToolError::InvalidInput(format!(
            "unsupported CToolScopeBase: {value}"
        ))),
    }
}
```

注意：只改这个函数，不改其他 Scope 合并规则。

---

## 修改 3：修正 /cs base 的解析与保存口径

目标文件：

```text
codex-rs\utils\ctool\src\scope_command.rs
```

搜索一次：

```text
fn parse_base_scope
```

读取命中点前后 20 行。

如果已经完全符合下面口径，则跳过本文件。

将 `parse_base_scope` 调整为和 `scope_config.rs` 一致：

```rust
fn parse_base_scope(value: &str) -> CToolResult<CToolScopeBase> {
    match value.to_ascii_lowercase().as_str() {
        "none" => Ok(CToolScopeBase::None),

        "selected-only" | "selectedonly" | "selected_only" => {
            Ok(CToolScopeBase::SelectedOnly)
        }

        "cool-workspace" | "coolworkspace" | "cool_workspace" | "workspace" => {
            Ok(CToolScopeBase::CoolWorkspace)
        }

        "the-eye-of-providence"
        | "theeyeofprovidence"
        | "the_eye_of_providence" => Ok(CToolScopeBase::TheEyeOfProvidence),

        _ => Err(CToolError::InvalidInput(format!(
            "unsupported CToolScopeBase: {value}"
        ))),
    }
}
```

确认 `save_character_base_scope` 保存时使用的是：

```rust
base_scope.as_str().to_string()
```

如果已经是这样，不要改。

---

## 修改 4：确认 SafeMode CLI 参数默认 On

目标文件：

```text
codex-rs\tui\src\cli.rs
```

搜索一次：

```text
pub enum SafeModeCliArg
```

读取命中点前后 20 行。

确认以下口径：

```rust
#[derive(clap::ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum SafeModeCliArg {
    On,
    Off,
}
```

并确认 `safe_mode` 参数默认值是：

```rust
#[arg(long = "safe-mode", value_enum, default_value = "on")]
```

如果已经符合，只记录跳过。  
如果不符合，只做最小替换。

---

## 修改 5：在 cli main 启动早期读取 COOL_SAFE_MODE，并检查 BAT 启动

目标文件：

```text
codex-rs\cli\src\main.rs
```

搜索一次：

```text
async fn cli_main
```

读取命中点前后 20 行。

目标：在 `MultitoolCli::parse()` 之后、`safe_mode::init(...)` 之前，加入最小启动门禁逻辑。

需要实现两个 helper 函数，放在 `cli_main` 附近即可：

```rust
#[cfg(windows)]
fn ensure_cool_launched_by_bat() -> anyhow::Result<()> {
    match std::env::var("COOL_LAUNCHED_BY_BAT") {
        Ok(value) if value.trim() == "1" => Ok(()),
        _ => anyhow::bail!(
            "CoolCodex must be started by LauncherBat. Please use RunCodex.bat."
        ),
    }
}

#[cfg(not(windows))]
fn ensure_cool_launched_by_bat() -> anyhow::Result<()> {
    Ok(())
}

fn read_cool_safe_mode_env() -> anyhow::Result<Option<bool>> {
    let Ok(value) = std::env::var("COOL_SAFE_MODE") else {
        return Ok(None);
    };

    let normalized = value.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "" => Ok(None),
        "on" => Ok(Some(true)),
        "off" => Ok(Some(false)),
        _ => anyhow::bail!("unsupported COOL_SAFE_MODE value: {value}"),
    }
}
```

然后在 `cli_main` 中，`MultitoolCli::parse()` 完成后，立刻执行：

```rust
ensure_cool_launched_by_bat()?;

if let Some(env_safe_mode) = read_cool_safe_mode_env()? {
    interactive.safe_mode = if env_safe_mode {
        SafeModeCliArg::On
    } else {
        SafeModeCliArg::Off
    };
}
```

确保这段逻辑发生在：

```rust
let root_safe_mode = interactive.safe_mode.is_on();
safe_mode::init(root_safe_mode);
```

之前。

如果文件里已经有等价逻辑，不要重复添加。

---

## 修改 6：补 CToolCommandPolicy 基础枚举

目标文件优先：

```text
codex-rs\utils\ctool\src\command_request.rs
```

搜索一次：

```text
CToolCommandPolicy
```

如果没找到，再搜索一次：

```text
pub enum CToolCommandRisk
```

读取命中点前后 20 行。

如果已经有 `CToolCommandPolicy` 且符合下面口径，则跳过。

如果没有，就在 `CToolCommandRisk` 附近新增：

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CToolCommandPolicy {
    Green,
    Yellow,
    Red,
    Blocked,
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
```

本次不要把它接入 command.toml 解析和命令判定逻辑。  
完整 CommandPolicy 合并和权重规则留到 Task 0003。

---

## 完成记录

完成后在：

```text
C:\Arsenal\CoolAI\CoolCodex\CoolCodex_MechaForge\TempLog.md
```

追加记录：

```text
# Task 0001 Result

Changed:
- 列出实际修改过的文件

Skipped:
- 列出已经符合要求而跳过的文件

Failed:
- 列出没找到、读取不匹配、或不敢修改的文件和原因

Notes:
- 未运行构建
- 未运行测试
- 未执行 git
- 未操作 Launcher 文件夹
```

每个文件修改完或放弃后，立即进入下一个文件。  
严禁全量读取。  
严禁扩大搜索。  
严禁反复换关键词。  
如果一次搜索和一次局部读取无法确认，就放弃该文件并记录失败。