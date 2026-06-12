use std::path::Component;
use std::path::Path;
use std::path::PathBuf;

use crate::error::CToolError;
use crate::error::CToolResult;
use crate::scope::CToolScopeBase;
use crate::scope_config::CToolScopeConfig;
use crate::scope_config::CToolScopeRuleSet;
use crate::scope_config::empty_scope_config;
use crate::scope_config::load_optional_cool_character_config;
use crate::scope_config::load_optional_cool_config;
use crate::scope_config::locate_cool_command_path;
use crate::scope_config::locate_cool_config_path;
use crate::scope_config::locate_cool_dir;
use crate::scope_config::locate_cool_scope_path;
use crate::scope_config::locate_cool_system_command_path;
use crate::scope_config::locate_cool_system_config_path;
use crate::scope_config::locate_cool_system_dir;
use crate::scope_config::locate_cool_system_dir_from_launcher;
use crate::scope_config::locate_cool_system_scope_path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CToolScopeContext {
    pub current_dir: PathBuf,
    pub character_root: PathBuf,
    pub cool_workspace: PathBuf,
    pub base_scope: CToolScopeBase,

    pub user_config_path: PathBuf,
    pub character_config_path: PathBuf,
    pub system_config_path: Option<PathBuf>,
    pub character_command_path: PathBuf,
    pub system_command_path: Option<PathBuf>,
    pub cool_system_dir: Option<PathBuf>,

    pub user_config: CToolScopeConfig,
    pub system_config: CToolScopeConfig,
}

pub fn build_ctool_scope_context(
    current_dir: impl AsRef<Path>,
    fallback_base_scope: CToolScopeBase,
    legacy_system_config_path: Option<PathBuf>,
) -> CToolResult<CToolScopeContext> {
    build_ctool_scope_context_with_launcher(
        current_dir.as_ref(),
        current_dir.as_ref(),
        fallback_base_scope,
        legacy_system_config_path,
    )
}

pub fn build_ctool_scope_context_with_launcher(
    launcher_dir: impl AsRef<Path>,
    character_root: impl AsRef<Path>,
    fallback_base_scope: CToolScopeBase,
    legacy_system_config_path: Option<PathBuf>,
) -> CToolResult<CToolScopeContext> {
    let launcher_dir = canonicalize_existing_path(launcher_dir.as_ref())?;
    let character_root = canonicalize_existing_path(character_root.as_ref())?;
    let character_config_path = locate_cool_config_path(&character_root);
    let character_scope_path = locate_cool_scope_path(&character_root);
    let character_command_path = locate_cool_command_path(&character_root);

    let character_config = load_optional_cool_character_config(&character_config_path)?;
    let base_scope = character_config.scope_base.unwrap_or(fallback_base_scope);

    let cool_workspace = match character_config.cool_workspace {
        Some(path) if path.is_absolute() => canonicalize_existing_path(path)?,
        Some(path) => canonicalize_existing_path(character_root.join(path))?,
        None => character_root.clone(),
    };

    let user_config = normalize_scope_config(
        load_optional_cool_config(&character_scope_path)?,
        &character_root,
    );

    let default_cool_system_dir = locate_cool_system_dir_from_launcher(&launcher_dir);
    let cool_system_dir = locate_cool_system_dir().or(Some(default_cool_system_dir));
    let system_config_path = locate_cool_system_config_path()
        .or(legacy_system_config_path)
        .or_else(|| cool_system_dir.as_ref().map(|dir| dir.join("config.toml")));
    let system_scope_path = locate_cool_system_scope_path()
        .or_else(|| cool_system_dir.as_ref().map(|dir| dir.join("scope.toml")));
    let system_command_path = locate_cool_system_command_path()
        .or_else(|| cool_system_dir.as_ref().map(|dir| dir.join("command.toml")));
    let system_config = match system_scope_path.as_deref() {
        Some(path) => normalize_scope_config(load_optional_cool_config(path)?, &character_root),
        None => empty_scope_config(),
    };

    Ok(CToolScopeContext {
        current_dir: character_root.clone(),
        character_root,
        cool_workspace,
        base_scope,
        user_config_path: character_scope_path,
        character_config_path,
        system_config_path,
        character_command_path,
        system_command_path,
        cool_system_dir,
        user_config,
        system_config,
    })
}

pub fn normalize_scope_config(config: CToolScopeConfig, root: &Path) -> CToolScopeConfig {
    CToolScopeConfig {
        files: normalize_rule_set(config.files, root),
        folders: normalize_rule_set(config.folders, root),
    }
}

fn normalize_rule_set(rule_set: CToolScopeRuleSet, root: &Path) -> CToolScopeRuleSet {
    CToolScopeRuleSet {
        readwrite: normalize_scope_paths(rule_set.readwrite, root),
        readonly: normalize_scope_paths(rule_set.readonly, root),
        hidden: normalize_scope_paths(rule_set.hidden, root),
    }
}

fn normalize_scope_paths(paths: Vec<PathBuf>, root: &Path) -> Vec<PathBuf> {
    paths
        .into_iter()
        .map(|path| resolve_config_path(root, &path))
        .collect()
}

fn resolve_config_path(root: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        lexical_normalize_path(path)
    } else {
        lexical_normalize_path(&root.join(path))
    }
}

pub fn resolve_user_path(ctx: &CToolScopeContext, path: impl AsRef<Path>) -> PathBuf {
    let path = path.as_ref();

    if path.is_absolute() {
        lexical_normalize_path(path)
    } else {
        lexical_normalize_path(&ctx.cool_workspace.join(path))
    }
}

pub fn canonicalize_existing_path(path: impl AsRef<Path>) -> CToolResult<PathBuf> {
    std::fs::canonicalize(path.as_ref()).map_err(|error| {
        CToolError::InvalidInput(format!(
            "failed to canonicalize existing path: {} ({error})",
            path.as_ref().display()
        ))
    })
}

pub fn canonicalize_parent_for_new_path(path: impl AsRef<Path>) -> CToolResult<PathBuf> {
    let path = path.as_ref();

    let Some(parent) = path.parent() else {
        return Err(CToolError::InvalidInput(format!(
            "path has no parent directory: {}",
            path.display()
        )));
    };

    let Some(file_name) = path.file_name() else {
        return Err(CToolError::InvalidInput(format!(
            "path has no file name: {}",
            path.display()
        )));
    };

    let parent = canonicalize_existing_path(parent)?;

    Ok(lexical_normalize_path(&parent.join(file_name)))
}

pub fn matches_any_exact_path(path: impl AsRef<Path>, paths: &[PathBuf]) -> bool {
    let path = lexical_normalize_path(path.as_ref());
    paths
        .iter()
        .any(|rule_path| path == lexical_normalize_path(rule_path))
}

pub fn path_matches_rule(path: impl AsRef<Path>, rule_path: impl AsRef<Path>) -> bool {
    let path = lexical_normalize_path(path.as_ref());
    let rule_path = lexical_normalize_path(rule_path.as_ref());

    path == rule_path || path.starts_with(&rule_path)
}

pub fn matches_any_path(path: impl AsRef<Path>, paths: &[PathBuf]) -> bool {
    paths
        .iter()
        .any(|rule_path| path_matches_rule(path.as_ref(), rule_path))
}

pub fn is_visible_by_base_scope(ctx: &CToolScopeContext, path: impl AsRef<Path>) -> bool {
    let path = lexical_normalize_path(path.as_ref());

    match ctx.base_scope {
        CToolScopeBase::None => false,
        CToolScopeBase::CoolWorkspace => path_matches_rule(path, &ctx.cool_workspace),
        CToolScopeBase::SelectedOnly => false,
        CToolScopeBase::TheEyeOfProvidence => true,
    }
}

pub fn can_read_path(ctx: &CToolScopeContext, path: impl AsRef<Path>) -> bool {
    match path_access(ctx, path) {
        PathAccess::Hidden => false,
        PathAccess::Readonly | PathAccess::Readwrite => true,
        PathAccess::Unspecified => false,
    }
}

pub fn can_search_path(ctx: &CToolScopeContext, path: impl AsRef<Path>) -> bool {
    can_read_path(ctx, path)
}

pub fn is_hard_protected_config_path(ctx: &CToolScopeContext, path: impl AsRef<Path>) -> bool {
    let path = lexical_normalize_path(path.as_ref());
    let cool_dir = lexical_normalize_path(locate_cool_dir(&ctx.character_root).as_path());

    if path_matches_rule(&path, cool_dir) && !is_web_search_cache_path(ctx, &path) {
        return true;
    }

    ctx.cool_system_dir
        .as_deref()
        .is_some_and(|dir| path_matches_rule(&path, dir))
}

pub fn is_web_search_cache_path(ctx: &CToolScopeContext, path: impl AsRef<Path>) -> bool {
    let path = lexical_normalize_path(path.as_ref());

    path_matches_rule(path, web_search_cache_root(ctx))
}

pub fn web_search_cache_root(ctx: &CToolScopeContext) -> PathBuf {
    lexical_normalize_path(
        &locate_cool_dir(&ctx.character_root)
            .join("cache")
            .join("web_search"),
    )
}

pub fn is_protected_path(ctx: &CToolScopeContext, path: impl AsRef<Path>) -> bool {
    let path = lexical_normalize_path(path.as_ref());

    is_critical_root_path(ctx, &path) || matches!(path_access(ctx, path), PathAccess::Readonly)
}

fn is_critical_root_path(ctx: &CToolScopeContext, path: impl AsRef<Path>) -> bool {
    let path = lexical_normalize_path(path.as_ref());

    path == lexical_normalize_path(&ctx.character_root)
        || path == lexical_normalize_path(&ctx.cool_workspace)
        || path == lexical_normalize_path(locate_cool_dir(&ctx.character_root).as_path())
        || ctx
            .cool_system_dir
            .as_deref()
            .is_some_and(|dir| path == lexical_normalize_path(dir))
}

pub fn can_write_path(ctx: &CToolScopeContext, path: impl AsRef<Path>) -> bool {
    matches!(path_access(ctx, path), PathAccess::Readwrite)
}

pub fn can_create_path(ctx: &CToolScopeContext, path: impl AsRef<Path>) -> bool {
    let path = lexical_normalize_path(path.as_ref());

    if is_hard_protected_config_path(ctx, &path) || is_critical_root_path(ctx, &path) {
        return false;
    }

    let Ok(path_with_canonical_parent) = canonicalize_parent_for_new_path(&path) else {
        return false;
    };

    let Some(parent) = path_with_canonical_parent.parent() else {
        return false;
    };

    can_write_path(ctx, parent)
        && !matches!(
            path_access(ctx, &path),
            PathAccess::Hidden | PathAccess::Readonly
        )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PathAccess {
    Hidden,
    Readonly,
    Readwrite,
    Unspecified,
}

fn path_access(ctx: &CToolScopeContext, path: impl AsRef<Path>) -> PathAccess {
    let path = lexical_normalize_path(path.as_ref());

    if is_web_search_cache_path(ctx, &path) {
        return PathAccess::Readonly;
    }
    if is_hard_protected_config_path(ctx, &path) {
        return PathAccess::Hidden;
    }
    if matches_any_exact_path(&path, &ctx.system_config.files.hidden) {
        return PathAccess::Hidden;
    }
    if matches_any_exact_path(&path, &ctx.system_config.files.readonly) {
        return PathAccess::Readonly;
    }
    if matches_any_exact_path(&path, &ctx.system_config.files.readwrite) {
        return PathAccess::Readwrite;
    }
    if matches_any_path(&path, &ctx.system_config.folders.hidden) {
        return PathAccess::Hidden;
    }
    if matches_any_path(&path, &ctx.system_config.folders.readonly) {
        return PathAccess::Readonly;
    }
    if matches_any_path(&path, &ctx.system_config.folders.readwrite) {
        return PathAccess::Readwrite;
    }
    if matches_any_exact_path(&path, &ctx.user_config.files.hidden) {
        return PathAccess::Hidden;
    }
    if matches_any_exact_path(&path, &ctx.user_config.files.readonly) {
        return PathAccess::Readonly;
    }
    if matches_any_exact_path(&path, &ctx.user_config.files.readwrite) {
        return PathAccess::Readwrite;
    }
    if matches_any_path(&path, &ctx.user_config.folders.hidden) {
        return PathAccess::Hidden;
    }
    if matches_any_path(&path, &ctx.user_config.folders.readonly) {
        return PathAccess::Readonly;
    }
    if matches_any_path(&path, &ctx.user_config.folders.readwrite) {
        return PathAccess::Readwrite;
    }
    if is_visible_by_base_scope(ctx, &path) {
        return PathAccess::Readwrite;
    }

    PathAccess::Unspecified
}

pub fn ensure_read_allowed_by_scope(
    ctx: &CToolScopeContext,
    path: impl AsRef<Path>,
) -> CToolResult<PathBuf> {
    let resolved_path = resolve_user_path(ctx, path);
    let path = canonicalize_existing_path(&resolved_path)?;

    if can_read_path(ctx, &path) {
        Ok(path)
    } else {
        Err(CToolError::OutOfScope {
            path: path.display().to_string(),
            operation: "read",
        })
    }
}

pub fn ensure_search_allowed_by_scope(
    ctx: &CToolScopeContext,
    path: impl AsRef<Path>,
) -> CToolResult<PathBuf> {
    let resolved_path = resolve_user_path(ctx, path);
    let path = canonicalize_existing_path(&resolved_path)?;

    if can_search_path(ctx, &path) {
        Ok(path)
    } else {
        Err(CToolError::OutOfScope {
            path: path.display().to_string(),
            operation: "search",
        })
    }
}

pub fn ensure_write_allowed_by_scope(
    ctx: &CToolScopeContext,
    path: impl AsRef<Path>,
) -> CToolResult<PathBuf> {
    let resolved_path = resolve_user_path(ctx, path);
    let path = canonicalize_existing_path(&resolved_path)?;

    if can_write_path(ctx, &path) {
        Ok(path)
    } else {
        Err(CToolError::OutOfScope {
            path: path.display().to_string(),
            operation: "write",
        })
    }
}

pub fn ensure_create_allowed_by_scope(
    ctx: &CToolScopeContext,
    path: impl AsRef<Path>,
) -> CToolResult<PathBuf> {
    let resolved_path = resolve_user_path(ctx, path);
    let path = canonicalize_parent_for_new_path(&resolved_path)?;

    if can_create_path(ctx, &path) {
        Ok(path)
    } else {
        Err(CToolError::OutOfScope {
            path: path.display().to_string(),
            operation: "create",
        })
    }
}

pub fn ensure_delete_allowed_by_scope(
    ctx: &CToolScopeContext,
    path: impl AsRef<Path>,
) -> CToolResult<PathBuf> {
    let path = ensure_write_allowed_by_scope(ctx, path)?;

    if is_hard_protected_config_path(ctx, &path) || is_critical_root_path(ctx, &path) {
        return Err(CToolError::OutOfScope {
            path: path.display().to_string(),
            operation: "delete",
        });
    }

    Ok(path)
}

pub fn ensure_move_allowed_by_scope(
    ctx: &CToolScopeContext,
    from: impl AsRef<Path>,
    to: impl AsRef<Path>,
) -> CToolResult<(PathBuf, PathBuf)> {
    let from = ensure_write_allowed_by_scope(ctx, from)?;

    if is_hard_protected_config_path(ctx, &from) || is_critical_root_path(ctx, &from) {
        return Err(CToolError::OutOfScope {
            path: from.display().to_string(),
            operation: "move",
        });
    }

    let to = ensure_create_allowed_by_scope(ctx, to)?;

    if is_hard_protected_config_path(ctx, &to) || is_critical_root_path(ctx, &to) {
        return Err(CToolError::OutOfScope {
            path: to.display().to_string(),
            operation: "move",
        });
    }

    Ok((from, to))
}

fn lexical_normalize_path(path: &Path) -> PathBuf {
    let mut output = PathBuf::new();

    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                output.pop();
            }
            _ => output.push(component.as_os_str()),
        }
    }

    output
}

#[cfg(test)]
#[path = "scope_context_tests.rs"]
mod tests;
