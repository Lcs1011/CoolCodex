# TaskLog — 0001-Fix-B：替换 CToolCommandPolicy 枚举

## command_request.rs — CToolCommandPolicy 枚举替换
- **状态**：✅ 完成
- **说明**：将旧的 `Safe`/`Ask` 枚举替换为权威口径的 `Green`/`Yellow`/`Red`/`Blocked`/`BlockAll` 枚举，每个变体使用 `#[serde(rename = "...")]` 属性，`Blocked` 和 `BlockAll` 额外带有 `alias` 属性。
