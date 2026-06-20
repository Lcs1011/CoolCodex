use pretty_assertions::assert_eq;

use super::*;

#[test]
fn blocked_icon_uses_three_red_markers() {
    assert_eq!(CToolCommandRisk::Blocked.icon(), "🔴🔴🔴");
}

#[test]
fn red_second_confirmation_accepts_y_prefix() {
    assert_eq!(
        parse_red_second_confirmation_input("Y"),
        CToolCommandUserDecision::Approved
    );
    assert_eq!(
        parse_red_second_confirmation_input("yes run it"),
        CToolCommandUserDecision::Approved
    );
}
#[test]
fn yellow_confirmation_accepts_y_prefix() {
    assert_eq!(
        parse_yellow_confirmation_input("Y"),
        CToolCommandUserDecision::Approved
    );
    assert_eq!(
        parse_yellow_confirmation_input("yes run it"),
        CToolCommandUserDecision::Approved
    );
}

#[test]
fn yellow_confirmation_rejects_empty_input() {
    assert_eq!(
        parse_yellow_confirmation_input(""),
        CToolCommandUserDecision::Rejected { feedback: None }
    );
}

#[test]
fn yellow_confirmation_rejects_n_with_feedback() {
    assert_eq!(
        parse_yellow_confirmation_input("N only modify files first"),
        CToolCommandUserDecision::Rejected {
            feedback: Some("only modify files first".to_string()),
        }
    );
}

#[test]
fn yellow_confirmation_rejects_unknown_input_as_feedback() {
    assert_eq!(
        parse_yellow_confirmation_input("wait, use cargo check -p ctool instead"),
        CToolCommandUserDecision::Rejected {
            feedback: Some("wait, use cargo check -p ctool instead".to_string()),
        }
    );
}

#[test]
fn red_first_confirmation_requires_y_prefix() {
    assert_eq!(
        parse_red_first_confirmation_input("Y"),
        CToolCommandUserDecision::NeedsSecondRedConfirmation
    );
    assert_eq!(
        parse_red_first_confirmation_input("yes continue"),
        CToolCommandUserDecision::NeedsSecondRedConfirmation
    );
}

#[test]
fn red_first_confirmation_rejects_n_with_feedback() {
    assert_eq!(
        parse_red_first_confirmation_input("N do not run powershell"),
        CToolCommandUserDecision::Rejected {
            feedback: Some("do not run powershell".to_string()),
        }
    );
}

#[test]
fn red_second_confirmation_rejects_n_with_feedback() {
    assert_eq!(
        parse_red_second_confirmation_input("N too risky"),
        CToolCommandUserDecision::Rejected {
            feedback: Some("too risky".to_string()),
        }
    );
}

#[test]
fn red_second_confirmation_rejects_empty_input() {
    assert_eq!(
        parse_red_second_confirmation_input(""),
        CToolCommandUserDecision::Rejected { feedback: None }
    );
}
fn rule_matching_test_config() -> CToolCommandConfig {
    let mut config = default_command_config();
    config.policy = CToolCommandPolicy::Yellow;
    config
}
fn clean_command_config(policy: CToolCommandPolicy) -> CToolCommandConfig {
    let mut config = CToolCommandConfig::default();
    config.policy = policy;
    config.green_exact_commands.clear();
    config.green_prefixes.clear();
    config.yellow_prefixes.clear();
    config.red_prefixes.clear();
    config.red_contains.clear();
    config.blocked_prefixes.clear();
    config.blocked_contains.clear();
    config
}
fn test_cache_root(name: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!(
        "ctool_command_request_{name}_{}",
        std::process::id()
    ));

    if path.exists() {
        std::fs::remove_dir_all(&path).unwrap();
    }

    std::fs::create_dir_all(&path).unwrap();
    path
}

#[test]
fn default_command_config_blocks_all_commands() {
    let classification = classify_command("git status", &default_command_config());

    assert_eq!(
        classification,
        CToolCommandClassification {
            command: "git status".to_string(),
            risk: CToolCommandRisk::Blocked,
            reason: "policy is block-all, all commands are blocked".to_string(),
        }
    );
}

#[test]
fn python_environment_creation_is_blocked() {
    let classification = classify_command("python -m venv .venv", &rule_matching_test_config());

    assert_eq!(
        classification,
        CToolCommandClassification {
            command: "python -m venv .venv".to_string(),
            risk: CToolCommandRisk::Blocked,
            reason: "python -m venv .venv: matched blocked contains rule: venv".to_string(),
        }
    );
}

#[test]
fn ctool_scope_commands_are_blocked() {
    let classification = classify_command("/cs hidden .cool", &rule_matching_test_config());

    assert_eq!(
        classification,
        CToolCommandClassification {
            command: "/cs hidden .cool".to_string(),
            risk: CToolCommandRisk::Blocked,
            reason: "/cs hidden .cool: matched blocked contains rule: /cs".to_string(),
        }
    );
}

#[test]
fn ctool_scope_segment_in_mixed_command_is_blocked() {
    let classification =
        classify_command("git status && /cs hidden .cool", &rule_matching_test_config());

    assert_eq!(classification.risk, CToolCommandRisk::Blocked);
    assert!(
        classification
            .reason
            .contains("git status: matched yellow prefix rule: git status")
    );
    assert!(
        classification
            .reason
            .contains("/cs hidden .cool: matched blocked contains rule: /cs")
    );
}

#[test]
fn directory_switch_is_red() {
    let classification = classify_command("cd ..", &rule_matching_test_config());

    assert_eq!(
        classification,
        CToolCommandClassification {
            command: "cd ..".to_string(),
            risk: CToolCommandRisk::Red,
            reason: "cd ..: cd/pushd directory switch is at least red".to_string(),
        }
    );
}
#[test]
fn green_rule_cannot_downgrade_directory_switch() {
    let mut config = rule_matching_test_config();
    config.green_prefixes.push("cd".to_string());

    let classification = classify_command("cd ..", &config);

    assert_eq!(
        classification,
        CToolCommandClassification {
            command: "cd ..".to_string(),
            risk: CToolCommandRisk::Red,
            reason: "cd ..: matched green prefix rule: cd; cd/pushd directory switch is at least red".to_string(),
        }
    );
}
#[test]
fn green_rule_cannot_downgrade_python_environment_creation() {
    let mut config = rule_matching_test_config();
    config.green_prefixes.push("python -m venv".to_string());

    let classification = classify_command("python -m venv .venv", &config);

    assert_eq!(
        classification,
        CToolCommandClassification {
            command: "python -m venv .venv".to_string(),
            risk: CToolCommandRisk::Blocked,
            reason: "python -m venv .venv: matched blocked contains rule: venv".to_string(),
        }
    );
}

#[test]
fn green_rule_cannot_downgrade_shell_command() {
    let mut config = rule_matching_test_config();
    config.green_prefixes.push("powershell".to_string());

    let classification = classify_command("powershell -Command echo hi", &config);

    assert_eq!(
        classification,
        CToolCommandClassification {
            command: "powershell -Command echo hi".to_string(),
            risk: CToolCommandRisk::Red,
            reason: "powershell -Command echo hi: matched red prefix rule: powershell".to_string(),
        }
    );
}

#[test]
fn policy_green_cannot_downgrade_download_url() {
    let mut config = CToolCommandConfig::default();
    config.policy = CToolCommandPolicy::Green;

    let classification = classify_command("custom-tool https://example.com/file.zip", &config);

    assert_eq!(
        classification,
        CToolCommandClassification {
            command: "custom-tool https://example.com/file.zip".to_string(),
            risk: CToolCommandRisk::Red,
            reason: "custom-tool https://example.com/file.zip: shell, interpreter, download, website, deletion, process, registry, or network configuration command is at least red".to_string(),
        }
    );
}
#[test]
fn yellow_rule_overrides_green_rule() {
    let mut config = rule_matching_test_config();
    config.green_exact_commands.push("cargo check".to_string());

    let classification = classify_command("cargo check", &config);

    assert_eq!(
        classification,
        CToolCommandClassification {
            command: "cargo check".to_string(),
            risk: CToolCommandRisk::Yellow,
            reason: "cargo check: matched yellow prefix rule: cargo check".to_string(),
        }
    );
}

#[test]
fn unknown_command_uses_policy_fallback() {
    let mut config = CToolCommandConfig::default();
    config.policy = CToolCommandPolicy::Green;

    let classification = classify_command("custom-tool --version", &config);

    assert_eq!(
        classification,
        CToolCommandClassification {
            command: "custom-tool --version".to_string(),
            risk: CToolCommandRisk::Green,
            reason: "custom-tool --version: unknown command, defaulting to policy Green".to_string(),
        }
    );
}

#[test]
fn merged_policy_uses_more_restrictive_fallback() {
    let character_config = clean_command_config(CToolCommandPolicy::Green);
    let system_config = clean_command_config(CToolCommandPolicy::Red);

    let merged = merge_command_configs(character_config, system_config);
    let classification = classify_command("unknown-tool --version", &merged);

    assert_eq!(
        classification,
        CToolCommandClassification {
            command: "unknown-tool --version".to_string(),
            risk: CToolCommandRisk::Red,
            reason: "unknown-tool --version: unknown command, defaulting to policy Red".to_string(),
        }
    );
}

#[test]
fn merged_blocked_rule_overrides_red_yellow_green_rules() {
    let mut character_config = clean_command_config(CToolCommandPolicy::Green);
    let mut system_config = clean_command_config(CToolCommandPolicy::Green);

    character_config.green_exact_commands.push("demo command".to_string());
    character_config.yellow_prefixes.push("demo".to_string());
    system_config.red_prefixes.push("demo".to_string());
    character_config.blocked_prefixes.push("demo".to_string());

    let merged = merge_command_configs(character_config, system_config);
    let classification = classify_command("demo command", &merged);

    assert_eq!(
        classification,
        CToolCommandClassification {
            command: "demo command".to_string(),
            risk: CToolCommandRisk::Blocked,
            reason: "demo command: matched blocked prefix rule: demo".to_string(),
        }
    );
}

#[test]
fn merged_red_rule_overrides_yellow_and_green_rules() {
    let mut character_config = clean_command_config(CToolCommandPolicy::Green);
    let mut system_config = clean_command_config(CToolCommandPolicy::Green);

    character_config.green_exact_commands.push("demo command".to_string());
    character_config.yellow_prefixes.push("demo".to_string());
    system_config.red_prefixes.push("demo".to_string());

    let merged = merge_command_configs(character_config, system_config);
    let classification = classify_command("demo command", &merged);

    assert_eq!(
        classification,
        CToolCommandClassification {
            command: "demo command".to_string(),
            risk: CToolCommandRisk::Red,
            reason: "demo command: matched red prefix rule: demo".to_string(),
        }
    );
}

#[test]
fn merged_yellow_rule_overrides_green_rule() {
    let mut character_config = clean_command_config(CToolCommandPolicy::Green);
    let mut system_config = clean_command_config(CToolCommandPolicy::Green);

    character_config.green_exact_commands.push("demo command".to_string());
    system_config.yellow_prefixes.push("demo".to_string());

    let merged = merge_command_configs(character_config, system_config);
    let classification = classify_command("demo command", &merged);

    assert_eq!(
        classification,
        CToolCommandClassification {
            command: "demo command".to_string(),
            risk: CToolCommandRisk::Yellow,
            reason: "demo command: matched yellow prefix rule: demo".to_string(),
        }
    );
}

#[test]
fn explicit_green_rule_overrides_policy_fallback() {
    let mut character_config = clean_command_config(CToolCommandPolicy::Red);
    let system_config = clean_command_config(CToolCommandPolicy::Green);

    character_config.green_exact_commands.push("demo command".to_string());

    let merged = merge_command_configs(character_config, system_config);
    let classification = classify_command("demo command", &merged);

    assert_eq!(
        classification,
        CToolCommandClassification {
            command: "demo command".to_string(),
            risk: CToolCommandRisk::Green,
            reason: "demo command: matched green exact rule: demo command".to_string(),
        }
    );
}
#[test]
fn yellow_banner_uses_confirmation_prompt() {
    let preview = build_command_request_preview(
        "C:\\CodexLab\\codex",
        vec!["cargo check -p ctool".to_string()],
        &rule_matching_test_config(),
        /*ai_risk_upgrade*/ None,
    )
    .unwrap();

    assert_eq!(preview.final_risk, CToolCommandRisk::Yellow);
    assert!(
        render_command_request_banner(&preview).contains("Confirm? Type Y to run, N to reject:")
    );
}

#[test]
fn blocked_request_record_writes_result_and_log() {
    let cache_root = test_cache_root("blocked_record");
    let current_dir = "C:\\CodexLab\\codex";
    let preview = build_command_request_preview(
        current_dir,
        vec!["python -m venv .venv".to_string()],
        &default_command_config(),
        /*ai_risk_upgrade*/ None,
    )
    .unwrap();

    let report = record_unexecuted_command_request(
        current_dir,
        &cache_root,
        &preview,
        CToolCommandRequestRecordStatus::Blocked,
        "Blocked by command policy.",
        None,
    )
    .unwrap();

    assert!(!report.executed);
    assert!(!report.all_success);
    assert!(report.commands.is_empty());

    let result_text = std::fs::read_to_string(&report.result_file).unwrap();
    let log_text = std::fs::read_to_string(&report.log_file).unwrap();

    assert!(result_text.contains("Approved: No"));
    assert!(result_text.contains("Status: Blocked"));
    assert!(result_text.contains("Blocked by command policy."));
    assert!(result_text.contains("python -m venv .venv"));

    assert!(log_text.contains("Approved: No"));
    assert!(log_text.contains("Status: Blocked"));
    assert!(log_text.contains("Output:"));
    assert!(log_text.contains(&report.result_file));
}

#[test]
fn rejected_request_record_writes_user_feedback() {
    let cache_root = test_cache_root("rejected_record");
    let current_dir = "C:\\CodexLab\\codex";
    let preview = build_command_request_preview(
        current_dir,
        vec!["cargo check -p ctool".to_string()],
        &rule_matching_test_config(),
        /*ai_risk_upgrade*/ None,
    )
    .unwrap();

    let report = record_unexecuted_command_request(
        current_dir,
        &cache_root,
        &preview,
        CToolCommandRequestRecordStatus::Rejected,
        "Rejected by user.",
        Some("只修改文件，暂时不要跑测试"),
    )
    .unwrap();

    assert!(!report.executed);
    assert!(!report.all_success);
    assert!(report.commands.is_empty());

    let result_text = std::fs::read_to_string(&report.result_file).unwrap();
    let log_text = std::fs::read_to_string(&report.log_file).unwrap();

    assert!(result_text.contains("Approved: No"));
    assert!(result_text.contains("Status: Rejected"));
    assert!(result_text.contains("Rejected by user."));
    assert!(result_text.contains("## User Feedback"));
    assert!(result_text.contains("只修改文件，暂时不要跑测试"));

    assert!(log_text.contains("Approved: No"));
    assert!(log_text.contains("Status: Rejected"));
    assert!(log_text.contains("User Feedback:"));
    assert!(log_text.contains("只修改文件，暂时不要跑测试"));
}
#[test]
fn approved_green_request_executes_and_writes_result_and_log() {
    let current_dir = test_cache_root("green_execute_workspace");
    let cache_root = test_cache_root("green_execute_cache");
    let mut config = clean_command_config(CToolCommandPolicy::Red);
    config
        .green_exact_commands
        .push("echo ctool-green-execute".to_string());

    let preview = build_command_request_preview(
        &current_dir,
        vec!["echo ctool-green-execute".to_string()],
        &config,
        /*ai_risk_upgrade*/ None,
    )
    .unwrap();

    assert_eq!(preview.final_risk, CToolCommandRisk::Green);
    assert_eq!(preview.approval, CToolCommandApproval::AutoApprovedGreen);

    let report = execute_approved_command_request(&current_dir, &cache_root, &preview).unwrap();

    assert!(report.executed);
    assert!(report.all_success);
    assert_eq!(report.commands.len(), 1);
    assert!(report.commands[0].success);
    assert_eq!(report.commands[0].exit_code, Some(0));
    assert!(
        report.commands[0]
            .stdout_preview
            .contains("ctool-green-execute")
    );

    let result_text = std::fs::read_to_string(&report.result_file).unwrap();
    let log_text = std::fs::read_to_string(&report.log_file).unwrap();

    assert!(result_text.contains("Approved: Yes"));
    assert!(result_text.contains("Risk: GREEN"));
    assert!(result_text.contains("echo ctool-green-execute"));
    assert!(result_text.contains("Success: true"));
    assert!(result_text.contains("ctool-green-execute"));

    assert!(log_text.contains("Approved: Yes"));
    assert!(log_text.contains("AllSuccess: true"));
    assert!(log_text.contains("echo ctool-green-execute"));
    assert!(log_text.contains("Output:"));
    assert!(log_text.contains(&report.result_file));
}
#[test]
fn blocked_banner_is_visible() {
    let preview = build_command_request_preview(
        "C:\\CodexLab\\codex",
        vec!["python -m venv .venv".to_string()],
        &default_command_config(),
        /*ai_risk_upgrade*/ None,
    )
    .unwrap();

    assert_eq!(preview.final_risk, CToolCommandRisk::Blocked);
    assert!(render_command_request_banner(&preview).contains("🔴🔴🔴 COMMAND REQUEST: BLOCKED"));
}
