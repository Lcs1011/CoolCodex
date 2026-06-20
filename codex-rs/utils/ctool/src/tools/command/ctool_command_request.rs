use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::command_request::CToolCommandApproval;
use crate::command_request::CToolCommandRequestRecordStatus;
use crate::command_request::CToolCommandRisk;
use crate::command_request::CToolCommandUserDecision;
use crate::command_request::build_command_request_preview;
use crate::command_request::execute_approved_command_request;
use crate::command_request::parse_red_first_confirmation_input;
use crate::command_request::parse_red_second_confirmation_input;
use crate::command_request::parse_yellow_confirmation_input;
use crate::command_request::record_unexecuted_command_request;
use crate::command_request::render_command_request_banner;
use crate::context::CToolContext;
use crate::error::CToolResult;
use crate::scope_config::COOL_DIR_NAME;
use crate::scope_config::load_merged_cool_command_config;
use crate::tool::CTool;
use crate::tool::CToolSpec;

pub const CTOOL_COMMAND_REQUEST_TOOL_NAME: &str = "ctool_command_request";

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CToolCommandRequestInput {
    pub commands: Vec<String>,
    #[serde(default)]
    pub ai_risk_upgrade: Option<CToolCommandRisk>,
    #[serde(default)]
    pub reason: Option<String>,
    #[serde(default)]
    pub yellow_confirmation: Option<String>,
    #[serde(default)]
    pub red_first_confirmation: Option<String>,
    #[serde(default)]
    pub red_second_confirmation: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CToolCommandRequestCommandOutput {
    pub command: String,
    pub risk: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CToolCommandRequestOutput {
    pub will_execute: bool,
    pub executed: bool,
    pub blocked: bool,
    pub rejected: bool,
    pub all_success: Option<bool>,
    pub result_file: Option<String>,
    pub log_file: Option<String>,
    pub current_dir: String,
    pub command_count: usize,
    pub system_risk: String,
    pub ai_risk_upgrade: Option<String>,
    pub final_risk: String,
    pub approval_required: String,
    pub request_reason: Option<String>,
    pub user_feedback: Option<String>,
    pub commands: Vec<CToolCommandRequestCommandOutput>,
    pub display_text: String,
    pub banner: String,
    pub note: String,
}

pub struct CToolCommandRequest;

impl CTool for CToolCommandRequest {
    fn spec(&self) -> CToolSpec {
        CToolSpec {
            name: CTOOL_COMMAND_REQUEST_TOOL_NAME,
            description: "Preview and optionally execute a controlled command request using GREEN / YELLOW / RED / BLOCKED policy.",
        }
    }

    fn run_json(&self, ctx: &CToolContext, input: Value) -> CToolResult<Value> {
        let input: CToolCommandRequestInput = serde_json::from_value(input)?;
        let output = preview_command_request(ctx, input)?;
        Ok(serde_json::to_value(output)?)
    }
}

pub fn preview_command_request(
    ctx: &CToolContext,
    input: CToolCommandRequestInput,
) -> CToolResult<CToolCommandRequestOutput> {
    let request_reason = input.reason.clone();
    let command_config = load_merged_cool_command_config(
        &ctx.scope_context.character_command_path,
        ctx.scope_context.system_command_path.as_deref(),
    )?;

    let preview = build_command_request_preview(
        &ctx.scope_context.cool_workspace,
        input.commands,
        &command_config,
        input.ai_risk_upgrade,
    )?;
    let banner = render_command_request_banner(&preview);

    let mut will_execute = false;
    let mut executed = false;
    let blocked = preview.final_risk == CToolCommandRisk::Blocked;
    let mut rejected = false;
    let mut all_success = None;
    let mut result_file = None;
    let mut log_file = None;
    let mut user_feedback = None;
    let mut note = match preview.approval {
        CToolCommandApproval::AutoApprovedGreen => {
            "Green command request will auto-execute by user whitelist.".to_string()
        }
        CToolCommandApproval::ConfirmOnce => {
            "Yellow command request is waiting for one user confirmation.".to_string()
        }
        CToolCommandApproval::ConfirmTwice => {
            "Red command request is waiting for two user confirmations.".to_string()
        }
        CToolCommandApproval::Blocked => {
            "Blocked command request cannot be confirmed or executed.".to_string()
        }
    };

    if !blocked {
        let cache_root = command_request_cache_root(ctx);
        match preview.approval {
            CToolCommandApproval::AutoApprovedGreen => {
                will_execute = true;
                let report = execute_approved_command_request(
                    &ctx.scope_context.cool_workspace,
                    &cache_root,
                    &preview,
                )?;
                executed = true;
                all_success = Some(report.all_success);
                result_file = Some(report.result_file);
                log_file = Some(report.log_file);
                note = "Green command request auto-executed by user whitelist.".to_string();
            }
            CToolCommandApproval::ConfirmOnce => {
                if let Some(confirmation) = input.yellow_confirmation.as_deref() {
                    match parse_yellow_confirmation_input(confirmation) {
                        CToolCommandUserDecision::Approved => {
                            will_execute = true;
                            let report = execute_approved_command_request(
                                &ctx.scope_context.cool_workspace,
                                &cache_root,
                                &preview,
                            )?;
                            executed = true;
                            all_success = Some(report.all_success);
                            result_file = Some(report.result_file);
                            log_file = Some(report.log_file);
                            note = "Yellow command request executed after user confirmation."
                                .to_string();
                        }
                        CToolCommandUserDecision::Rejected { feedback } => {
                            rejected = true;
                            user_feedback = feedback;
                            note =
                                "Yellow command request was rejected by user confirmation input."
                                    .to_string();
                        }
                        CToolCommandUserDecision::NeedsSecondRedConfirmation => {}
                    }
                }
            }
            CToolCommandApproval::ConfirmTwice => {
                if let Some(first) = input.red_first_confirmation.as_deref() {
                    match parse_red_first_confirmation_input(first) {
                        CToolCommandUserDecision::Approved => {}
                        CToolCommandUserDecision::Rejected { feedback } => {
                            rejected = true;
                            user_feedback = feedback;
                            note = "Red command request was rejected at first confirmation."
                                .to_string();
                        }
                        CToolCommandUserDecision::NeedsSecondRedConfirmation => {
                            note = "Red command request passed first confirmation and is waiting for second confirmation.".to_string();
                            if let Some(second) = input.red_second_confirmation.as_deref() {
                                match parse_red_second_confirmation_input(second) {
                                    CToolCommandUserDecision::Approved => {
                                        will_execute = true;
                                        let report = execute_approved_command_request(
                                            &ctx.scope_context.cool_workspace,
                                            &cache_root,
                                            &preview,
                                        )?;
                                        executed = true;
                                        all_success = Some(report.all_success);
                                        result_file = Some(report.result_file);
                                        log_file = Some(report.log_file);
                                        note = "Red command request executed after two user confirmations.".to_string();
                                    }
                                    CToolCommandUserDecision::Rejected { feedback } => {
                                        rejected = true;
                                        user_feedback = feedback;
                                        note = "Red command request was rejected at second confirmation.".to_string();
                                    }
                                    CToolCommandUserDecision::NeedsSecondRedConfirmation => {}
                                }
                            }
                        }
                    }
                }
            }
            CToolCommandApproval::Blocked => {}
        }
    }

    if (blocked || rejected) && result_file.is_none() {
        let cache_root = command_request_cache_root(ctx);
        let status = if blocked {
            CToolCommandRequestRecordStatus::Blocked
        } else {
            CToolCommandRequestRecordStatus::Rejected
        };
        let report = record_unexecuted_command_request(
            &ctx.scope_context.cool_workspace,
            &cache_root,
            &preview,
            status,
            &note,
            user_feedback.as_deref(),
        )?;
        all_success = Some(report.all_success);
        result_file = Some(report.result_file);
        log_file = Some(report.log_file);
    }
    let commands = preview
        .commands
        .iter()
        .map(|command| CToolCommandRequestCommandOutput {
            command: command.command.clone(),
            risk: command.risk.label().to_string(),
            reason: command.reason.clone(),
        })
        .collect::<Vec<_>>();

    let display_text = render_command_request_display_text(
        &banner,
        executed,
        blocked,
        rejected,
        all_success,
        result_file.as_deref(),
        log_file.as_deref(),
        &note,
        user_feedback.as_deref(),
    );

    Ok(CToolCommandRequestOutput {
        will_execute,
        executed,
        blocked,
        rejected,
        all_success,
        result_file,
        log_file,
        current_dir: preview.current_dir.clone(),
        command_count: commands.len(),
        system_risk: preview.system_risk.label().to_string(),
        ai_risk_upgrade: preview.ai_risk_upgrade.map(|risk| risk.label().to_string()),
        final_risk: preview.final_risk.label().to_string(),
        approval_required: approval_label(preview.approval).to_string(),
        request_reason,
        user_feedback,
        commands,
        display_text,
        banner,
        note,
    })
}

fn approval_label(approval: CToolCommandApproval) -> &'static str {
    match approval {
        CToolCommandApproval::AutoApprovedGreen => "none_green_auto_approved",
        CToolCommandApproval::ConfirmOnce => "confirm_once",
        CToolCommandApproval::ConfirmTwice => "confirm_twice",
        CToolCommandApproval::Blocked => "blocked",
    }
}

fn command_request_cache_root(ctx: &CToolContext) -> PathBuf {
    ctx.scope_context
        .character_root
        .join(COOL_DIR_NAME)
        .join("cache")
        .join("command_request")
}

fn render_command_request_display_text(
    banner: &str,
    executed: bool,
    blocked: bool,
    rejected: bool,
    all_success: Option<bool>,
    result_file: Option<&str>,
    log_file: Option<&str>,
    note: &str,
    user_feedback: Option<&str>,
) -> String {
    let mut text = String::new();
    text.push_str(banner);
    text.push_str("\n\n");
    text.push_str("COMMAND REQUEST RESULT\n");
    text.push_str("==============================\n");
    text.push_str(&format!("executed: {executed}\n"));
    text.push_str(&format!("blocked: {blocked}\n"));
    text.push_str(&format!("rejected: {rejected}\n"));

    if let Some(all_success) = all_success {
        text.push_str(&format!("all_success: {all_success}\n"));
    }
    if let Some(result_file) = result_file {
        text.push_str("result_file: ");
        text.push_str(result_file);
        text.push('\n');
    }
    if let Some(log_file) = log_file {
        text.push_str("log_file: ");
        text.push_str(log_file);
        text.push('\n');
    }
    if let Some(user_feedback) = user_feedback {
        text.push_str("user_feedback: ");
        text.push_str(user_feedback);
        text.push('\n');
    }

    text.push_str("note: ");
    text.push_str(note);
    text.push('\n');
    text.push_str("==============================");
    text
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::path::PathBuf;

    use super::*;
    use crate::context::CToolContext;
    use crate::scope::CToolScopeBase;
    use crate::scope_config::COOL_DIR_NAME;
    use crate::scope_config::COOL_SYSTEM_DIR_NAME;
    use crate::scope_config::empty_scope_config;
    use crate::scope_context::CToolScopeContext;

    fn test_root(name: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!(
            "ctool_command_request_tool_{name}_{}",
            std::process::id()
        ));

        if path.exists() {
            std::fs::remove_dir_all(&path).unwrap();
        }

        std::fs::create_dir_all(&path).unwrap();
        path
    }

    fn write_text(path: &Path, text: &str) {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(path, text).unwrap();
    }

    fn test_context(
        name: &str,
        character_command_toml: &str,
        system_command_toml: &str,
    ) -> CToolContext {
        let root = test_root(name);
        let launcher_dir = root.join("launcher");
        let character_root = root.join("character");
        let cool_workspace = root.join("workspace");
        let character_cool_dir = character_root.join(COOL_DIR_NAME);
        let system_dir = launcher_dir.join(COOL_SYSTEM_DIR_NAME);

        std::fs::create_dir_all(&launcher_dir).unwrap();
        std::fs::create_dir_all(&character_cool_dir).unwrap();
        std::fs::create_dir_all(&system_dir).unwrap();
        std::fs::create_dir_all(&cool_workspace).unwrap();

        let character_command_path = character_cool_dir.join("command.toml");
        let system_command_path = system_dir.join("command.toml");

        write_text(&character_command_path, character_command_toml);
        write_text(&system_command_path, system_command_toml);

        CToolContext::new(CToolScopeContext {
            current_dir: character_root.clone(),
            character_root: character_root.clone(),
            cool_workspace: cool_workspace.clone(),
            base_scope: CToolScopeBase::CoolWorkspace,
            user_config_path: character_cool_dir.join("scope.toml"),
            character_config_path: character_cool_dir.join("config.toml"),
            system_config_path: Some(system_dir.join("config.toml")),
            character_command_path,
            system_command_path: Some(system_command_path),
            cool_system_dir: Some(system_dir),
            user_config: empty_scope_config(),
            system_config: empty_scope_config(),
        })
    }

    #[test]
    fn green_tool_output_executes_and_returns_result_paths() {
        let ctx = test_context(
            "green_output",
            r#"
[ctool_command]
policy = "red"
green_exact_commands = ["echo ctool-tool-output"]
"#,
            r#"
[ctool_command]
policy = "green"
"#,
        );

        let output = preview_command_request(
            &ctx,
            CToolCommandRequestInput {
                commands: vec!["echo ctool-tool-output".to_string()],
                ai_risk_upgrade: None,
                reason: Some("test green output".to_string()),
                yellow_confirmation: None,
                red_first_confirmation: None,
                red_second_confirmation: None,
            },
        )
        .unwrap();

        assert!(output.will_execute);
        assert!(output.executed);
        assert!(!output.blocked);
        assert!(!output.rejected);
        assert_eq!(output.all_success, Some(true));
        assert!(output.result_file.is_some());
        assert!(output.log_file.is_some());
        assert_eq!(output.current_dir, ctx.scope_context.cool_workspace.display().to_string());
        assert_eq!(output.command_count, 1);
        assert_eq!(output.system_risk, "GREEN");
        assert_eq!(output.final_risk, "GREEN");
        assert_eq!(output.approval_required, "none_green_auto_approved");
        assert_eq!(output.request_reason, Some("test green output".to_string()));
        assert_eq!(output.user_feedback, None);
        assert_eq!(output.commands.len(), 1);
        assert_eq!(output.commands[0].command, "echo ctool-tool-output");
        assert_eq!(output.commands[0].risk, "GREEN");
        assert!(output.display_text.contains("executed: true"));
        assert!(output.display_text.contains("result_file:"));
        assert!(output.display_text.contains("log_file:"));

        let result_text = std::fs::read_to_string(output.result_file.unwrap()).unwrap();
        assert!(result_text.contains("ctool-tool-output"));
    }

    #[test]
    fn yellow_tool_output_rejected_with_feedback_returns_result_paths() {
        let ctx = test_context(
            "yellow_rejected_output",
            r#"
[ctool_command]
policy = "red"
yellow_prefixes = ["demo-yellow"]
"#,
            r#"
[ctool_command]
policy = "green"
"#,
        );

        let output = preview_command_request(
            &ctx,
            CToolCommandRequestInput {
                commands: vec!["demo-yellow do something".to_string()],
                ai_risk_upgrade: None,
                reason: Some("test yellow rejection".to_string()),
                yellow_confirmation: Some("N keep editing first".to_string()),
                red_first_confirmation: None,
                red_second_confirmation: None,
            },
        )
        .unwrap();

        assert!(!output.will_execute);
        assert!(!output.executed);
        assert!(!output.blocked);
        assert!(output.rejected);
        assert_eq!(output.all_success, Some(false));
        assert!(output.result_file.is_some());
        assert!(output.log_file.is_some());
        assert_eq!(output.current_dir, ctx.scope_context.cool_workspace.display().to_string());
        assert_eq!(output.command_count, 1);
        assert_eq!(output.system_risk, "YELLOW");
        assert_eq!(output.final_risk, "YELLOW");
        assert_eq!(output.approval_required, "confirm_once");
        assert_eq!(output.request_reason, Some("test yellow rejection".to_string()));
        assert_eq!(output.user_feedback, Some("keep editing first".to_string()));
        assert_eq!(output.commands.len(), 1);
        assert_eq!(output.commands[0].command, "demo-yellow do something");
        assert_eq!(output.commands[0].risk, "YELLOW");
        assert!(output.display_text.contains("executed: false"));
        assert!(output.display_text.contains("rejected: true"));
        assert!(output.display_text.contains("user_feedback: keep editing first"));

        let result_text = std::fs::read_to_string(output.result_file.unwrap()).unwrap();
        assert!(result_text.contains("Status: Rejected"));
        assert!(result_text.contains("keep editing first"));
    }
}
