use std::path::PathBuf;

use crate::scope::CToolScopeBase;
use crate::scope_config::empty_scope_config;

use super::*;

fn test_context() -> CToolScopeContext {
    let session_root = std::env::temp_dir().join("ctool_scope_context_tests");

    CToolScopeContext {
        current_dir: session_root.clone(),
        session_root: session_root.clone(),
        cool_workspace: session_root.clone(),
        base_scope: CToolScopeBase::CoolWorkspace,
        user_config_path: session_root.join(".cool").join("scope.toml"),
        session_config_path: session_root.join(".cool").join("config.toml"),
        system_config_path: None,
        session_command_path: session_root.join(".cool").join("command.toml"),
        system_command_path: None,
        user_config: empty_scope_config(),
        system_config: empty_scope_config(),
    }
}

#[test]
fn web_search_cache_is_readonly_exception_under_cool_dir() {
    let ctx = test_context();
    let path = ctx
        .session_root
        .join(".cool")
        .join("cache")
        .join("web_search")
        .join("2026-06-10")
        .join("00000_search_rust.md");

    assert!(is_web_search_cache_path(&ctx, &path));
    assert!(!is_hard_protected_config_path(&ctx, &path));
    assert!(can_read_path(&ctx, &path));
    assert!(!can_write_path(&ctx, &path));
    assert!(!can_create_path(&ctx, &path));
}

#[test]
fn cool_config_remains_hard_protected() {
    let ctx = test_context();
    let path = PathBuf::from(&ctx.session_root)
        .join(".cool")
        .join("config.toml");

    assert!(is_hard_protected_config_path(&ctx, &path));
    assert!(!can_read_path(&ctx, &path));
    assert!(!can_write_path(&ctx, &path));
}
