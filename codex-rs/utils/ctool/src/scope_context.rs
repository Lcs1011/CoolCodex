use std::path::Component;
use std::path::Path;
use std::path::PathBuf;

use crate::error::CToolError;
use crate::error::CToolResult;
use crate::scope::CToolBaseScope;
use crate::scope_config::CToolScopeConfig;
use crate::scope_config::SYSTEM_CONFIG_FILE_NAME;
use crate::scope_config::USER_CONFIG_FILE_NAME;
use crate::scope_config::empty_scope_config;
use crate::scope_config::load_optional_cool_config;
use crate::scope_config::locate_cool_config_path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CToolScopeContext {
    pub current_dir: PathBuf,
    pub base_scope: CToolBaseScope,

    pub user_config_path: PathBuf,
    pub system_config_path: Option<PathBuf>,

    pub user_config: CToolScopeConfig,
    pub system_config: CToolScopeConfig,
}

pub fn build_ctool_scope_context(
    current_dir: impl AsRef<Path>,
    base_scope: CToolBaseScope,
    system_config_path: Option<PathBuf>,
) -> CToolResult<CToolScopeContext> {
    let current_dir = canonicalize_existing_path(current_dir.as_ref())?;

    let user_config_path = locate_cool_config_path(&current_dir);
    let user_config = load_optional_cool_config(&user_config_path)?;
    let system_config = match system_config_path.as_deref() {
        Some(path) => load_optional_cool_config(path)?,
        None => empty_scope_config(),
    };

    let user_config = normalize_scope_config(user_config, &current_dir);
    let system_config = normalize_scope_config(system_config, &current_dir);

    Ok(CToolScopeContext {
        current_dir,
        base_scope,
        user_config_path,
        system_config_path,
        user_config,
        system_config,
    })
}

pub fn normalize_scope_config(config: CToolScopeConfig, current_dir: &Path) -> CToolScopeConfig {
    CToolScopeConfig {
        visible_paths: normalize_scope_paths(config.visible_paths, current_dir),
        hide_paths: normalize_scope_paths(config.hide_paths, current_dir),
        protected_paths: normalize_scope_paths(config.protected_paths, current_dir),
    }
}

fn normalize_scope_paths(paths: Vec<PathBuf>, current_dir: &Path) -> Vec<PathBuf> {
    paths
        .into_iter()
        .map(|path| resolve_config_path(current_dir, &path))
        .collect()
}

fn resolve_config_path(current_dir: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        lexical_normalize_path(path)
    } else {
        lexical_normalize_path(&current_dir.join(path))
    }
}

pub fn resolve_user_path(ctx: &CToolScopeContext, path: impl AsRef<Path>) -> PathBuf {
    let path = path.as_ref();

    if path.is_absolute() {
        lexical_normalize_path(path)
    } else {
        lexical_normalize_path(&ctx.current_dir.join(path))
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
        CToolBaseScope::None => false,
        CToolBaseScope::Workspace => path_matches_rule(path, &ctx.current_dir),
        CToolBaseScope::SelectedOnly | CToolBaseScope::TheEyeofProvidence => false,
    }
}

pub fn can_read_path(ctx: &CToolScopeContext, path: impl AsRef<Path>) -> bool {
    let path = lexical_normalize_path(path.as_ref());

    if matches_any_path(&path, &ctx.system_config.hide_paths) {
        return false;
    }

    if matches_any_path(&path, &ctx.system_config.visible_paths) {
        return true;
    }

    if matches_any_path(&path, &ctx.user_config.hide_paths) {
        return false;
    }

    if matches_any_path(&path, &ctx.user_config.visible_paths) {
        return true;
    }

    is_visible_by_base_scope(ctx, path)
}

pub fn can_search_path(ctx: &CToolScopeContext, path: impl AsRef<Path>) -> bool {
    can_read_path(ctx, path)
}

pub fn is_hard_protected_config_path(ctx: &CToolScopeContext, path: impl AsRef<Path>) -> bool {
    let path = lexical_normalize_path(path.as_ref());
    let user_config_path = lexical_normalize_path(&ctx.user_config_path);

    if path == user_config_path {
        return true;
    }

    if let Some(system_config_path) = &ctx.system_config_path {
        let system_config_path = lexical_normalize_path(system_config_path);
        if path == system_config_path {
            return true;
        }
    }

    let Some(file_name) = path.file_name().and_then(|file_name| file_name.to_str()) else {
        return false;
    };

    file_name.eq_ignore_ascii_case(USER_CONFIG_FILE_NAME)
        || file_name.eq_ignore_ascii_case(SYSTEM_CONFIG_FILE_NAME)
}

pub fn is_protected_path(ctx: &CToolScopeContext, path: impl AsRef<Path>) -> bool {
    let path = lexical_normalize_path(path.as_ref());

    if matches_any_path(&path, &ctx.system_config.protected_paths) {
        return true;
    }

    if matches_any_path(&path, &ctx.user_config.protected_paths) {
        return true;
    }

    is_hard_protected_config_path(ctx, path)
}

pub fn can_write_path(ctx: &CToolScopeContext, path: impl AsRef<Path>) -> bool {
    let path = lexical_normalize_path(path.as_ref());

    can_read_path(ctx, &path) && !is_protected_path(ctx, path)
}

pub fn can_create_path(ctx: &CToolScopeContext, path: impl AsRef<Path>) -> bool {
    let path = lexical_normalize_path(path.as_ref());

    if is_hard_protected_config_path(ctx, &path) {
        return false;
    }

    if matches_any_path(&path, &ctx.system_config.hide_paths)
        || matches_any_path(&path, &ctx.user_config.hide_paths)
    {
        return false;
    }

    let Ok(path_with_canonical_parent) = canonicalize_parent_for_new_path(&path) else {
        return false;
    };

    let Some(parent) = path_with_canonical_parent.parent() else {
        return false;
    };

    can_write_path(ctx, parent)
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

    if is_hard_protected_config_path(ctx, &path) {
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
    let to = ensure_create_allowed_by_scope(ctx, to)?;

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
