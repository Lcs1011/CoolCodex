是这样：**你这套冲突优先级，大体“逻辑顺序”已经生效了，但现在被两个实现 bug 影响，所以实际效果不可靠。**

当前远程仓库里 `path_access()` 的顺序是：

```text
CToolScopeBase::None
↓
System privileged
↓
Character privileged
↓
web_search cache readonly
↓
hard protected config hidden
↓
System normal rules
↓
Character normal rules
↓
CToolScopeBase 非 none
```

其中 `CToolScopeBase::None` 确实最先 return，非 none 的 base scope 确实最后兜底。

普通规则内部 `path_access_from_rule_sets()` 的顺序也基本符合你写的：

```text
Files.Hidden
Files.ReadOnly
Files.ReadWrite
Folders.Hidden
Folders.ReadOnly
Folders.ReadWrite
```

源码就是这个顺序。

所以，**冲突优先级本身大体没错**。但现在有两个问题会让规则“看起来生效，实际不生效”：

1. `C:\...` 和 `\\?\C:\...` 没统一。规则路径现在只做 `lexical_normalize_path()`，没有 canonicalize；而实际读文件时路径会被 canonicalize 成 `\\?\...`，所以 Windows 下 `starts_with` 可能失败。

2. System scope 的相对路径现在仍然按 `character_root` 展开，不是按 `system scope.toml` 所在目录展开。这个会导致你截图里出现 `CoolCodexV1\LauncherDir\.cool-system` 这种错误路径。

你可以把下面这段直接发给 Codex：

请只修 CToolScope 视野算法，不要改 BAT，不要改导航规则，不要改 CharacterRoot / CoolWorkspace 语义，不要改 CommandPolicy。

用户要求的冲突优先级是：

CToolScopeBase:none >

Privileged 整体 >

System.Files.Hidden > System.Files.ReadOnly > System.Files.ReadWrite >  
System.Folders.Hidden > System.Folders.ReadOnly > System.Folders.ReadWrite >  
Character.Files.Hidden > Character.Files.ReadOnly > Character.Files.ReadWrite >  
Character.Folders.Hidden > Character.Folders.ReadOnly > Character.Folders.ReadWrite >

CToolScopeBase:非 none

当前源码里大体顺序已经接近这个规则，但还有两个 bug 必须修：

1. Windows 路径规范化 bug  
   scope.toml 规则里可能是普通绝对路径：

C:\Arsenal\CoolAI\CoolMechaForge

但工具实际读文件时会 canonicalize 成：

\?\C:\Arsenal\CoolAI\CoolMechaForge\必坑指南.md

现在规则路径只做 lexical_normalize_path，没有 canonicalize，所以 path.starts_with(rule_path) 可能失败。

要求：

- 增加一个 helper，比如：

```rust
fn normalize_existing_or_lexical(path: &Path) -> PathBuf {
    std::fs::canonicalize(path)
        .map(|path| lexical_normalize_path(&path))
        .unwrap_or_else(|_| lexical_normalize_path(path))
}
```

- 修改 resolve_config_path：
  
  - 先把相对路径 join root
  
  - 如果 resolved path 存在，就 canonicalize
  
  - 如果不存在，才 lexical_normalize
  
  - 不要让不存在的新文件路径直接报错

建议：

```rust
fn resolve_config_path(root: &Path, path: &Path) -> PathBuf {
    let resolved = if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.join(path)
    };

    normalize_existing_or_lexical(&resolved)
}
```

- 修改 path_matches_rule 和 matches_any_exact_path，比较前也使用同一套 normalize_existing_or_lexical，避免 C:\ 和 \?\C:\ 混用。
2. System scope 相对路径 root 错误  
   当前 system scope 是这样处理的：

```rust
Some(path) => normalize_scope_config(load_optional_cool_config(path)?, &character_root),
```

这是错的。System scope.toml 里的相对路径应该按 system scope.toml 所在目录展开，不应该按 CharacterRoot 展开。

要求：

- Character scope 相对路径仍然按 CharacterRoot 展开。

- System scope 相对路径按 system scope.toml 所在目录展开。

- 也就是：

```rust
let system_config = match system_scope_path.as_deref() {
    Some(path) => {
        let system_scope_root = path
            .parent()
            .map(normalize_existing_or_lexical)
            .unwrap_or_else(|| character_root.clone());

        normalize_scope_config(load_optional_cool_config(path)?, &system_scope_root)
    }
    None => empty_scope_config(),
};
```

3. 不要改变冲突优先级  
   保留 path_access 的大结构：
- CToolScopeBase::None 最先 return Unspecified

- privileged 在 normal rules 之前

- normal system rules 在 normal character rules 之前

- CToolScopeBase 非 none 最后兜底

但请注意：如果 hard protected config 继续存在，它应该保持在 privileged 之后、normal rules 之前。也就是说 privileged 仍然可以覆盖 hard protected config。

4. 加测试  
   至少加这些测试：
- CToolScopeBase::None 优先于所有规则，结果不可读不可写。

- System file hidden 优先于 system folder readwrite。

- System file readwrite 优先于 system folder hidden。

- System folder hidden 优先于 character file readwrite。

- Character file hidden 优先于 character folder readwrite。

- Character folder readwrite 最后能放行。

- Base scope cool-workspace 只在所有显式规则都没命中时兜底。

- Windows 路径测试：规则路径是普通 C:...，目标路径 canonical 后是 \?\C:...，必须能匹配。

- System scope 相对路径测试：system scope.toml 里的相对路径必须按 system scope.toml 所在目录展开，不准按 CharacterRoot 展开。
5. 只跑：  
   cargo fmt -p ctool  
   cargo test -p ctool -- --nocapture  
   cargo check -p ctool -p codex-core -p codex-cli

不要跑 release。  
不要覆盖 CoolCodexV1.exe。

我建议你让它先按这段修。修完之后你再让我看一眼远程 `scope_context.rs`，确认没把优先级改乱。
