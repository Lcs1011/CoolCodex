# TaskLog — 0001：修复启动门禁与基础枚举口径

## 修改 1：`scope.rs` — CToolScopeBase 枚举顺序
- **状态**：✅ 完成（无需修改）
- **说明**：`CToolScopeBase` 枚举顺序已为 `None, SelectedOnly, CoolWorkspace, TheEyeOfProvidence`，与权威口径一致。`as_str()` 在 `TheEyeOfProvidence` 上返回 `"the-eye-of-providence"`。

## 修改 2：`scope_config.rs` — `parse_scope_base`
- **状态**：✅ 完成（无需修改）
- **说明**：`parse_scope_base` 已处理 `"the-eye-of-providence"` 并正确映射到 `CToolScopeBase::TheEyeOfProvidence`。

## 修改 3：`scope_command.rs` — `parse_base_scope`
- **状态**：✅ 完成（无需修改）
- **说明**：`parse_base_scope` 中的 `"TheEyeOfProvidence"` → `CToolScopeBase::TheEyeOfProvidence` 映射已存在。

## 修改 4：`tui/src/cli.rs` — `SafeModeCliArg` 添加 `Ask` 变体
- **状态**：✅ 完成
- **说明**：在 `SafeModeCliArg` 枚举中成功添加了 `Ask` 变体。

## 修改 5：`cli/src/main.rs` — BAT 门禁 + safe_mode 环境变量覆盖
- **状态**：✅ 完成
- **说明**：添加了 `ensure_cool_launched_by_bat()` 和 `read_cool_safe_mode_env()` 两个 helper 函数，在 `cli_main` 中 `parse()` 之后插入了 BAT 门禁检查及环境变量覆盖 safe_mode 的逻辑。

## 修改 6：`command_request.rs` — 添加 `CToolCommandPolicy` 枚举
- **状态**：✅ 完成
- **说明**：在 `CToolCommandRisk` 之前添加了 `CToolCommandPolicy` 枚举，包含 `Safe` 和 `Ask` 两个变体，使用 `#[serde(rename_all = "snake_case")]`。
