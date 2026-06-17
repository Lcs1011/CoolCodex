``````CoolOperation
# OperationID 0008 修复ScopePathAccess权重

## ModifyFile
### Path C:\Arsenal\CoolAI\CoolCodex\codex-rs\utils\ctool\src\scope_context.rs

### DeleteLine
306-372

### InsertContent
305
#### Content
`````
fn path_access(ctx: &CToolScopeContext, path: impl AsRef<Path>) -> PathAccess {
    let path = lexical_normalize_path(path.as_ref());

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
}

fn path_access_from_rule_sets(
    path: &Path,
    file_rules: &CToolScopeRuleSet,
    folder_rules: &CToolScopeRuleSet,
) -> Option<PathAccess> {
    if matches_any_exact_path(path, &file_rules.hidden) {
        return Some(PathAccess::Hidden);
    }

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
}
`````

## End
``````