use std::fmt::Write as _;
use std::path::Path;
use std::path::PathBuf;

use crate::error::CToolError;
use crate::error::CToolResult;
use crate::scope::CToolScopeBase;
use crate::scope_config::CToolScopeConfig;
use crate::scope_config::load_optional_cool_config;
use crate::scope_context::CToolScopeContext;
use crate::scope_context::normalize_scope_config;
use crate::scope_context::resolve_user_path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CToolScopeCommand {
    Show,
    Help,
    UpdateRule {
        layer: CToolScopeLayer,
        target: CToolScopeTarget,
        rule: CToolScopeRule,
        action: CToolScopeAction,
        path: PathBuf,
    },
    SetBaseScope(CToolScopeBase),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CToolScopeTarget {
    File,
    Folder,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CToolScopeLayer {
    Normal,
    Privileged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CToolScopeRule {
    Readwrite,
    Readonly,
    Hidden,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CToolScopeAction {
    Add,
    Remove,
}

pub fn parse_ctool_scope_command(input: &str) -> CToolResult<CToolScopeCommand> {
    let tokens: Vec<&str> = input.split_whitespace().collect();

    if tokens.is_empty() {
        return Err(CToolError::InvalidInput(
            "empty CToolScope command".to_string(),
        ));
    }

    let command_name = tokens[0].trim_start_matches('/').to_ascii_lowercase();

    if command_name != "cs" && command_name != "ctoolscope" {
        return Err(CToolError::InvalidInput(format!(
            "not a CToolScope command: {}",
            tokens[0]
        )));
    }

    if tokens.len() == 1 {
        return Ok(CToolScopeCommand::Show);
    }

    match tokens[1].to_ascii_lowercase().as_str() {
        "show" => {
            ensure_token_len(&tokens, 2)?;
            Ok(CToolScopeCommand::Show)
        }
        "help" | "h" | "?" => {
            ensure_token_len(&tokens, 2)?;
            Ok(CToolScopeCommand::Help)
        }
        "base" => {
            ensure_token_len(&tokens, 3)?;
            let base_scope = parse_base_scope(tokens[2])?;
            Ok(CToolScopeCommand::SetBaseScope(base_scope))
        }
        _ => parse_scope_rule_command(&tokens),
    }
}

fn parse_scope_rule_command(tokens: &[&str]) -> CToolResult<CToolScopeCommand> {
    let mut index = 1;
    let layer = if tokens.get(index).is_some_and(|token| {
        token.eq_ignore_ascii_case("p")
            || token.eq_ignore_ascii_case("priv")
            || token.eq_ignore_ascii_case("privileged")
    }) {
        index += 1;
        CToolScopeLayer::Privileged
    } else {
        CToolScopeLayer::Normal
    };

    let target = if tokens
        .get(index)
        .is_some_and(|token| token.eq_ignore_ascii_case("f"))
    {
        index += 1;
        CToolScopeTarget::File
    } else {
        CToolScopeTarget::Folder
    };

    let rule = match tokens.get(index).map(|token| token.to_ascii_lowercase()) {
        Some(token) if token == "ro" => {
            index += 1;
            CToolScopeRule::Readonly
        }
        Some(token) if token == "hidden" || token == "hide" => {
            index += 1;
            CToolScopeRule::Hidden
        }
        _ => CToolScopeRule::Readwrite,
    };

    let action = if tokens.get(index).is_some_and(|token| *token == "-") {
        index += 1;
        CToolScopeAction::Remove
    } else {
        CToolScopeAction::Add
    };

    let path = parse_path_from_tokens(tokens, index)?;

    Ok(CToolScopeCommand::UpdateRule {
        layer,
        target,
        rule,
        action,
        path,
    })
}

pub fn handle_ctool_scope_command(
    command: CToolScopeCommand,
    ctx: &mut CToolScopeContext,
) -> CToolResult<String> {
    match command {
        CToolScopeCommand::Show => Ok(show_ctool_scope(ctx)),
        CToolScopeCommand::Help => Ok(show_ctool_scope_help()),
        CToolScopeCommand::SetBaseScope(base_scope) => {
            ctx.base_scope = base_scope;
            save_character_base_scope(ctx, base_scope)?;
            Ok(format!(
                "CToolScopeBase set to {} for current Character.",
                ctx.base_scope
            ))
        }
        CToolScopeCommand::UpdateRule {
            layer,
            target,
            rule,
            action,
            path,
        } => {
            let path = resolve_user_path(ctx, path);
            update_user_config(ctx, |config| {
                match layer {
                    CToolScopeLayer::Normal => {
                        update_scope_rule(config, target, rule, action, path.clone())
                    }
                    CToolScopeLayer::Privileged => {
                        update_privileged_scope_rule(config, target, rule, action, path.clone())
                    }
                }
            })?;
            Ok(format!(
                "{} {} {} {} path: {}",
                action.past_tense(),
                layer.label(),
                target.label(),
                rule.label(),
                path.display()
            ))
        }
    }
}

pub fn show_ctool_scope_help() -> String {
    [
        "CToolScope commands:",
        "",
        "  /cs",
        "  /cs show",
        "      Show current CToolScope configuration.",
        "",
        "  /cs help",
        "      Show this help message.",
        "",
        "  /cs base none",
        "  /cs base selected-only",
        "  /cs base cool-workspace",
        "      Set CToolScopeBase for current Character.",
        "",
        "  /cs <rule> <path>",
        "  /cs f <rule> <path>",
        "      Add normal folder/file rule.",
        "",
        "  /cs - <path>",
        "  /cs f - <path>",
        "      Remove normal folder/file readwrite rule.",
        "",
        "  /cs p <rule> <path>",
        "  /cs p f <rule> <path>",
        "      Add privileged folder/file rule.",
        "",
        "  /cs p - <path>",
        "  /cs p f - <path>",
        "      Remove privileged folder/file readwrite rule.",
        "",
        "Rules:",
        "  <empty>  readwrite",
        "  ro       readonly",
        "  hidden   hidden",
        "",
        "Examples:",
        "  /cs C:\\Work",
        "  /cs ro C:\\Work",
        "  /cs hidden C:\\Secret",
        "  /cs f ro C:\\Work\\note.txt",
        "  /cs p C:\\Trusted",
        "  /cs p f ro C:\\Trusted\\note.txt",
    ]
    .join("\n")
}

pub fn show_ctool_scope(ctx: &CToolScopeContext) -> String {
    let mut output = String::new();

    let _ = writeln!(output, "CToolScopeBase: {}", ctx.base_scope);
    let _ = writeln!(output, "CharacterRoot: {}", ctx.character_root.display());
    let _ = writeln!(output, "CoolWorkspace: {}", ctx.cool_workspace.display());
    let _ = writeln!(
        output,
        "CharacterConfig: {}",
        ctx.character_config_path.display()
    );
    let _ = writeln!(output, "CharacterScope: {}", ctx.user_config_path.display());

    match &ctx.system_config_path {
        Some(path) => {
            let _ = writeln!(output, "SystemConfig: {}", path.display());
        }
        None => {
            let _ = writeln!(output, "SystemConfig: <none>");
        }
    }

    let _ = writeln!(output);
    let _ = writeln!(output, "[Character files]");
    write_path_list(&mut output, "readwrite", &ctx.user_config.files.readwrite);
    write_path_list(&mut output, "readonly", &ctx.user_config.files.readonly);
    write_path_list(&mut output, "hidden", &ctx.user_config.files.hidden);

    let _ = writeln!(output);
    let _ = writeln!(output, "[Character folders]");
    write_path_list(&mut output, "readwrite", &ctx.user_config.folders.readwrite);
    write_path_list(&mut output, "readonly", &ctx.user_config.folders.readonly);
    write_path_list(&mut output, "hidden", &ctx.user_config.folders.hidden);

    let _ = writeln!(output);
    let _ = writeln!(output, "[Character privileged files]");
    write_path_list(
        &mut output,
        "readwrite",
        &ctx.user_config.privileged_files.readwrite,
    );
    write_path_list(
        &mut output,
        "readonly",
        &ctx.user_config.privileged_files.readonly,
    );
    write_path_list(
        &mut output,
        "hidden",
        &ctx.user_config.privileged_files.hidden,
    );

    let _ = writeln!(output);
    let _ = writeln!(output, "[Character privileged folders]");
    write_path_list(
        &mut output,
        "readwrite",
        &ctx.user_config.privileged_folders.readwrite,
    );
    write_path_list(
        &mut output,
        "readonly",
        &ctx.user_config.privileged_folders.readonly,
    );
    write_path_list(
        &mut output,
        "hidden",
        &ctx.user_config.privileged_folders.hidden,
    );

    let _ = writeln!(output);
    let _ = writeln!(output, "[System files]");
    write_path_list(&mut output, "readwrite", &ctx.system_config.files.readwrite);
    write_path_list(&mut output, "readonly", &ctx.system_config.files.readonly);
    write_path_list(&mut output, "hidden", &ctx.system_config.files.hidden);

    let _ = writeln!(output);
    let _ = writeln!(output, "[System folders]");
    write_path_list(
        &mut output,
        "readwrite",
        &ctx.system_config.folders.readwrite,
    );
    write_path_list(&mut output, "readonly", &ctx.system_config.folders.readonly);
    write_path_list(&mut output, "hidden", &ctx.system_config.folders.hidden);

    let _ = writeln!(output);
    let _ = writeln!(output, "[System privileged files]");
    write_path_list(
        &mut output,
        "readwrite",
        &ctx.system_config.privileged_files.readwrite,
    );
    write_path_list(
        &mut output,
        "readonly",
        &ctx.system_config.privileged_files.readonly,
    );
    write_path_list(
        &mut output,
        "hidden",
        &ctx.system_config.privileged_files.hidden,
    );

    let _ = writeln!(output);
    let _ = writeln!(output, "[System privileged folders]");
    write_path_list(
        &mut output,
        "readwrite",
        &ctx.system_config.privileged_folders.readwrite,
    );
    write_path_list(
        &mut output,
        "readonly",
        &ctx.system_config.privileged_folders.readonly,
    );
    write_path_list(
        &mut output,
        "hidden",
        &ctx.system_config.privileged_folders.hidden,
    );

    output
}

impl CToolScopeTarget {
    fn label(self) -> &'static str {
        match self {
            CToolScopeTarget::File => "file",
            CToolScopeTarget::Folder => "folder",
        }
    }
}

impl CToolScopeLayer {
    fn label(self) -> &'static str {
        match self {
            CToolScopeLayer::Normal => "normal",
            CToolScopeLayer::Privileged => "privileged",
        }
    }
}

impl CToolScopeRule {
    fn label(self) -> &'static str {
        match self {
            CToolScopeRule::Readwrite => "readwrite",
            CToolScopeRule::Readonly => "readonly",
            CToolScopeRule::Hidden => "hidden",
        }
    }
}

impl CToolScopeAction {
    fn past_tense(self) -> &'static str {
        match self {
            CToolScopeAction::Add => "Added",
            CToolScopeAction::Remove => "Removed",
        }
    }
}

pub fn add_visible_path(config: &mut CToolScopeConfig, path: PathBuf) -> bool {
    update_scope_rule(
        config,
        CToolScopeTarget::Folder,
        CToolScopeRule::Readwrite,
        CToolScopeAction::Add,
        path,
    )
}

pub fn remove_visible_path(config: &mut CToolScopeConfig, path: &Path) -> bool {
    update_scope_rule(
        config,
        CToolScopeTarget::Folder,
        CToolScopeRule::Readwrite,
        CToolScopeAction::Remove,
        path.to_path_buf(),
    )
}

pub fn add_hide_path(config: &mut CToolScopeConfig, path: PathBuf) -> bool {
    update_scope_rule(
        config,
        CToolScopeTarget::Folder,
        CToolScopeRule::Hidden,
        CToolScopeAction::Add,
        path,
    )
}

pub fn remove_hide_path(config: &mut CToolScopeConfig, path: &Path) -> bool {
    update_scope_rule(
        config,
        CToolScopeTarget::Folder,
        CToolScopeRule::Hidden,
        CToolScopeAction::Remove,
        path.to_path_buf(),
    )
}

pub fn add_protected_path(config: &mut CToolScopeConfig, path: PathBuf) -> bool {
    update_scope_rule(
        config,
        CToolScopeTarget::Folder,
        CToolScopeRule::Readonly,
        CToolScopeAction::Add,
        path,
    )
}

pub fn remove_protected_path(config: &mut CToolScopeConfig, path: &Path) -> bool {
    update_scope_rule(
        config,
        CToolScopeTarget::Folder,
        CToolScopeRule::Readonly,
        CToolScopeAction::Remove,
        path.to_path_buf(),
    )
}

fn update_scope_rule(
    config: &mut CToolScopeConfig,
    target: CToolScopeTarget,
    rule: CToolScopeRule,
    action: CToolScopeAction,
    path: PathBuf,
) -> bool {
    let paths = match (target, rule) {
        (CToolScopeTarget::File, CToolScopeRule::Readwrite) => &mut config.files.readwrite,
        (CToolScopeTarget::File, CToolScopeRule::Readonly) => &mut config.files.readonly,
        (CToolScopeTarget::File, CToolScopeRule::Hidden) => &mut config.files.hidden,
        (CToolScopeTarget::Folder, CToolScopeRule::Readwrite) => &mut config.folders.readwrite,
        (CToolScopeTarget::Folder, CToolScopeRule::Readonly) => &mut config.folders.readonly,
        (CToolScopeTarget::Folder, CToolScopeRule::Hidden) => &mut config.folders.hidden,
    };

    match action {
        CToolScopeAction::Add => add_unique_path(paths, path),
        CToolScopeAction::Remove => remove_path(paths, &path),
    }
}

fn update_privileged_scope_rule(
    config: &mut CToolScopeConfig,
    target: CToolScopeTarget,
    rule: CToolScopeRule,
    action: CToolScopeAction,
    path: PathBuf,
) -> bool {
    let paths = match (target, rule) {
        (CToolScopeTarget::File, CToolScopeRule::Readwrite) => {
            &mut config.privileged_files.readwrite
        }
        (CToolScopeTarget::File, CToolScopeRule::Readonly) => {
            &mut config.privileged_files.readonly
        }
        (CToolScopeTarget::File, CToolScopeRule::Hidden) => {
            &mut config.privileged_files.hidden
        }
        (CToolScopeTarget::Folder, CToolScopeRule::Readwrite) => {
            &mut config.privileged_folders.readwrite
        }
        (CToolScopeTarget::Folder, CToolScopeRule::Readonly) => {
            &mut config.privileged_folders.readonly
        }
        (CToolScopeTarget::Folder, CToolScopeRule::Hidden) => {
            &mut config.privileged_folders.hidden
        }
    };

    match action {
        CToolScopeAction::Add => add_unique_path(paths, path),
        CToolScopeAction::Remove => remove_path(paths, &path),
    }
}

pub fn save_current_dir_cool_config(path: &Path, config: &CToolScopeConfig) -> CToolResult<()> {
    let text = toml::to_string_pretty(config).map_err(|error| {
        CToolError::InvalidInput(format!("failed to serialize Cool scope TOML: {error}"))
    })?;

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|error| {
            CToolError::InvalidInput(format!(
                "failed to create Cool config directory: {} ({error})",
                parent.display()
            ))
        })?;
    }

    std::fs::write(path, text).map_err(|error| {
        CToolError::InvalidInput(format!(
            "failed to save Cool config file: {} ({error})",
            path.display()
        ))
    })
}

fn save_character_base_scope(
    ctx: &CToolScopeContext,
    base_scope: CToolScopeBase,
) -> CToolResult<()> {
    let path = &ctx.character_config_path;
    let mut value = if path.exists() {
        let text = std::fs::read_to_string(path).map_err(|error| {
            CToolError::InvalidInput(format!(
                "failed to read Cool character config file: {} ({error})",
                path.display()
            ))
        })?;
        toml::from_str::<toml::Value>(&text).map_err(|error| {
            CToolError::InvalidInput(format!(
                "failed to parse Cool character config file: {} ({error})",
                path.display()
            ))
        })?
    } else {
        toml::Value::Table(toml::map::Map::new())
    };

    let Some(table) = value.as_table_mut() else {
        return Err(CToolError::InvalidInput(format!(
            "Cool character config must be a TOML table: {}",
            path.display()
        )));
    };

    table.insert(
        "ctool_scope_base".to_string(),
        toml::Value::String(base_scope.as_str().to_string()),
    );

    let text = toml::to_string_pretty(&value).map_err(|error| {
        CToolError::InvalidInput(format!(
            "failed to serialize Cool character config TOML: {error}"
        ))
    })?;

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|error| {
            CToolError::InvalidInput(format!(
                "failed to create Cool config directory: {} ({error})",
                parent.display()
            ))
        })?;
    }

    std::fs::write(path, text).map_err(|error| {
        CToolError::InvalidInput(format!(
            "failed to save Cool character config file: {} ({error})",
            path.display()
        ))
    })
}

fn update_user_config(
    ctx: &mut CToolScopeContext,
    update: impl FnOnce(&mut CToolScopeConfig) -> bool,
) -> CToolResult<()> {
    let mut raw_config = load_optional_cool_config(&ctx.user_config_path)?;
    let _changed = update(&mut raw_config);

    save_current_dir_cool_config(&ctx.user_config_path, &raw_config)?;

    ctx.user_config = normalize_scope_config(raw_config, &ctx.character_root);

    Ok(())
}

fn parse_base_scope(value: &str) -> CToolResult<CToolScopeBase> {
    match value.to_ascii_lowercase().as_str() {
        "none" => Ok(CToolScopeBase::None),

        "selected-only" | "selectedonly" | "selected_only" => {
            Ok(CToolScopeBase::SelectedOnly)
        }

        "cool-workspace" | "coolworkspace" | "cool_workspace" | "workspace" => {
            Ok(CToolScopeBase::CoolWorkspace)
        }

        "the-eye-of-providence"
        | "theeyeofprovidence"
        | "the_eye_of_providence" => Ok(CToolScopeBase::TheEyeOfProvidence),

        _ => Err(CToolError::InvalidInput(format!(
            "unsupported CToolScopeBase: {value}"
        ))),
    }
}

fn parse_path_from_tokens(tokens: &[&str], start_index: usize) -> CToolResult<PathBuf> {
    if start_index >= tokens.len() {
        return Err(CToolError::InvalidInput(
            "missing path argument".to_string(),
        ));
    }

    let path = tokens[start_index..].join(" ");
    let path = path.trim();

    if path.is_empty() {
        return Err(CToolError::InvalidInput(
            "missing path argument".to_string(),
        ));
    }

    Ok(PathBuf::from(path))
}

fn ensure_token_len(tokens: &[&str], expected_len: usize) -> CToolResult<()> {
    if tokens.len() == expected_len {
        Ok(())
    } else {
        Err(CToolError::InvalidInput(format!(
            "unexpected argument count: expected {}, got {}",
            expected_len,
            tokens.len()
        )))
    }
}

fn add_unique_path(paths: &mut Vec<PathBuf>, path: PathBuf) -> bool {
    if paths.iter().any(|existing| existing == &path) {
        false
    } else {
        paths.push(path);
        true
    }
}

fn remove_path(paths: &mut Vec<PathBuf>, path: &Path) -> bool {
    let old_len = paths.len();
    paths.retain(|existing| existing != path);
    paths.len() != old_len
}

fn write_path_list(output: &mut String, label: &str, paths: &[PathBuf]) {
    let _ = writeln!(output, "{label}:");

    if paths.is_empty() {
        let _ = writeln!(output, "  []");
        return;
    }

    for path in paths {
        let _ = writeln!(output, "  - {}", path.display());
    }
}

#[cfg(test)]
#[path = "scope_command_tests.rs"]
mod tests;
