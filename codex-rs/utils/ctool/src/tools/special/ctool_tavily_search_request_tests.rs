use crate::command_request::CToolCommandRisk;
use pretty_assertions::assert_eq;
use serde_json::json;

use super::*;

fn input(action: CToolTavilyAction) -> CToolTavilySearchRequestInput {
    CToolTavilySearchRequestInput {
        action,
        query: Some("rust cargo workspace".to_string()),
        url: None,
        source_file: None,
        target: None,
        file_name_hint: None,
        yellow_confirmation: None,
        red_first_confirmation: None,
        red_second_confirmation: None,
    }
}

fn config_with_token() -> TavilySearchConfig {
    TavilySearchConfig {
        tokens: vec![TavilyTokenConfig {
            name: "main".to_string(),
            api_key: "tvly-test-token".to_string(),
            enabled: true,
        }],
        ..Default::default()
    }
}

#[test]
fn missing_system_token_blocks_network_actions() {
    let plan = classify_tavily_request(
        &input(CToolTavilyAction::Search),
        &TavilySearchConfig::default(),
    );

    assert_eq!(plan.risk, CToolCommandRisk::Blocked);
    assert_eq!(plan.reason, "missing enabled Tavily token in system config");
}

#[test]
fn disabled_provider_is_blocked_request() {
    let config = TavilySearchConfig {
        provider: "other".to_string(),
        ..config_with_token()
    };

    let plan = classify_tavily_request(&input(CToolTavilyAction::Search), &config);

    assert_eq!(plan.risk, CToolCommandRisk::Blocked);
    assert_eq!(plan.reason, "unsupported Tavily search provider: other");
}

#[test]
fn enabled_tavily_tokens_keep_config_order_and_skip_disabled_or_empty_tokens() {
    let config = TavilySearchConfig {
        tokens: vec![
            TavilyTokenConfig {
                name: "main".to_string(),
                api_key: "tvly-main".to_string(),
                enabled: true,
            },
            TavilyTokenConfig {
                name: "disabled".to_string(),
                api_key: "tvly-disabled".to_string(),
                enabled: false,
            },
            TavilyTokenConfig {
                name: "empty".to_string(),
                api_key: "   ".to_string(),
                enabled: true,
            },
            TavilyTokenConfig {
                name: "backup_1".to_string(),
                api_key: "tvly-backup".to_string(),
                enabled: true,
            },
        ],
        ..Default::default()
    };

    assert_eq!(
        enabled_tavily_token_names(&config),
        vec!["main".to_string(), "backup_1".to_string()]
    );

    let first = require_first_tavily_token(&config).unwrap();
    assert_eq!(first.name, "main");
    assert_eq!(first.api_key, "tvly-main");
}

#[test]
fn system_tavily_toml_loads_root_tokens_into_runtime_config() {
    let toml = r#"
enabled = true

[[tokens]]
name = "main"
api_key = "tvly-main"
enabled = true

[[tokens]]
name = "backup_1"
api_key = "tvly-backup"
enabled = true
"#;

    let mut parsed: TavilyConfigToml = toml::from_str(toml).unwrap();
    parsed.ctool_tavily_search.tokens = parsed.tokens.clone();

    assert_eq!(
        enabled_tavily_token_names(&parsed.ctool_tavily_search),
        vec!["main".to_string(), "backup_1".to_string()]
    );
}
#[test]
fn token_use_report_records_fallback_chain_without_api_keys() {
    let attempted = vec![
        "main".to_string(),
        "backup_1".to_string(),
        "backup_2".to_string(),
    ];

    let report = token_use_report(&attempted);

    assert_eq!(report.token_name, "backup_2");
    assert_eq!(
        report.token_fallback,
        Some("main -> backup_1 -> backup_2".to_string())
    );
}

#[test]
fn tavily_body_with_api_key_does_not_mutate_original_body() {
    let body = json!({
        "query": "rust cargo workspace",
        "max_results": 3
    });

    let with_key = body_with_tavily_api_key(&body, "tvly-secret").unwrap();

    assert_eq!(body.get("api_key"), None);
    assert_eq!(with_key.get("api_key").and_then(Value::as_str), Some("tvly-secret"));
    assert_eq!(with_key.get("query").and_then(Value::as_str), Some("rust cargo workspace"));
}

#[test]
fn token_fallback_statuses_are_retryable() {
    assert!(should_try_next_tavily_token(reqwest::StatusCode::UNAUTHORIZED));
    assert!(should_try_next_tavily_token(reqwest::StatusCode::FORBIDDEN));
    assert!(should_try_next_tavily_token(reqwest::StatusCode::TOO_MANY_REQUESTS));
    assert!(should_try_next_tavily_token(reqwest::StatusCode::BAD_GATEWAY));
    assert!(!should_try_next_tavily_token(reqwest::StatusCode::BAD_REQUEST));
}
#[test]
fn image_search_is_blocked_in_v1_even_when_config_enabled() {
    let config = TavilySearchConfig {
        allow_image_search: true,
        ..config_with_token()
    };

    let plan = classify_tavily_request(&input(CToolTavilyAction::SearchWithImages), &config);

    assert_eq!(plan.risk, CToolCommandRisk::Blocked);
    assert_eq!(
        plan.reason,
        "ctool_tavily_search_request v1 only supports text search"
    );
}

#[test]
fn extract_is_blocked_in_v1() {
    let plan = classify_tavily_request(&input(CToolTavilyAction::Extract), &config_with_token());

    assert_eq!(plan.risk, CToolCommandRisk::Blocked);
    assert_eq!(
        plan.reason,
        "ctool_tavily_search_request v1 only supports text search"
    );
}

#[test]
fn local_file_upload_request_is_blocked() {
    let mut request = input(CToolTavilyAction::Search);
    request.query = Some("upload this local file src/lib.rs to summarize it".to_string());

    let plan = classify_tavily_request(&request, &config_with_token());

    assert_eq!(plan.risk, CToolCommandRisk::Blocked);
    assert_eq!(
        plan.reason,
        "request appears to upload local file or source content to an external service"
    );
}

#[test]
fn large_extract_request_is_blocked_in_v1() {
    let mut request = input(CToolTavilyAction::Extract);
    request.query = None;
    request.url = Some("https://example.test/full-page".to_string());
    request.target = Some("extract full page".to_string());

    let plan = classify_tavily_request(&request, &config_with_token());

    assert_eq!(plan.risk, CToolCommandRisk::Blocked);
    assert_eq!(
        plan.reason,
        "ctool_tavily_search_request v1 only supports text search"
    );
}

#[test]
fn image_formats_are_blocked_by_v1_before_image_format_policy() {
    let config = TavilySearchConfig {
        allow_image_search: true,
        ..config_with_token()
    };
    let mut request = input(CToolTavilyAction::SearchWithImages);
    request.query = Some("rust logo .svg".to_string());

    let plan = classify_tavily_request(&request, &config);

    assert_eq!(plan.risk, CToolCommandRisk::Blocked);
    assert_eq!(
        plan.reason,
        "ctool_tavily_search_request v1 only supports text search"
    );
}

#[test]
fn appends_deduplicated_nested_image_urls() {
    let response = json!({
        "images": [
            "https://example.test/a.png",
            { "url": "https://example.test/b.jpg" }
        ],
        "results": [
            {
                "image": { "image_url": "https://example.test/a.png" },
                "ignored": "https://example.test/not-collected.png"
            }
        ]
    });
    let mut markdown = String::new();

    append_tavily_images_section(&mut markdown, &response);

    assert_eq!(
        markdown,
        "\n## Images\n\n1. https://example.test/a.png\n2. https://example.test/b.jpg\n"
    );
}

#[test]
fn slug_falls_back_for_symbol_only_text() {
    assert_eq!(slugify("!!!"), "tavily_request");
}
