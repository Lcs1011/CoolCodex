use std::path::PathBuf;

use crate::scope::CToolScopeBase;
use crate::scope_config::empty_scope_config;

use super::*;

fn test_context() -> CToolScopeContext {
    let character_root = std::env::temp_dir().join("ctool_scope_context_tests");

    CToolScopeContext {
        current_dir: character_root.clone(),
        character_root: character_root.clone(),
        cool_workspace: character_root.clone(),
        base_scope: CToolScopeBase::CoolWorkspace,
        user_config_path: character_root.join(".cool").join("scope.toml"),
        character_config_path: character_root.join(".cool").join("config.toml"),
        system_config_path: None,
        character_command_path: character_root.join(".cool").join("command.toml"),
        system_command_path: None,
        cool_system_dir: None,
        user_config: empty_scope_config(),
        system_config: empty_scope_config(),
    }
}

#[test]
fn web_search_cache_is_readonly_exception_under_cool_dir() {
    let ctx = test_context();
    let path = ctx
        .character_root
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
    let path = PathBuf::from(&ctx.character_root)
        .join(".cool")
        .join("config.toml");

    assert!(is_hard_protected_config_path(&ctx, &path));
    assert!(!can_read_path(&ctx, &path));
    assert!(!can_write_path(&ctx, &path));
}

#[test]
fn cool_system_dir_is_hard_protected() {
    let mut ctx = test_context();
    let cool_system_dir = std::env::temp_dir().join("ctool_scope_system_dir_tests");
    let path = cool_system_dir.join("scope.toml");
    ctx.cool_system_dir = Some(cool_system_dir);

    assert!(is_hard_protected_config_path(&ctx, &path));
    assert!(!can_read_path(&ctx, &path));
    assert!(!can_write_path(&ctx, &path));
}

#[test]
fn critical_roots_are_protected() {
    let mut ctx = test_context();
    let cool_system_dir = std::env::temp_dir().join("ctool_scope_system_dir_protected_tests");
    ctx.cool_system_dir = Some(cool_system_dir.clone());

    assert!(is_protected_path(&ctx, &ctx.character_root));
    assert!(is_protected_path(&ctx, &ctx.cool_workspace));
    assert!(is_protected_path(&ctx, ctx.character_root.join(".cool")));
    assert!(is_protected_path(&ctx, cool_system_dir));
}

#[test]
fn system_folder_hidden_beats_character_file_readwrite() {
    let mut ctx = test_context();
    let file = ctx.character_root.join("secret").join("open.txt");

    ctx.system_config.folders.hidden = vec![ctx.character_root.join("secret")];
    ctx.user_config.files.readwrite = vec![file.clone()];

    assert!(!can_read_path(&ctx, &file));
    assert!(!can_write_path(&ctx, &file));
}

#[test]
fn character_file_readwrite_beats_character_folder_hidden() {
    let mut ctx = test_context();
    let file = ctx.character_root.join("secret").join("open.txt");

    ctx.user_config.folders.hidden = vec![ctx.character_root.join("secret")];
    ctx.user_config.files.readwrite = vec![file.clone()];

    assert!(can_read_path(&ctx, &file));
    assert!(can_write_path(&ctx, &file));
}
#[test]
fn base_none_blocks_even_explicit_character_rules() {
    let mut ctx = test_context();
    ctx.base_scope = CToolScopeBase::None;
    let file = ctx.character_root.join("selected").join("open.txt");
    let other_file = ctx.character_root.join("other").join("closed.txt");

    ctx.user_config.files.readwrite = vec![file.clone()];
    ctx.user_config.files.readonly = vec![other_file.clone()];

    assert!(!can_read_path(&ctx, &file));
    assert!(!can_write_path(&ctx, &file));
    assert!(!can_read_path(&ctx, &other_file));
    assert!(!can_write_path(&ctx, &other_file));
}

#[test]
fn base_none_blocks_even_privileged_rules() {
    let mut ctx = test_context();
    ctx.base_scope = CToolScopeBase::None;
    let file = ctx.character_root.join(".cool").join("config.toml");

    ctx.system_config.privileged_files.readwrite = vec![file.clone()];

    assert!(!can_read_path(&ctx, &file));
    assert!(!can_write_path(&ctx, &file));
}

#[test]
fn privileged_file_readwrite_beats_hard_protected_cool_config() {
    let mut ctx = test_context();
    let file = ctx.character_root.join(".cool").join("config.toml");

    ctx.user_config.privileged_files.readwrite = vec![file.clone()];

    assert!(is_hard_protected_config_path(&ctx, &file));
    assert!(can_read_path(&ctx, &file));
    assert!(can_write_path(&ctx, &file));
}

#[test]
fn privileged_folder_readwrite_allows_create_under_cool_dir() {
    let mut ctx = test_context();
    let cool_dir = ctx.character_root.join(".cool");
    let cache_dir = cool_dir.join("cache");
    let file = cache_dir.join("created.txt");

    std::fs::create_dir_all(&cache_dir).expect("create temp cache dir");
    ctx.user_config.privileged_folders.readwrite =
        vec![canonicalize_existing_path(&cool_dir).expect("canonicalize cool dir")];

    assert!(can_create_path(&ctx, &file));
}

#[test]
fn privileged_file_readwrite_allows_delete_under_cool_dir() {
    let mut ctx = test_context();
    let cool_dir = ctx.character_root.join(".cool");
    let file = cool_dir.join("delete.txt");

    std::fs::create_dir_all(&cool_dir).expect("create temp cool dir");
    std::fs::write(&file, "delete me").expect("write temp file");
    ctx.user_config.privileged_files.readwrite =
        vec![canonicalize_existing_path(&file).expect("canonicalize temp file")];

    assert!(ensure_delete_allowed_by_scope(&ctx, &file).is_ok());
}
