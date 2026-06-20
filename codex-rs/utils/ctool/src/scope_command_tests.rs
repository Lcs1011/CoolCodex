use std::path::PathBuf;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use pretty_assertions::assert_eq;

use crate::scope::CToolScopeBase;
use crate::scope_config::empty_scope_config;
use crate::scope_context::CToolScopeContext;

use super::*;

fn unique_temp_dir(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("{name}_{nanos}"))
}

fn test_context_with_workspace() -> CToolScopeContext {
    let character_root = unique_temp_dir("ctool_scope_command_character");
    let cool_workspace = character_root.join("workspace");
    std::fs::create_dir_all(&cool_workspace).expect("create test workspace");

    CToolScopeContext {
        current_dir: character_root.clone(),
        character_root: character_root.clone(),
        cool_workspace,
        base_scope: CToolScopeBase::None,
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
fn parses_file_readonly_command() {
    let command = parse_ctool_scope_command("/cs f ro src/lib.rs").expect("parse command");

    assert_eq!(
        command,
        CToolScopeCommand::UpdateRule {
            layer: CToolScopeLayer::Normal,
            target: CToolScopeTarget::File,
            rule: CToolScopeRule::Readonly,
            action: CToolScopeAction::Add,
            path: PathBuf::from("src/lib.rs"),
        }
    );
}

#[test]
fn parses_privileged_file_readonly_command() {
    let command = parse_ctool_scope_command("/cs p f ro src/lib.rs").expect("parse command");

    assert_eq!(
        command,
        CToolScopeCommand::UpdateRule {
            layer: CToolScopeLayer::Privileged,
            target: CToolScopeTarget::File,
            rule: CToolScopeRule::Readonly,
            action: CToolScopeAction::Add,
            path: PathBuf::from("src/lib.rs"),
        }
    );
}

#[test]
fn update_rule_resolves_relative_paths_against_cool_workspace() {
    let mut ctx = test_context_with_workspace();
    let command = parse_ctool_scope_command("/cs src").expect("parse command");

    handle_ctool_scope_command(command, &mut ctx).expect("handle command");

    assert_eq!(
        ctx.user_config.folders.readwrite,
        vec![ctx.cool_workspace.join("src")]
    );
}
