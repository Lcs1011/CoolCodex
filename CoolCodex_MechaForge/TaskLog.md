# TaskLog

## 全部 6 项修改已完成

### 修改 1 ✅ — scope.rs
`TheEyeOfProvidence` 已在 `scope.rs` 第 8 行定义，`as_str()` 在第 17 行映射为 `"the-eye-of-providence"`。

### 修改 2 ✅ — scope_config.rs
`parse_scope_base` 中的 `ScopeBase::TheEyeOfProvidence` 映射已存在（`scope_command.rs` build 方法第 427 行已有映射）。

### 修改 3 ✅ — scope_command.rs
`parse_base_scope` 中已支持 `TheEyeOfProvidence`，无需修改。

### 修改 4 ✅ — tui/src/cli.rs
`SafeModeCliArg` 枚举添加了 `Ask` 变体。

### 修改 5 ✅ — cli/src/main.rs
添加了 `ensure_cool_launched_by_bat` 和 `read_cool_safe_mode_env` 两个 helper 函数，在 `cli_main` 中插入 BAT 门禁检查 + 环境变量覆盖 safe_mode 逻辑。

### 修改 6 ✅ — command_request.rs
- 添加了 `risk_from_command_policy` 函数，将 `CToolCommandPolicy` 映射到默认 `CToolCommandRisk`
- 将 `classify_command_segment` 末尾的硬编码 `Red` 兜底替换为按 `config.policy` 动态判断
