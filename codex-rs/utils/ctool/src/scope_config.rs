use std::env;
use std::path::Path;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

use crate::command_request::CToolCommandConfig;
use crate::command_request::default_command_config;
use crate::command_request::merge_command_configs;
use crate::error::CToolError;
use crate::error::CToolResult;
use crate::scope::CToolScopeBase;

pub const COOL_DIR_NAME: &str = ".cool";
pub const COOL_SYSTEM_DIR_NAME: &str = ".cool-system";
pub const CONFIG_FILE_NAME: &str = "config.toml";
pub const SCOPE_FILE_NAME: &str = "scope.toml";
pub const COMMAND_FILE_NAME: &str = "command.toml";
pub const COOL_SYSTEM_DIR_ENV: &str = "COOL_SYSTEM_DIR";
pub const COOL_SYSTEM_CONFIG_ENV: &str = "COOL_SYSTEM_CONFIG";

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CToolCharacterConfig {
    pub scope_base: Option<CToolScopeBase>,
    pub cool_workspace: Option<PathBuf>,
}

pub type CToolSessionConfig = CToolCharacterConfig;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct CToolScopeConfig {
    #[serde(default)]
    pub files: CToolScopeRuleSet,
    #[serde(default)]
    pub folders: CToolScopeRuleSet,
    #[serde(default)]
    pub privileged_files: Vec<String>,
    #[serde(default)]
    pub privileged_folders: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct CToolScopeRuleSet {
    #[serde(default)]
    pub readwrite: Vec<PathBuf>,
    #[serde(default)]
    pub readonly: Vec<PathBuf>,
    #[serde(default, alias = "hide")]
    pub hidden: Vec<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Default)]
struct CoolCharacterConfigToml {
    #[serde(default)]
    ctool_scope_base: Option<String>,
    #[serde(default)]
    cool_workspace: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct CoolCommandConfigToml {
    #[serde(default = "default_command_config")]
    ctool_command: CToolCommandConfig,
}

impl Default for CoolCommandConfigToml {
    fn default() -> Self {
        Self {
            ctool_command: default_command_config(),
        }
    }
}

pub fn empty_character_config() -> CToolCharacterConfig {
    CToolCharacterConfig::default()
}

pub fn empty_session_config() -> CToolCharacterConfig {
    empty_character_config()
}

pub fn empty_scope_config() -> CToolScopeConfig {
    CToolScopeConfig::default()
}

pub fn locate_cool_dir(character_root: impl AsRef<Path>) -> PathBuf {
    character_root.as_ref().join(COOL_DIR_NAME)
}

pub fn locate_cool_config_path(character_root: impl AsRef<Path>) -> PathBuf {
    locate_cool_dir(character_root).join(CONFIG_FILE_NAME)
}

pub fn locate_cool_scope_path(character_root: impl AsRef<Path>) -> PathBuf {
    locate_cool_dir(character_root).join(SCOPE_FILE_NAME)
}

pub fn locate_cool_command_path(character_root: impl AsRef<Path>) -> PathBuf {
    locate_cool_dir(character_root).join(COMMAND_FILE_NAME)
}

pub fn locate_cool_system_dir_from_launcher(launcher_dir: impl AsRef<Path>) -> PathBuf {
    launcher_dir.as_ref().join(COOL_SYSTEM_DIR_NAME)
}

pub fn locate_cool_system_dir() -> Option<PathBuf> {
    let value = env::var(COOL_SYSTEM_DIR_ENV).ok()?;
    let value = value.trim();

    if value.is_empty() {
        None
    } else {
        Some(PathBuf::from(value))
    }
}

pub fn locate_legacy_cool_system_config_path() -> Option<PathBuf> {
    let value = env::var(COOL_SYSTEM_CONFIG_ENV).ok()?;
    let value = value.trim();

    if value.is_empty() {
        None
    } else {
        Some(PathBuf::from(value))
    }
}

pub fn locate_cool_system_config_path() -> Option<PathBuf> {
    if let Some(system_dir) = locate_cool_system_dir() {
        return Some(system_dir.join(CONFIG_FILE_NAME));
    }

    locate_legacy_cool_system_config_path()
}

pub fn locate_cool_system_scope_path() -> Option<PathBuf> {
    locate_cool_system_dir().map(|dir| dir.join(SCOPE_FILE_NAME))
}

pub fn locate_cool_system_command_path() -> Option<PathBuf> {
    locate_cool_system_dir().map(|dir| dir.join(COMMAND_FILE_NAME))
}

pub fn load_optional_cool_character_config(path: &Path) -> CToolResult<CToolCharacterConfig> {
    if !path.exists() {
        return Ok(empty_character_config());
    }

    let text = std::fs::read_to_string(path).map_err(|error| {
        CToolError::InvalidInput(format!(
            "failed to read Cool character config file: {} ({error})",
            path.display()
        ))
    })?;

    parse_cool_character_config_toml(&text).map_err(|error| {
        CToolError::InvalidInput(format!(
            "failed to parse Cool character config file: {} ({error})",
            path.display()
        ))
    })
}

pub fn load_optional_cool_session_config(path: &Path) -> CToolResult<CToolCharacterConfig> {
    load_optional_cool_character_config(path)
}

pub fn parse_cool_character_config_toml(text: &str) -> CToolResult<CToolCharacterConfig> {
    let file: CoolCharacterConfigToml = toml::from_str(text).map_err(|error| {
        CToolError::InvalidInput(format!("invalid Cool character TOML config: {error}"))
    })?;

    let scope_base = match file.ctool_scope_base.as_deref() {
        Some(value) => Some(parse_scope_base(value)?),
        None => None,
    };

    Ok(CToolCharacterConfig {
        scope_base,
        cool_workspace: file.cool_workspace,
    })
}

pub fn parse_cool_session_config_toml(text: &str) -> CToolResult<CToolCharacterConfig> {
    parse_cool_character_config_toml(text)
}

pub fn load_optional_cool_config(path: &Path) -> CToolResult<CToolScopeConfig> {
    if !path.exists() {
        return Ok(empty_scope_config());
    }

    let text = std::fs::read_to_string(path).map_err(|error| {
        CToolError::InvalidInput(format!(
            "failed to read Cool scope file: {} ({error})",
            path.display()
        ))
    })?;

    parse_cool_config_toml(&text).map_err(|error| {
        CToolError::InvalidInput(format!(
            "failed to parse Cool scope file: {} ({error})",
            path.display()
        ))
    })
}

pub fn parse_cool_config_toml(text: &str) -> CToolResult<CToolScopeConfig> {
    toml::from_str(text)
        .map_err(|error| CToolError::InvalidInput(format!("invalid Cool scope TOML: {error}")))
}

fn parse_scope_base(value: &str) -> CToolResult<CToolScopeBase> {
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

pub fn empty_command_config() -> CToolCommandConfig {
    default_command_config()
}

pub fn load_optional_cool_command_config(path: &Path) -> CToolResult<CToolCommandConfig> {
    if !path.exists() {
        return Ok(empty_command_config());
    }

    let text = std::fs::read_to_string(path).map_err(|error| {
        CToolError::InvalidInput(format!(
            "failed to read Cool command config file: {} ({error})",
            path.display()
        ))
    })?;

    parse_cool_command_config_toml(&text).map_err(|error| {
        CToolError::InvalidInput(format!(
            "failed to parse Cool command config file: {} ({error})",
            path.display()
        ))
    })
}

pub fn parse_cool_command_config_toml(text: &str) -> CToolResult<CToolCommandConfig> {
    let file: CoolCommandConfigToml = toml::from_str(text).map_err(|error| {
        CToolError::InvalidInput(format!("invalid Cool command TOML config: {error}"))
    })?;

    Ok(file.ctool_command)
}

pub fn load_merged_cool_command_config(
    character_command_path: &Path,
    system_command_path: Option<&Path>,
) -> CToolResult<CToolCommandConfig> {
    let character_config = load_optional_cool_command_config(character_command_path)?;
    let system_config = match system_command_path {
        Some(path) => load_optional_cool_command_config(path)?,
        None => empty_command_config(),
    };

    Ok(merge_command_configs(character_config, system_config))
}
