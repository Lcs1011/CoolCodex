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
fn python_environment_creation_is_blocked() {
    let classification = classify_command("python -m venv .venv", &default_command_config());

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
    let classification = classify_command("/cs hidden .cool", &default_command_config());

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
        classify_command("git status && /cs hidden .cool", &default_command_config());

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
    let classification = classify_command("cd ..", &default_command_config());

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
fn yellow_banner_uses_confirmation_prompt() {
    let preview = build_command_request_preview(
        "C:\\CodexLab\\codex",
        vec!["cargo check -p ctool".to_string()],
        &default_command_config(),
        /*ai_risk_upgrade*/ None,
    )
    .unwrap();

    assert_eq!(preview.final_risk, CToolCommandRisk::Yellow);
    assert!(
        render_command_request_banner(&preview).contains("Confirm? Type Y to run, N to reject:")
    );
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
